---
title: "ADR-013: Multi-hop flood forwarding via packet_id dedup"
status: accepted
date: 2026-04-26
type: adr
tags: [decision, protocol, multi-hop, forwarding, dedup]
supersedes-parts-of: ADR-012
---

# ADR-013: Multi-hop flood forwarding via packet_id dedup

**Status:** Accepted
**Date:** 2026-04-26
**Supersedes (in part):** ADR-012's role enum, SIGHTING packet type, RELAY_INFO-with-role, and three-table schema. The v1a/v1b scope split, tag buzzer, and non-goals list survive.

## Context

The system's reason to exist is multi-hop relay coverage. ADR-012 drifted into a wire-level role taxonomy (HIKER_TAG / FIXED_RELAY / AERIAL_RELAY) plus a separate SIGHTING packet type for observer records. None of that solved a real problem — both were modelling for the sake of modelling, triggered by adding drones to the conversation.

A first rollback attempt replaced SIGHTING with a FORWARD envelope carrying explicit path arrays and per-hop RSSI/SNR. That was the same mistake in different syntax: putting observation/coverage-analysis data into the forwarding protocol. The resulting 45-byte frame put the relay over its sub-band M duty cycle limit at SOS rates, and required priority/throttle rules in firmware to mitigate a problem that the design itself created.

The correct shape: forwarding protocol stays minimal. Observation data is a separate concern, deferred until v1 works.

## Decision

### 1. v1 is multi-hop on a single LoRa channel

All nodes — tags, relays, gateway — park on 868.1 MHz (per [ADR-010](ADR-010-sos-encoding.md)). The packet `TYPE` byte is the only semantic discriminator on the wire; no channel splitting.

### 2. One packet type for v1 core forwarding: POSITION

Tags emit POSITION to self-report. Relays emit POSITION to self-report (they are stationary or slow-moving and announce themselves at a slow cadence). Relays **rebroadcast** received POSITION packets unchanged. There is no FORWARD packet type. There is no SIGHTING. There is no RELAY_INFO. The wire carries POSITION and nothing else in v1 core.

### 3. POSITION layout (v1, 22 B frame)

| Offset | Field | Type | Bytes | Description |
|--------|-------|------|-------|-------------|
| 0 | MAGIC | u8 | 1 | 0xA5 |
| 1 | VER | u8 | 1 | 0x01 |
| 2 | TYPE | u8 | 1 | 0x01 = POSITION |
| 3 | LEN | u8 | 1 | 16 (payload length) |
| 4 | node_id | u8 | 1 | 0–254 (255 reserved) |
| 5 | seq_nr | u32 BE | 4 | monotonic per-node, wraps at 2^32 |
| 9 | flags | u8 | 1 | bit 0: GPS_VALID, bit 1: SOS, bit 2: BATT_LOW, bits 3–7: reserved |
| 10 | lat_e7 | i32 BE | 4 | latitude × 10^7 |
| 14 | lon_e7 | i32 BE | 4 | longitude × 10^7 |
| 18 | alt_m | i16 BE | 2 | altitude in metres, signed |
| 20 | CRC16 | u16 BE | 2 | CRC-16/CCITT-FALSE over bytes 0…19 |

Total: **22 bytes** on the wire.

### 4. packet_id is the dedup key

`packet_id` is the composite `(node_id, seq_nr)`. It is not a separate field; it is derived from the two fields already in POSITION. Every forwarding and dedup decision in the network uses this composite as its dedup key.

Tags must increment `seq_nr` monotonically for each new POSITION they emit. A tag MUST NOT reuse a `(node_id, seq_nr)` within the network's seen-cache expiry window (60 s in v1).

### 5. Relay forwarding rule

On receipt of any valid POSITION packet (CRC passes, MAGIC/VER/TYPE/LEN match):

```
derive packet_id = (node_id, seq_nr)
if packet_id is in seen_cache: drop, log DUP
if node_id == my_node_id:      drop, log SELF_ECHO
else:
    insert packet_id into seen_cache (60 s expiry)
    enqueue an unmodified rebroadcast of the same POSITION bytes
    relay's TX engine emits the rebroadcast after CAD + backoff,
        subject to ADR-014 duty-cycle priority rules
```

The rebroadcast is byte-identical to the received frame. The relay does **not** add any path information, RSSI, or any other field.

### 6. Unknown TYPE/VER frames are dropped, not forwarded

With one canonical packet type in v1, an unknown `TYPE` byte means a malformed or unauthorised frame. Unknown forwarding is structurally unsafe without its own TTL/loop-prevention mechanism, and v1 has only one known type. Drop and log `UNKNOWN_TYPE`. Future protocol versions that introduce new types will define their own forwarding semantics and loop control at that point.

### 7. Loop prevention is dedup-only in v1

No TTL, no `hop_count`, no self-in-path. The seen_cache (32 entries, 60 s expiry, dispatched on `(node_id, seq_nr)`) is the entire loop-prevention mechanism. This is sufficient because:

- Once a relay has rebroadcast a packet, it will not rebroadcast the same packet again until the cache entry expires.
- 60 s is far longer than the realistic propagation time for a flood through any plausible v1 topology.
- When the entry expires, the network has long since quieted.

If field testing shows the seen_cache is too small or expires too quickly under realistic load (multiple tags, dense relay deployments), expiry and capacity are the parameters to tune. A TTL byte is a future protocol-version concern, not a v1 fix.

### 8. Gateway behaviour

On receipt of any valid POSITION:

```
derive packet_id = (node_id, seq_nr)
if packet_id is already stored: ignore (dedup)
else: store one row in tag_reports
```

The gateway does not distinguish "directly heard" from "via relay" in v1 — that information is not in the protocol. Coverage analysis (who heard what, with what link quality) is a future concern; see (10) below.

### 9. Node presentation is gateway configuration, not protocol

A TOML file on the gateway maps `node_id` to display label and UI icon:

```toml
[nodes.1] label = "tag-1"        ui_kind = "hiker"
[nodes.2] label = "garden relay" ui_kind = "relay"
[nodes.3] label = "drone pod"    ui_kind = "drone-relay"
```

Adding a drone-pod relay is a config edit on the Pi plus a fresh `node_id` flashed into a Tracker V2. No wire-format change.

### 10. Coverage / science telemetry is deferred

Pieter wants to collect per-hop RSSI/SNR data for downstream coverage analysis. That data does **not** live in the forwarding protocol. It will live in a separate reception-log layer designed in a future ADR — likely a locally-logged-then-uploaded scheme. The shape, transport, and schedule of that layer are explicit non-decisions in v1. Build the forwarding network first; design the analytics layer when v1 packets are flowing and the actual analysis question is concrete.

This is the seam: forwarding is one thing, observation is another. Mixing them was the original error. They stay separate.

## Consequences

- ADR-012's role enum: gone.
- ADR-012's SIGHTING packet type: gone.
- ADR-012's RELAY_INFO packet type: gone. Relays self-announce by emitting POSITION at a slow cadence (e.g. once per 30 minutes).
- The first-attempt FORWARD envelope with path arrays: gone. Never shipped to ADR-013-published; this clarifies it for any reader who saw the chat draft.
- Dedup is on `(node_id, seq_nr)` globally — at relays for forwarding control, at the gateway for storage. ADR-009's existing recent-window dedup pattern applies.
- Duty cycle is dramatically simpler than the FORWARD-envelope proposal. A relay rebroadcasting a tag's POSITION emits a 22-byte frame, same airtime as the tag's own emission. See [ADR-014](ADR-014-duty-cycle-budget-as-gate.md).
- v1a remains: tag + 1 relay + gateway, garden test, single forwarding hop physically exercised.
- v1b remains: tag + 2 relays (paal + drone-pod) + gateway, multi-hop chain physically exercised. Drone-pod is just another `node_id` with `ui_kind = "drone-relay"` in the TOML.
- v1a/v1b scope split (from ADR-012): preserved.
- Tag SOS buzzer (from ADR-012): preserved.
- Non-goals list (from ADR-012): preserved.
- DB schema collapses to one core table (`tag_reports`) — see [ARCHITECTURE.md §10](../ARCHITECTURE.md). The `relays`, `relay_reports`, `sightings`, and `path_observations` tables from ADR-012 / ADR-013 drafts are not created.
- Future reception-log / coverage-analysis layer is explicitly noted as deferred. No v1 code or schema accommodates it; that's correct.

## Alternatives considered

- **Keep ADR-012's role enum and SIGHTING.** Rejected: solves no real problem; the role byte's contradictory mapping across ADR-012's own text was the visible symptom of an unnecessary abstraction.
- **FORWARD envelope with path arrays + per-hop RSSI/SNR.** Drafted, rejected before publication. 45-byte frame put relay over duty-cycle budget at SOS rates; required priority/throttle firmware rules to fix a problem the design itself created. Putting analytics data into the forwarding packet is the original error in different syntax.
- **Channel split (CH_TAG + CH_FWD).** Rejected: doesn't prevent loops in multi-hop, complicates single-radio gateway scheduling, and pushes loop-prevention work somewhere else without removing it. Single channel + dedup is simpler and correct.
- **TTL byte in v1 POSITION.** Rejected: dedup-only loop prevention is sufficient at v1 scale and topology. Adding TTL is a future protocol-version decision triggered by measured need, not speculation.
- **Path signature (shift-XOR rolling hash).** Rejected: opaque, lossy, and doesn't carry per-hop RSSI/SNR anyway — but the deeper reason is that path tracking belongs in the deferred reception-log layer, not the forwarding packet.
- **Bloom filter for relay-set-traversed.** Rejected, same reason.

## References

- [ADR-009](ADR-009-database-sqlite.md) — recent-window dedup pattern, reused for the gateway-side packet_id dedup.
- [ADR-010](ADR-010-sos-encoding.md) — single-channel SOS encoding, unchanged.
- [ADR-011](ADR-011-gateway-time-source.md) — gateway time source, unchanged.
- [ADR-012](ADR-012-node-roles-and-sighting-semantics.md) — partly superseded; v1a/v1b split, buzzer, non-goals survive.
- [ADR-014](ADR-014-duty-cycle-budget-as-gate.md) — duty-cycle budget gate.
