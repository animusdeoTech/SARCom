---
title: "Spike ticket — Gateway RX bring-up on Pi 5 + Dragino LoRa/GPS HAT"
status: open
type: spike-ticket
timebox: TBD (decide when scoped)
opened: 2026-05-05
updated: 2026-05-06 (handheld pivot)
---

# Spike ticket: gateway RX bring-up on Pi 5 + Dragino LoRa/GPS HAT

## PIVOT NOTE 2026-05-06

The 2026-05-06 architecture pivot makes the gateway **handheld portable battery-powered with a 5" display + custom 3D-printed waterproof shell**. That changes the substrate question (now load-bearing on Pi 5, not "Pi 3B+ or 4 whichever has healthy ports"), the antenna question (must route through a sealed shell), and the enclosure question (the Pi + HAT + display + battery must physically coexist inside the handheld shell).

**Sequencing under the pivot:**
- This spike is now **downstream of `spikes/gateway-handheld-substrate-spike.md`**. The substrate spike picks Pi 5 vs alternatives, antenna paths, and the HAT compatibility verdict. This spike inherits the chosen substrate and proves byte-correct RX.
- B10 ("Kiwi cart") is **moot**: per memory correction 2026-05-06 the cart was never placed; the substrate spike re-opens the procurement question against the new handheld envelope.
- B11 ("WiFi+cloud architectural pivot") is **resolved**: the handheld pivot is now the architecture. There is no "should we even open this ticket" question — this spike is the first step toward v0 once the substrate spike closes. ADR-008 amendment work is the `tak-cot-integration-spike.md` thread, not this one.
- The polled-RX-on-RP1 fallback (per dev-log 2026-05-05) survives the pivot unchanged.

The rest of this ticket below is the pre-pivot 2026-05-05 problem description. The hardware-rev questions (silkscreen revs on the 3 on-hand Dragino HATs, bent-pin damage check, antenna for bench testing) and the stack questions (lora-phy on Linux for SX1276, rpi-pal SPI clamp, RST line wiring) are unchanged by the pivot and remain the substantive content.

---

> This ticket is a **problem description**, not a plan. It captures what we want to know, what is already known, what is unknown, and what could go wrong. The actual bring-up methodology is a separate artifact and should not be written until the open questions below are at least surveyed.

## Why this ticket exists

Before any production gateway code commits to a stack, we need to know with concrete certainty that **a Raspberry Pi 5 + a Dragino LoRa/GPS HAT (SX1276 + L80 GNSS) + a Rust binary using `lora-phy` + `linux-embedded-hal` + `rpi-pal` can reliably receive LoRa packets on 868.1 MHz / SF10 / BW 125 kHz / sync 0x12 from a Heltec Wireless Tracker V2 transmitter, decode them, and pass the bytes to the existing `protocol` crate.**

If any layer in that stack does not work as documented, we want to discover it now — when the hardware just arrived and we have time and attention — not in week three of gateway code authorship when we are committed to architectural assumptions that are silently wrong.

This is the single largest unknown on the gateway critical path. Resolving it unblocks: gateway crate skeleton, kiosk read-model, persistence integration, and ultimately the v0 acceptance gate (gateway prints a parsed POSITION received from a relay).

## Problem statement

We need to confidently answer:

> "Given the hardware Pieter actually owns or will own (Pi 5 2 GB host + Dragino LoRa/GPS HAT rev TBD + Heltec Tracker V2 transmitter + 868 MHz antenna), and the Rust crate stack the project is committed to (`lora-phy` from `lora-rs/lora-rs`, `linux-embedded-hal`, `rpi-pal` for GPIO), is there a path that delivers a byte-correct LoRa frame from a Tracker V2 transmitter to the Pi gateway's user-space Rust binary at a packet rate of ≥ 1 frame / 5 min, sustained over multiple hours, without falling over?"

We are NOT asking yet:
- How to integrate this with the persistence crate
- How to bridge this to the kiosk UI
- How to scale to multiple simultaneous transmitters
- How to handle interrupt-driven RX (polled is the v1a fallback per dev log 2026-05-05)

Those are downstream questions that only matter after the answer to the question above is "yes."

## What we already know (from research 2026-05-05)

Captured in `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`. Summary:

- **`rppal` archived 2025-07-01.** Maintained successor is `rpi-pal` (drop-in fork, Pi 5 compatible). Doc swap already applied across CLAUDE.md, ARCHITECTURE.md §X.7, ADR-004, docs/claude-code-setup.md.
- **Pi 5's RP1 chip routes GPIO/SPI/UART through PCIe.** Historical note: this differs from earlier Pi-class direct-mapped peripherals — GPIO-chip mapping (`gpiochip0`) and UART config (`enable_uart=1`) do not carry over. Technical detail in [`../dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](../dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md).
- **The Dragino LoRa/GPS HAT is officially documented as Pi 2 / Pi 3 compatible only.** Pi 5 is undocumented by Dragino, but the 40-pin header + standard SPI + GPIO + UART make it electrically compatible. We are the canary on Pi 5 + this HAT.
- **GPIO interrupt mode on Pi 5 is reportedly flaky** for SX1262 LoRa HATs (RadioLib issue #1200). Polled RX is the documented fallback.
- **Some old Dragino LoRa/GPS HAT revisions have an SPI CS routing defect** — chip-select wired to physical pin 22 (BCM GPIO 25) instead of pin 24 (CE0). Affected boards need software CS via GPIO 25.
- **The `protocol` crate parser is already validated** by 22 unit tests against frozen canonical vectors (heartbeat, SOS) per TODO.md. Bytes-to-struct decoding is not in scope of this spike — only bytes-from-radio-to-program-buffer is.

## What we think we'll need to do, in broad strokes

This is the **shape** of work, not the steps. Each line below is itself another smaller question that the spike will answer.

- Verify the silkscreen revision on each of the 3× Dragino HATs Pieter owns. Knowing whether they have the GPIO 25 CS defect determines our SPI CS strategy.
- Establish that the Pi 5 boots cleanly with the right config.txt + cmdline.txt for SPI, UART (L80), and a removed serial console.
- Establish that an SX1276 chip on the HAT is actually addressable over SPI from user-space Rust. ("Hello, I see your version register" level.)
- Establish that `lora-phy` on Linux + `linux-embedded-hal` + `rpi-pal` actually compiles, links, and runs on a Pi 5 — and that the API for SX1276 in polled mode is what we expect from reading the docs.
- Establish that a Heltec Tracker V2 can transmit a sentinel frame on our chosen modulation parameters (868.1 MHz / SF10 / BW 125 kHz / CR 4/5 / sync 0x12) such that another Tracker V2 receives it. (Removes the Pi side from the equation when isolating early failures.)
- Establish that the same sentinel transmission is received correctly by the Pi-side gateway path with byte-identical payload.
- Decide what ongoing reliability evidence we need (5-minute test? 1-hour soak? 12-hour overnight?).

Concrete test methodology, hardware setup steps, and code skeletons are explicitly **not** in this ticket. Those belong in a follow-up doc once the open questions below are surveyed.

## Open questions

### Hardware-rev questions (Pieter, eyes-on-board needed)

1. Which silkscreen revision are Pieter's 3× Dragino LoRa/GPS HATs? Are any affected by the GPIO 25 CS defect?
2. Are the bent-pin HATs (referenced in TODO.md / ADR-004) the same HATs we plan to use here, or different units? Do the bends affect any pin we care about (SPI MOSI/MISO/SCK/CS, DIO0, UART)?
3. What antenna will we use for the gateway during bench testing — order separately, or use one of the antennas planned for the relay Solar Kit? (The bench test does not need the exact production antenna; even a poor whip is fine for short-range desk-test.)

### Stack questions (need the hardware in hand or a documented reference)

4. Does `lora-phy` (current main from `lora-rs/lora-rs`) actually have a polished API path for SX1276 + Linux + polled RX? The crate's documented examples lean heavily on Embassy / esp-hal MCU usage. Is the Linux gateway path equally maintained, or is it under-tested?
5. Is there an existing public Rust binary out there that runs `lora-phy` against an SX1276 on a Linux host (any Pi model) we can learn from? (Worth checking before we write it cold.)
6. Does `rpi-pal` expose the SPI clock-rate clamp we need? SX1276 max SPI is 10 MHz; Pi 5 may try higher by default.
7. How do we drive the SX1276 reset line (RST) from `lora-phy`? It expects a `embedded-hal::OutputPin` — does that wire cleanly through `rpi-pal`?

### L80 GPS / time-source questions (adjacent — affects whether the gateway can apply timestamps to received packets)

8. On Pi 5, does the L80 NMEA stream actually appear on `/dev/ttyAMA0` after `dtparam=uart0` + cmdline cleanup, or is there a Pi 5 specific UART quirk we'll discover only on first boot?
9. Does the L80 PPS pin get exposed on the HAT in a way `chrony` / `gpsd` can consume on Pi 5? (PPS is critical for sub-second time discipline per ADR-011, but **not** required to receive LoRa packets — so this question is parallel, not blocking.)
10. If gpsd does not play nice with Pi 5 + L80, do we accept "RTC + manual time set" for v1a and defer PPS to later? (Probably yes per ADR-011's "opportunistic" framing — but worth deciding explicitly.)

### Methodology questions (affect the spike's own design)

11. What counts as "this works" — minutes of clean RX? Hours? A specific number of frames received without loss? Pieter's call.
12. Which Tracker V2 sketch do we use as the test transmitter — write a sentinel TX from scratch, or pull a known-working LoRa TX example from `lora-rs/lora-rs` examples?
13. Do we run the spike on Raspberry Pi OS (faster bring-up, well-documented) or jump straight to Yocto (matches the production target per ADR-004 but adds a meta-rust layer of risk)?

## Potential blockers

Each blocker below has a likelihood guess and a fallback. Likelihoods are gut-feel, not measured.

### Hardware

- **B1. All 3 HATs have the GPIO 25 CS defect.** *Likelihood: medium.* All three units are old enough to plausibly be affected revisions. **Fallback:** software CS via GPIO 25 in our Rust binary. `lora-phy` accepts an explicit `OutputPin` for CS — not a code rewrite, just an extra parameter.
- **B2. Bent pins on the HATs damaged a critical signal.** *Likelihood: low.* TODO.md mentions bent pins; we don't yet know which pins. **Fallback:** straighten or reflow; if a HAT is unusable Pieter still has 2 more.
- **B3. Pi 5 + Dragino HAT have undocumented physical incompatibility.** *Likelihood: low.* The 40-pin header is electrically unchanged on Pi 5. But we have no public reports of *anyone* running this HAT on Pi 5 — we'd be the canary. **Fallback:** re-open [`gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md) to consider CM5 or Zero 2W as alternative substrates; Pi 4 is retired per [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md) and is NOT a fallback option.

### Stack

- **B4. `lora-phy` on Linux + SX1276 + polled mode is rough or buggy.** *Likelihood: medium.* The crate's documentation and examples are MCU-focused. We may have to file issues, write workarounds, or contribute upstream patches. **Fallback:** drop to a thinner SX1276 crate (e.g. `sx127x_lora`) for v0 just to prove the path; revisit `lora-phy` adoption later. This is undesirable because the protocol crate's tests assume `lora-phy`-shaped types — but it is recoverable.
- **B5. `rpi-pal` has a Pi 5 quirk we hit.** *Likelihood: low-medium.* The fork is young. **Fallback:** patch upstream (small surface) or work around with `lgpio` directly via FFI. Not a v1 blocker.
- **B6. Pi 5 SPI clock setup conflicts with SX1276 max.** *Likelihood: low.* Standard config issue, well understood. **Fallback:** clamp SPI clock in `rpi-pal` config.

### Ecosystem / process

- **B7. No transmitter is ready when the gateway hardware arrives.** *Likelihood: medium.* Heltec ETA is 2-4 weeks; gateway hardware (if Kiwi cart is placed) arrives in 3-5 days. We could have the Pi sitting on the desk with nothing to test it against. **Fallback:** delay the spike start until at least one Tracker V2 is in hand and flashable; or start with the SPI-version-register check (Layer 2 of the prior planned methodology), which doesn't need a transmitter.
- **B8. Pi 5 thermal throttling masquerades as a logic bug.** *Likelihood: low under bench test, high under sustained load.* Pi 5 active cooler not yet ordered. **Fallback:** order the cooler before any soak test; for short bring-up tests passive cooling is fine.
- **B9. No public reference implementation exists.** *Likelihood: high.* We have not found a working "Pi 5 + Dragino HAT + Rust + lora-phy" example anywhere. We are writing this fresh. **Consequence (not a fallback):** estimate generously. False starts on lora-phy's API are likely. This is also where the OSS contribution opportunity sits — see `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md` "Open source contribution opportunities" section.

### Project-shape

- **B10. The Pi 5 procurement decision has not been placed yet (as of 2026-05-07).** *Likelihood: blocker if not addressed.* This spike cannot start without the gateway substrate hardware. Either the order goes in or we re-scope to use a Pi-5-class alternative (e.g. Pi Zero 2W as the only on-hand-compatible alternative; full ranking owned by [`gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md)). Pi 4 is retired and is not a substitute path — see [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).
- **B11. Architectural pivot to WiFi+cloud floated by Pieter on 2026-05-05.** **RESOLVED 2026-05-06.** The pivot landed as "local-first handheld gateway with opportunistic base-mode export" — not WiFi+cloud, but adds conditional outbound LAN CoT/TAK gated on power-good + WiFi-stable + manual enable. ADR-008 amendment is the `tak-cot-integration-spike.md` thread, not this one. This spike is no longer at risk of being moot.

## What "answered" looks like for this spike

This ticket is closed when we can answer **yes or no** with documented evidence to the question:

> "Can the Pi 5 + Dragino LoRa/GPS HAT + Rust + `lora-phy` gateway path receive a byte-correct LoRa frame from a Heltec Tracker V2 transmitter at our chosen modulation parameters, in polled RX mode, sustained for at least N minutes without loss?"

`N` to be defined when the spike is scoped (see open question 11).

**Hard pass criterion (added 2026-05-06):** the first gateway RX commit MUST set the SX1276 syncword `0x12` explicitly in code and carry a unit test asserting that constant exists. This is verified before the spike is closed. The default `lora-phy` syncword for SX1276 may be `0x34` (LoRaWAN public) or `0x12` (private) depending on chip / version; relying on the default loses the SX1262↔SX1276 private-network compatibility called out in `ARCHITECTURE.md` §10 and §16 risk #6.

If the answer is **yes**: open the gateway-crate skeleton epic and proceed.

If the answer is **no**: write a follow-up ticket describing exactly which layer failed, propose a fallback (alternative SX1276 crate? different HAT?), and only then make a path-forward decision. Substrate alternatives (CM5, Zero 2W, Pi 5 + USB SX1276) are owned by [`gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md), not improvised here.

## Out of scope for this ticket

Listed here so they don't creep in during scoping:

- Interrupt-driven RX. Polled is sufficient for v1a; interrupt is a v2 optimization.
- Persistence integration. Bytes received from the radio is enough for this spike — writing them to SQLite is a separate problem.
- Kiosk UI rendering of received packets.
- L80 GPS PPS-disciplined time discipline. Coexists with this spike but does not gate it.
- Multi-transmitter dedup. v0 is single-tag → gateway. Dedup is exercised in v1a, not here.
- Yocto image build. Bring-up on Raspberry Pi OS is acceptable for this spike (see open question 13).
- 5"-vs-7" display deviation. Adjacent doc debt; not gating gateway RX.
- Architectural pivot question (WiFi + cloud). Separate decision; if Pieter pivots, this spike is moot.

## Cross-references

- `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md` — research session that surfaced this spike's questions
- `decisions/ADR-004-gateway-platform.md` — gateway platform decision (Yocto + Pi + Dragino HAT)
- `decisions/ADR-011-gateway-time-source.md` — adjacent (L80 / RTC / PPS)
- `decisions/ADR-013-multi-hop-flood-via-packet-id.md` — defines the wire format the gateway must decode
- `TODO.md` "v0 — when hardware arrives" section — this spike is the first v0 task in practice
- `crates/protocol/` — already-passing test vectors define what "byte-correct" means here
- `spikes/gateway-handheld-substrate-spike.md` — **upstream of this spike post-pivot**; substrate decision (Pi 5 vs alternatives, antenna paths, HAT-on-Pi-5 verdict) feeds in here
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar for the 2026-05-06 pivot

## Required next step before this spike can be timeboxed

**Sequencing under the 2026-05-06 pivot:**
- `spikes/gateway-handheld-substrate-spike.md` must close first (picks Pi 5 vs alternatives, antenna paths, HAT-on-Pi-5 compatibility verdict).
- Hardware-rev questions 1-3 (silkscreen revs on the 3 on-hand Dragino HATs, bent-pin damage check, antenna for bench testing) need eyes-on-board; this is unchanged by the pivot.
- B11 (WiFi+cloud pivot) is resolved; the handheld pivot is the architecture, this spike is on the v0 critical path.
- B10 (Kiwi cart) is moot per memory correction 2026-05-06; the substrate spike re-opens procurement against the new envelope.

Once the substrate spike closes and the hardware-rev questions are answered the spike can be scoped (timebox + concrete pass/fail criterion) and then executed.
