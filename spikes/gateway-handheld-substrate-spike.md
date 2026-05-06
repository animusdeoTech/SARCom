---
title: "Spike — Handheld gateway substrate, HAT stack, IO + antenna paths"
status: open
type: spike
timebox: 1 day
opened: 2026-05-06
---

# Spike: Handheld gateway substrate, HAT stack, and IO / antenna paths

## Why this spike exists

The pivot to a **handheld portable battery-powered gateway** with a **5-inch display** and a **custom 3D-printed waterproof enclosure** changes which SBC + HAT + display combinations are actually viable. ADR-004 currently names "Raspberry Pi 3B+ or 4 + Dragino LoRa/GPS HAT + 7" DSI" with mains power assumed. The pivot context names a "Pi 5 class gateway substrate" and "5-inch local display"; the prior Kiwi cart for Pi 5 + 5" Touch Display 2 was prepared and never ordered (per memory correction 2026-05-06), so the substrate decision is genuinely open.

The hardware envelope of a **handheld** is materially different from a wall kiosk:

- antennas (LoRa SMA, optional GPS SMA, Pi 5 onboard WiFi/BLE) must route through or terminate at a sealed shell
- onboard PCB antennas behind a 3D-printed wall behave differently than in free air
- HAT + DSI display + SBC must physically stack inside a hand-holdable shell with battery alongside
- thermal envelope (Pi 5 in a sealed plastic enclosure on battery) is different from a Pi 4 on mains in open air
- USB-C must be reachable for charging without breaking the seal
- GPIO/UART/SPI bring-up on Pi 5 RP1 is materially different from Pi 4 (per dev-log 2026-05-05); the Dragino HAT is documented for Pi 2/3 only and *community-validated* on Pi 4

This spike is **survey + decide on the substrate shape**, not implement bring-up. RX bring-up is the existing `gateway-rx-bringup-spike.md`; that spike is now downstream of this one.

## Hypothesis / research question

**H1.** Pi 5 (4 GB) + Dragino LoRa/GPS HAT (SX1276 + L80 M39 GNSS) + a 5" DSI panel + USB-C-PD charging is a viable handheld substrate, with the constraints that (a) the LoRa SMA bulkhead through the enclosure stays the production antenna path, (b) Pi 5 onboard WiFi/BLE work acceptably with a plastic-walled enclosure (no metal Faraday cage), and (c) one of the 3 on-hand Dragino HATs survives a Pi 5 + RP1 + polled-RX bring-up without a defect that forces a different HAT.

**H2.** Pi 5 onboard WiFi/BLE through a sealed plastic enclosure is too lossy for reliable BLE commissioning at arm's length. Mitigation: add a USB BLE/WiFi dongle with an external whip, or move to a CM5 + carrier with explicit IPEX-to-bulkhead antenna paths.

**H0.** Pi 5 thermals or HAT-fit issues kill the substrate. Fall back to Pi 4 (one of the 3 on-hand units, contingent on at least one being non-bricked per dev-log audit A10) and accept the reduced compute headroom.

## Scope fence

- **No ordering decision yet.** The spike produces a substrate verdict and a procurement list for the follow-up ADR-amendment ticket. Carts are not orders (per persistent feedback memory).
- **No firmware writing.** Polled-RX behavior verification is the existing `gateway-rx-bringup-spike.md`; this spike feeds it the substrate, not vice versa.
- **No 3D modelling.** Enclosure mechanicals are the `gateway-handheld-enclosure-spike.md`; this spike notes physical-fit constraints in millimetres so the enclosure spike has a working envelope.
- **No power-supply design.** Battery topology / charging / power-good belong to `gateway-handheld-power-architecture-spike.md`; this spike notes the Pi 5 5V/5A peak and what that implies for the power spike.
- **No display-orientation decision** unless the substrate constrains it (DSI cable lengths / connector positions can constrain layout — call those out, but the UI-side question lives in the pmtiles-walkers retarget).

## What to verify

### Substrate options (must rank, must give reason)

1. **Pi 5 (4 GB) + Dragino HAT + 5" DSI + USB-C-PD** — current pivot framing. Verify GPIO / SPI / UART pinout via RP1, whether the Dragino HAT physically clears a 5" DSI ribbon, and what changes vs Pi 4 documentation.
2. **Pi 5 (4 GB) + USB SX1276 dongle (no HAT) + 5" DSI** — sidesteps HAT-on-Pi-5 risk, costs a USB port and an antenna pigtail. Note feasible USB SX1276 modules (RAK, etc.) without committing to a SKU.
3. **CM5 + carrier board + 5" DSI + Dragino HAT (or equivalent SX1276 module)** — adds engineering overhead but unlocks explicit antenna routing (CM5 carriers expose IPEX onboard) and a more compact PCB stack inside a handheld shell.
4. **Pi 4 (2 GB / 4 GB) + Dragino HAT + 5" DSI + USB-C-PD** — fallback; community-validated HAT path, lower thermal envelope. Contingent on dev-log audit A10 follow-up (verify the on-hand Pi 4s are not bricked).
5. **Pi Zero 2 W + USB SX1276 + 5" DSI** — smallest substrate. Likely too compute-light for `egui` + `walkers` + `lora-phy` + `tokio` + SQLite + BLE + WiFi monitor + CoT emitter; record as out-of-envelope unless surprising data appears.

For each option, list:
- physical envelope (max length × width × height with HAT, DSI, battery, antennas)
- power draw idle / peak; what that implies for the battery spike
- onboard radios available (Pi 5: WiFi 5 + BT 5.0; Pi 4: WiFi 5 + BT 5.0; CM5: depends on variant; Zero 2 W: WiFi 4 + BT 4.2)
- antenna situation: onboard PCB antenna (Pi 5/4) vs IPEX (CM4/5 with carrier); what survives the plastic enclosure
- HAT mechanical/electrical fit risks (RP1 differences, SPI clock cap, GPIO 25 CS routing defect on some Dragino revisions per `gateway-rx-bringup-spike.md` B1)
- DSI 5" panel availability and orientation (portrait vs landscape; ribbon length)
- realistic cost envelope (vendor-checked at scoping, not pre-locked)

### IO / signal verification (Pi 5 specific)

- SPI on Pi 5 via RP1: clock rate cap behavior, default driver path under Linux 6.x, `linux-embedded-hal` + `rpi-pal` interaction.
- UART for L80 M39 GNSS NMEA on Pi 5: which `/dev/ttyAMA*` or `/dev/ttyS*` is wired through `dtparam=uart0` cleanup; differs from Pi 4.
- I²C for DS3231 RTC (per ADR-011): bus availability after the HAT is seated; whether the Dragino HAT blocks the I²C pins or passes them through.
- GPIO interrupt mode flakiness on Pi 5 RP1 (RadioLib #1200) — the gateway-rx-bringup spike already names this; restate the implication for substrate choice (polled RX is the v1a default).

### Antenna paths through the handheld enclosure

- **LoRa (SX1276)**: HAT-mounted SMA → external whip OR internal coax to bulkhead. Path through the 3D-printed wall is the same problem as the relay's IPEX→SMA pigtail (per ARCHITECTURE.md §16 risk #13); strain-relief and mating-cycle concerns from `production-concerns.md` §3 apply.
- **GPS L80 M39**: HAT-mounted SMA on the Dragino, same analysis. Whether the GPS antenna mounts inside the shell with a sky-window or out a bulkhead.
- **Pi 5 WiFi/BLE (onboard PCB antenna)**: behavior behind a 3D-printed plastic wall (PETG/ASA) — attenuation is small but real, and metallic battery cells on the same side of the antenna can detune. Bench-measure RSSI degradation.
- **Optional external USB WiFi/BLE dongle with whip**: viable if onboard radios fail the through-shell test.

### Cross-spike implications (record, don't solve)

- `gateway-handheld-power-architecture-spike.md`: peak 5V/5A demand for Pi 5 — informs USB-C PD profile choice.
- `gateway-handheld-enclosure-spike.md`: minimum internal volume, bulkhead count, gasket location, display window dimensions, USB-C charging-port location.
- `gateway-runtime-task-architecture-spike.md`: polled-RX impact on CPU budget, BLE central + WiFi monitor co-existing on Pi 5 BlueZ + wpa_supplicant.

## Pass criteria

- Substrate ranking (Options 1–5 above), with one-paragraph rationale per option, **and** a single recommended option for the v1 handheld with explicit "next step is ADR-004 amendment" callout.
- Pi 5 IO verification list filled in (SPI / UART / I²C / GPIO interrupts) with documented sources for each claim (datasheet, kernel docs, dev-log entry, vendor doc).
- Per-option physical envelope + cost envelope + antenna path documented enough that the enclosure spike has a working bounding box.
- Cross-spike implications recorded and named (which downstream spike picks each thread up).

## Fail criteria

- All 3 on-hand Dragino HATs are confirmed defective on Pi 5 and no alternative SX1276 source can be named within the timebox — escalate to a procurement decision (USB SX1276 dongle vs CM5 + IPEX-routed module) and rescope the bringup spike.
- Pi 5 onboard WiFi/BLE attenuation through PETG/ASA at expected wall thicknesses (3–4 mm) is measured >20 dB or fails BLE association at arm's length — flips the BLE commissioning architecture (external dongle becomes the path).
- The 5" Touch Display 2 cannot physically coexist with a HAT-stacked Pi 5 in a hand-holdable form factor (>180 mm long axis) — display becomes the variable; pmtiles-walkers retarget gets a constraint to absorb.

## Fallback / next action

- If H1 holds: write ADR-004 amendment ticket; downstream spikes (power, enclosure, runtime) proceed against the chosen substrate.
- If H2 (BLE-through-shell fails): document the dongle path; BLE commissioning spike inherits the constraint.
- If H0 (Pi 5 fails): Pi 4 fallback contingent on A10 follow-up. Re-cost the handheld; the enclosure spike re-bounds for Pi 4 form factor.

## Decision note template

```
Date:
Recommended substrate: option ___ (Pi 5 / Pi 5+USB / CM5 / Pi 4 / Zero 2W)
Rationale (one paragraph):

Substrate ranking:
  1. ___ — pros / cons / cost / antenna path:
  2. ___ — ...
  3. ___ — ...
  4. ___ — ...
  5. ___ — ...

Pi 5 IO verification (or fallback):
  SPI / RP1: state, source:
  UART for L80: state, source:
  I²C for DS3231: state, source:
  GPIO interrupts (polled-RX implication): state, source:
  Dragino HAT GPIO 25 CS defect (silkscreen rev on the 3 on-hand HATs): state:

Antenna paths through 3D-printed shell:
  LoRa SMA: chosen path:
  GPS SMA: chosen path / sky-window-only / external bulkhead:
  Pi 5 onboard WiFi/BLE through shell: measured / estimated / out-of-envelope:
  External BLE/WiFi dongle needed? yes / no / TBD by enclosure spike:

Physical envelope (chosen substrate, with HAT + 5" panel + battery):
  L × W × H mm:
  Internal bulkhead count:

Cross-spike implications recorded:
  power spike:           ___
  enclosure spike:       ___
  runtime spike:         ___
  ble commissioning:     ___
  rx bringup spike:      ___
  pmtiles retarget:      ___

Follow-up: ADR-004 amendment ticket filed? yes / no, reason:

Not implemented in this spike: code, BOM commitments, ordering decisions.

Next action:
```

## Cross-references

- `decisions/ADR-004-gateway-platform.md` — base ADR; substrate change requires amendment or new ADR.
- `spikes/gateway-rx-bringup-spike.md` — downstream consumer; this spike feeds the substrate decision into bringup.
- `spikes/gateway-handheld-power-architecture-spike.md` — sibling; takes the substrate's power profile.
- `spikes/gateway-handheld-enclosure-spike.md` — sibling; takes the physical envelope.
- `spikes/ble-commissioning-scope-spike.md` — sibling; affected by BLE-through-shell verdict.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar; this spike is one of the threads it cross-links.
- `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md` — Pi 5 / RP1 / `rpi-pal` research.
- `production-concerns.md` §3 — IPEX strain relief; same problem class for the gateway's external antenna path.
