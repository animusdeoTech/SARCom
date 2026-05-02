---
title: "Bill of Materials"
status: living
type: hardware
tags: [hardware, bom, shopping]
---

# Bill of Materials

**Purpose:** the versioned shopping list. Source of truth for "what do we buy" and "what did we already buy." When in doubt, this doc wins over any other list.

**Aligned to:** [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), [ADR-004](decisions/ADR-004-gateway-platform.md), [ADR-011](decisions/ADR-011-gateway-time-source.md), [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md), [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), dated 2026-04-22 / 2026-04-24 / 2026-04-25 / 2026-04-26.

## Heltec (order from heltec.org DE warehouse)

- [ ] **3× Heltec Wireless Tracker V2** (ESP32-S3FN8 + SX1262 + UC6580 multi-constellation GNSS, 863–928 MHz variant, 28 dBm). 1 for primary tag, 1 for paal-relay (v1a), 1 reflashed as drone-pod (v1b). Per [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), and [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md). *Note: the UC6580 is on-board — no external GNSS module needed.*
- [ ] **1× Heltec Solar Kit for Dev-board** (IP67 enclosure ~178×178×35 mm, 5 W 6 V solar panel, charge controller, 1–4× 18650 holder, pre-reserved SMA bulkhead hole for external LoRa antenna). Per [ADR-003](decisions/ADR-003-relay-hardware.md).

**Explicitly NOT ordering** (superseded by ADR-002 / ADR-003):
- ~~WiFi LoRa 32 V4~~ — superseded
- ~~V4 Expansion Kit~~ — superseded
- ~~L76K GNSS module~~ — Wireless Tracker V2 has the UC6580 on-board

## Antennas + RF interconnect (CRITICAL — do not skip)

The Tracker V2 exposes the SX1262 LoRa port on an **IPEX1.0 (u.FL)** connector. The Solar Kit enclosure has an **SMA bulkhead** hole in the wall. The onboard LDS ceramic antenna gets buried inside the IP67 box and is useless for the relay use case. So:

- [ ] **1× IPEX1.0 → SMA female bulkhead pigtail**, ~15 cm. For the relay: Tracker V2 LoRa port → bulkhead of the Solar Kit enclosure. Verify gender against the Solar Kit panel (SMA female bulkhead ↔ SMA male antenna is the standard combo).
- [ ] **1× 868 MHz external SMA antenna** for the relay (omnidirectional, ~3 dBi, half-wave). Mounted on the Solar Kit bulkhead. Not optional — the onboard LDS antenna is sealed inside the enclosure.
- [ ] **1× 868 MHz half-wave SMA stubby antenna** for the Dragino HAT on the gateway Pi. The HAT does not ship with a suitable ETSI 868 antenna.
- [ ] **Tag antennas:** the Tracker V2's onboard LDS antenna is usable for the pocket-carried tag. **If range is poor during bring-up**, add 1× IPEX1.0 → SMA pigtail + 1× stubby per tag. Defer this purchase until after v0.

## Gateway time source (CRITICAL for "last seen" not to lie)

A Pi with no internet and no RTC will boot with bogus system time. The kiosk renders "last seen X minutes ago" — without a real time source, that string lies after every power cycle. Per [ADR-011](decisions/ADR-011-gateway-time-source.md):

- [ ] **1× DS3231 RTC module** with battery holder (I2C, ±2 ppm, ~€3). Connect to the Pi's I2C bus on the GPIO header.
- [ ] **1× CR2032 coin cell** for the RTC (some modules ship with one, many don't — order one regardless).
- [ ] *Opportunistic:* the Dragino HAT's MTK3339 GPS can provide PPS-disciplined time when it has a fix. Useful, but not a substitute for the RTC.

## Batteries (local / Amazon / Tinytronics)

Accounting: 2 tags × 1 cell + 1 relay × 2 cells = **4 consumed**. Add 2 real spares → order 6.

- [ ] **6× Samsung INR18650-25R** flat-top (or equivalent quality cell: Molicel P26A, LG HG2). 2 for the tags, 2 for the Solar Kit, 2 genuine spares.

## Tag SOS audible cue (per ADR-012)

Per [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) (preserved through the ADR-013 rollback), the tag drives a piezo buzzer on a GPIO line for the duration of distress state — the last-meter audible cue for the human searcher.

- [ ] **1× 3.3 V active piezo buzzer**, low-current, GPIO-driveable directly from the ESP32-S3 (~€1, tag SOS audible cue per [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md)).

*Note: the Wireless Tracker V2 has its own battery input via the SH1.25-2 "battery" interface, which on the relay will be fed from the Solar Kit charge controller's battery bus — one charge path, not two. See [ADR-003](decisions/ADR-003-relay-hardware.md) §Consequences.*

## Relay mounting workaround (CRITICAL — see ADR-003)

The Solar Kit's default bracket fits the V3 / V4 / T114 footprint — **not** the Wireless Tracker V2. The workaround is:

- [ ] **M2.5 self-adhesive PCB standoffs** (4× minimum, ideally 6). Nylon, ~6–10 mm.
- [ ] **3M VHB (Very High Bond) tape** for extra hold on the enclosure inner wall.
- [ ] M2.5 × 6 mm screws (small pack) for fastening the Tracker V2 to the standoffs.

No 3D-printing. No machining. No "we'll figure it out."

## Dragino + Pi fastening

- [ ] **M2.5 × 6 mm brass standoffs + screws** kit (for HAT-to-Pi and Pi-to-touchscreen). Buy a kit, not individual pieces — this is the "missing screws" gap.
- [ ] **M2.5 × 4 mm screws** pack (short screws for display mount).
- [ ] 40-pin M/F Dupont jumper wire set (workaround for any Dragino bent-pin that can't be straightened).
- [ ] Precision tweezers (bent-pin straightening).
- [ ] PH0 + PH00 precision screwdriver set.

## USB + cables

- [ ] 2× USB-C data cable (Anker / Belkin quality; NOT charge-only — needed for `espflash`).
- [ ] 1× powered USB hub 4+ ports (for simultaneous flashing + debug + keyboard).
- [ ] 1× Ethernet cable CAT6.
- [ ] 1× official Raspberry Pi PSU. Verify Pi model first — 5V/3A USB-C for Pi 4, 5V/2.5A micro-USB for Pi 3B+.

## Pi reliability

- [ ] **3× microSD 32–64 GB High/Max Endurance** (SanDisk Max Endurance or Samsung Pro Endurance). Six-year-old SDs rot silently — we assume none of the existing SDs survive.
- [ ] Passive heatsink kit for the Pi (aluminium blocks on SoC + RAM + PMIC). Yocto images can run the Pi warm.
- [ ] USB current meter (Ruideng UM25C class, ~€15). Used for tag and relay power measurements during bring-up.

## Relay pole

- [ ] **Wooden pole**, local hardware store. Spec: ~2.5 m height, ~7–10 cm diameter, pressure-treated for outdoor.
- [ ] Stainless hose clamps (2–4×) for strapping the Solar Kit enclosure to the pole.
- [ ] Optional: pole anchor (concrete block or ground spike) depending on garden ground.

## Software / image toolchain (no purchase — just noting the fact)

- [ ] Yocto (`meta-raspberrypi` + `meta-rust`) build environment on the laptop. Per [ADR-004](decisions/ADR-004-gateway-platform.md). No Raspbian / Raspberry Pi OS.
- [ ] `esp-rs` / `espup` toolchain for the `xtensa-esp32s3-none-elf` target. Per [ADR-001](decisions/ADR-001-firmware-language.md).

## v1b parking lot — do not order yet

Per [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md), v1b is the drone-pod aerial overlay and is **gated on v1a passing**. The items below exist for v1b reference only and are intentionally **not** in the cart sanity-check list. Do not order with the v1a items.

- Optional barometer breakout: **BMP280 or BME280** (I²C, ~€2). For drone-pod relay altitude telemetry. Order only after v1a passes.
- **1S LiPo for drone-pod**, ~500–800 mAh, with JST connector. Powers the drone-pod relay (a Tracker V2 with `ui_kind = "drone-relay"` in `nodes.toml`) independently of the drone's flight battery.
- **Drone under-mount:** 3D-printed pod, zip-ties, or velcro strap — choice depends on the drone airframe. Mechanical detail finalised at v1b time.

These are listed for v1b reference only and are **NOT** in the cart sanity-check list. Do not order with the v1a items.

## Cart sanity-check before you click buy

One more pass, in order, so nothing falls through:

1. 3× Wireless Tracker V2 (EU 863–928 MHz variant)
2. 1× Solar Kit for Dev-board
3. 1× IPEX1.0 → SMA female bulkhead pigtail (for the relay)
4. 1× 868 MHz external SMA antenna (for the relay bulkhead)
5. 1× 868 MHz SMA stubby antenna (for the Dragino HAT on the gateway)
6. 6× Samsung INR18650-25R (or equivalent) — 18650 cells
7. 1× DS3231 RTC module + 1× CR2032 coin cell
8. 1× small piezo active buzzer (tag SOS audible cue)
9. M2.5 self-adhesive PCB standoffs + 3M VHB tape + M2.5×6 screws (relay mounting)
10. M2.5 brass standoff + screw kit (Pi + HAT + touchscreen)
11. 2× USB-C data cable + 1× powered USB hub + 1× Ethernet + 1× official Pi PSU
12. 3× microSD 32–64 GB High Endurance + heatsink kit + USB current meter
13. Precision screwdriver set + fine tweezers + Dupont jumpers
14. Wooden pole + stainless hose clamps (local hardware store, not Heltec cart)

## To record as items arrive

- [ ] Total cost with dates and invoice references
- [ ] Supplier for each item (Heltec DE / Amazon / Tinytronics.nl / Opencircuit.nl / local hardware / other)
- [ ] Which items arrived and when — opens the natural "ready for assembly" checkpoint
