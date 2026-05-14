---
title: "Bill of Materials"
status: living
type: hardware
tags: [hardware, bom, shopping]
---

# Bill of Materials

**Purpose:** the versioned shopping list. Source of truth for "what do we buy" and "what did we already buy." When in doubt, this doc wins over any other list.

**Aligned to:** [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), [ADR-004](decisions/ADR-004-gateway-platform.md), [ADR-011](decisions/ADR-011-gateway-time-source.md), [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) (buzzer + non-goals only; rest superseded by [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) / [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md)), [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md), [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md), dated 2026-04-22 / 2026-04-24 / 2026-04-25 / 2026-04-26. Cascading 2026-05-06 handheld pivot per [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md); gateway substrate + display + battery + enclosure are open pending ADR-015 / ADR-016 / ADR-017 (working titles).[^pivot]

[^pivot]: 2026-05-06 form-factor pivot — see [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md). Three new ADRs proposed: ADR-015 (handheld substrate + form factor; supersedes-in-part ADR-004; refines-in-part ADR-005/006/007), ADR-016 (base-mode export gate; supersedes-in-part ADR-008), ADR-017 (custom 3D-printed waterproof enclosures for gateway and tag; refines-in-part ADR-002).

---

## Order 1 — Heltec DE warehouse (heltec.org)

- [ ] **10× Heltec Wireless Tracker V2** (ESP32-S3FN8 + SX1262 + UC6580 GNSS, **EU 863–870 MHz variant**, 28 dBm). 2 tags (v1a/v1b), 1 paal-relay (v1a), 1 drone-pod relay (v1b), 6 spares/future use. Per [ADR-002](decisions/ADR-002-tag-hardware.md), [ADR-003](decisions/ADR-003-relay-hardware.md), [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md). Product page: https://heltec.org/project/wireless-tracker-v2/
  *UC6580 is on-board — no external GNSS module needed.*
  *BLE commissioning is v1 per [ADR-006](decisions/ADR-006-relay-has-gnss.md) (audit-close 2026-05-07 resolves the prior ARCHITECTURE.md / TODO.md drift on the v1-vs-v2+ question on the v1 side). Topology is gateway-as-BLE-central + relay/tag-as-peripherals (no phone in v1) per [`spikes/ble-commissioning-scope-spike.md`](spikes/ble-commissioning-scope-spike.md); kiosk-side flow per [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md). The on-board ESP32-S3 BLE radio is the peer; no separate BLE module to order.*

- [x] **2× Heltec Solar Kit for Dev-board** — ordered as part of Heltec #110639 on 2026-05-05 (per [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md); A6 reconcile 2026-05-07). Second kit is v1b drone-pod / fleet-inventory spare, not v1a critical path. **"LoRa + 2.4G"** variant (in stock as of 2026-05-04; 868 MHz single-LoRa variant is out of stock). The extra 2.4G SMA bulkhead is unused — plug or leave empty. IP67 enclosure ~178×178×35 mm, 5W 6V solar panel, charge controller, 18650 holder. Product page: https://heltec.org/project/solar-kit-for-dev-board-waterproof-enclosure-for-outdoor-meshtastic-meshcore/
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

> **Pending ADR-015.** The 2026-05-06 pivot makes the gateway a handheld portable with a custom 3D-printed waterproof enclosure (pending ADR-017) and a battery + USB-C charging path (pending ADR-016 export gate). Pi class (4 vs 5 vs CM5 vs Zero 2W) and display class (size, orientation, panel) are open per [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md); battery topology + protections + signal contract are open per [`spikes/gateway-handheld-power-architecture-spike.md`](spikes/gateway-handheld-power-architecture-spike.md); enclosure mechanicals + bulkhead inventory are open per [`spikes/gateway-handheld-enclosure-spike.md`](spikes/gateway-handheld-enclosure-spike.md). The standoff-kit line below still applies for desk bring-up; specific Pi PSU / SD / heatsink / display / battery / enclosure SKUs are not committed to here.

- [ ] **M2.5 brass standoff + nut + screw assortment kit** (covers HAT-to-Pi, plus generic mechanical work; the prior "Pi-to-touchscreen back panel" assumption no longer holds — handheld enclosure standoffs are pending ADR-017). Fixes the "missing screws" gap on all three Pi units.
  Search: "M2.5 brass standoff assortment kit"

### Desk development setup

- [ ] **1× powered USB hub, 4–7 ports**, USB-A, externally powered. Connects keyboard, mouse, espflash cable(s), and debug serial — all from one hub on the bench. Anker or UGREEN recommended.
  Search: "Anker powered USB hub 4 port"

- [ ] **2× USB-C data cable**, 1 m, quality brand (Anker / Belkin). NOT charge-only — required for `espflash` to enumerate the ESP32-S3. Verify data capability on product page.

- [ ] **1× USB-A to USB-C cable** (for Heltec boards that ship with micro-USB or older connectors — check board spec on arrival).

- [ ] **1× Raspberry Pi 5 official 27W USB-C-PD PSU (5V/5A).** Pi 5 needs the upgraded supply; the on-hand Pi 4s are out of order (2026-05-07 — see [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md)) so legacy 5V/3A and micro-USB PSUs are no longer applicable. Order alongside whichever Pi 5 RAM variant the substrate spike picks.
  Search: "Raspberry Pi 5 official 27W PSU USB-C"

- [ ] **1× SD card reader** (USB-A, single slot). For writing Yocto images to the three microSD cards sequentially. Order one if your dev workstation does not have a built-in SD slot.

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

These are NOT in the immediate cart. v0 desk bring-up runs against the dev workstation's manually-set system clock; the DS3231 + CR2032 only become load-bearing at v1a, when the gateway moves into field deployment and there is no operator-set clock to lean on. Per [ADR-011](decisions/ADR-011-gateway-time-source.md), no NTP at any deployment stage — the DS3231 + opportunistic GPS-PPS path is the only sanctioned time source, and CLAUDE.md's "Do NOT re-open" list explicitly closes the WiFi-NTP door.

- [ ] **1× DS3231 RTC module** (I²C, ±2 ppm, ~€3). Connect to Pi I²C GPIO header for field deployment.
- [ ] **1× CR2032 coin cell** for the RTC backup (order one regardless of whether the module ships with one).

Per [ADR-011](decisions/ADR-011-gateway-time-source.md). ADR-011 is unchanged.

---

## Relay tripod + Solar Kit adapter (selection per spike)

The relay mount for v0 / v1 / v2 is an **off-the-shelf plastic tripod** with a standard mount thread, plus a printed/machined adapter between the tripod head and the Heltec Solar Kit enclosure. Selection (tripod model + adapter material + mount thread + dimensions + load rating + folded length) is owned by [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md). **No SKUs are committed here** — the spike picks the parts.

Wooden-pole + Fusion-360 designed-pole approaches are retired for v0/v1/v2 — see [`dev-log/2026-05-08-relay-mount-tripod-decision.md`](dev-log/2026-05-08-relay-mount-tripod-decision.md). [ADR-003](decisions/ADR-003-relay-hardware.md)'s relay-hardware decision (Tracker V2 + Solar Kit + adhesive PCB standoffs + 3M VHB) is unchanged.

Placeholder line items (selection per spike, not committed):
- Plastic tripod with standard 1/4"-20 or 3/8" head thread (load rating + folded length + operating-temp envelope per spike)
- Solar-Kit-to-tripod adapter (printed for prototype; production material + screw thread per spike)
- Anti-rotation feature for the adapter (per spike)

---

## Explicitly NOT ordering

These are superseded or out of scope:

- ~~WiFi LoRa 32 V4~~ — superseded by ADR-002/ADR-003
- ~~V4 Expansion Kit~~ — superseded
- ~~L76K GNSS module~~ — UC6580 is on-board
- ~~Stainless hose clamps~~ — retired 2026-05-08 (see [`dev-log/2026-05-08-relay-mount-tripod-decision.md`](dev-log/2026-05-08-relay-mount-tripod-decision.md)); off-the-shelf plastic tripod + adapter replaces this approach. Selection per [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md).
- ~~Wooden pole from hardware store~~ — retired 2026-05-08 (same dev-log); off-the-shelf plastic tripod + adapter replaces this approach.
- ~~Fusion 360 designed three-legged base + ground-stake~~ — retired 2026-05-08 (same dev-log); off-the-shelf plastic tripod + adapter replaces this approach. Selection per [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md).
- ~~Tag enclosure~~ — deferred; garden v1a doesn't need it; v1b (Terril Waterschei) will need a proper solution, designed then
- ~~Magnetic-pogo charging connector + cable accessories~~ — retired 2026-05-14, gateway runs on removable battery pack only (see [`dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md))

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
2. 2× Solar Kit for Dev-board (**LoRa + 2.4G variant**) — ordered as part of Heltec #110639 (2026-05-05); second kit is v1b drone-pod / fleet-inventory spare.
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
- DS3231 RTC + CR2032 — buy at v1a prep

---

## To record as items arrive

- [ ] Total cost with dates and invoice references
- [ ] Supplier for each item
- [ ] Which items arrived and when — opens the "ready for assembly" checkpoint
