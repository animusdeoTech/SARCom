---
title: "ADR-003: Relay hardware — Wireless Tracker V2 + Solar Kit"
status: accepted
date: 2026-04-22
supersedes: "ADR-003 (2026-04-19): WiFi LoRa 32 V4 + L76K + Solar Kit"
type: adr
tags: [decision, hardware, relay, heltec, wireless-tracker-v2, solar]
---

# ADR-003: Relay hardware — Heltec Wireless Tracker V2 + Heltec Solar Kit

**Status:** Accepted
**Date:** 2026-04-22
**Supersedes:** the 2026-04-19 version of this ADR, which specified the WiFi LoRa 32 V4 + L76K.

## Context

The relay is pole-mounted outdoors and runs on solar. Requirements:

- Same firmware family as the tag — shared BSP, shared `protocol` crate ([ADR-001](ADR-001-firmware-language.md))
- Same SX1262 on 868 MHz
- GNSS required — see [ADR-006](ADR-006-relay-has-gnss.md), used only during commissioning/maintenance, OFF during forwarding
- IP67 enclosure with integrated solar panel, charge controller, 18650 holder, LoRa antenna
- Mounts to a wooden pole with off-the-shelf hardware — **no 3D printing**
- Must survive unattended deployment from day one of the garden v1a test (per [ADR-012](ADR-012-node-roles-and-sighting-semantics.md), what was previously called the v1 garden test)
- For the PoC, the whole point is "prove it runs on solar" — using Heltec's own Solar Kit is the honest way to do that

## Decision

**Heltec Wireless Tracker V2 + Heltec Solar Kit for Dev-board + 2× 18650.**

Components:

- 1× Heltec Wireless Tracker V2 (863–928 MHz, 28 dBm) — same board as the tag
- 1× Heltec Solar Kit for Dev-board: IP67 enclosure (178 × 178 × 35 mm), 5W 6V solar panel, SH1.25-2 3.7V output, 1–4× 18650 holder in parallel, USB-C charge input, pre-reserved SMA bulkhead for 868 MHz antenna
- 2× Samsung INR18650-25R (or equivalent) — ≈5000 mAh in parallel at 3.7V nominal
- Wooden pole (local hardware store, ~2.5 m, pressure-treated), u-bolts or stainless hose clamps

Quantity for v0/v1: **1 relay.** The v1 success criterion is "one relay on a pole in the garden, running unattended on solar for 72 hours, forwarding packets from tag to gateway." Scale to multiple relays after that works.

## Consequences

- **Bracket mismatch — known issue.** The Solar Kit's default internal bracket fits the WiFi LoRa 32 V3, V4, and Mesh Node T114 PCB outline. It does **not** fit the Wireless Tracker V2 form factor. Electrical/enclosure compatibility is fine; the V2 board needs a different mechanical mount inside the enclosure.
  - **Chosen workaround:** adhesive PCB standoffs (M2.5) + 3M VHB tape to bond standoffs to the enclosure floor, then screw the V2 board down. No 3D printing. Alternative: drill new holes in the existing bracket to match the V2's mounting pattern.
  - Documented in [../hardware/relay-assembly.md](../hardware/relay-assembly.md) (planned; for now see this ADR).
- **Single BSP** across tag and relay. Same `heltec-wireless-tracker-v2-bsp` crate, same pinout.
- **GNSS integrated onboard** — no external L76K wiring, no SH1.25-8P connector between two modules.
- **Charge-controller routing.** Both the Solar Kit and the V2 have charge management. Do **not** connect the solar panel to the V2's onboard SH1.25-2 solar interface when the Solar Kit is already doing solar-to-battery regulation. Pipeline: solar panel → Solar Kit charge controller → 18650 pack → V2's battery input. One charge path. Verify against the Solar Kit schematic during assembly.
- **Relay firmware modes** (per [ADR-006](ADR-006-relay-has-gnss.md)). Note: per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md), the relay's commissioning broadcast and slow self-announce both use the single v1 wire packet type **POSITION** — the `RELAY_INFO` packet type from ADR-012 is rolled back.
  1. **Forwarding (default, low power):** validate frames, check seen_cache on `(node_id, seq_nr)`, byte-identical rebroadcast. GNSS OFF. LoRa RX continuous.
  2. **Commissioning (first boot, or on operator trigger):** GNSS ON, acquire fix, broadcast a few `POSITION` packets carrying the surveyed position, power GNSS OFF, return to forwarding.
  3. **BLE maintenance:** future, not v1. Service engineer reads health/logs/RSSI over BLE next to the pole. Out of scope.
- **Physical mounting to a wooden pole** lives in [../hardware/relay-assembly.md](../hardware/relay-assembly.md) (planned; for now see this ADR). Pole spec (~2.5 m height, ~7–10 cm diameter, pressure-treated), u-bolts or hose clamps, south-facing solar panel tilt ≈ latitude.

## Alternatives considered

- **WiFi LoRa 32 V4 + L76K + Solar Kit (the 2026-04-19 decision).** Bracket fits out of the box. Rejected on this rewrite: two separate modules wired together, two BSPs across tag/relay, lower TX ceiling, and the L76K adds an SH1.25-8P connector and a soft-power control line for no mission benefit the onboard UC6580 doesn't already deliver.
- **Wireless Tracker V1.1 + Solar Kit.** Same bracket issue. Rejected: V2 supersedes V1.1.
- **RAK WisBlock (RAK19007 + RAK4631 + solar).** Different SoC (nRF52840), different ecosystem, breaks the shared-BSP story.
- **Commercial Meshtastic solar node (off the shelf).** Closed firmware, won't run the custom uplink-only protocol.
- **Roll our own solar: bare V2 + bare 5W panel + bare MPPT + aftermarket IP67 box.** More flexible, a lot more assembly and verification, and the PoC goal is explicitly "prove the Heltec solar story works."
- **Skip GNSS on relay.** Rejected — see [ADR-006](ADR-006-relay-has-gnss.md).

## Order checklist

- [ ] 1× Heltec Wireless Tracker V2 (863–928 MHz, 28 dBm)
- [ ] 1× Heltec Solar Kit for Dev-board (868 MHz, single LoRa antenna)
- [ ] 2× Samsung INR18650-25R
- [ ] Adhesive PCB standoffs (M2.5, 4–6 mm height) + 3M VHB tape — for the bracket workaround
- [ ] Wooden pole (~2.5 m, pressure-treated), u-bolts or stainless hose clamps — local hardware store
