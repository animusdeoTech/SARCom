---
title: "System Architecture (v10)"
status: living
type: architecture
tags: [architecture, system, overview]
---

# LoRa SAR — System Architecture

**Version 10.** Version 10 rolls back ADR-012's role enum and SIGHTING in favour of the simpler shape decided in [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md): one packet type (POSITION), packet_id-based dedup as the entire loop-prevention mechanism, no path tracking on the wire. [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) introduces a mandatory duty-cycle budget table (§13). Coverage / science telemetry is deferred to a future ADR. The v1a/v1b scope split, tag buzzer, and non-goals list from ADR-012 survive the rollback.

> **2026-05-06 form-factor pivot.** The gateway moves from "fixed Pi + 7" DSI on a wall- or shelf-mount at a mountain hut" to "handheld portable Rust device with a touchscreen, battery + USB-C charging, custom 3D-printed waterproof enclosure, and an ADR-016-gated outbound LAN CoT/TAK export path." Sectional edits below cite "(pending ADR-015)" / "(pending ADR-016)" / "(pending ADR-017)" parentheticals where the pre-pivot text named a fixed-kiosk substrate or a categorical no-outbound-network stance. See [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md) for the per-file edit checklist and ADR-ledger verdicts. ADRs themselves are not edited here.[^pivot]

[^pivot]: 2026-05-06 form-factor pivot — see [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md). Three new ADRs proposed: ADR-015 (handheld substrate + form factor; supersedes-in-part ADR-004; refines-in-part ADR-005/006/007), ADR-016 (base-mode export gate; supersedes-in-part ADR-008), ADR-017 (custom 3D-printed waterproof enclosures for gateway and tag; refines-in-part ADR-002).

> Long-term plan is to decompose this into `architecture/{system-overview, sighting-model, protocol, operational-modes, non-goals}.md` per the [README](README.md). For now this is the consolidated source of truth.

---

## 1. Mission and operational context

This system exists to do three things:

1. **Track a person's location** on a hiking trail or mountain environment by producing periodic sightings.
2. **Allow the person to send a distress signal** (SOS) when something has gone wrong.
3. **Show on a map**, on a handheld touchscreen carried by the operator, where each hiker is or was last seen, and when. The mountain hut is one possible deployment site, not the only one (pending ADR-015).

Intended users: rangers, mountain hut staff, trail operators, rescue-adjacent personnel — people who need a simple operational picture at a glance: where were hikers last seen, how stale is that, and is anyone signalling distress.

The system is a **low-bandwidth safety telemetry network, local-first**. It produces sightings, not live tracks. The fundamental data primitive is a sighting:

> *"Tag X was heard at time T, with state S, and position P (if a GPS fix was available)."*

A no-fix sighting is still useful — it proves the tag was alive and in radio range at time T.

The deployment scenario: mountain terrain, internet is scarce, power is limited, weather is hostile. Relays sit on exposed poles. The gateway is carried by the operator (or, when convenient, set down somewhere with a power source) and is also the operator workstation. **There is no internet-hosted server, no cloud, no external dashboard.** Outbound LAN-bounded CoT/TAK export to TAK-compatible clients on the same WiFi is the one network surface the gateway can have, and only when WiFi + manual opt-in are both present (pending ADR-016).[^export-gate][^gate-2026-05-14] A snowstorm taking WiFi down silences that export path; the LoRa-side telemetry path keeps working.

[^export-gate]: Base-mode CoT/TAK export under pending ADR-016 is the **one** layer that depends on WiFi. WiFi gone → export path silent → everything else still works. The system property "WiFi loss does not take the system down" still holds; it is just narrower in scope (the LoRa-side telemetry path) than the pre-pivot wording suggested. See [`spikes/tak-cot-integration-spike.md`](spikes/tak-cot-integration-spike.md) and [`spikes/gateway-handheld-power-architecture-spike.md`](spikes/gateway-handheld-power-architecture-spike.md) for the gate predicate.

---

## 2. Non-goals

Explicitly **not** part of this system:

- **No cloud, no server, no REST API, no mobile app, no web dashboard.** Everything lives on the gateway Pi. See [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md).
- **No downlink.** Tag transmit schedule is autonomous. Gateway cannot command tags. Pure uplink. See [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md).
- **No authentication, no user management, no login.** Kiosk is read-only and anyone at the hut can glance at it. See [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md).
- **No precision ranging or RSSI-based positioning.** One packet with a GPS fix is a complete position. No trilateration.
- **No trail guidance, navigation, or deviation detection.** The system reports where people are, not where they should be.
- **No routed mesh, no addressing, no path selection.** Relays flood-forward.
- **No live streaming.** Sightings arrive every 2–5 minutes. The UI must never imply real-time.
- **No general-purpose IoT platform.** Every design decision is driven by the three mission goals.
- **No radio direction finding, homing beacons, or continuous-carrier emissions on any band.** Last-meter acquisition is the tag buzzer audible to the human searcher, not RF triangulation. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No 121.5 MHz or other aviation-distress-band emissions.** This is a sub-GHz ISM hobby stack, not an aviation transponder. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No RSSI-based fine localization.** RSSI describes the last radio hop only and is too noisy in real terrain to claim metre-level accuracy. Multi-observer sightings give bounding-box evidence, nothing finer. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No audio detection by relays.** The buzzer is for human ears at the search end, not for the network. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No autonomous drone search behaviour.** v1b drones are dumb moving observers — they fly where the operator flies them. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No role-by-altitude auto-detection.** Role is a build-time constant per node. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).
- **No wire-level role enum.** Node presentation (hiker / relay / drone) is gateway configuration in a TOML file (`nodes.toml`), not a protocol field. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

Future extensions (cloud sync, mobile map, downlink control, BLE relay maintenance) are kept out of v1. Decisions in v1 prefer options that do not *close the door* to those — e.g., SQLite as a single source of truth, protocol versioned by a `VER` byte — but we do not spend v1 engineering budget building those seams.

---

## 3. System concept

A hiker carries a small Wireless Tracker V2 (tag, pending ADR-017 enclosure). It acquires a GNSS fix, broadcasts a 22-byte `POSITION` frame over LoRa, goes back to sleep. Fire-and-forget — the tag does not know whether anyone heard it. A solar-powered Wireless Tracker V2 (relay) on an off-the-shelf plastic tripod (selection per [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md); relay enclosure stays OEM Solar Kit per [ADR-003](decisions/ADR-003-relay-hardware.md)) receives, validates, queues, and rebroadcasts packets toward the gateway. The gateway is a handheld Rust device built on a Pi-class SBC with a Dragino LoRa/GPS HAT and a touchscreen, in a custom 3D-printed waterproof enclosure with a battery + USB-C charging path (substrate / display / battery / enclosure pending ADR-015 / ADR-017). The gateway receives the packet, persists it in a local SQLite file, and draws a dot on the map the operator sees. **That is the whole system.**

One received packet from any relay, or from the tag directly, is sufficient. There is no minimum infrastructure requirement beyond a single radio path from tag to gateway.

---

## 4. Architecture overview

```
                    LoRa 868 MHz              LoRa 868 MHz
┌──────────────┐  ┌──────────────┐          ┌──────────────────────┐
│     TAG      │─▶│    RELAY     │─ ─ ─ ─ ─▶│       GATEWAY        │
│ Heltec       │  │ Heltec       │          │  Pi-class SBC +      │
│ Wireless     │  │ Wireless     │          │  Dragino LoRa HAT +  │
│ Tracker V2   │  │ Tracker V2   │          │  handheld touchscreen│
│ + 18650      │  │ + Solar Kit  │          │  Yocto Linux         │
│ ESP32-S3     │  │ + 2×18650    │          │  battery + USB-C     │
│ SX1262 +     │  │ ESP32-S3     │          │  ┌────────────────┐  │
│ UC6580 GNSS  │  │ SX1262 +     │          │  │ Rust binary:    │  │
│ + custom     │  │ UC6580 GNSS  │          │  │ LoRa RX loop    │  │
│ 3D shell     │  └──────────────┘          │  │ SQLite writer   │  │
│ (ADR-017)    │                            │  │ Kiosk UI (egui  │  │
└──────────────┘  tag can also be           │  │ + walkers, no   │  │
                  heard directly by gateway │  │ browser)        │  │
                                            │  │ BLE central     │  │
                                            │  │ (commissioning) │  │
                                            │  │ CoT/TAK emitter │  │
                                            │  │ (gated, off by  │  │
                                            │  │ default)        │  │
                                            │  └────────────────┘  │
                                            └─────────┬────────────┘
                                                      ┊
                                                      ┊  WiFi + power-good
                                                      ┊  + manual opt-in:
                                                      ┊  CoT/TAK to LAN
                                                      ┊  (pending ADR-016;
                                                      ┊   silent by default)
                                                      ▼
                                              ATAK / iTAK / WinTAK on
                                              same LAN; no internet-bound
                                              arrow ever leaves here
                                              (no cloud, no server,
                                              no inbound surface)
```

Tag broadcasts. Any node that hears it acts on it. Relays rebroadcast over LoRa. The gateway's Rust binary receives via SPI from the SX1276 on the Dragino HAT, writes to SQLite, and the same binary renders a live map to the touchscreen. **The LoRa-side telemetry path is fully local.** The dashed `CoT/TAK to LAN` arrow is the one place an outbound network packet can leave the gateway, and only when all three gate inputs are true (pending ADR-016). It is RFC1918 / link-local / multicast-only — never internet-routed — and it is read-only, outbound-only; the gateway never accepts inbound network traffic.

---

## 5. Why this architecture fits the mission

The mission requires coverage over a large mountain area with no cellular infrastructure. The constraints are:

- **No internet on the trail.** Relays run on solar and battery.
- **No power on the trail.** Every component must survive unattended.
- **Hostile environment.** Rain, snow, condensation, lightning, thermal cycling, animal damage.
- **Sparse, delayed data is acceptable.** A sighting every 5 minutes is operationally useful.
- **Graceful degradation is essential.** A relay dies, another path works. The gateway loses the internet… except in this architecture the LoRa-side telemetry path doesn't *need* the internet. That whole category of failure is eliminated for the LoRa side; the optional CoT/TAK export under pending ADR-016 is the one path that does need WiFi, and turns off when WiFi is gone.

This leads to:

- **Self-contained packets** — any single receiver is sufficient, relays are stateless for payload.
- **Dumb-but-disciplined relays** — RAM queue, SOS priority, CAD before TX. Simple enough to bolt to a pole and forget; not so blind it wastes airtime.
- **Gateway as the terminal node AND the workstation.** Complexity stops at the gateway.
- **Local-first data.** No internet-bound arrow means no class of "cloud backhaul is down" failure modes; outbound LAN CoT/TAK under pending ADR-016 is bounded to RFC1918 / link-local / multicast destinations and silenced by default.

Complexity flows downhill toward the user. The user is five metres from the kiosk, not a thousand kilometres away in a datacentre.

---

## 6. The sighting model

A sighting in v1 is the simplest possible thing: one row in `tag_reports`, keyed by `(node_id, seq_nr)`, holding what a node said about itself. The gateway does not distinguish "heard directly" from "heard via relay" — that information is not on the wire. If the gateway hears the same POSITION via the tag directly AND via a relay's rebroadcast, it stores the row once (dedup on `(node_id, seq_nr)`) and silently discards the duplicate. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

Coverage analysis — who heard what, with what link quality — is the concern of a future reception-log layer (see [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md)), not v1 core.

One POSITION packet is sufficient to render a position — the gateway does not need to correlate multiple packets to place a marker. There is no assembly step.

Why this matters:

1. In mountain terrain, packet loss is high. If you need three packets to compute a position, you may never get one.
2. Relays are stateless forwarders for payload — they rebroadcast unchanged.
3. The operator question is "where was this person last seen?" — one POSITION packet answers that.

### No-fix POSITION

When the tag cannot obtain a GPS fix (deep canyon, indoors), it transmits with `GPS_VALID=0` and sentinel values (`0x7FFFFFFF` for lat/lon, `0x7FFF` for alt) in the coordinate fields. It does **not** send last-known coordinates. Stale coordinates with a no-fix flag create dangerous ambiguity — staff cannot tell whether the position is where the person is now, or where they were before entering a canyon. Sentinels are unambiguous.

Both conditions must agree: `GPS_VALID=0` AND sentinels in every coordinate field. If they disagree, the packet is malformed — the kiosk logs the anomaly and does not trust the coordinates. The row in `tag_reports` carries the sentinels and the kiosk surfaces the tag in a no-fix side list rather than placing a marker.

---

## 7. Wire protocol

Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), v1 carries **one packet type only: POSITION.** No FORWARD envelope, no SIGHTING, no RELAY_INFO. Tags emit POSITION to self-report. Relays emit POSITION to self-report (slow cadence, see §12) and rebroadcast received POSITION packets unchanged.

### Frame layout (POSITION, 22 B on the wire)

| Offset | Field | Type | Bytes | Description |
|--------|-------|------|-------|-------------|
| 0 | MAGIC | u8 | 1 | 0xA5 |
| 1 | VER | u8 | 1 | 0x01 |
| 2 | TYPE | u8 | 1 | 0x01 = POSITION |
| 3 | LEN | u8 | 1 | 16 (payload length) |
| 4 | node_id | u8 | 1 | 0–254 (255 reserved) |
| 5 | seq_nr | u32 BE | 4 | monotonic per-node, wraps at 2^32 |
| 9 | flags | u8 | 1 | bit 0: GPS_VALID, bit 1: SOS, bit 2: BATT_LOW, bits 3–7: reserved (must be zero; relay must preserve on rebroadcast) |
| 10 | lat_e7 | i32 BE | 4 | latitude × 10^7 |
| 14 | lon_e7 | i32 BE | 4 | longitude × 10^7 |
| 18 | alt_m | i16 BE | 2 | altitude in metres, signed |
| 20 | CRC16 | u16 BE | 2 | CRC-16/CCITT-FALSE over bytes 0…19 |

Total: **22 bytes**. Design principles: self-contained, explicitly framed, no struct dumps, big-endian, no padding, small.

### packet_id

`packet_id` is the composite `(node_id, seq_nr)` derived from fields already in the frame; it is not a separate field. Every forwarding and dedup decision in the network keys on this composite. Tags must increment `seq_nr` monotonically per emitted POSITION; a tag MUST NOT reuse a `(node_id, seq_nr)` within the network's seen-cache expiry window (60 s in v1).

### CRC specification

**CRC-16/CCITT-FALSE** (also known as CRC-16/AUTOSAR, CRC-16/IBM-3740). Parameters:

| Parameter | Value |
|-----------|-------|
| Polynomial | `0x1021` |
| Init | `0xFFFF` |
| RefIn | `false` |
| RefOut | `false` |
| XorOut | `0x0000` |
| Check (ASCII "123456789") | `0x29B1` |

This is **not** the "XMODEM" variant (which also uses `0x1021` but `init = 0x0000`). The `protocol` crate must pick an implementation that matches these exact parameters — `crc` crate with `CRC_16_IBM_3740` is correct. A test vector for `"123456789"` producing `0x29B1` must sit in the crate's unit tests.

### Why `u32` for seq_nr

At a 45 s minimum SOS interval, `u16` wraps in ~34 days; at a 300 s heartbeat cadence, ~227 days. Combined with the gateway's permanent dedup index, a `u16` wrap would silently start dropping real new sightings as "duplicates" months into deployment. `u32` defers the problem to decades. See [ADR-009](decisions/ADR-009-database-sqlite.md) for the matching gateway-side dedup policy (recent-window, not table-wide unique).

### Distress semantics

SOS is a **flag bit on `POSITION`** (bit 1 of the `flags` byte; mask `0x02`). Per [ADR-010](decisions/ADR-010-sos-encoding.md), a tag that entered distress sets `flags.SOS = 1` on every subsequent POSITION it emits until distress is cleared. The gateway classifies a tag as "currently in distress" iff at least one `SOS=1` frame has been received from that `node_id` within `DISTRESS_WINDOW` seconds (default: `DISTRESS_WINDOW = 10 × SOS_MIN_INTERVAL = 450 s`).

### Test vectors

Written as byte strings in the `protocol` crate alongside the encoder/decoder, agreed in hex before any device sends or parses a byte on the wire. This eliminates one class of "which side is wrong?" debugging. At minimum:

- One canonical **POSITION** packet (`node_id=1`, `seq_nr=42`, `flags.GPS_VALID=1`, `lat=47.1234567`, `lon=13.5678901`, `alt=1847`)
- One canonical POSITION with `flags.SOS=1` to lock the distress-bit encoding

---

## 8. Tag responsibilities

The tag is hiker-carried. It acquires a GPS fix, builds a self-contained packet, broadcasts over LoRa, and sleeps. No knowledge of relays, gateway, or the world. See [ADR-002](decisions/ADR-002-tag-hardware.md) for hardware.

While distress is active, the tag also drives a GPIO-connected piezo buzzer with a periodic pulse pattern; the buzzer state follows the SOS distress state and is cleared when distress is cleared. The buzzer is the last-meter audible cue for the human searcher and exists on the tag only — relays do **not** have buzzers as search beacons. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md).

### State machine

Two entry paths to `BUILD_PKT`:

- **Timer-driven** (heartbeat or recurring SOS). Wake → `GPS_ACQUIRE` → `BUILD_PKT` → `TRANSMIT`.
- **SOS-entry** (button press while not already in distress). Wake → **skip `GPS_ACQUIRE`** → `BUILD_PKT` with `GPS_VALID=0` + sentinels + `flags.SOS=1` → `TRANSMIT` immediately, no first-frame jitter. Subsequent SOS frames follow the timer-driven path on the SOS cadence.

Distress must not be gated by GNSS acquisition. A 90-second wait between button press and first distress frame would be the wrong shape. See [ADR-010](decisions/ADR-010-sos-encoding.md).

```
                          ┌─────────────────┐
                  ┌──────▶│      SLEEP      │ deep sleep, RTC timer or GPIO wake
                  │       └────┬───────┬────┘
                  │            │       │
                  │   timer    │       │  SOS button pressed
                  │   expired  │       │  (entry — distress not yet active)
                  │            │       │
                  │     ┌──────▼──┐    │
                  │     │  GPS_   │    │  power on GNSS, wait for fix
                  │     │ ACQUIRE │    │  (timeout 90 s)
                  │     └──────┬──┘    │
                  │            │       │  bypass GPS_ACQUIRE on
                  │            │       │  SOS-entry first frame
                  │     ┌──────▼───────▼──┐
                  │     │    BUILD_PKT    │ serialise POSITION
                  │     │                 │ no fix or SOS-entry → GPS_VALID=0 + sentinels
                  │     │                 │ flags.SOS per current distress state
                  │     └────────┬────────┘
                  │     ┌────────▼────────┐
                  │     │    TRANSMIT     │ LoRa TX (freq, SF, power per mode)
                  │     │                 │ SOS-entry: no first-frame jitter
                  │     └────────┬────────┘
                  │     ┌────────▼────────┐
                  │     │    HOUSEKEEP    │ increment seq_nr, read battery,
                  │     │                 │ persist SOS flag to NVS
                  │     └────────┬────────┘
                  └──────────────┘
                       deep sleep for interval:
                       - heartbeat: 300 s ± 10%
                       - SOS active: 45–60 s, positive-only jitter, minimum interval 45 s
```

### GNSS cold start

UC6580 cold start: 30–90 seconds after extended power-off. Hot start via battery-backed RAM: 1–5 seconds across deep sleep cycles. `GPS_ACQUIRE` timeout is 90 seconds; the tag transmits anyway (no-fix sighting is better than silence). The 90-second budget applies only to the timer-driven path; on SOS entry the `GPS_ACQUIRE` step is skipped entirely for the first frame.

---

## 9. Relay responsibilities

The relay is a solar-powered box on a wooden pole. It receives LoRa packets, validates them, queues them, and forwards them when the channel is locally clear. It is not a dumb repeater — it is a disciplined forwarder with explicit queue policy. See [ADR-003](decisions/ADR-003-relay-hardware.md) for hardware.

Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), the relay's forwarding semantics are minimal: rebroadcast valid unseen POSITION frames byte-for-byte, dedup on `(node_id, seq_nr)`, drop unknown TYPE/VER. No FORWARD envelope, no SIGHTING emission, no path tracking. The relay is a known-good originator of POSITION packets too — it self-announces at a slow cadence so the gateway can render it on the map.

### Modes

Per [ADR-006](decisions/ADR-006-relay-has-gnss.md):

1. **Forwarding (default).** GNSS OFF (after commissioning fix is cached). LoRa RX continuous. Validate → check seen_cache → enqueue rebroadcast or drop. This is where the relay lives for years.
2. **Commissioning.** Triggered on first boot, magnet/reed-switch signal, or BLE request (v2+). GNSS ON, fix acquired, the surveyed position is cached in NVS, the relay emits its first self-POSITION carrying that position, GNSS OFF, back to forwarding.
3. **Self-announce.** While in Forwarding mode, the relay periodically emits its own POSITION (cadence 1800 s — once per 30 minutes — per [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md)) so the gateway / kiosk can render it. The relay's `node_id` is its own; the position carried is the cached commissioned position.
4. **BLE maintenance.** v2+. Explicitly not v1.

A drone-pod relay is the same firmware reflashed with a different `node_id`; node presentation (drone icon vs pole icon) is gateway TOML configuration, not a wire-level distinction. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) §9.

### Validation rules (drop or accept)

1. Received bytes ≥ 6 (minimum frame size)
2. Received bytes ≤ MAX_FRAME (e.g. 64)
3. Byte 0 = MAGIC (0xA5)
4. Byte 1 = VER (must be `0x01` in v1)
5. Byte 2 = TYPE (must be `0x01` POSITION in v1)
6. Byte 3 (LEN) + 6 = total received length (must equal 22 in v1)
7. CRC16 over bytes 0…N-2 matches last 2 bytes
8. All pass → continue to forwarding rule. Any fail → drop, log appropriate reason.

Unknown VER or TYPE values: **drop and log `UNKNOWN_TYPE`.** With one canonical packet type in v1, an unknown TYPE byte is a malformed or unauthorised frame, and unknown forwarding is structurally unsafe without its own loop control. Future protocol versions that introduce new types will define their own forwarding semantics at that point. See [ADR-013 §6](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

### Forwarding rule

On receipt of any valid POSITION packet:

```
derive packet_id = (node_id, seq_nr)
if packet_id is in seen_cache:    drop, log DUP
if node_id == my_node_id:         drop, log SELF_ECHO
else:
    insert packet_id into seen_cache (60 s expiry)
    enqueue an unmodified rebroadcast of the same POSITION bytes
    relay's TX engine emits the rebroadcast after CAD + backoff
```

The rebroadcast is **byte-identical** to the received frame. The relay does not add any path information, RSSI, or any other field. Per [ADR-013 §5](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

### Forwarding queue

A **RAM queue** (not flash — flash write endurance would be exhausted in days) holds POSITION packets ready for transmit. Two sources:

- **Other-node POSITION** awaiting rebroadcast (the bulk of TX traffic).
- **Own-node POSITION** awaiting initial transmit (relay self-announce, every 1800 s).

Priority order:

1. SOS-flagged POSITION (`flags.SOS = 1` in the inner payload), regardless of source. Distress packets must not wait behind heartbeats.
2. Heartbeat POSITION from other nodes (rebroadcast).
3. Relay's own self-POSITION.

Queue parameters: max depth 16, packet expiry 30 s (a packet that sat in queue longer than that is stale and dropped on the way out).

### Channel access: CAD + backoff

Before transmitting, the relay uses the SX1262's Channel Activity Detection (CAD) hardware feature.

1. Pop highest-priority packet from queue
2. Run CAD on the channel
3. If busy: back off 50–200 ms (random), retry CAD, up to 3 attempts
4. If clear: transmit
5. If all CAD attempts show busy: transmit anyway (bounded backoff — do not hold packets forever)
6. Return to RX immediately after TX

CAD is not a guarantee. Two relays that cannot hear each other may both see a clear channel and collide at the gateway (hidden-node problem). CAD reduces avoidable local collisions; the system's resilience to packet loss handles the rest.

### Loop prevention (seen_cache, dedup-only)

Per [ADR-013 §7](decisions/ADR-013-multi-hop-flood-via-packet-id.md), loop prevention in v1 is **dedup-only**. No TTL, no `hop_count`, no self-in-path encoding.

Ring buffer of 32 entries, 60 s expiry, keyed on `(node_id, seq_nr)`. Once a relay has rebroadcast a packet, the cache entry blocks re-rebroadcast until expiry. 60 s is far longer than the realistic propagation time for a flood through any plausible v1 topology; by the time an entry expires, the network has long since quieted. If field testing shows the cache is too small or the window too short under realistic load, expiry and capacity are the parameters to tune. A TTL byte is a future protocol-version concern, not a v1 fix.

### Half-duplex reality

During TX (~371 ms at SF10), the relay cannot receive. Packets arriving during that window are lost. Acceptable — the tag retransmits on its own schedule, and a relay's rebroadcast still costs the same RX-deaf budget. See [§13](#13-duty-cycle-budget) for airtime accounting.

### Debug logging (serial only)

`len` is frame length on the wire (always 22 in v1).

```
ENQ len=22 CRC=OK node=1 seq=42 fix=YES sos=NO
FWD len=22 node=1 seq=42 CAD=clear
FWD len=22 node=1 seq=42 CAD=busy,retry=2
DROP len=22 CRC=FAIL
DROP len=? UNKNOWN_TYPE ver=2 type=99
DUP  len=22 node=1 seq=42
SELF len=22 node=2 seq=42  (own node_id, dropped)
EXP  len=22 node=1 seq=40
SELF_ANNOUNCE len=22 node=2 seq=17
```

---

## 10. Gateway responsibilities

The gateway is a handheld Pi-class SBC with a Dragino LoRa/GPS HAT (SX1276) and a touchscreen, running Yocto Linux. Substrate (Pi 5 / CM5 / Zero 2W) and display class (size, orientation, panel) are open per [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md) (pending ADR-015). Pi 4 is retired (out of order; see [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md)). See [ADR-004](decisions/ADR-004-gateway-platform.md).

### What the gateway does

1. Read system time from the DS3231 RTC at boot (see [ADR-011](decisions/ADR-011-gateway-time-source.md)); optionally discipline the RTC from the Dragino HAT's Quectel L80-M39 GPS via `chrony`/`gpsd` when a fix is available
2. Listen continuously for LoRa packets via SX1276 over SPI (the gateway uses the same `lora-phy` crate as the tag/relay — it supports SX127x as well as SX126x, so there is no separate driver to write)
3. Validate frame integrity (MAGIC, VER, TYPE, LEN, CRC-16/CCITT-FALSE)
4. Parse the payload — one known type in v1: POSITION. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md). Unknown VER or TYPE is dropped and logged.
5. Deduplicate locally using a **recent-window** filter on `(node_id, seq_nr)` over the last 24 h — *not* a permanent table-wide UNIQUE index; see [ADR-009](decisions/ADR-009-database-sqlite.md) for why
6. Write to SQLite — one core table, `tag_reports` (see schema below)
7. Kiosk UI thread reads SQLite and renders the map ([ADR-005](decisions/ADR-005-map-and-ui.md), [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md)). The UI additionally refuses to render "last seen X ago" strings if the system clock is unset (see [ADR-011](decisions/ADR-011-gateway-time-source.md))

The gateway does not distinguish "directly heard" from "via relay" — that information is not on the wire. If the gateway hears the same POSITION via the tag directly AND via a relay's rebroadcast, the row is stored once (dedup wins) and the duplicate is silently discarded. Per [ADR-013 §8](decisions/ADR-013-multi-hop-flood-via-packet-id.md). The `gateway_rssi_dbm` / `gateway_snr_db` columns describe the **last hop into the gateway only** and are operational telemetry, not coverage analysis.

### What the gateway does NOT do

Pending ADR-016 splits the network surface into four categories:

- **(a) No inbound network surface.** No HTTP server. No SSH publicly accessible (dev-only). No service that accepts a TCP connection from anywhere. Closed.
- **(b) No internet-bound network calls.** No POST to any external endpoint. No phone-home. No NTP — see [ADR-011](decisions/ADR-011-gateway-time-source.md). Closed.
- **(c) No cloud-hosted dependency.** No external service the gateway needs to reach to function. No third-party API, no auth provider, no telemetry sink. Closed.
- **(d) Outbound LAN multicast / unicast under explicit gate.** This is the **new** surface introduced under pending ADR-016: CoT/TAK to TAK-compatible clients on the same WiFi LAN. The emitter task fires only when WiFi is associated + has-default-gateway + has-DHCP-lease, AND a config-file flag explicitly opts in. (Was a 3-input gate also requiring `POWER_GOOD` from the power-monitor task; **the `POWER_GOOD` input was retired 2026-05-14** when the magnetic-pogo charging connector was dropped from the v1 gateway — without an in-shell charging input there is no "external power present" state for the SBC to read. See [^gate-2026-05-14].) Destinations are validated to be RFC1918 / link-local / multicast in code, not by trust. Cadence and message format follow [`spikes/tak-cot-integration-spike.md`](spikes/tak-cot-integration-spike.md). The gateway does not accept any inbound CoT.

[^gate-2026-05-14]: Gate re-scope: see [`dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md), [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md), `spikes/gateway-handheld-power-architecture-spike.md` 2026-05-14 amendment, `spikes/tak-cot-integration-spike.md` 2026-05-14 amendment, `spikes/gateway-runtime-task-architecture-spike.md` 2026-05-14 amendment.

And, unchanged across the pivot:

- No ACK back to relays
- No downlink to tags
- No alert escalation (no SMS, no Telegram, no email) — alerts show on the map, nothing more

### Database schema (v1 draft)

One core table, `tag_reports`, holding one row per accepted POSITION. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), tag and relay self-reports both arrive as POSITION packets and both produce rows in `tag_reports` — `node_id` distinguishes them, and `nodes.toml` (see §11) maps `node_id` to display label and UI icon.

`seq_nr` columns hold the wire `u32` value; SQLite `INTEGER` is 64-bit signed, so `u32` fits natively. The `received_at` column is filled from the RTC-disciplined system clock (see [ADR-011](decisions/ADR-011-gateway-time-source.md)).

```sql
CREATE TABLE tag_reports (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    received_at      TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    gateway_rssi_dbm INTEGER,                  -- last hop into gateway only
    gateway_snr_db   REAL,                     -- last hop into gateway only
    node_id          INTEGER NOT NULL,
    seq_nr           INTEGER NOT NULL,         -- u32 wire value (fits in SQLite INTEGER)
    flags            INTEGER NOT NULL,
    gps_valid        INTEGER NOT NULL,
    sos_active       INTEGER NOT NULL,         -- flag.SOS at reception time
    low_battery      INTEGER NOT NULL,
    lat_e7           INTEGER,
    lon_e7           INTEGER,
    alt_m            INTEGER,
    raw_packet_hex   TEXT
);
-- No permanent UNIQUE index on (node_id, seq_nr). Dedup is recent-window only.
CREATE INDEX idx_tag_reports_dedup ON tag_reports(node_id, seq_nr, received_at DESC);
CREATE INDEX idx_tag_reports_time  ON tag_reports(node_id, received_at DESC);
```

Dedup is enforced in the INSERT path, not by a table constraint:

```sql
INSERT INTO tag_reports (...)
SELECT ...
WHERE NOT EXISTS (
    SELECT 1 FROM tag_reports
    WHERE node_id = :node_id
      AND seq_nr  = :seq_nr
      AND received_at > datetime('now', '-1 day')
);
```

Rationale: a permanent UNIQUE index collides with `seq_nr` wraparound (even at `u32`, it would eventually bite; at `u16` it bites within months). A 24 h window is longer than any plausible packet-in-flight delay and short enough that the same `seq_nr` genuinely recurring after wrap is a fresh report, not a duplicate. See [ADR-009](decisions/ADR-009-database-sqlite.md).

Sentinel lat/lon values (`0x7FFFFFFF`) are stored as-is; the UI layer interprets them as "no fix" and does not place a marker.

**A future reception-log table for per-hop RSSI/SNR coverage analysis will be designed when v1 forwarding is working.** Per [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md), it is not in v1 scope. The single `tag_reports` table above is the entire v1 schema.

### SX1276 ↔ SX1262 syncword

The Dragino HAT has an SX1276 (1-byte syncword, 0x12 private). The Heltec boards have SX1262 (2-byte syncword, 0x1424 private). These are **on-air equivalent for the "private" networks**. `lora-phy` handles the SX1262 side; the Rust gateway driver must program 0x12 on the SX1276 side. Verify empirically during v0 bring-up.

---

## 11. Kiosk UI

See [ADR-005](decisions/ADR-005-map-and-ui.md) and [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md).

Native Rust, single fullscreen process on the handheld touchscreen, running as a module of the gateway binary (not a separate crate — see §17). Starting library bet: `egui` + `walkers`. Offline **PMTiles** archive bundled in the Yocto image (per [ADR-005](decisions/ADR-005-map-and-ui.md); `walkers` documents native `.pmtiles` support, so no custom tile provider is written unless the kiosk spike fails). No Chromium, no browser, no web. Display class (size, orientation, panel) is open per pending ADR-015 — the prior "7" DSI 1024×600 wall- or shelf-mount" assumption no longer holds for a handheld; retarget owned by [`spikes/pmtiles-walkers-spike.md`](spikes/pmtiles-walkers-spike.md). The read-only-map invariant from [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md) is preserved by [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md): commissioning interaction lives inside an explicit modal opened from a marker by deliberate gesture, not on the map itself.

### Node presentation via `nodes.toml`

Per [ADR-013 §9](decisions/ADR-013-multi-hop-flood-via-packet-id.md), node visual style is driven by a TOML file on the gateway, keyed on `node_id`. There is no role byte on the wire.

```toml
[nodes.1] label = "tag-1"        ui_kind = "hiker"
[nodes.2] label = "garden relay" ui_kind = "relay"
[nodes.3] label = "drone pod"    ui_kind = "drone-relay"
```

`ui_kind` selects the marker glyph: `hiker` → small filled dot; `relay` → pole/antenna icon; `drone-relay` → drone icon. Adding or reflashing a node is a config edit on the Pi — no protocol change.

### What the UI shows

For each `node_id` present in `tag_reports`, the kiosk renders a marker at the most recent valid-fix position, with the icon and label from `nodes.toml`. Specifically:

- Last known position as a marker, colour-coded by staleness.
- SOS-active nodes (classified per §7 *Distress semantics*) flash red.
- Nodes with `GPS_VALID=0` in the most recent row appear in a side list ("tag-1: last seen 14:32, no GPS fix") but do NOT get a map marker at sentinel coordinates.
- For `ui_kind = "hiker"` nodes, recent track history as a line connecting recent valid-fix positions (discrete points, not interpolated).
- For `ui_kind = "drone-relay"` nodes, the marker is redrawn as fresh `tag_reports` rows arrive; a short trailing track is shown.
- For `ui_kind = "relay"` nodes (slow self-announce), the marker is greyed out if no row has arrived in the last N days.

**Clock-not-set banner.** If the RTC read at boot failed, or the system clock sits before a baked-in sentinel date (e.g. the firmware's compile date), the kiosk replaces every relative-time string ("last seen X min ago") with a literal "clock not set" token and shows a persistent banner at the top of the screen asking the operator to SSH in and set the time. Silent fallback to wrong times is worse than a visible warning. See [ADR-011](decisions/ADR-011-gateway-time-source.md).

**What the UI must never do:**

- Show a stale position as if it were current
- Show sentinel coordinates as a map marker
- Imply real-time tracking
- Render "last seen" strings against an unset or clearly-wrong system clock

---

## 12. Operating modes (regulatory and power)

Per [ADR-010](decisions/ADR-010-sos-encoding.md), **all tag uplink — heartbeat and SOS — transmits on a single channel in ETSI sub-band M.** There is no separate SOS band in v1. This closes off a structural phase-lock bug that would otherwise let a distressed tag transmit correctly while the relay never hears it. A future ADR may re-open a dedicated SOS channel on sub-band P if field measurements show the link budget is inadequate; v1 accepts the +14 dBm ERP ceiling as the cost of a reliably received distress event.

### Tag — heartbeat (default)

- Wake every interval drawn uniformly from **[300 s, 330 s]** (positive-only jitter; decorrelates from any plausible relay scan cycle and prevents synchronised collisions)
- Acquire GNSS fix (timeout 90 s)
- Transmit on 868.1 MHz (ETSI sub-band M), SF10, +14 dBm ERP
- Airtime: ~371 ms. Duty cycle: ~0.12% (legal limit: 1%)
- `flags.SOS = 0`
- Deep sleep

### Tag — SOS

- **First SOS frame on entry: transmit immediately**, with `GPS_VALID=0` + sentinels and `flags.SOS=1`, no first-frame jitter, no waiting for a GNSS fix. Distress must not be gated by GPS acquisition. See [ADR-010](decisions/ADR-010-sos-encoding.md) and §8 state machine.
- After the immediate first frame, wake every interval drawn uniformly from **[45 s, 60 s]** per transmission (positive-only jitter; minimum interval bounded at 45 s for duty-cycle compliance per [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md)). Mean ≈ 52.5 s.
- For each subsequent frame: acquire GNSS fix (timeout 90 s); transmit anyway on timeout with sentinels + `GPS_VALID=0`
- Transmit on **868.1 MHz** (same channel as heartbeat), SF10, +14 dBm ERP
- Airtime per frame ~371 ms; max 80 TX/hour × 371 ms = 29.68 s/hour ≈ 0.82% duty cycle (1% legal limit on sub-band M). Headroom ~0.18%; the tag's MAC layer must still enforce the hourly budget and defer if a burst of retries would exceed it. See §13.
- `flags.SOS = 1` (bit 1 of the `flags` byte) on every frame while distress is active
- Deep sleep between frames

SOS persists across deep sleep via NVS flag. The `tag` crate enforces three invariants by unit test:

1. **Constant-cadence regression test.** A non-jittered SOS schedule must fail.
2. **Immediate-first-frame test.** Time from SOS button press to first TX must be bounded by GNSS-independent firmware delay only (no GPS_ACQUIRE call on the entry path).
3. **Duty-cycle cap test.** Over a simulated hour with worst-case minimum-interval-back-to-back transmissions (45 s spacing throughout), total TX airtime must not exceed the sub-band M 1% budget.

See [ADR-010](decisions/ADR-010-sos-encoding.md) and [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md).

### ETSI EN 300 220 compliance

| Sub-band | Frequency | Max ERP | Duty cycle | v1 use |
|----------|-----------|---------|------------|--------|
| M | 868.0–868.6 MHz | 25 mW (+14 dBm) | 1% | Heartbeat *and* SOS |
| P | 869.4–869.65 MHz | 500 mW (+27 dBm) | 10% | Not used in v1 (see ADR-010) |

### Relay duty cycle

At 10 tags on heartbeat with worst-case minimum interval (300 s), a relay rebroadcasts at most 120 unique packet_ids/hour. 120 rebroadcasts × 371 ms = 44.5 s airtime/h ≈ 1.24% — marginally over the 1% sub-band M duty-cycle budget. At v1 single-tag scale this is a non-issue; multi-tag relay duty-cycle budgeting is a v2 concern.

Note: each tag's heartbeat causes at most one relay rebroadcast per relay after dedup. The relay is bounded by tag emission rate, not by its own scan rate. Detailed per-mode arithmetic lives in §13.

### Link budget

| Mode | TX power | ERP (after antenna) | Rx sensitivity | Link budget | Practical mountain LoS |
|------|----------|---------------------|----------------|-------------|-----------------------|
| Heartbeat (SF10, +14 dBm ERP) | 14 dBm | ~14 dBm | -134 dBm | 148 dB | 3–8 km |
| SOS (SF10, +14 dBm ERP) | 14 dBm | ~14 dBm | -134 dBm | 148 dB | 3–8 km |

Identical mode == identical link budget. SOS is distinguishable from heartbeat only by the flag bit in the payload, not by radio parameters. This is the deliberate v1 trade described in [ADR-010](decisions/ADR-010-sos-encoding.md): a reliably received SOS with a smaller link budget beats a higher-power SOS that can never be heard because of scan-phase lock.

### Channel listening

All v1 LoRa transmissions use **868.1 MHz**. Receivers in v1 — relays and gateway — listen continuously on 868.1 MHz. Tags are transmit-only during their wake window and otherwise sleep: no tag RX, no downlink, no scan rotation, no CH_TAG / CH_FWD split. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) the only semantic discriminator on the wire is the `TYPE` byte; in v1 there is exactly one valid TYPE (POSITION = 0x01).

---

## 13. Duty-cycle budget

Per [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md), this section is a **mandatory protocol gate**. Any change to packet size, cadence, retransmit behaviour, hop limit (if hops are ever introduced), or any other variable in the airtime calculation MUST update this section in the same edit. An ADR or commit that changes one of these without updating §13 is incomplete.

### Canonical airtime calculator parameters

All airtime values in the table below are produced from a single canonical LoRa airtime calculator, with explicit parameters:

```
SF                          = 10
BW                          = 125 kHz
CR                          = 4/5
preamble                    = 8 symbols
explicit header             = on
CRC                         = on
low-data-rate optimisation  = off
```

A 22-byte frame (4 header + 16 payload + 2 CRC) at these parameters is ~371 ms on-air. Disagreement between rough estimates and the calculator is a sign that the calculator is not being used; resolve by running the calculator.

### Budget table (v1)

| Transmitter / mode | Frame | Airtime | Cadence | Max TX/h | Airtime/h | Duty | Verdict |
|--------------------|-------|---------|---------|----------|-----------|------|---------|
| Tag heartbeat POSITION | 22 B | ~371 ms | [300 s, 330 s] positive-only jitter | 12 | ≈ 4.5 s | ≈ 0.12% | OK |
| Tag SOS POSITION | 22 B | ~371 ms | min 45 s, jitter +0…15 s (range 45–60 s) | 80 | ≈ 29.7 s | ≈ 0.82% | OK (1% cap) |
| Relay rebroadcast under tag heartbeat | 22 B | ~371 ms | 1 per unique packet_id | 12 / source tag | ≈ 4.5 s / source | ≈ 0.12% / source | OK |
| Relay rebroadcast under tag SOS | 22 B | ~371 ms | 1 per unique packet_id | 80 / source tag | ≈ 29.7 s / source | ≈ 0.82% / source | OK at 1 source |
| Relay self-POSITION (slow self-announce) | 22 B | ~371 ms | 1800 s (30 min) | 2 | ≈ 0.74 s | ≈ 0.02% | OK |

**Two-simultaneous-SOS-tags scenario does NOT fit.** Two SOS tags in flight at the same relay sum to ~1.64% — over budget. v1 garden test deploys one tag; multi-tag scale is a v2 concern with its own prioritisation/throttling rules.

**Future packet types** (a future reception-log telemetry layer, a future maintenance protocol, etc.) will each need a row in this table at the time they are introduced. No exceptions.

---

## 14. Graceful degradation

### Tag loses GPS fix

Transmits with `GPS_VALID=0` and sentinels. Kiosk shows "last seen, no fix"; map marker stays at last valid position.

### Relay dies

Packets may still reach the gateway via a different relay or directly. Coverage degrades; system does not break. Relays self-announce sparsely via POSITION every 1800 s (per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md)). The gateway has no active relay-health protocol in v1; a stale relay marker (no fresh POSITION rows for that `node_id` in many hours) suggests degraded coverage but is not a definitive health check. Coverage gaps also become visible as previously-seen tags go stale.

### Gateway loses power

Coverage area goes dark. Tags and relays keep transmitting and forwarding — they do not know. On power return, the kiosk boots, Rust binary starts, SQLite state is intact, and new sightings begin writing again. Historical data is preserved.

### WiFi dies (the snowstorm scenario)

The LoRa-side telemetry path is unaffected. The relays on poles keep running. The gateway keeps receiving. The kiosk keeps drawing. **This is the headline design property of the local-first architecture.**

The one exception: the optional CoT/TAK export under pending ADR-016 silences when WiFi is gone (it is gated on WiFi association + power-good + manual opt-in; if WiFi drops, the gate closes and the emitter task stops). Phones running ATAK / iTAK / WinTAK on the same LAN therefore stop receiving updates until WiFi is back. This is a narrowing of the pre-pivot "WiFi loss does not take the system down" wording: the property still holds for the LoRa-side telemetry path, which is the load-bearing one. The export path is auxiliary and explicitly off by default.

### Tag battery dies

Last sighting shows `LOW_BATTERY=1`. After that, silence. Kiosk shows the tag as stale with a battery warning.

### Multiple relays hear the same packet

Each relay rebroadcasts independently after its own seen_cache check. The gateway may receive multiple copies via different paths. Dedup on `(node_id, seq_nr)` at the gateway handles this cleanly. Multiple receptions are not a bug — they are evidence of good coverage.

### Trust model

**This is a prototype / garden demonstrator. It is not a field-deployable SAR tool as-is.**

No encryption, no authentication in v1. Anyone with a LoRa radio on 868 MHz and knowledge of the frame format can inject packets with valid MAGIC and a matching CRC-16/CCITT-FALSE. The CRC is an integrity check against noise and bit-flips; it is explicitly **not** a check against forgery or replay. Any real SAR-adjacent deployment — i.e. one that rescue coordinators, hut staff, or hikers would actually rely on — requires per-packet authentication (HMAC or signature), key management, and replay protection before it can be put into operational use. That work is deferred to a future hardening pass and gated by a v2+ ADR.

Even within the prototype envelope, the gateway treats data as "best available information", not ground truth. Positions are where the tag said it was, not where the person necessarily is. The hiker may have left the tag behind. The UI language avoids any claim of certainty.

---

## 15. Implementation roadmap

### v0 — Indoor bring-up (3 nodes on the desk)

**Goal:** *"Gateway prints a parsed `POSITION` packet from the tag, via the relay, on the desk."*

Tag transmits sentinel-value packets every 5 s (`GPS_VALID=0`, lat/lon/alt sentinels). Relay validates + queues + CAD + forwards. Gateway receives and parses. No kiosk UI yet. Two-hop verification: tag off → silence; relay off → silence; both on → packets.

**Hard gate:** gateway stdout shows `node=1 seq=N fix=NO lat=SENTINEL lon=SENTINEL crc=OK`, confirming CRC-16/CCITT-FALSE round-trips end-to-end.

Also in scope for v0 (so later work doesn't bog down on it):

- DS3231 RTC wired to the Pi I²C bus and reading correctly; `hwclock --systohc` / `--hctosys` integrated into the Yocto boot sequence. See [ADR-011](decisions/ADR-011-gateway-time-source.md).
- PMTiles-on-`walkers` kiosk spike on the dev workstation first, then on the Pi. Must render a small local `.pmtiles` before any other kiosk work lands. See [ADR-005](decisions/ADR-005-map-and-ui.md). If the spike fails on the Pi's GPU stack, this is the moment to fall back to `iced`/`slint` or write a custom MBTiles tile provider — not at v1.

### v0.5 — Gateway writes to SQLite; kiosk renders a marker

**Goal:** *"A dot appears on the touchscreen for every packet the gateway receives."*

Rust gateway binary does RX + dedup + DB write. Kiosk module (same binary) reads SQLite and draws a marker. **The tag transmits a hardcoded valid test coordinate** (e.g. Pieter's desk lat/lon) with `GPS_VALID=1` so the kiosk has a real point to render. Do not render sentinel coordinates as a placeholder marker — §6 and §11 forbid it, and the v0 firmware already emits sentinel frames that the v0.5 kiosk must correctly route to the "no fix" side list rather than to a map marker. Proves the pipeline end-to-end on both paths (valid-fix → marker, no-fix → side list).

Also in scope for v0.5:

- Recent-window dedup works correctly: a retransmitted `(node_id, seq_nr)` inside 24 h is suppressed; a deliberate `seq_nr` reset outside the window is accepted as fresh.
- Clock-not-set banner renders if the RTC is absent at boot.

### v1a — Single-relay garden test (multi-hop protocol, single hop physically deployed)

**Goal:** *"Walk around the garden with the tag; the dot moves on the touchscreen; the relay rebroadcasts on solar."*

Tag with real GPS (UC6580) and a piezo buzzer on a GPIO line, solar-powered relay on the designed garden pole, gateway carried by the operator (or placed near a window during early bring-up — handheld substrate per pending ADR-015), kiosk showing live position. Single packet type on the wire (POSITION); single forwarding hop physically exercised but the protocol is multi-hop-capable from day one. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md). No internet required on the LoRa side; the optional CoT/TAK export under pending ADR-016 stays OFF by default during the v1a acceptance test.

**Hard gates:**

- Tag broadcasts real coordinates from a real GNSS fix
- Relay rebroadcasts received POSITIONs by `packet_id`, dedup'd locally on `(node_id, seq_nr)` with 60 s expiry
- Gateway dedup'd writes to `tag_reports`
- Kiosk shows the dot moving with correct "last seen X ago" timestamps from the RTC-disciplined clock; node icon and label come from `nodes.toml`
- Tag SOS button (or bench trigger) flips marker to red within `DISTRESS_WINDOW`; tag buzzer pulses while SOS is active and silences on clear
- Relay 72 h unattended on solar
- Entire stack has never touched the internet during the test
- Duty-cycle behaviour matches §13 table within ±10% measured

### v1b — Two-relay chained garden test (drone-pod overlay)

**Goal:** *"Tag in a deliberate blind spot the gateway cannot hear directly; two relays in chain (paal-relay + drone-pod) bridge the gap."*

A 3rd Tracker V2 reflashed with a drone-pod `node_id`, attached to a drone with zip-ties or velcro under-mount, powered from a small 1S LiPo independent of the drone's flight battery. Optional barometer (BMP280 / BME280) on I²C for altitude observation; if absent, GNSS altitude is the fallback. The drone-pod runs the same relay firmware as the paal-relay — node presentation as a drone vs a pole is a single line in `nodes.toml` (`ui_kind = "drone-relay"`).

**v1b is gated on v1a hard gates passing.** No v1b firmware work begins before that — structural defence against scope creep.

**Hard gates:**

- Tag in blind spot (verified: gateway alone sees no packets)
- Both relays rebroadcast; loop prevention exercised — deliberate test where both relays hear each other and the seen_cache catches the loop (`DUP` log line on the second hearing of each `packet_id`)
- Kiosk shows tag position; the drone-pod node renders with the drone icon per `nodes.toml`
- Duty-cycle behaviour at the relays matches §13 within ±10% measured

### v2+ — Explicitly future

- BLE relay maintenance CLI
- Downlink control
- Cloud sync
- Phone-accessible read-only map over the hut's local network
- HMAC / auth
- Multi-gateway
- Extended payloads (HDOP, sat count, temperature)

---

## 16. Open technical risks

### 1. Relay echo/loop
Two relays hearing each other will bounce packets without the seen_cache. The cache exists per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) — single layer, dedup-only on `(node_id, seq_nr)`, 32 entries, 60 s expiry. v1b acceptance includes a deliberate two-relays-hearing-each-other test that exercises this. A TTL byte is a future protocol-version concern, triggered by measured need rather than speculation.

### 2. Relay duty cycle at scale
Per [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) and §13: single-tag SOS rebroadcast fits inside 1% sub-band M (~0.82%). At 10 tags on heartbeat (worst-case 300 s minimum interval), a relay rebroadcasts ~120 packets/hour ≈ 1.24% — marginally over budget. Two simultaneous SOS tags is the same shape (~1.64%). Both are multi-tag-scale concerns outside v1, with their own prioritisation/throttling design when they land.

### 3. Multi-hop latency and reliability
Each hop adds 50–200 ms plus airtime. Three hops at 90%/hop = 73% end-to-end. Reliability falloff with hops is a phenomenon to measure, not hide from — v1b will produce the first real numbers. Acceptable because the tag retransmits on its own schedule.

### 4. GNSS cold start
30–90 s for first fix. Battery-backed RAM mitigates across deep sleep. Unavoidable after extended power-off.

### 5. Battery in cold temperatures
Li-Ion loses 20–40% below 0°C. Real concern for alpine winter.

### 6. SX1276 ↔ SX1262 syncword
Private-network syncwords are on-air equivalent (0x12 on SX1276 ≡ 0x1424 on SX1262). `lora-phy` handles SX1262; the Rust gateway driver must set 0x12 explicitly on the SX1276. Verify during v0.

### 7. Dragino HAT pin numbering
Dragino docs use WiringPi numbers, not BCM. Verify against `rpi-pal` before writing SPI glue. (`rpi-pal` is the maintained fork of the archived `rppal`; same API, drop-in, Pi 5 compatible — see dev log 2026-05-05.)

### 8. Native-Rust map tile rendering on Pi GPU
`walkers` + `wgpu` on the Pi's graphics stack is unproven for this team. Prototype early so we can fall back to `iced` or `slint` if `egui`/`walkers` is flaky on the Pi.

### 9. Yocto upfront cost
First image build is slow; device tree overlays for the DSI touchscreen need hand-tuning. Pay once, benefit forever.

### 10. Wooden pole weather exposure
Pressure-treated wood, south-facing solar panel, pole anchoring to survive wind. Hardware problem, solvable with normal outdoor fastening.

### 11. Tracker V2 in the Solar Kit enclosure
Default bracket does not fit the Tracker V2 outline — see [ADR-003](decisions/ADR-003-relay-hardware.md). Mount via adhesive standoffs. Verify during first assembly.

### 12. Gateway time source
Pi has no on-chip RTC and we explicitly have no NTP. Without the DS3231 module + coin cell from [ADR-011](decisions/ADR-011-gateway-time-source.md), `received_at` is unreliable across power cycles and every "last seen" string the kiosk renders is a lie. The RTC is a load-bearing component, not a convenience; absent RTC is a v1 blocker.

### 13. IPEX1.0 ↔ SMA antenna path
Tracker V2 exposes LoRa on IPEX1.0 (u.FL); the Solar Kit bulkhead is SMA. The pigtail connecting the two is ordered via [bom.md](bom.md) but gender (female vs male bulkhead) must be verified on the shipped Solar Kit panel before antennas are fitted. Bench-check before fitting.

### 14. Handheld gateway battery + cold-charge envelope
Pending ADR-015 puts the gateway on a battery + USB-C-PD charging path. The same lithium-cold-charge physics that drives `production-concerns.md` §2 for the relay 18650 applies to the gateway pack. Topology + protections + signal contract owned by [`spikes/gateway-handheld-power-architecture-spike.md`](spikes/gateway-handheld-power-architecture-spike.md); cold-charge cutoff via NTC is mandatory in the design, not optional. Runtime targets (≥4 h typical, ≥1 h peak) are working hypotheses, not committed numbers — bench measurement is part of the spike's pass criteria.

### 15. 3D-printed enclosure IP rating + IPEX strain relief on the gateway
Pending ADR-017 commits the gateway to a custom 3D-printed waterproof shell. IP target (65 / 67 / 54-fallback), material (PETG vs ASA), display window seal, USB-C bulkhead, and antenna bulkhead choices are open per [`spikes/gateway-handheld-enclosure-spike.md`](spikes/gateway-handheld-enclosure-spike.md). The gateway's external LoRa SMA pigtail through the printed shell shares the IPEX strain-relief problem class with the relay's pigtail (`production-concerns.md` §3); both promote into v1-active scope under the pivot.

### 16. Pi-class onboard BLE/WiFi attenuation through a plastic shell
The handheld gateway's commissioning surface (BLE central) and base-mode export gate (WiFi monitor) both rely on the SBC's onboard radios behind a 3D-printed plastic wall. PETG/ASA at 3–4 mm thickness should attenuate 2.4 GHz only modestly, but with a metallic battery cell on the same side of the antenna, detuning is plausible. Bench-measure RSSI degradation per [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md) before committing the substrate. If through-shell BLE fails arm's-length commissioning, an external USB BLE/WiFi dongle with a whip becomes the path, and [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md) inherits the constraint.

---

## 17. Appendix: repo layout

Cargo workspace. See [software/repo-layout.md](software/repo-layout.md) for the full story.

```
lora-sar/
├── Cargo.toml                       ← workspace root, resolver = "2"
├── .cargo/config.toml               ← per-target settings
│
├── crates/
│   ├── protocol/                    ← no_std, shared by all binaries
│   │   └── src/lib.rs
│   ├── persistence/                 ← SQLite schema + migrations
│   │   └── src/lib.rs
│   └── heltec-wireless-tracker-v2-bsp/
│       └── src/lib.rs
│
├── firmware/
│   ├── tag/                         ← binary, targets xtensa-esp32s3-none-elf
│   │   └── src/main.rs
│   └── relay/                       ← binary, same target, different logic
│       └── src/main.rs
│
└── gateway/                         ← binary, targets aarch64 or armv7
    └── src/
        ├── main.rs                  ← entry
        ├── lora_rx.rs               ← SPI to SX1276
        ├── db.rs                    ← rusqlite / sqlx
        └── ui/                      ← egui + walkers
```

---

## Appendix: what changed (v9 → v10)

| Change | Why |
|--------|-----|
| Three packet types (POSITION, RELAY_INFO with role byte, SIGHTING) → **one packet type: POSITION** | ADR-012's role enum and SIGHTING were modelling for the sake of modelling, triggered by adding drones to the conversation. Tags and relays both self-report as POSITION; relays additionally rebroadcast received POSITION unchanged. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) |
| Three-table DB schema (`tag_reports`, `relay_reports`, `sightings`) → **one core table `tag_reports`** keyed on `(node_id, seq_nr)` | Symmetric with the protocol collapse. Relay self-reports flow into the same table; presentation distinguished by `nodes.toml` |
| TYPE-dispatched forwarding queue with same-tag replacement and SIGHTING emission → **byte-identical rebroadcast of valid POSITION; drop unknown TYPE/VER; loop prevention is dedup-only on `(node_id, seq_nr)`, 60 s expiry** | First-attempt FORWARD envelope (path arrays + per-hop RSSI) was 45 B and put relays over duty-cycle budget at SOS rates — same mistake in different syntax. Forwarding stays minimal; observation is a separate concern. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) |
| Wire-level role enum (HIKER_TAG / FIXED_RELAY / AERIAL_RELAY) → **no role byte on the wire**; node presentation is gateway TOML config (`nodes.toml`) keyed on `node_id` | Adding a drone-pod relay is a config edit on the Pi plus a fresh `node_id` flashed into a Tracker V2. No protocol change to add new node kinds |
| SOS cadence "45 s ± 25%" → **"minimum interval 45 s, positive-only jitter to 60 s"** | "45 ± 11 s" gives a worst-case minimum of 34 s, which is over the 1% sub-band M cap. Bounding the minimum bounds the worst-case rate. See [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) |
| Duty-cycle math tracked informally in prose → **mandatory budget table in §13**, updated as a protocol gate on every change | Three failures of the same kind (ADR-010's two cadence revisions, ADR-012's casual "well within duty cycle" claim, ADR-013-draft's FORWARD blowing the budget) is a process problem. See [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) |
| "Heard via relay" overlay reading from `sightings` table → **dropped** | The directly-vs-via-relay distinction is not on the wire in v1; the gateway records one row per `(node_id, seq_nr)`. Coverage analysis (who heard what, with what link quality) is deferred to a future reception-log layer (see [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md)) |
| AERIAL_RELAY treated as a build-time role with separate firmware mode → **same relay firmware, different node_id, `ui_kind = "drone-relay"` in `nodes.toml`** | Drones are nodes that move; the protocol does not need to know |
| §13 (Graceful degradation) renumbered to §14; §14 (Implementation roadmap) → §15; §15 (Open technical risks) → §16; §16 (Appendix repo layout) → §17 | New §13 is the duty-cycle budget table |

## Appendix: what changed (v8 → v9)

| Change | Why |
|--------|-----|
| One node role implied → **three explicit roles, build-time-configured: HIKER_TAG, FIXED_RELAY, AERIAL_RELAY** | Aerial relays are coming in v1b and need to be a first-class concept, not a special case on top of "relay". See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) |
| Two packet types (POSITION, RELAY_INFO) → **three: POSITION, RELAY_INFO (with `role` byte; total frame 26 B), SIGHTING (new, observer-centric, total frame 27 B)** | v8's `sightings` table conflated tag self-claim with observer record; with moving observers (drones) that conflation breaks. Each observer now owns the truth about its own position. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) |
| One DB table for "sightings" → **three tables: `tag_reports`, `relay_reports`, `sightings`** | Symmetric with the three packet types. The kiosk renders markers from `tag_reports` / `relay_reports`; `sightings` powers the optional "heard via" overlay |
| v1 single phase → **v1a (ground stack + tag buzzer) and v1b (drone-pod aerial overlay), v1b gated on v1a passing** | Doing both concurrently is too many simultaneous debug surfaces. Structural defence against scope creep. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) |
| Tag SOS had no audible cue → **HIKER_TAG drives a GPIO-connected piezo buzzer that pulses while distress is active** | Last-meter audible cue for the searcher. Tag-side only — relays do **not** have buzzers as search beacons (would draw the searcher to the wrong location) |
| §2 non-goals expanded by six bullets | Closes off RF direction finding, 121.5 MHz emissions, RSSI-based fine localization, audio detection by relays, autonomous drone behaviour, role-by-altitude auto-detection. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) |

## Appendix: what changed (v7 → v8)

| Change | Why |
|--------|-----|
| SOS on sub-band P (869.525 MHz, SF12, +22 dBm) → **SOS on sub-band M, same 868.1 MHz channel as heartbeat, flag bit in payload, jittered cadence** | v7's design had a structural phase-lock bug against the relay's 55 s / 5 s scan cycle; a distressed tag could transmit correctly while never being heard. See [ADR-010](decisions/ADR-010-sos-encoding.md) |
| Relay / gateway dual-channel scan (55 s / 5 s) → **single-channel continuous RX on 868.1 MHz** | Direct consequence of the SOS collapse; nothing else to scan for in v1 |
| `seq_nr: u16 BE` → **`u32 BE`**; total POSITION frame 20 B → 22 B; total RELAY_INFO frame 23 B → 25 B | `u16` wraps in ~22 days at 30 s SOS cadence, which combined with the permanent UNIQUE dedup index silently dropped real new sightings as "duplicates". See [ADR-009](decisions/ADR-009-database-sqlite.md) |
| SQLite `UNIQUE INDEX idx_dedup ON sightings(tag_id, seq_nr)` → **removed; recent-window dedup only (24 h)** | Same bug as above — a permanent uniqueness constraint fights `seq_nr` wraparound |
| CRC loosely described as "CRC-CCITT" → **CRC-16/CCITT-FALSE, fully specified (poly 0x1021, init 0xFFFF, no reflect, xorout 0x0000)** | Prevents implementation drift between tag / relay / gateway CRC libraries |
| Relay queue policy read fixed POSITION byte offsets for unknown TYPE/VER → **TYPE-dispatched; unknown types forwarded as FIFO with hash-based echo suppression** | Forward-compatibility: future packet types don't get misinterpreted as if bytes 4–9 were a POSITION tag_id/seq_nr/flags |
| Gateway had no time source described → **DS3231 RTC primary, GPS/PPS opportunistic** | A Pi with no NTP and no RTC boots with bogus time; every "last seen" string would be a lie after every power cycle. See [ADR-011](decisions/ADR-011-gateway-time-source.md) |
| Kiosk tiles: MBTiles → **PMTiles** | `walkers` documents native `.pmtiles` support; MBTiles would require writing a custom tile provider. See [ADR-005](decisions/ADR-005-map-and-ui.md) |
| v0.5 acceptance used sentinel coordinates as a placeholder map marker → **v0.5 uses a hardcoded valid test coordinate with `GPS_VALID=1`** | Contradicted §6/§11's own "never render sentinels as a marker" rule |
| Gateway RSSI/SNR implied tag-link quality → **documented as last-hop only** | A relay-forwarded frame's RSSI describes relay→gateway, not tag→relay |
| Trust model wording "acceptable for v1" → **"prototype / garden demonstrator; real SAR-adjacent deployment requires auth before operational use"** | Distinguishes the CRC's integrity role from an authentication role it cannot fulfil |
| Gateway stack omitted `lora-phy` → **`lora-phy` on gateway as well (it supports SX126x *and* SX127x)** | Single radio crate across firmware and gateway; no hand-written SX1276 driver |

### Appendix: what changed (v6 → v7)

| Change | Why |
|--------|-----|
| Tag and relay hardware: V4 Expansion Kit / V4 + L76K → **Wireless Tracker V2** | Onboard GNSS, onboard battery mgmt, single board across tag and relay. See [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md) |
| GNSS chip: L76K / UC6580 (mixed) → **UC6580 everywhere** | Integrated on Tracker V2; one chip, one NMEA path |
| Firmware stack: C++ / Arduino / PlatformIO → **Rust + esp-hal + Embassy + lora-phy** (was already decided in ADR-001, now reflected throughout) | [ADR-001](decisions/ADR-001-firmware-language.md) |
| Gateway language: C → **Rust** | Single-language stack |
| Gateway OS: Raspbian/Yocto split → **Yocto from day one** | [ADR-004](decisions/ADR-004-gateway-platform.md) |
| Server + web UI (FastAPI + Leaflet) → **removed, no server, local kiosk only** | [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md) |
| Map library: Leaflet → MapLibre GL JS → **native Rust (egui + walkers)** | [ADR-005](decisions/ADR-005-map-and-ui.md) |
| Database: implied Postgres for server → **SQLite on the Pi, single file** | [ADR-009](decisions/ADR-009-database-sqlite.md) |
| Downlink control posture: "planned evolution path, compatible with design" → **explicitly out of v1, pure uplink** | [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md) |
| Cloud sync posture: "opportunistic when connectivity allows" → **removed; system is local-only** | [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md) |
| Relay GNSS usage: implied forwarding-time → **commissioning/maintenance only, OFF during forwarding** | [ADR-006](decisions/ADR-006-relay-has-gnss.md) |
