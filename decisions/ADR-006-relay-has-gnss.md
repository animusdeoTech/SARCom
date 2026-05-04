---
title: "ADR-006: Relay has GNSS (commissioning + maintenance only)"
status: accepted
date: 2026-04-22
type: adr
tags: [decision, relay, gnss, commissioning, maintenance]
---

# ADR-006: Relay has GNSS, used only during commissioning and maintenance

**Status:** Accepted
**Date:** 2026-04-22

## Context

Relays are poles in the field. Their positions must appear on the kiosk map so hut staff know where coverage exists. Options:

1. Survey each relay manually and hardcode coordinates — fragile, stale after any reposition
2. Give the relay a GNSS it uses continuously — wastes power on a node that does not move
3. Give the relay a GNSS it uses only in commissioning / maintenance — best of both

The chosen hardware (Wireless Tracker V2, see [ADR-003](ADR-003-relay-hardware.md)) has a UC6580 onboard whether we use it or not. Skipping it is the strange choice; using it occasionally is the obvious one.

## Decision

**Relay nodes have an active GNSS (UC6580 on Wireless Tracker V2). GNSS is ON only during commissioning and BLE maintenance modes. GNSS is OFF during normal forwarding.**

Operational modes:

1. **Forwarding (default, low power).** GNSS powered off. LoRa RX + validation + queue + CAD + TX only. Minimum CPU, minimum current. This is where the relay lives for years.
2. **Commissioning.** Triggered on first boot, on an explicit reboot-to-commissioning signal (e.g., a magnet on a reed switch — works through the IP67 enclosure without opening it), or on BLE request. Powers GNSS on, waits for fix (timeout 90s), broadcasts a small number of self-`POSITION` packets carrying the surveyed coordinates (per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md); the `RELAY_INFO` packet type from ADR-012 is rolled back), powers GNSS off, returns to forwarding.
3. **BLE maintenance.** **v1, not v0.** Service engineer stands next to the pole with a phone/laptop, connects over BLE, reads relay health (battery mV, recent RX count, last RSSI, GNSS fix age), optionally triggers a fresh commissioning broadcast. Implemented in v1 (after v0 desk prototype is working) — you cannot deploy a sealed solar relay in a field without a way to verify it is alive without opening the enclosure.

## Consequences

- **Relay self-announce uses the single v1 wire packet type, `POSITION`** — per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md), the `RELAY_INFO` packet type from ADR-012 is rolled back. Fields are exactly those of POSITION (per [ARCHITECTURE.md §7](../ARCHITECTURE.md)): `node_id`, `seq_nr`, `flags`, `lat_e7`, `lon_e7`, `alt_m`. Battery and firmware-version reporting are deferred to a future telemetry layer (see [ADR-013 §10](ADR-013-multi-hop-flood-via-packet-id.md)).
- Gateway parses `POSITION` and persists it in the single `tag_reports` table defined in [ARCHITECTURE.md §10](../ARCHITECTURE.md). Tags and relays both produce rows in `tag_reports`; presentation is distinguished by `nodes.toml` (`ui_kind` = `hiker` / `relay` / `drone-relay`). Dedup uses the recent-window `(node_id, seq_nr)` policy from [ADR-009](ADR-009-database-sqlite.md).
- Kiosk map renders relays with a distinct marker style (pole icon vs. hiker dot).
- **GNSS power control** via the Tracker V2's onboard gating. No external transistor, no extra GPIO.
- **BLE maintenance is v1 (not v0, not v2+).** Implement after v0 desk prototype is working. The Wireless Tracker V2 has BLE hardware on the ESP32-S3 — no additional components needed. Minimum viable: battery mV, RX count, last RSSI, GNSS fix age, trigger commissioning.
- **Commissioning trigger mechanism** is a small open question: magnet + reed switch is the leading candidate (works through sealed enclosure, cheap, robust). Final choice deferred to the relay bring-up ADR.

## Alternatives considered

- **No relay GNSS; survey manually.** Rejected: fragile, stale, and ignores hardware already on the board.
- **Relay broadcasts position continuously.** Rejected: wastes airtime for a stationary node, burns battery on an already power-constrained device, adds channel pressure on 868.1 MHz for no operational benefit.
- **Relay reports position to gateway over BLE instead of LoRa.** Rejected: the gateway has no BLE (Pi + Dragino HAT).
