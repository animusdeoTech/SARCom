---
title: "ADR-014: Duty-cycle budget table as mandatory protocol gate"
status: accepted
date: 2026-04-26
type: adr
tags: [decision, protocol, duty-cycle, etsi, process]
---

# ADR-014: Duty-cycle budget table as a mandatory protocol gate

**Status:** Accepted
**Date:** 2026-04-26

## Context

ETSI EN 300 220 caps duty cycle on sub-band M at 1% per transmitter, per hour. Across multiple ADRs the same failure mode has recurred:

- [ADR-010](ADR-010-sos-encoding.md) went through two cadence revisions before the SOS math was correct (30 s nominal turned out to be 1.24%, then was revised to 45 s, and even that needs the minimum-interval to be 45 s, not the mean — "45 ± 11 s" gives a minimum of 34 s and breaks the budget).
- [ADR-012](ADR-012-node-roles-and-sighting-semantics.md) introduced SIGHTING with a casual "well within duty cycle" claim that was not arithmetically checked.
- The [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md) first draft (FORWARD envelope, 45-byte frame) put relay forwarding over budget at SOS rates and would have shipped if not caught in review.

Three failures of the same kind in a row is a process problem, not a feature problem. Fixing it requires a gate.

## Decision

A duty-cycle budget table sits in [ARCHITECTURE.md §13](../ARCHITECTURE.md) (new section). It contains every transmitter / packet-type / cadence combination in v1, with airtime and duty-cycle percentage explicitly computed.

**The rule:** any change that affects packet size, cadence, retransmit behaviour, hop limit (if hops are ever introduced), or any other variable in the airtime calculation MUST update §13 in the same edit. An ADR or commit that changes one of these without updating §13 is incomplete.

All airtime values must be produced by a single canonical LoRa airtime calculator with explicit and consistent SF/BW/CR/preamble/header/CRC/LDRO parameters. Disagreement between rough estimates ("around 615 ms") and the calculator output is a sign that the calculator is not being used; resolve by running the calculator.

The v1 airtime parameters (canonical for the table):

```
SF = 10
BW = 125 kHz
CR = 4/5
preamble = 8 symbols
explicit header on
CRC on
low-data-rate optimisation off
```

The v1 budget table contents (to be inserted into [ARCHITECTURE.md §13](../ARCHITECTURE.md)):

- **Tag heartbeat POSITION**
  - frame size 22 B, airtime ~371 ms
  - cadence: positive-only jitter, [300 s, 330 s], max 12 TX/h
  - airtime/h ≈ 4.5 s ≈ 0.12% — OK

- **Tag SOS POSITION**
  - frame size 22 B, airtime ~371 ms
  - cadence: minimum interval 45 s, positive-only jitter +0…+15 s, range [45 s, 60 s], max 80 TX/h
  - airtime/h ≈ 29.7 s ≈ 0.82% — OK
  - (Note: ADR-010's "45 ± 11 s" formulation is corrected here to minimum-45 s with positive-only jitter. Mean ≈ 52.5 s; the worst-case rate is now bounded.)

- **Relay rebroadcast of one POSITION (tag heartbeat scenario)**
  - frame size 22 B, airtime ~371 ms
  - cadence: one rebroadcast per unique packet_id, driven by source rate = 12 TX/h per source tag
  - airtime/h ≈ 4.5 s ≈ 0.12% per source tag — OK

- **Relay rebroadcast of one POSITION (tag SOS scenario)**
  - frame size 22 B, airtime ~371 ms
  - rate = 80 TX/h per source tag in SOS
  - airtime/h ≈ 29.7 s ≈ 0.82% per source tag — OK
  - (Two simultaneous SOS tags would put the relay at ~1.64% — over budget. v1 is single-tag scale; multi-tag is a v2 concern with its own prioritisation/throttling rules.)

- **Relay self-POSITION (slow self-announce)**
  - frame size 22 B, airtime ~371 ms
  - cadence: 1800 s (30 minutes), max 2 TX/h
  - airtime/h ≈ 0.74 s ≈ 0.02% — OK

## Consequences

- Single-tag SOS scenario fits comfortably in 1% budget at the relay. This is a direct consequence of [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md)'s collapse of FORWARD into byte-identical POSITION rebroadcast.
- Two-simultaneous-SOS-tags scenario does NOT fit. Documented as a multi-tag-scale concern; v1 garden test deploys one tag.
- Future packet types (a future reception-log telemetry layer, a future maintenance protocol, etc.) will each need a row in the table at the time they are introduced. No exceptions.
- The [CLAUDE.md](../CLAUDE.md) "do not re-open" list gains an item about the airtime table.
- The [ADR-010](ADR-010-sos-encoding.md) SOS cadence formulation is updated from "45 ± 11 s" to "minimum interval 45 s with positive-only jitter to 60 s" — see [ARCHITECTURE.md §12](../ARCHITECTURE.md).

## Alternatives considered

- **Track duty cycle informally in prose.** Rejected: failed three times. The pattern is that prose claims "well within budget" without doing the arithmetic. A table that mechanically gets updated is the only reliable form.
- **Compute duty cycle in firmware at runtime and self-throttle.** Useful — and a relay should still rate-limit defensively — but this is enforcement, not design-time analysis. Both are needed. ADR-014 covers the design-time gate; runtime enforcement is a firmware concern in the relay crate.
- **Defer the budget table until "we need it."** Rejected: the recent ADR-013 first-draft incident shows the cost of catching duty cycle violations late.

## References

- [ETSI EN 300 220-2 V3.2.1](https://www.etsi.org/deliver/etsi_en/300200_300299/30022002/) — sub-band M (868.0–868.6 MHz), 25 mW ERP, 1% duty cycle per transmitter, per hour.
- [ADR-010](ADR-010-sos-encoding.md) — SOS cadence; this ADR corrects its "45 ± 11 s" wording to "minimum interval 45 s, positive-only jitter to 60 s."
- [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md) — multi-hop flood forwarding; airtime analysis for relay rebroadcast lives in this table.
