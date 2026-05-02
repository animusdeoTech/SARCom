---
title: "ADR-010: SOS encoding — single band, flag bit, jittered cadence"
status: accepted
date: 2026-04-24
type: adr
tags: [decision, protocol, sos, lora, duty-cycle, etsi]
---

# ADR-010: SOS encoding — single band, flag bit, jittered cadence

**Status:** Accepted
**Date:** 2026-04-24

## Context

A tag has two possible behaviours:

- **Normal beacon.** `POSITION` packets every 300 s (configurable).
- **SOS beacon.** Same `POSITION` payload with a distress indicator, emitted at a shorter cadence for as long as the operator holds the distress state.

Two questions this ADR closes:

1. On what channel does an SOS transmit?
2. How is the SOS state encoded in the packet?

An earlier sketch proposed that SOS transmissions would move to ETSI EN 300 220 sub-band **P** (869.4–869.65 MHz, +27 dBm ERP, 10% duty cycle) to buy link-budget headroom over the default sub-band **M** (868.0–868.6 MHz, +14 dBm ERP, 1% duty). That sketch was never accepted. When it was reviewed against the relay's scan-cycle implementation, a structural bug appeared:

**The phase-lock bug.** If the tag emits SOS on band P at a fixed cadence (say every 30 s), and the relay's P-band listening window is a short slice inside a longer RX rotation across M and P (say 100 ms of P every 2 s), then whenever the SOS cadence and the relay scan cycle share a common period — or the scan cycle cleanly divides the cadence — SOS transmissions can land in the same relay-listening phase forever. A tag in distress can transmit correctly for hours while the relay never hears a single frame. This is not a probabilistic loss that averages out; it is a periodic alias that the firmware cannot detect from either side.

The fix has to kill at least one of the three ingredients: (a) fixed SOS cadence, (b) fixed scan cycle, (c) scan cycle separate from the SOS band.

## Decision

**SOS uses the same band as normal beacons. The distress state is encoded as a flag in the packet header. The SOS cadence is jittered.**

Concrete rules:

1. **One band for tag uplink.** All tag transmissions — normal and SOS — use the default 868 MHz sub-band M LoRa channel the relay is already listening on. No band switching for distress. The relay's RX stays in a single continuous listen on that channel; no scan rotation is introduced on the tag side.

2. **SOS flag in the packet header.** The `POSITION` payload carries a `flags` byte. **Bit 1** (mask `0x02`) is `SOS`; bit 0 is `GPS_VALID`, bit 2 is `BATT_LOW`, bits 3–7 reserved. Tag firmware sets `flags.SOS` when the distress state is active and clears it otherwise. Gateway classifies a sighting as distress iff this bit is set in at least one packet for that tag during the last `DISTRESS_WINDOW` seconds (exact value in ARCHITECTURE.md §7 / §11). The full flag layout is authoritative in [ARCHITECTURE.md §7](../ARCHITECTURE.md); this ADR follows it.

3. **Immediate first SOS frame, then positive-only-jittered cadence.** The first frame after distress entry transmits **immediately** on the SOS button press, with `GPS_VALID=0` + sentinels and `flags.SOS=1`, no GNSS acquisition step, no jitter. Distress must not be gated by GPS — a 90-second wait between button press and first distress frame would be a usability and safety bug. After that first frame, subsequent SOS transmit intervals are drawn uniformly from **[45 s, 60 s]** per transmission (positive-only jitter; minimum interval bounded at 45 s). Mean interval ≈ 52.5 s. Each subsequent SOS frame goes through the standard `GPS_ACQUIRE` → `TRANSMIT` path; on GNSS timeout the tag still transmits with sentinels.

4. **Duty-cycle budget.** At +14 dBm ERP on sub-band M with 1% duty cycle, a tag emitting a 22-byte `POSITION` frame at SF10 (~371 ms airtime) is permitted at most ~97 TX/hour, i.e. one frame every ~37.1 s averaged over the hour. A 30 s nominal SOS cadence — the v8 draft of this ADR — would be **outside** that envelope: 120 TX/hour × 371 ms = 44.52 s/hour ≈ 1.24% duty cycle. The corrected formulation is **minimum interval 45 s with positive-only jitter to 60 s**, giving max 80 TX/hour × 371 ms = 29.7 s/hour ≈ 0.82% duty cycle on sub-band M (1% legal cap). The tag's MAC layer must still enforce the hourly budget and defer if a burst of retries would exceed it; the immediate-first-frame rule (point 3) consumes one TX from the hourly budget at distress entry, so the second SOS frame's randomised wait must not begin below 45 s after the first.

5. **Link budget note.** Staying on sub-band M means SOS transmissions are capped at +14 dBm ERP. This is a real loss relative to what sub-band P would allow (+27 dBm ERP, ~13 dB better link budget). We accept the loss because:
   - A dead radio link is worse than a reduced one. The phase-lock bug is a dead link failure mode.
   - The Heltec Wireless Tracker V2 is `+28 dBm` capable but must be kept below the ERP ceiling of whatever sub-band it transmits on anyway; the board's headline power figure is already de-rated by the regulatory limit.
   - SOS is a short-lived mode. A few dB of extra path loss on rare events is an acceptable trade for a reliably received event.
   - The practical link budget on mountain terrain is dominated by terrain shadowing, not the last dozen dB of TX power.

## Consequences

- **The `protocol` crate gains a `flags: u8` field on `POSITION`.** Layout per [ARCHITECTURE.md §7](../ARCHITECTURE.md): bit 0 = `GPS_VALID`, **bit 1 = `SOS`**, bit 2 = `BATT_LOW`, bits 3–7 reserved. Reserved bits must be zero on transmit and must be preserved by the relay on forward.
- **Gateway distress rendering** reads `flags & 0x02` per sighting (bit 1), not a packet type or band.
- **Relay RX simplifies.** Single-channel continuous RX on sub-band M (868.1 MHz). No scan rotation on the LoRa path at all. The relay parks on the channel and stays there. (This does not close the door on the relay reserving future channels for v2+ mesh work — it closes the door on SOS using a separate channel in v1.)
- **Tag firmware owns the jitter and the immediate-first-frame rule.** The SOS state-machine in tag firmware must (a) emit the first frame on distress entry without going through `GPS_ACQUIRE` and without applying jitter, and (b) draw a fresh randomised delay before each subsequent SOS await — either an Embassy timer whose delay is drawn freshly before each await, or a pseudo-RNG-seeded delay. A constant-cadence SOS is a **bug**. A first-frame delay larger than firmware-only path latency is a **bug**.
- **Unit tests required in the `tag` crate.** Three correctness invariants, each its own test:
  1. **Constant-cadence regression.** A non-jittered SOS schedule must fail this test. Guards against future refactors silently removing the jitter.
  2. **Immediate-first-frame.** Wall-clock time from simulated SOS button press to first TX must be bounded by GNSS-independent firmware delay only — no `GPS_ACQUIRE` call on the entry path.
  3. **Duty-cycle cap.** Over a simulated hour at minimum-45 s SOS intervals plus the immediate-first-frame, total TX airtime must not exceed the sub-band M 1% budget.
- **P-band as a v2 path stays open.** If post-v1 field measurements show that distress range is insufficient at +14 dBm ERP, a future ADR may add sub-band P as an SOS-only channel. That ADR would need to solve (or side-step) the phase-lock problem — most likely by guaranteeing jittered SOS cadence and extending the relay's RX strategy so that the P-band listening window itself is jittered. Not v1.

## Alternatives considered

- **Dual-band SOS on P with fixed cadence.** The original sketch. Rejected on the phase-lock bug.
- **Dual-band SOS on P with heavy jitter on both sides (tag cadence + relay scan).** Theoretically sound. Rejected for v1: doubles the complexity of the relay RX state machine for a power-budget win on a rare event, and introduces two jitter distributions whose statistical interaction must be analysed to prove reception. Out of scope for a single-pole garden PoC.
- **Separate `SOS` packet type (new `PacketType::SOS`).** Rejected. A separate type means relay firmware has to parse the type byte before deciding how to forward — but the relay's forwarding policy should be type-agnostic for unknown `TYPE`/`VER` frames (see [ARCHITECTURE.md §12](../ARCHITECTURE.md) relay queue policy). A flag bit inside a known packet is simpler and forward-compatible: an older relay forwards an SOS frame identically to a normal one, and only the gateway interprets the distinction.
- **Time-based SOS escalation (cadence increases over time while distress is active).** Interesting, but raises the duty-cycle question and the jitter analysis. Deferred.
- **Multi-packet "SOS burst" with a sequence of frames per SOS event.** Useful for robustness but collides with the duty-cycle budget on sub-band M. Deferred. A single jittered frame is sufficient for v1 acceptance (dot turns red on the kiosk when SOS bit is set).

## Order checklist

No hardware implication. This ADR is protocol-only.

## References

- [ETSI EN 300 220-2 V3.2.1](https://www.etsi.org/deliver/etsi_en/300200_300299/30022002/) — short-range devices, sub-GHz ISM bands, sub-band definitions and duty/ERP limits.
- [ADR-001](ADR-001-firmware-language.md) — shared `protocol` crate.
- [ADR-008](ADR-008-no-cloud-no-downlink.md) — no downlink means the tag cannot be commanded back into normal cadence; SOS ends when the tag-side operator clears it.
