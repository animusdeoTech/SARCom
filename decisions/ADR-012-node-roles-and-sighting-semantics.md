---
title: "ADR-012: Node roles, sighting semantics, v1a/v1b scope"
status: superseded
date: 2026-04-25
type: adr
tags: [decision, protocol, sighting, nodes, scope, v1a, v1b, superseded]
superseded-by: [ADR-013, ADR-014]
---

# ADR-012: Node roles, sighting semantics, v1a/v1b scope

**Status:** Superseded by [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md) (forwarding protocol) and [ADR-014](ADR-014-duty-cycle-budget-as-gate.md) (duty-cycle gate). The v1a/v1b scope split, the buzzer at the tag, and the non-goals list (RDF, 121.5 MHz, RSSI fine localization, audio detection by relays, autonomous drones, role-by-altitude auto-detection) survive the rollback and remain accepted — see ADR-013 for where they re-appear.
**Date:** 2026-04-25

> **Note (2026-04-26):** the role-enum, SIGHTING, RELAY_INFO-with-role, and three-table schema parts of this ADR are superseded by ADR-013. The v1a/v1b scope split, tag-side buzzer, and the non-goals list survive. This ADR is preserved as decision history; do not delete. See ADR-013 for the corrected forwarding protocol and ADR-014 for the duty-cycle budget gate.

## Context

Three issues converged in the v8 review pass:

(a) Pre-flight conversations about adding airborne relays surfaced that the v8 `sightings` table mixes two distinct semantic concepts in one row: a tag's own claim about its position (from its `POSITION` payload) and the gateway's last-hop RSSI/SNR (which describes radio reception, not the tag). With stationary observers at known surveyed positions this confusion is harmless. With moving observers it becomes structural — the observer's position is itself time-varying evidence and must be captured per reception, not assumed.

(b) The same conversations produced a backlog of feature ideas (RF direction finding, continuous-carrier homing beacons, RSSI-based fine localization, audio sensing by relays, autonomous drone search, role-by-altitude auto-detection) that need to be explicitly closed off before they leak into v1.

(c) v1 acceptance is currently a single phase, but the work splits cleanly into a ground stack (tag + fixed relay + gateway + audible last-meter cue) and an aerial overlay (drone-pod as moving relay). Doing both concurrently means too many simultaneous debug surfaces.

This ADR closes all three. It is intentionally chunky because the parts are tightly coupled — splitting would fragment one coherent course correction into 4–5 ADRs all dated the same day cross-referencing each other.

## Decision

### 1. Three node roles, build-time-configured

- **HIKER_TAG** — pocket-carried, periodic POSITION, SOS button, buzzer.
- **FIXED_RELAY** — pole-mounted, stationary, GNSS per [ADR-006](ADR-006-relay-has-gnss.md) used during commissioning + maintenance, caches its surveyed position.
- **AERIAL_RELAY** — drone-attached, moving, GNSS active at high cadence, optional barometer for altitude as additional telemetry.

Role is a firmware build-time constant per node. Role is **not** auto-derived from sensor readings. A relay does not become aerial because it climbed 10 m on a thermal; a relay is aerial because it was flashed as aerial.

### 2. Three packet types, no folding, no coupling

- **POSITION** — HIKER_TAG self-report only. Tags emit; relays do not. Existing layout per [ARCHITECTURE.md §7](../ARCHITECTURE.md) is correct and stays.
- **RELAY_INFO** — FIXED_RELAY or AERIAL_RELAY self-report. Existing layout gains one byte: a `role` field (`FIXED_RELAY = 0x01`, `AERIAL_RELAY = 0x02`). FIXED_RELAY emits rarely (commissioning + periodic health). AERIAL_RELAY emits at high cadence so the kiosk can render its movement.
- **SIGHTING** — NEW. Observer record. Emitted by any FIXED_RELAY or AERIAL_RELAY that received a packet from another node. Payload (draft layout — finalise in the `protocol` crate with test vectors before any firmware writes against it):

| Field | Type | Bytes | Notes |
|-------|------|-------|-------|
| observer_id | u8 | 1 | the relay/drone |
| observer_role | u8 | 1 | |
| observed_id | u8 | 1 | the heard node |
| observed_role | u8 | 1 | |
| observed_seq_nr | u32 BE | 4 | the observed packet's seq |
| observer_lat_e7 | i32 BE | 4 | observer's own GNSS |
| observer_lon_e7 | i32 BE | 4 | |
| observer_alt_m | i16 BE | 2 | |
| rssi | i8 | 1 | signed dBm |
| snr | i8 | 1 | signed dB × 4 or similar; exact encoding in `protocol` |
| flags | u8 | 1 | reserved 0 |

The observer's coordinates in SIGHTING come from the observer's own GNSS fix, never from gateway-side inference. **This is the load-bearing rule: each observer owns the truth about its own position.**

### 3. Sighting semantics are observer-centric

A sighting answers: *"I, observer O, heard packet P from node N at my position P_O at time T_O with link quality (RSSI, SNR)."*

Tag self-claims (POSITION) and observer records (SIGHTING) are different objects in the system. The gateway records both, in different tables, and correlates them at render time only.

When a relay receives a tag's POSITION it does **two** things:

- forwards the original POSITION unchanged (preserves the dumb-relay forwarding model so the tag's self-claim still reaches the gateway even via multi-hop)
- emits a SIGHTING describing its own reception of that POSITION

The gateway can hear a tag directly without involving any relay; in that case there is no SIGHTING for that reception, only the POSITION row. This is fine — direct gateway reception is itself the most authoritative case (one fewer hop).

### 4. v1a / v1b scope split

**v1a — ground stack:**

- HIKER_TAG with audible buzzer on SOS (tag-side only; relays do **not** have buzzers as search beacons — that would draw the searcher to the wrong location)
- FIXED_RELAY on a wooden pole in the garden
- Gateway implements all three packet types (POSITION, RELAY_INFO, SIGHTING) and the three-table schema in (5) below
- Acceptance gates from [ARCHITECTURE.md §14 v1](../ARCHITECTURE.md), plus: deliberate SOS triggers the tag buzzer; the kiosk distinguishes a directly-heard tag from one heard via the fixed relay (because the latter produces a SIGHTING row and the former does not)

**v1b — aerial overlay:**

- 3rd Tracker V2 reflashed as AERIAL_RELAY, attached to a drone with zip-ties or velcro under-mount, powered from a small 1S LiPo independent of the drone's flight battery
- Optional barometer (BMP280 or BME280) on I²C for altitude observation; if absent, GNSS altitude is the fallback
- Acceptance: tag in a deliberate blind spot the gateway cannot hear directly, drone overhead bridges the gap, kiosk shows the chain (tag's claimed position from forwarded POSITION, drone's moving position from RELAY_INFO at high cadence, SIGHTING row tying them)

v1b is gated on v1a passing all hard gates. No v1b firmware work begins before that. **This is structural defence against scope creep.**

### 5. SQLite schema (replaces v8 §10 schema)

Three tables. The v8 `sightings` table is renamed `tag_reports` because it always stored tag self-reports, never observer records.

- **tag_reports** — POSITION self-reports from HIKER_TAGs. Recent-window dedup on `(tag_id, seq_nr)` per [ADR-009](ADR-009-database-sqlite.md). Columns mirror v8 `sightings`, minus `rx_rssi_dbm` and `rx_snr_db` (those describe the gateway's last hop and do not belong on a tag self-claim).
- **relay_reports** — RELAY_INFO self-reports. Adds a `role` column (1 = FIXED, 2 = AERIAL). Same dedup pattern. Replaces v8's `relays` table, renamed for symmetry.
- **sightings** — SIGHTING packets. Columns: `id`, `received_at`, `observer_id`, `observer_role`, `observed_id`, `observed_role`, `observed_seq_nr`, `observer_lat_e7`, `observer_lon_e7`, `observer_alt_m`, `rssi`, `snr`, `raw_packet_hex`. Dedup on `(observer_id, observed_id, observed_seq_nr)` so multiple observers hearing the same observed packet produce multiple rows. **That is the point.**

The kiosk renders node markers from `tag_reports` and `relay_reports`. Optional "heard via X" overlay reads `sightings`.

### 6. Non-goals additions to ARCHITECTURE.md §2

The following are explicitly out of scope for v1 and v2 unless a future ADR supersedes this position:

- Radio direction finding, homing beacons, or continuous-carrier emissions on any band. Last-meter acquisition is the tag buzzer audible to the human searcher, not RF triangulation.
- 121.5 MHz or other aviation-distress-band emissions. We are a sub-GHz ISM hobby stack, not an aviation transponder.
- RSSI-based fine localization. RSSI describes the last radio hop only and is too noisy in real terrain to claim metre-level accuracy. Multi-observer sightings give bounding-box evidence, nothing finer.
- Audio detection by relays. The buzzer is for human ears at the search end, not for the network.
- Autonomous drone search behaviour. v1b drones are dumb moving observers — they fly where the operator flies them.
- Role-by-altitude auto-detection. Role is a build-time constant.

## Consequences

- The v8 `sightings` schema is incorrect by these new semantics. Migration is mechanical because no production data exists yet (pre-bring-up).
- The `protocol` crate gains SIGHTING. Three packet types in test vectors before any firmware writes against the wire.
- Relay firmware now does two TX per received tag-POSITION (forward + emit SIGHTING). At v1 single-tag scale this is well within duty cycle. At scale this enters the v2 duty-cycle accounting work already on the deferred list.
- RELAY_INFO total frame becomes **26 bytes** (existing 25 + 1 role byte). All [ARCHITECTURE.md](../ARCHITECTURE.md) references to "RELAY_INFO 25 B" must be updated.
- Kiosk UI complexity goes up modestly: three node visual styles, optional sightings overlay.
- Buzzer is one GPIO line + ~€1 hardware. v1a addition.
- Drone-pod is a third board flashed differently + optional barometer. Bounded v1b work.
- Six categories of feature drift are closed off permanently in §2.

## Alternatives considered

- **Fold SIGHTING into POSITION as additional fields.** Rejected: tags do not have observer metadata, and conflating self-claim with observer record was the original drift this ADR fixes. Pieter's hard rule: *"the relay has its own packet types, period. anything that we call a relay, whether its a stationary wooden pole, or a flying drone, if it creates the uplink, its a relay, and its going to have its own set of packet definitions."*
- **Have the gateway compute observer position from cached node positions.** Rejected: makes the gateway authoritative about positions of nodes it did not physically observe. Each observer owns its own truth.
- **Auto-detect aerial role from altitude or barometric pressure.** Rejected: brittle (a hiker on a peak is not a drone, a drone landed in a valley is not a hiker), and unnecessary given role is a build-time constant.
- **Make v1 a single phase.** Rejected: too many concurrent debug surfaces. v1a/v1b has a clean precondition.
- **Defer SIGHTING to v2.** Rejected: the bug bites in v1a already (two observers hearing the same tag-frame collapse into one row under v8), and fixing it after firmware is flashed means rewriting the wire format in the field.

## References

- [ADR-006](ADR-006-relay-has-gnss.md) — Relay GNSS. The relay's own GNSS is exactly what populates `observer_lat`/`lon` in SIGHTING.
- [ADR-009](ADR-009-database-sqlite.md) — SQLite, recent-window dedup. Dedup pattern reused across the three tables.
- [ADR-010](ADR-010-sos-encoding.md) — SOS. Flags layout and bit-1 SOS bit referenced from POSITION and from kiosk distress rendering.
