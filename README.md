---
title: "LoRa SAR — Project Index"
status: living
type: index
tags: [index, status]
---

# LoRa SAR — Search & Rescue telemetry network

**What it is.** Hiker-carried GPS tags beacon over LoRa 868 MHz. Solar-powered relays on poles forward those beacons. A handheld Rust gateway with a touchscreen receives them and renders a live read-only map. No cloud, no inbound network, no phone app, no downlink on the LoRa side. Local-first by default. Outbound LAN-bounded CoT/TAK export to TAK-compatible clients on the same WiFi is a v1 feature gated on WiFi + external power + manual opt-in (pending ADR-016); when any gate input is false, the export path is silent.[^pivot]

**Who it's for.** Hut staff or rescue-adjacent operators who carry the gateway and glance at the screen to see where hikers are. The mountain hut is one possible deployment site, not the only one. Not sysadmins. Not web users.

**Why this shape.** Snowstorm hits, WiFi dies, 4G dies. The relays stay up, the tags stay up, the gateway stays up, the map keeps working — the LoRa-side telemetry path has no internet dependency.[^export-gate] If any part of that path needed an internet handshake, the system would fail in the exact moment it matters.

**Mission goals.** (1) track hikers via periodic sightings, (2) distress signalling, (3) an honest operational map.

[^pivot]: 2026-05-06 form-factor pivot — see [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md). Three new ADRs proposed: ADR-015 (handheld substrate + form factor; supersedes-in-part ADR-004; refines-in-part ADR-005/006/007), ADR-016 (base-mode export gate; supersedes-in-part ADR-008), ADR-017 (custom 3D-printed waterproof enclosures for gateway and tag; refines-in-part ADR-002).

[^export-gate]: Base-mode CoT/TAK export under pending ADR-016 is the **one** layer that depends on WiFi. WiFi gone → export path silent → everything else still works. The system property "WiFi loss does not take the system down" still holds; it is just narrower in scope (the LoRa-side telemetry path) than the pre-pivot wording suggested.

## Status at a glance

| Area                                                   | State                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Architecture                                           | Locked to the decisions in `decisions/`. ADRs 001–009 dated 2026-04-22; ADR-010 / ADR-011 added 2026-04-24; ADR-012 added 2026-04-25 then **superseded in part** by ADR-013 (multi-hop flood via packet_id dedup) and ADR-014 (duty-cycle budget gate) on 2026-04-26. **ARCHITECTURE.md v10** reflects the current ledger.                                                      |
| Firmware stack                                         | **Decided:** Rust + `esp-hal` + Embassy + `lora-phy` (from `lora-rs/lora-rs`) on tag and relay. See [ADR-001](decisions/ADR-001-firmware-language.md).                                                                                                                                                                                                                          |
| Gateway stack                                          | **Decided in part:** Rust binary on Yocto Linux, Dragino HAT for SX1276 + L80-M39 GNSS, single binary. `lora-phy` used on the gateway too (SX127x support). **Open per pending ADR-015** (form factor + substrate + display class + battery + enclosure): the pre-pivot "Pi 3B+/4 + 7" DSI wall-mount" framing is no longer the working assumption — the gateway is now a handheld portable. See [ADR-004](decisions/ADR-004-gateway-platform.md), [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md). |
| Gateway time                                           | **Decided:** DS3231 RTC + CR2032 primary, opportunistic GPS via Quectel L80-M39. No NTP. See [ADR-011](decisions/ADR-011-gateway-time-source.md).                                                                                                                                                                                                                                       |
| Tag hardware                                           | **Decided:** Heltec Wireless Tracker V2 (ESP32-S3 + SX1262 + UC6580 GNSS). See [ADR-002](decisions/ADR-002-tag-hardware.md).                                                                                                                                                                                                                                                    |
| Relay hardware                                         | **Decided:** Heltec Wireless Tracker V2 + Heltec Solar Kit for Dev-board. See [ADR-003](decisions/ADR-003-relay-hardware.md).                                                                                                                                                                                                                                                   |
| Relay GNSS                                             | **Decided:** on board, used during commissioning/maintenance only. See [ADR-006](decisions/ADR-006-relay-has-gnss.md).                                                                                                                                                                                                                                                          |
| Kiosk UI                                               | **Decided:** native Rust GUI (`egui` + `walkers`), offline **PMTiles**. No browser, no React, no MapLibre. See [ADR-005](decisions/ADR-005-map-and-ui.md).                                                                                                                                                                                                                      |
| Touchscreen                                            | **Decided:** the touchscreen is the only UI. Read-only map. **Display class (size, orientation, panel) open per pending ADR-015**; the prior "7" DSI 1024×600 wall-mount" assumption no longer holds for a handheld. The read-only-map invariant survives — see also [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md) for how the BLE-commissioning modal preserves it. See [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md).                                                                                |
| SOS encoding                                           | **Decided:** single band + flag bit in `POSITION` + jittered cadence + buzzer on tag. No separate SOS frequency. See [ADR-010](decisions/ADR-010-sos-encoding.md).                                                                                                                                                                                                              |
| Node roles & sighting semantics                        | **ADR-012 superseded in part by ADR-013 / ADR-014.** v1a/v1b scope split, tag SOS buzzer, and the non-goals list survive. See [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) (history) and the rows below for what replaces it.                                                                                                                              |
| Multi-hop flood mesh (single channel, packet_id dedup) | **Decided:** v1 carries one wire packet type (POSITION). All nodes park on 868.1 MHz. Loop prevention is dedup-only on `(node_id, seq_nr)`, 60 s expiry. No FORWARD envelope, no SIGHTING, no role byte. Node presentation is gateway TOML config (`nodes.toml`). See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md).                                            |
| Duty-cycle budget gate                                 | **Decided:** [ARCHITECTURE.md §13](ARCHITECTURE.md) holds a mandatory budget table. Any change to packet size / cadence / retransmit / hop limit must update §13 in the same edit. Single-tag SOS rebroadcast fits in the 1% sub-band M cap; two simultaneous SOS tags do not (multi-tag scale is a v2 concern). See [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md). |
| Cloud / network                                        | **Decided:** no cloud-hosted dependency, no inbound network surface, no REST, no phone app, no downlink on the LoRa side. **Pending ADR-016** splits the wording into four categories — (a) no inbound surface, (b) no internet-bound calls, (c) no cloud-hosted dependency, *and* (d) outbound LAN multicast/unicast under explicit gate. Categories (a)-(c) stay closed; (d) is the new addition for base-mode CoT/TAK export. See [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md), [`spikes/tak-cot-integration-spike.md`](spikes/tak-cot-integration-spike.md). |
| Database                                               | **Decided:** SQLite (WAL mode), single file on the Pi. `seq_nr` is u32; dedup is recent-window, not a permanent UNIQUE index. See [ADR-009](decisions/ADR-009-database-sqlite.md).                                                                                                                                                                                              |
| Code                                                   | `crates/protocol` exists: encoder/decoder + relay decision logic + SeenCache, shared between tag, relay, and gateway. 22 unit tests, frozen canonical wire-format vectors. Tag/relay firmware binaries and the gateway binary are not yet started; the kiosk-lab UX prototype lives under `tools/`.                                                                                                                                                                |
| Hardware in hand                                       | 3× Raspberry Pi 4 Model B (some missing screws; power-on validation pending — `hardware/pi{1,2,3kiosk}/specs.md`), 1× 7" DSI touchscreen, 3× Dragino HAT (some bent pins; GPIO 25 CS-defect rev check pending). **Heltec order #110639 shipped 2026-05-05** — 10× Wireless Tracker V2 + 2× Solar Kit for Dev-board, ETA roughly 2026-05-19 → 2026-06-02 (per [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md)). Handheld gateway substrate (Pi class + display + battery + enclosure) not yet ordered — pending pivot spikes' close + ADR-015. |

## Folder structure

```
LoRa SAR/
├── README.md                 ← this file (index + status)
├── CLAUDE.md                 ← onboarding note for any future LLM instance
├── TODO.md                   ← ordered backlog
├── ARCHITECTURE.md           ← single-source architecture doc (v10, 2026-04-26)
├── bom.md                    ← bill of materials
├── explainers/               ← presentation-friendly system explanations
│   └── how-the-network-works.md
├── operations/               ← operator / bring-up troubleshooting docs
│   └── troubleshooting-guide.md
│
├── decisions/                ← Architecture Decision Records
│   ├── README.md             ← ADR index + status board
│   ├── ADR-001-firmware-language.md       ← Rust everywhere
│   ├── ADR-002-tag-hardware.md            ← Wireless Tracker V2
│   ├── ADR-003-relay-hardware.md          ← Wireless Tracker V2 + Solar Kit
│   ├── ADR-004-gateway-platform.md        ← Pi + Dragino + Yocto + touchscreen
│   ├── ADR-005-map-and-ui.md              ← Native Rust GUI (egui + walkers, PMTiles)
│   ├── ADR-006-relay-has-gnss.md          ← GNSS for commissioning only
│   ├── ADR-007-touchscreen-primary-ui.md  ← Kiosk map, no other UI
│   ├── ADR-008-no-cloud-no-downlink.md    ← Pure uplink, no cloud
│   ├── ADR-009-database-sqlite.md         ← SQLite, not PostgreSQL
│   ├── ADR-010-sos-encoding.md            ← SOS: single band + flag + jitter
│   ├── ADR-011-gateway-time-source.md     ← DS3231 RTC + opportunistic GPS
│   ├── ADR-012-node-roles-and-sighting-semantics.md  ← Superseded in part by ADR-013/014; v1a/v1b + buzzer + non-goals survive
│   ├── ADR-013-multi-hop-flood-via-packet-id.md      ← Multi-hop flood, one packet type, packet_id dedup
│   └── ADR-014-duty-cycle-budget-as-gate.md          ← Duty-cycle budget table as mandatory gate
│
└── archive/                  ← superseded docs, kept for history
    ├── zephyrOS_study.md     ← Zephyr was ruled out in ADR-001
    └── product-roadmap.md    ← calendar roadmap replaced by TODO.md
```

Future folders (`architecture/`, `hardware/`, `software/`) may appear as we break out detail from `ARCHITECTURE.md`. For now, `ARCHITECTURE.md` remains the single source of truth and the ADRs are the decision ledger.

## Read-first order (new contributor, 15 minutes)

1. `README.md` — this file
2. `CLAUDE.md` — how to work with Pieter and what's already decided
3. `decisions/README.md` — the ADR status board
4. `ARCHITECTURE.md` — the system, in one document
5. `TODO.md` — what's next, in order
6. `bom.md` — what to order

Everything else is detail.

## Presentation / operations docs

- `explainers/how-the-network-works.md` — presentation-friendly network walkthrough for contributors, reviewers, and operator-adjacent readers.
- `operations/troubleshooting-guide.md` — practical operator/developer troubleshooting reference for packet drops, map truth, clock/radio bring-up, and dedup behavior.
