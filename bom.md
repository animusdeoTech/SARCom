---
title: "Bill of Materials"
status: living
type: hardware
tags: [hardware, bom, shopping]
---

# Bill of Materials

**Purpose:** the versioned shopping list. Source of truth for "what do we buy" and "what did we already buy." When in doubt, this doc wins over any other list.

**Aligned to:** [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), [ADR-004](decisions/ADR-004-gateway-platform.md), [ADR-011](decisions/ADR-011-gateway-time-source.md), [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md), [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), dated 2026-04-22 / 2026-04-24 / 2026-04-25 / 2026-04-26.

---

## Order 1 — Heltec DE warehouse (heltec.org)

- [ ] **10× Heltec Wireless Tracker V2** (ESP32-S3FN8 + SX1262 + UC6580 GNSS, **EU 863–870 MHz variant**, 28 dBm). 2 tags (v1a/v1b), 1 paal-relay (v1a), 1 drone-pod relay (v1b), 6 spares/future use. Per [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md). Product page: https://heltec.org/project/wireless-tracker-v2/
  *UC6580 is on-board — no external GNSS module needed.*
  *BLE commissioning interface planned for v1 — see ADR-006 update pending.*

- [ ] **1× Heltec Solar Kit for Dev-board** — order the **"LoRa + 2.4G"** variant (in stock as of 2026-05-04; 868 MHz single-LoRa variant is out of stock). The extra 2.4G SMA bulkhead is unused — plug or leave empty. IP67 enclosure ~178×178×35 mm, 5W 6V solar panel, charge controller, 18650 holder. Product page: https://heltec.org/project/solar-kit-for-dev-board-waterproof-enclosure-for-outdoor-meshtastic-meshcore/
  **Verify at checkout:** does this variant include an IPEX1.0→SMA pigtail and/or 868 MHz antenna? If yes, remove items 1 and/or 2 from Order 2.

**Heltec warehouse note:** use heltec.org and select the DE warehouse at checkout. Verify ship-from location before paying — unconfirmed reports of some SKUs shipping from CN manufacturing.

---

## Order 2 — Amazon.de (or Tinytronics.nl as fallback)

### RF interconnect

- [ ] **1× IPEX1.0 (u.FL) → SMA female bulkhead pigtail**, ~15 cm. Relay: Tracker V2 LoRa port → Solar Kit enclosure bulkhead. **Not included in the Solar Kit** (verified 2026-05-04).
  Search: "u.FL IPEX to SMA female pigtail 15cm"

- [ ] **4× 868 MHz SMA stubby antenna** (omnidirectional, ~2–3 dBi, half-wave), SMA male connector. One per Dragino HAT (3× Pi gateway) + 1 spare. **The Solar Kit includes its own 868 MHz antenna for the relay bulkhead** (verified 2026-05-04) — these four are for the Dragino HATs only. Cannot buy at a local hardware store — order online.
  Search: "868MHz SMA antenna omnidirectional stub"

### Batteries

- [ ] **12× Samsung INR18650-25R flat-top** (or equivalent: Molicel P26A, LG HG2). Do not buy protected cells — the Solar Kit charge controller handles protection. 2 per tag, 2 per relay Solar Kit, 8 spares.
  Search: "Samsung INR18650-25R flat top" on Amazon.de or nkon.nl (NKON ships from NL, better pricing for cells).

### Tag SOS audible cue

- [ ] **1× 3.3V active piezo buzzer**, low-current, GPIO-driveable directly from ESP32-S3 GPIO (~€1). Connects via jumper cable to a free GPIO during bring-up — no soldering needed for v0/v1a bench testing.
  Search: "active piezo buzzer 3.3V arduino"

### Relay mounting

- [ ] **M2.5 self-adhesive PCB nylon standoffs** (pack of 10–20, ~6–10 mm height). Tracker V2 → Solar Kit inner wall.
- [ ] **3M VHB double-sided tape** (small roll or strips). Extra hold on enclosure wall.
- [ ] **M2.5 × 6 mm screws** (small pack, nylon or steel). For the standoffs.
  Search: "M2.5 PCB standoffs adhesive nylon" + "M2.5 6mm screws"

### Pi + HAT + touchscreen fastening

- [ ] **M2.5 brass standoff + nut + screw assortment kit** (covers HAT-to-Pi, Pi-to-touchscreen back panel, M2.5×4 and M2.5×6 variants). Fixes the "missing screws" gap on all three Pi units.
  Search: "M2.5 brass standoff assortment kit"

### Desk development setup

- [ ] **1× powered USB hub, 4–7 ports**, USB-A, externally powered. Connects keyboard, mouse, espflash cable(s), and debug serial — all from one hub on the bench. Anker or UGREEN recommended.
  Search: "Anker powered USB hub 4 port"

- [ ] **2× USB-C data cable**, 1 m, quality brand (Anker / Belkin). NOT charge-only — required for `espflash` to enumerate the ESP32-S3. Verify data capability on product page.

- [ ] **1× USB-A to USB-C cable** (for Heltec boards that ship with micro-USB or older connectors — check board spec on arrival).

- [ ] **Pi PSU(s)** — quantity and spec depends on Pi models. Write the desk-inventory note first (see TODO). Likely: 5V/3A USB-C for Pi 4, 5V/2.5A micro-USB for Pi 3B/3B+. Official Raspberry Pi PSU recommended to avoid undervolt throttling. Buy one per Pi that is missing a PSU.
  Search: "Raspberry Pi 4 official PSU USB-C" / "Raspberry Pi 3 official PSU micro-USB"

- [ ] **1× SD card reader** (USB-A, single slot). For writing Yocto images to the three microSD cards sequentially. Most laptops have one — only order if yours doesn't.

- [ ] **1× Ethernet cable CAT6**, 2–3 m. Pi → router during initial bring-up (before WiFi config).

### Storage + cooling

- [ ] **3× microSD 32–64 GB High Endurance** (SanDisk Max Endurance or Samsung Pro Endurance). One per Pi. Do not reuse existing old SDs — they rot silently.
  Search: "SanDisk Max Endurance 32GB microSD"

- [ ] **1× passive heatsink kit** for the gateway Pi (aluminium blocks on SoC + RAM + PMIC). Yocto builds can run the Pi warm.

### Measurement

- [ ] **1× USB current meter** (Ruideng UM25C class, ~€15). Tag and relay power measurements during bring-up.
  Search: "Ruideng UM25C USB power meter"

### Tooling

- [ ] PH0 + PH00 precision screwdriver set.
- [ ] Fine tweezers (bent Dragino HAT pin straightening).
- [ ] 40-pin M/F Dupont jumper wire set (GPIO workaround for bent pins; buzzer wiring during bench testing).

---

## Deferred — v1a prep (order when starting v1a field deployment)

These are NOT in the immediate cart. v0 runs behind WiFi with NTP at mom's place — no RTC needed yet. v1a is the first field deployment where the gateway has no internet.

- [ ] **1× Raspberry Pi RTC Battery** (official, JST connector for Pi 5 RTC header — or LIR2032 + matching JST pigtail). Pi 5 has a built-in RTC; just needs the battery.

Per [ADR-011](decisions/ADR-011-gateway-time-source.md). DS3231 external module removed — Pi 5 built-in RTC replaces it. For v0 desk prototyping, NTP over local WiFi is acceptable.

---

## Relay pole — local build (not online order)

The relay pole is a designed physical component, not a bought-off-the-shelf item. Off-the-shelf approaches (hose clamps, store-bought brackets) are explicitly rejected — this is a core product component.

**Plan:**
- Design in **Fusion360**: pole with a cantilevered mount protrusion for the Solar Kit enclosure, three-legged base structure, ground-stake tail of correct diameter.
- Build at the **woodworking shop** under supervision.
- Ground anchor: drill hole with hand drill, correct diameter for stake tail. No concrete required for garden v1a.

*No hose clamps. No improvised strapping. Do it once, do it right.*

Items needed (source locally from woodworking shop / hardware store):
- Appropriate timber stock (pressure-treated, ~7–10 cm diameter or square section)
- Drill bits (correct diameter for ground stake)
- Fasteners for Solar Kit enclosure to mount protrusion (size TBD from Fusion360 drawing)

---

## Explicitly NOT ordering

These are superseded or out of scope:

- ~~WiFi LoRa 32 V4~~ — superseded by ADR-002/ADR-003
- ~~V4 Expansion Kit~~ — superseded
- ~~L76K GNSS module~~ — UC6580 is on-board
- ~~Stainless hose clamps~~ — replaced by pole design approach
- ~~Wooden pole from hardware store~~ — replaced by pole design approach
- ~~Tag enclosure~~ — deferred; garden v1a doesn't need it; v1b (Terril Waterschei) will need a proper solution, designed then

---

## v1b parking lot — do not order yet

Gated on v1a passing. Do not add to cart until v1a acceptance criteria are met.

- Optional barometer: **BMP280 or BME280** (I²C, ~€2). Drone-pod altitude telemetry.
- **1S LiPo ~500–800 mAh** with JST connector. Drone-pod relay independent power.
- **UAV platform**: likely a PX4-equipped modular UAV with compatible accessories. Selection deferred to v1b design phase.
- Drone-pod mount: 3D-printed pod or equivalent — designed at v1b time.

---

## Cart sanity-check before you click buy

### Heltec DE cart
1. 10× Wireless Tracker V2 (EU 863–870 MHz variant)
2. 1× Solar Kit for Dev-board (**LoRa + 2.4G variant**)
   → Verify: pigtail included? 868 MHz antenna included? Remove Amazon items accordingly.

### Amazon.de cart
3. 1× IPEX1.0 → SMA female pigtail ~15cm (**not** included in Solar Kit — confirmed)
4. 4× 868 MHz SMA stubby antenna (Dragino HATs only — Solar Kit includes its own relay antenna)
5. 12× 18650 quality cells (Samsung INR18650-25R or equivalent)
6. 1× 3.3V active piezo buzzer
7. M2.5 self-adhesive PCB standoffs + 3M VHB tape + M2.5×6 screws
8. M2.5 brass standoff + nut + screw assortment kit
9. 1× powered USB hub (externally powered, 4–7 ports)
10. 2× USB-C data cable (verified data, not charge-only)
11. Pi PSU(s) — quantity after desk-inventory note
12. 1× Ethernet cable CAT6
13. 3× microSD 32–64 GB High Endurance
14. 1× passive heatsink kit (for gateway Pi)
15. 1× USB current meter (Ruideng UM25C class)
16. PH0 + PH00 precision screwdriver set
17. Fine tweezers
18. 40-pin M/F Dupont jumper set

### Local / woodworking shop
19. Timber + fasteners for relay pole (after Fusion360 drawing is done)

### Deferred (not in this cart)
- Raspberry Pi RTC Battery (JST, for Pi 5 built-in RTC) — buy at v1a prep

---

## To record as items arrive

- [ ] Total cost with dates and invoice references
- [ ] Supplier for each item
- [ ] Which items arrived and when — opens the "ready for assembly" checkpoint
