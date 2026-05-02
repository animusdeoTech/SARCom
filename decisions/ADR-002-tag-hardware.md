---
title: "ADR-002: Tag hardware — Wireless Tracker V2"
status: accepted
date: 2026-04-22
supersedes: "ADR-002 (2026-04-19): WiFi LoRa 32 V4 Expansion Kit"
type: adr
tags: [decision, hardware, tag, heltec, wireless-tracker-v2]
---

# ADR-002: Tag hardware — Heltec Wireless Tracker V2

**Status:** Accepted
**Date:** 2026-04-22
**Supersedes:** the 2026-04-19 version of this ADR, which specified the WiFi LoRa 32 V4 Expansion Kit.

## Context

The tag is hiker-carried. It acquires a GNSS fix, broadcasts a `POSITION` packet over LoRa (22-byte frame per [ARCHITECTURE.md §7](../ARCHITECTURE.md)), and goes back to sleep. Requirements:

- ESP32-S3 (firmware stack per [ADR-001](ADR-001-firmware-language.md))
- SX1262 LoRa, 868 MHz. The board is capable of +28 dBm peak (which would legally fit ETSI sub-band P at +27 dBm ERP), but per [ADR-010](ADR-010-sos-encoding.md) v1 tag uplink — heartbeat and SOS alike — operates on sub-band M at +14 dBm ERP. The headroom above that is v2+ territory.
- Integrated GNSS — external wiring means more failure points
- 18650-compatible power with integrated charge management
- USB-C for flashing and charging
- Small enough to carry in a pocket or clip externally on a pack

Previous decision (2026-04-19) chose the WiFi LoRa 32 V4 Expansion Kit because it shipped as a fully enclosed unit (touchscreen-glass aluminium enclosure, bundled L76K GNSS, bundled 2800 mAh cell). That decision predated a closer look at the Wireless Tracker V2.

## Decision

**Heltec Wireless Tracker V2** (863–928 MHz EU variant, bare board).

Integrated onboard:

- ESP32-S3FN8 (8 MB flash)
- SX1262 LoRa, up to 28 ±1 dBm TX
- UC6580 multi-constellation GNSS (GPS, GLONASS, BDS, Galileo, NAVIC, QZSS) with upgraded LNA
- LDS antennas for 2.4 GHz (WiFi/BLE) and GNSS
- Lithium battery management (charge/discharge, overcharge protection, battery monitoring, automatic USB/battery switching)
- SH1.25-2 battery + solar interfaces
- USB-C
- Small OLED (unused in tag firmware)

Quantity: **2 boards** — 1 primary tag, 1 spare for failure testing and a second `tag_id` during garden tests.

Source: Heltec DE warehouse (Brussels shipping, no customs). Backups: Hexaspot, OpenELAB, Amazon EU, AliExpress Heltec store.

Battery: 1× Samsung INR18650-25R per tag (or equivalent: Molicel P26A, LG HG2). Not included with the board.

Enclosure: aftermarket. Either a plastic project box drilled for USB-C access, or one of the many 3D-printed cases available ready-made on Etsy / Thingiverse for the Tracker V2 footprint. The tag lives in a pocket — this is not an IP67 concern.

## Consequences

- **One board type across tag and relay** (see [ADR-003](ADR-003-relay-hardware.md)). One BSP, one pinout, one GPIO map.
- **GNSS is UC6580, integrated.** No external L76K module, no SH1.25-8P wiring, no firmware power-control line for a separate GNSS chip.
- **The `heltec-wireless-tracker-v2-bsp` crate is the BSP we write.** V2 is not yet in `esp-hal`. A sister board (`heltec_wireless_tracker` V1.1) exists in Zephyr but not `esp-hal`; we use it as a reference, not a dependency.
- **Enclosure quality drops vs. the V4 Expansion Kit.** We accept this tradeoff: the integration win on the PCB (onboard GNSS, onboard charge mgmt) beats the integration win on the mechanical side (kit enclosure) for a board that lives in a pocket.
- **No touchscreen on the tag.** The V4 Expansion Kit's glass-front touchscreen would have been decorative only (tag firmware does not need or use a UI). Dropping it is a win.
- **USB-C data cables (not charge-only).** Two minimum (one tag + one relay during bring-up). In [../bom.md](../bom.md).
- **Charge/power verification during bring-up:** the V2's onboard PMIC and overcharge cutoff should be bench-tested with a 4.25V lab supply before trusting it with an 18650 in the field.

## Alternatives considered

- **Heltec WiFi LoRa 32 V4 Expansion Kit (the 2026-04-19 decision).** Polished enclosure, touchscreen glass front, bundled L76K + 2800 mAh 18650. Rejected on this rewrite because:
  1. V4 has no onboard GNSS — L76K hangs off the SH1.25-8P connector
  2. Shipping a different board for tag vs relay means two BSPs
  3. The kit's touchscreen is irrelevant for tag firmware
  4. The Tracker V2 is smaller, cheaper, and has equal-or-better GNSS
- **Heltec Wireless Tracker V1.1.** Previous generation, ~21 dBm TX, single-band GNSS. Rejected: V2 supersedes it with higher TX (28 dBm), multi-constellation GNSS, upgraded LNA.
- **LilyGo T-Beam Supreme.** Excellent tag hardware (better GNSS, proper PMU, 18650 holder built-in). Rejected: different vendor ecosystem, would break the single-BSP story with the relay.
- **Custom PCB.** Out of scope for v0/v1.

## Order checklist

- [ ] 2× Heltec Wireless Tracker V2 (863–928 MHz, 28 dBm)
- [ ] 2× Samsung INR18650-25R (or equivalent)
- [ ] Verify SMA vs IPEX LoRa antenna connector on the shipped board (EU variant)
- [ ] Aftermarket tag enclosure (plastic project box, or Etsy/Thingiverse 3D-printed case for Tracker V2)
- [ ] Quality USB-C data cables — covered in [../bom.md](../bom.md)
