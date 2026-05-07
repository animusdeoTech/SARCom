---
title: "Spike — Handheld gateway battery + charging + power-good architecture"
status: open
type: spike
timebox: 1 day
opened: 2026-05-06
---

# Spike: Handheld gateway battery + charging + power-good architecture

## Why this spike exists

The pivot makes the gateway **rechargeable, battery-powered, handheld**. None of the existing ADRs cover battery topology, charge/discharge management, or a *power-good* signal — yet several pivot features depend on that signal:

- **Conditional outbound CoT/TAK export** must only fire when external power is present *and* the battery is not in a critical state, so a brown-out mid-emit doesn't corrupt SQLite WAL or end-stream a TAK client mid-message (see `tak-cot-integration-spike.md`).
- **Pi 5 5V/5A peak** is not a casual power supply requirement; mis-sized supplies will throttle the SBC under load.
- **Cold-charge cutoff** is a real safety question — `production-concerns.md` §2 already flags lithium plating below 0°C for the relay's 18650; the gateway pack is a larger version of the same problem.
- **Clean shutdown on low battery** prevents SD-card / SQLite corruption; `production-concerns.md` §4 already flags this against mains-loss; battery operation makes it a daily concern.

This spike scopes the topology, the protections, the signals, and the runtime measurement method. It does **not** pick parts.

## Hypothesis / research question

**H1.** A 2S Li-Ion pack (e.g. 2× 21700 or 2× 18650, ~6000–10000 mAh @ 7.2–8.4 V nominal) feeding a regulated 5V buck to the Pi 5, with a USB-C-PD-fed integrated BMS+charger IC that exposes a power-good and a charge-state pin to the SBC, will deliver ≥4 hours of typical operation (kiosk + LoRa RX + GPS + occasional WiFi) and ≥1 hour at peak (Pi 5 thermal load + WiFi associated + CoT emitting). Cold-charge cutoff via NTC is mandatory, not optional.

**H2.** A USB-power-bank-class topology (single 5V/5A USB power bank, gateway fed via USB-C from the bank) is enough — sidesteps custom BMS design, but loses the power-good signal unless the bank exposes one (most don't), and ties charging behavior to whatever the bank does internally. Acceptable for v1a if instrumentation is added on the input side.

**H0.** v1 ships with a single 18650-class cell + a hobby BMS module (TP4056-style) and a buck converter. Cheapest, fastest, has known cold-charge limitations and may not deliver Pi 5 peak. Documented as a desk demonstrator with explicit operating-envelope caveats; not a deployable handheld.

## Scope fence

- **No IC selection.** The spike does not pick "TI BQ25798" or any specific charger / BMS part. Part selection is the follow-up implementation ticket.
- **No cell SKU lock-in.** Cell chemistry / capacity are not frozen here.
- **No PCB design.** Custom carrier-board questions are out of scope; the spike picks topology shape and protection requirements, not layout.
- **No firmware coding.** Power-monitor task design is in `gateway-runtime-task-architecture-spike.md`; this spike specifies the *signals* the firmware reads, not the firmware reading them.
- **No mechanical decisions.** Battery placement / ventilation / serviceability live in `gateway-handheld-enclosure-spike.md`.
- **No retroactive change to relay / tag power.** Tag is single 18650 via Tracker V2 PMIC (ADR-002); relay is single 18650 via Solar Kit charge controller (ADR-003); this spike does not move either.

## What to verify

### Topology

1. Battery chemistry + cell count + nominal voltage band: 1S Li-Ion, 2S Li-Ion, LiFePO4 4S, or USB power bank class. Compute energy budget per option against:
   - typical-load runtime target (≥4 h, working hypothesis)
   - peak-load duration (Pi 5 thermal + WiFi + CoT emit; ≥1 h, working hypothesis)
   - charge time from "near-empty" to 80% with a USB-C-PD source at 30 W or 65 W
2. **Pi 5 supply path:** 5V/5A regulated input. Verify candidate buck-converter envelopes can deliver this from each chemistry option without dropout under transient.
3. **USB-C charging path:** USB-C-PD profile. Confirm whether the chosen topology requires a PD-aware charger IC or a fixed-input charger with a PD trigger downstream.
4. **Output topology:** does the system run from buck-only-when-on-battery and pass-through-when-on-power, or is the Pi always fed from the buck regardless of source? Path-controller behavior matters for the *power-good* signal.

### Protections (each is a yes/no/defer for v1)

- Per-cell BMS / balancing (mandatory for 2S+; optional but recommended for 1S protected cells)
- Inrush current limit
- Over-current fuse (resettable polyfuse vs glass)
- NTC on the cell pack with a low-temperature charge cutoff (≥0°C cell-internal before charge is allowed) — **mandatory** per the same physics that drives `production-concerns.md` §2
- Over-temperature shutdown (high-side, ~50°C cell-internal)
- Over-voltage / under-voltage cutoff at the pack
- Reverse-polarity protection if a user-replaceable cell is anywhere in the design
- ESD protection on the USB-C connector

### Signals to expose to the SBC

- **`POWER_GOOD`** — high while *both* (a) external power is present at the USB-C input *and* (b) the charger/path-controller has stabilised to charging-or-fully-charged state. This is the gate for outbound CoT/TAK emit (per `tak-cot-integration-spike.md`).
- **`BATTERY_STATE`** — readable cell voltage and/or fuel-gauge SoC. I²C fuel gauge (BQ27xxx-class) vs simple ADC on a divider — pick by topology.
- **`CHARGE_STATE`** — charging / charged / fault. Single bit or two-bit, exposed to GPIO or via I²C from the charger IC.
- **`THERMAL_STATE`** — NTC voltage on an ADC; cold-charge cutoff also exposed as a status bit.
- **`SHUTDOWN_REQUEST`** — momentary signal from a discrete watchdog when battery hits the firmware-defined critical threshold and a clean shutdown must begin.
- **Power button** — debounced; tied into shutdown/wake. Single hardware button (no display modal required to power-cycle, per ADR-007 read-only kiosk).

### Runtime budget — measurement method

- Runtime measurement target on the bench: how long does the unit run from full to "low battery, clean shutdown" under (a) idle kiosk + LoRa RX, (b) typical case, (c) peak (WiFi associated + CoT emitting).
- USB current meter on the input side (already on the desk-hygiene order in TODO.md), DMM on the cell side, optional shunt + INA219 if continuous logging is needed.
- Logging cadence and data path to SQLite (so power data lives next to `tag_reports` for later analysis) — note as a request to the runtime-task spike, do not implement here.

### Cross-spike implications (record, don't solve)

- `tak-cot-integration-spike.md`: `POWER_GOOD` is the gate; this spike defines the signal contract.
- `gateway-runtime-task-architecture-spike.md`: a `power_monitor` task reads `POWER_GOOD`/`BATTERY_STATE`/`CHARGE_STATE`/`THERMAL_STATE`; this spike specifies the inputs.
- `gateway-handheld-enclosure-spike.md`: battery placement / vent / serviceability; this spike says how big the cell envelope is and whether it's user-replaceable.
- `gateway-handheld-substrate-spike.md`: Pi 5 5V/5A peak is the dominant load; CM5 / Zero 2W shift the envelope. (Pi 4 retired 2026-05-07 — not a candidate.)
- `production-concerns.md` §2 (cold charge) and §4 (clean-shutdown rootfs/SQLite): this spike pulls both into v1 scope.

## Pass criteria

- Topology recommended (1S / 2S / LiFePO4 / USB-power-bank), with energy-budget arithmetic for typical / peak / charge-time targets.
- Protection list filled: each item marked mandatory / recommended / deferred-with-reason.
- Signal contract drafted: each of `POWER_GOOD` / `BATTERY_STATE` / `CHARGE_STATE` / `THERMAL_STATE` / `SHUTDOWN_REQUEST` defined with electrical type (GPIO level / I²C address / ADC channel) and which firmware task consumes it.
- Runtime measurement method documented: the bench rig, the loads, the data capture path. Numbers are not produced in this spike — the *method* is.
- Cross-spike implications recorded as one-line notes referencing the consuming spike.

## Fail criteria

- No topology survives the energy-budget arithmetic for both ≥4 h typical and ≥1 h peak with a credible cell envelope — re-scope the runtime targets explicitly with the user; the handheld may need to accept ≥2 h typical, or the substrate may need to drop from Pi 5 to CM5 / Zero 2W.
- USB-C-PD source negotiation cannot deliver enough power into the chosen topology to charge while running at peak — explicitly accept "charge-only-while-idle" as a v1 constraint in the decision note.
- Cold-charge cutoff cannot be implemented without a custom IC selection; declare cold-charge an explicit out-of-envelope warning for v1 and add a "do not charge below 0°C" operational note (mirrors the same outcome as `production-concerns.md` §2 for the relay).

## Fallback / next action

- If H1 holds: write the part-selection ticket against the topology + protection list + signal contract.
- If H2 (USB power bank) is chosen: spec which power-bank features must be exposed (≥30 W PD output, pass-through charging, ideally a power-good output); add an explicit "no cold-charge cutoff" warning since most banks lack one.
- If H0 (single-cell hobby BMS) is chosen: document operating envelope honestly and downgrade the runtime targets in the decision note.

## Decision note template

```
Date:
Topology recommended: 1S / 2S / LiFePO4 / USB-power-bank / other:
Cell chemistry assumed (not SKU-locked): Li-Ion / LiFePO4 / NMC:
Nominal pack voltage / capacity range:
Buck topology to 5V/5A: yes / no / via PD trigger:

Energy budget arithmetic:
  typical (kiosk + LoRa RX + GPS): __ Wh, __ h runtime
  peak   (Pi 5 + WiFi assoc + CoT emit): __ Wh, __ h runtime
  charge time, near-empty → 80% via USB-C-PD: __ minutes at __ W

Protections:
  per-cell BMS:                mandatory / recommended / deferred:
  balancing:                   mandatory / recommended / deferred:
  inrush limit:                mandatory / recommended / deferred:
  over-current fuse:           mandatory / recommended / deferred:
  NTC cold-charge cutoff:      mandatory (≥0°C):
  over-temp shutdown:          mandatory / recommended / deferred:
  OV / UV cutoff:              mandatory / recommended / deferred:
  reverse-polarity:            mandatory / recommended / deferred:
  USB-C ESD:                   mandatory / recommended / deferred:

Signal contract:
  POWER_GOOD       — type:           consumed by:
  BATTERY_STATE    — type:           consumed by:
  CHARGE_STATE     — type:           consumed by:
  THERMAL_STATE    — type:           consumed by:
  SHUTDOWN_REQUEST — type:           consumed by:
  Power button     — debounce / wake:

Runtime measurement method:
  rig:
  loads (idle / typical / peak):
  capture path (USB current meter / DMM / INA219 / SQLite power table):

Cross-spike implications recorded:
  tak-cot-integration:    ___
  gateway runtime tasks:  ___
  enclosure (battery placement / vent / service): ___
  substrate (5V/5A peak): ___
  production-concerns §2 and §4 promoted: ___

Operating-envelope caveats accepted (write here, do not silently bury):
  - cold charge: ___
  - peak-while-charging: ___
  - other: ___

Not implemented in this spike: part selection, PCB design, firmware code, BOM commitments.

Next action:
```

## Cross-references

- `production-concerns.md` §2 (cold-charging Li-Ion) and §4 (clean-shutdown filesystem behavior) — promoted from "post-v1 risk register" to v1 scope.
- `decisions/ADR-004-gateway-platform.md` — does not currently address power.
- `spikes/gateway-handheld-substrate-spike.md` — peak-load profile.
- `spikes/gateway-handheld-enclosure-spike.md` — battery envelope, ventilation, service door.
- `spikes/gateway-runtime-task-architecture-spike.md` — power_monitor task.
- `spikes/tak-cot-integration-spike.md` — POWER_GOOD as the export gate.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
