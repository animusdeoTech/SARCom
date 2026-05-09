---
title: "Spike — Handheld gateway substrate, HAT stack, IO + antenna paths"
status: closed
type: spike
timebox: 1 day
opened: 2026-05-06
closed: 2026-05-08
---

# Spike: Handheld gateway substrate, HAT stack, and IO / antenna paths

## Closed 2026-05-08

**Verdict — mixed.** H1 (Pi 5 class + Dragino HAT + 22-pin DSI panel + USB-C PD) is committed as the substrate **shape**, but the specific Pi 5 RAM variant (2 / 4 / 8 GB) is **not pre-picked** — it is deferred to a per-variant empirical comparison (v0.6 substrate empirical test pass). H2 (BLE-through-shell) becomes part of that same test pass instead of a parallel investigation. H0 (CM5 / Zero 2W fallback) remains the credible escape if all three Pi 5 variants fail the envelope, but is no longer the working path.

**Resolution shape.** Three Pi 5 RAM variants are being procured (1× each: 2 GB, 4 GB, 8 GB). Per-variant measurement of (a) idle current, (b) typical-load current at full `walkers` + PMTiles render, (c) peak current under stress, (d) sustained thermal under passive cooling, (e) UX feel of map render at native refresh rate. Comparison feeds both the substrate-variant pick and the ADR-015 authoring. Cheaper to measure than to argue from spec sheets.

**Display class committed.** 2× new **Raspberry Pi Touch Display 2 (7-inch; Pi-5-native 22-pin DSI; 720×1280 portrait native, rotated to landscape via DRM/KMS)** for the v1 build path. 1× Pi-5-compatible 5-inch 22-pin DSI panel procured and **parked for v2 portrait variant** — out of v1 enclosure-spike scope. The on-hand 15-pin 7" DSI Touchscreen (originally on retired `pi3kiosk`) is **not on the v1 critical path** — it would need a 22-pin→15-pin adapter ribbon and an extra wire-routing complication; salvage value retained for ad-hoc bench tests only.

**Active cooling rejected.** Passive aluminum heat-spreader to rear shell handles Pi 5 thermal dissipation. No fan, no vent. The heat-path mechanical detail belongs to the enclosure-spike close.

Named follow-up: **power-architecture-spike close** + a separate **bom.md procurement update commit**. ADR-015 authoring is downstream of the v0.6 empirical test pass closing — not filed in this commit.

Decisions captured below in the §Decision note.

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

**Pi 4 retired 2026-05-07.** All three on-hand Pi 4 Model B units tested out of order; substrate is no longer Pi-4-vs-Pi-5. The spike is **unblocked** (the dev-log audit A10 follow-up is resolved), and the H0 fallback is no longer Pi 4 — see updated H0 below. Candidates are Pi 5 / Pi 5 + USB SX1276 / CM5 / Zero 2W. See [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).

## Hypothesis / research question

**H1.** Pi 5 (4 GB) + Dragino LoRa/GPS HAT (SX1276 + L80 M39 GNSS) + a 5" DSI panel + USB-C-PD charging is a viable handheld substrate, with the constraints that (a) the LoRa SMA bulkhead through the enclosure stays the production antenna path, (b) Pi 5 onboard WiFi/BLE work acceptably with a plastic-walled enclosure (no metal Faraday cage), and (c) one of the 3 on-hand Dragino HATs survives a Pi 5 + RP1 + polled-RX bring-up without a defect that forces a different HAT.

**H2.** Pi 5 onboard WiFi/BLE through a sealed plastic enclosure is too lossy for reliable BLE commissioning at arm's length. Mitigation: add a USB BLE/WiFi dongle with an external whip, or move to a CM5 + carrier with explicit IPEX-to-bulkhead antenna paths.

**H0.** Pi 5 thermals or HAT-fit issues kill the substrate. Fall back to CM5 (compute) or Zero 2W (compute floor) and accept reduced compute headroom; if neither survives the envelope, re-scope the runtime targets explicitly. (Pre-2026-05-07 H0 was "fall back to Pi 4 contingent on A10"; Pi 4 is retired — see [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).)

## Scope fence

- **No ordering decision yet.** The spike produces a substrate verdict and a procurement list for the follow-up ADR-amendment ticket. Carts are not orders (per persistent feedback memory).
- **No firmware writing.** Polled-RX behavior verification is the existing `gateway-rx-bringup-spike.md`; this spike feeds it the substrate, not vice versa.
- **No 3D modelling.** Enclosure mechanicals are the `gateway-handheld-enclosure-spike.md`; this spike notes physical-fit constraints in millimetres so the enclosure spike has a working envelope.
- **No power-supply design.** Battery topology / charging / power-good belong to `gateway-handheld-power-architecture-spike.md`; this spike notes the Pi 5 5V/5A peak and what that implies for the power spike.
- **No display-orientation decision** unless the substrate constrains it (DSI cable lengths / connector positions can constrain layout — call those out, but the UI-side question lives in the pmtiles-walkers retarget).

## What to verify

### Substrate options (must rank, must give reason)

1. **Pi 5 (4 GB) + Dragino HAT + 5" DSI + USB-C-PD** — current pivot framing. Verify GPIO / SPI / UART pinout via RP1, whether the Dragino HAT physically clears a 5" DSI ribbon.
2. **Pi 5 (4 GB) + USB SX1276 dongle (no HAT) + 5" DSI** — sidesteps HAT-on-Pi-5 risk, costs a USB port and an antenna pigtail. Note feasible USB SX1276 modules (RAK, etc.) without committing to a SKU.
3. **CM5 + carrier board + 5" DSI + Dragino HAT (or equivalent SX1276 module)** — adds engineering overhead but unlocks explicit antenna routing (CM5 carriers expose IPEX onboard) and a more compact PCB stack inside a handheld shell.
4. **Pi Zero 2 W + USB SX1276 + 5" DSI** — smallest substrate. Likely too compute-light for `egui` + `walkers` + `lora-phy` + `tokio` + SQLite + BLE + WiFi monitor + CoT emitter; record as out-of-envelope unless surprising data appears.

(Historical: a Pi 4 + Dragino HAT option was carried in earlier drafts as a "community-validated HAT path, lower thermal envelope" fallback. It was retired 2026-05-07 — the on-hand Pi 4s tested out of order; new Pi 4 acquisition is not on the table. See [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).)

For each option, list:
- physical envelope (max length × width × height with HAT, DSI, battery, antennas)
- power draw idle / peak; what that implies for the battery spike
- onboard radios available (Pi 5: WiFi 5 + BT 5.0; CM5: depends on variant; Zero 2 W: WiFi 4 + BT 4.2)
- antenna situation: onboard PCB antenna (Pi 5) vs IPEX (CM5 with carrier); what survives the plastic enclosure
- HAT mechanical/electrical fit risks (RP1 differences, SPI clock cap, GPIO 25 CS routing defect on some Dragino revisions per `gateway-rx-bringup-spike.md` B1)
- DSI 5" panel availability and orientation (portrait vs landscape; ribbon length)
- realistic cost envelope (vendor-checked at scoping, not pre-locked)

### IO / signal verification (Pi 5 specific)

- SPI on Pi 5 via RP1: clock rate cap behavior, default driver path under Linux 6.x, `linux-embedded-hal` + `rpi-pal` interaction.
- UART for L80 M39 GNSS NMEA on Pi 5: which `/dev/ttyAMA*` or `/dev/ttyS*` is wired through `dtparam=uart0` cleanup. (The pre-2026-05-07 wording "differs from Pi 4" is preserved as historical context in `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`; Pi 4 itself is retired.)
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
- If H0 (Pi 5 fails): fall back to CM5 / Zero 2W per updated H0 above. Re-cost the handheld; the enclosure spike re-bounds for the chosen fallback form factor. (Pi 4 is retired and not a fallback — see [`../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).)

## Decision note

```
Date: 2026-05-08

Recommended substrate: empirical-test-resolution — Pi 5 class + Dragino
LoRa/GPS HAT + Raspberry Pi Touch Display 2 (7", 22-pin DSI) + USB-C PD is
the committed substrate SHAPE. The specific Pi 5 RAM variant (2 / 4 / 8 GB)
is NOT pre-picked; it is deferred to v0.6 substrate empirical test pass.

Rationale: hardware envelope (footprint, GPIO header, RP1 IO behavior, USB-C
PD profile, DSI 22-pin connector, thermal class) is identical across the
three Pi 5 RAM SKUs. The only differentiator under SARCOM's load profile is
sustained RAM ceiling under egui + walkers + PMTiles + Yocto + Rust binary +
BLE central + WiFi monitor + LoRa RX. That ceiling is cheaper to measure
empirically across the three variants than to argue from spec sheets, and
the cost delta between the three variants is small enough that procuring
all three to test is the rational move. Empirical result also feeds the
ADR-015 authoring with measured numbers instead of guesses.

Substrate ranking (post-Pi-4-retirement):

  1. Pi 5 (any RAM, empirical) + Dragino HAT + Pi Touch Display 2 (7") +
     USB-C PD — COMMITTED as substrate shape; RAM-variant pick deferred
     to v0.6 empirical test pass. Onboard Wi-Fi 5 + BT 5.0; LoRa via
     Dragino HAT SX1276; GPS via Dragino HAT L80 (M39 variant per
     ADR-011 framing); USB-C PD wall path for desk bring-up, battery +
     buck topology for handheld (power spike scope).

  2. Pi 5 + USB SX1276 dongle (no HAT) + Pi Touch Display 2 (7") —
     contingency. Picked up only if all three on-hand Dragino HATs are
     confirmed defective on Pi 5 during v0.6 empirical test pass.

  3. CM5 + carrier + 22-pin DSI panel + SX1276 module — H0 fallback if
     Pi 5 fails the envelope (all three RAM variants throttle, cannot
     sustain map render, or BLE-through-shell fails irrecoverably).

  4. Zero 2 W — out-of-envelope per scoping; not pursued.

Pi 5 IO verification (deferred to v0.6 + gateway-rx-bringup-spike):
  SPI / RP1:                          deferred to v0.6 bring-up
  UART for L80:                       deferred to v0.6 bring-up
  I²C for DS3231:                     deferred to v0.6 bring-up
  GPIO interrupts (polled-RX impl.):  deferred — polled-RX remains v1a
                                      default per gateway-rx-bringup-spike
  Dragino HAT GPIO 25 CS defect on the 3 on-hand HATs:
                                      deferred — visual silkscreen-rev
                                      check + bench-test in v0.6 bring-up

v0.6 substrate empirical test pass — hard gate per Pi 5 RAM variant:
  (a) idle current at the 5V rail
  (b) typical-load current under walkers + PMTiles render at native
      refresh on Pi Touch Display 2 (7")
  (c) peak current under stress (boot transient + LoRa TX + map redraw)
  (d) sustained thermal under sealed-shell passive cooling
      (aluminum heat-spreader path; enclosure-spike scope)
  (e) UX feel of map render at native refresh (subjective; recorded
      with frame-time numbers where available)
  Result feeds: substrate-variant pick + ADR-015 authoring + power-
                architecture-spike close (idle/typical/peak numbers).

Display class:
  v1 build path:    Raspberry Pi Touch Display 2 (7", 22-pin DSI;
                    720×1280 native portrait, rotated to landscape via
                    DRM/KMS). 2× new units in procurement.
  v2 portrait:      Pi-5-compatible 5-inch 22-pin DSI panel (1× new
                    unit in procurement; parked, out of v1 scope).
  Salvage:          on-hand 15-pin 7" DSI Touchscreen from retired
                    pi3kiosk — NOT on v1 critical path (would require
                    22-pin→15-pin adapter ribbon + extra wire routing).
                    Retained for ad-hoc bench tests only.

Antenna paths through 3D-printed shell:
  LoRa SMA:                   bulkhead through shell — committed; mech
                              detail in enclosure-spike close.
  GPS SMA:                    open — sky-window-only vs external bulkhead
                              decided by enclosure-spike close (gated on
                              GPS-discipline runtime priority; ADR-011
                              keeps DS3231 RTC primary, GPS opportunistic,
                              so "no external GPS" is defensible).
  Pi 5 onboard WiFi/BLE
  through shell:              measure during v0.6 empirical test pass
                              (BLE-through-shell at arm's length is the
                              H2 trigger condition; result decides
                              external-dongle question).
  External BLE/WiFi dongle:   TBD by v0.6 BLE-through-shell measurement
                              + enclosure-spike close.

Cooling:
  active cooler:              REJECTED — no fan, no vent.
  passive heat-spreader:      aluminum plate, SoC-pad → plate → rear
                              shell inner wall as thermal mass + radiating
                              surface. Mech detail owned by enclosure-
                              spike close.
  thermal pad/paste:           cooling paste between SoC and heatsink/
                              spreader (one-time); thermal pad between
                              spreader and rear shell (re-workable).

Physical envelope (chosen substrate, indicative — exact L × W × H is
enclosure-spike scope):
  Pi 5 board:                 85 × 56 mm
  Dragino HAT Z-stack:        ~25-30 mm above Pi PCB (HAT + GPIO header
                              + SMA pigtail clearance)
  Pi Touch Display 2 (7"):    Pi-5-native 22-pin DSI; mounting via the
                              display's own threaded standoffs holding
                              the Pi PCB on the back face. Final L × W ×
                              H + bulkhead count: enclosure-spike close.

Cross-spike implications recorded:
  power spike:        receives v0.6 idle/typical/peak current measurements
                      per RAM variant; closes battery + buck topology
                      against the worst-case envelope. Named follow-up.
  enclosure spike:    Pi 5 + Pi Touch Display 2 (7", 22-pin DSI) +
                      Dragino HAT Z-stack + battery; passive aluminum
                      heat-spreader to rear shell (no active cooler);
                      LoRa SMA bulkhead committed; GPS SMA bulkhead vs
                      sky-window open; USB-C PD charge-port placement
                      open.
  runtime spike:      BLE central in a separate Yocto service (not the
                      kiosk binary) per the ble-gateway-ui-flow-spike
                      close — process / IPC architecture is the runtime
                      spike's domain.
  ble commissioning:  BLE-through-shell measurement is part of v0.6
                      empirical test pass, not a parallel investigation.
  rx bringup spike:   receives the chosen Pi 5 RAM variant after v0.6
                      close; bring-up runs against the picked variant.
  pmtiles retarget:   Pi Touch Display 2 (7") native 720×1280 portrait,
                      rotated to landscape via DRM/KMS; rotation handled
                      at the display layer, not in the egui/walkers code.

Procurement (placement is a separate bom.md update commit, not this one):
  1× Pi 5 2 GB
  1× Pi 5 4 GB
  1× Pi 5 8 GB
  3× Pi 5 official 27 W USB-C-PD PSU
  NO Pi 5 active cooler  (passive heat-spreader handles thermal)
  3× SanDisk High Endurance 64 GB microSD
  2× Raspberry Pi Touch Display 2 (7"; 22-pin DSI; Pi-5-native)
  1× Raspberry Pi Touch Display 2 (5") if available from RPi Foundation,
        else equivalent Pi-5-compatible 22-pin DSI 5" panel (verify SKU
        at checkout) — parked for v2 portrait variant.

Follow-up: ADR-004 amendment / ADR-015 authoring deferred to AFTER v0.6
empirical test pass closes (RAM variant + measured numbers feed it).
Named follow-ups for THIS spike close are:
  (1) power-architecture-spike close
  (2) bom.md procurement update (separate commit)

Not implemented in this spike: code, BOM commit (separate commit),
                                ADR edits, ordering decisions, enclosure
                                mechanical detail.

Next action: power-architecture-spike close + bom.md procurement update
             (separate commit). v0.6 empirical test pass opens once
             hardware arrives.
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
