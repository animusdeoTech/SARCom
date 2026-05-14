---
title: "Spike — Handheld gateway battery + charging + power-good architecture"
status: closed
type: spike
timebox: 1 day
opened: 2026-05-06
closed: 2026-05-08
amended: 2026-05-14
---

# Spike: Handheld gateway battery + charging + power-good architecture

## 2026-05-14 partial supersession — magnetic-pogo charging retired

The **magnetic-pogo half of the H2 verdict is superseded.** The gateway no longer has any external charging input in the enclosure wall. Recharging is now strictly external: open the battery service door, remove the power bank, charge the bank via its own built-in USB-C cable to a wall adapter, return it. The bank-inside-the-shell architectural commitment from §Closed 2026-05-08 is unchanged; only the charging path and the `POWER_GOOD` signal contract change.

Concrete consequences:

- **`POWER_GOOD` is removed from the signal contract.** Without an external charger input in the shell, there is no "external power present" state for the SBC to read. The signal does not exist in v1.
- **CoT/TAK export gate language changes** to **"WiFi + manual opt-in"** (two inputs, not three). The "external power" input is gone; near-empty-battery emit is allowed and the existing low-VBUS clean-shutdown path catches the consequence at file-system level. Pending ADR-016 gate wording updates accordingly.
- **`BATTERY_STATE` and `CHARGE_STATE`** were already declared not-firmware-readable in the 2026-05-08 contract (commercial bank exposes neither over a usable interface). The 2026-05-14 amendment removes them from the firmware signal surface entirely; they remain operator-visible via the bank's own LED count during service.
- **`SHUTDOWN_REQUEST` survives unchanged** — the Pi 5 still reads VBUS-droop on its own USB-C-PD input and raises a clean-shutdown request; no enclosure-side signal is needed.
- **One magnetic-pogo bulkhead** disappears from the enclosure bulkhead inventory (handled separately in `spikes/gateway-handheld-enclosure-spike.md` 2026-05-14 amendment).
- **Magnetic charge cable accessories** (cable + spare) drop from the BOM.

The §Closed 2026-05-08 and §Decision sections below are amended inline; the original text remains visible with the changed clauses replaced. Decision-note dated `2026-05-14` amendment block at top of §Decision lists the exact clause-level changes.

See also `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`.

## Closed 2026-05-08

**Verdict — H2.** A commercial USB-C-PD power bank, mounted **inside** the handheld shell as a serviceable internal component, replaces the H1 "custom 2S Li-Ion + BMS + buck + charger IC" path. H1 (custom charging IC + BMS + buck topology) and H0 (single-cell hobby-BMS desk demonstrator) are both **rejected** for v1 — the commercial bank already integrates BMS, balancing, OV/UV, OCP, and 5V/5A PD output to the Pi 5; reproducing that in a one-off design is engineering risk without payoff at this stage.

**Working candidate:** Anker A1689 25600 mAh / 87 W / ~95 Wh, **119.9 × 73.4 × 31.4 mm** `[CORRECTED 2026-05-14 — Anker official spec; anker.com/eu-en/products/a1689]` (was: "~25600 mAh / 87 W / ~95 Wh / 154 × 62 × 30 mm" in 2026-05-08 verdict; the capacity / wattage / Wh figures were also part of the 2026-05-08 hand-wave and remain unverified against the official spec — flagged for procurement-ticket follow-up). Other PD-capable power banks of similar spec are interchangeable — the architectural commitment is **"~20-25 Ah PD-capable power bank inside the shell"**, not a vendor lock. SKU pinning is a procurement-ticket detail.

**Runtime envelope.** Conservative 9 W typical worst-case (Pi 5 8 GB + Pi Touch Display 2 7" + Dragino HAT active LoRa RX). 95 Wh / 9 W ≈ 10.5 h typical; 95 Wh × ~80 % effective / 12 W peak ≈ 6 h peak. Targets (≥6 h typical, ≥4 h worst-case) hold with margin.

**Charging path.** **External only — pack out, pack's own USB-C to wall charger, pack back.** No charging input in the enclosure wall at all in v1. The Anker A1689 has an integrated USB-C input cable; recharging means opening the battery service door, removing the bank, plugging its own USB-C into any wall adapter (or PD source), then returning it. ~~External magnetic-pogo connector in the shell wall…~~ — *retired 2026-05-14 per top-of-file partial supersession; see dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md.* The trade is that the device cannot run while the bank is being charged in v1 (no pass-through), but this matches the SARCOM use case (shift-change swap, not mid-incident charging) and eliminates one IP65 sealing surface, daily-cycled connector wear, and a magnetic cable accessory category.

**Output path.** Power bank's USB-C-PD output → internal USB-C cable → Pi 5's USB-C-PD input at 5V/5A. Pi 5 powers the entire stack including display backlight via the Pi 5 PMIC. **No custom buck. No TP4056. No BQ24074. No PMIC integration.** The bank IS the power architecture.

**Clean-shutdown contract — OPEN.** Three candidate paths surfaced, final pick deferred to runtime-task-architecture-spike close: (a) Pi 5 firmware low-VBUS detection → graceful shutdown via systemd; (b) external small UPS HAT (~30 s buffer) between bank and Pi 5; (c) accept unclean shutdown — rely on SQLite WAL durability + a conservative read-only Yocto rootfs partition layout. **v1 default: (a) + (c) combined** — soft-shutdown on detected low-VBUS, accept unclean tail.

**Service.** **Battery service door is the only regular access path.** The 2026-05-14 amendment promotes the previously-optional service door to mandatory and removes the alternative "clamshell open → battery compartment" path from regular service — the main clamshell stays sealed except for non-battery service events. Workflow: service door open → power bank slides out → bank charged externally on the bench via its own integrated USB-C cable → return same bank or a charged spare → close door. **Hot-swap NOT supported in v1** (Pi 5 powers down during swap; operationally acceptable in the SARCOM use case — hut staff swap on shift change, not mid-incident).

Named follow-ups: **enclosure-spike close** (consumes the **119.9 × 73.4 × 31.4 mm** `[CORRECTED 2026-05-14 — Anker official spec; anker.com/eu-en/products/a1689]` power-bank envelope + magnetic-pogo bulkhead + service-door question for internal layout) and **runtime-task-architecture-spike close** (formalises the SHUTDOWN_REQUEST signal sequencing and the systemd-graceful-shutdown daemon).

Decisions captured below in the §Decision note.

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

## Decision note

```
Date: 2026-05-14 amendment (pogo half of H2 superseded)

Changed clauses (this amendment overrides the 2026-05-08 text below
for these clauses only; everything else stays):

  - Charging input → external only (pack out, bank's own USB-C to
    wall, pack back). No enclosure-wall connector. Magnetic-pogo
    bulkhead, magnetic cable accessory + spare: REMOVED.
  - Signal contract / POWER_GOOD: REMOVED. No external charger
    presence to detect in v1.
  - Signal contract / BATTERY_STATE, CHARGE_STATE: REMOVED from
    firmware signal surface. Operator-visible via bank's own LED
    count during service only.
  - CoT/TAK export gate language: "WiFi + manual opt-in"
    (2 inputs, not 3). The "external power" input is gone.
  - Service: battery service door promoted from optional to the
    ONLY regular access path. Main clamshell stays sealed except
    for non-battery service.
  - Operating-envelope caveat "peak-while-charging": REMOVED
    (no in-shell charging in v1; charging happens off-device).

See dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md for the
session context that produced this amendment, and
spikes/gateway-handheld-enclosure-spike.md 2026-05-14 amendment
for the mirrored bulkhead-inventory change.

================================================================

Date: 2026-05-08

Topology recommended: USB-power-bank class — commercial USB-C-PD power
  bank as a SERVICEABLE INTERNAL COMPONENT inside the handheld shell.
  Working candidate: Anker A1689 25600 mAh / 87 W / ~95 Wh
  (119.9 × 73.4 × 31.4 mm) [CORRECTED 2026-05-14 — Anker official
  spec; anker.com/eu-en/products/a1689; the 2026-05-08 verdict had
  154 × 62 × 30 mm which does not match the Anker product page].
  Capacity / wattage / Wh figures inherited from 2026-05-08
  hand-wave; verify at procurement-ticket time. Other PD-capable
  banks of similar spec are interchangeable; SKU pinning is a
  procurement-ticket detail.
  Architectural commitment: ~20-25 Ah, PD-capable, ≥87 W output, fits
  the internal envelope.

H1 (custom 2S + BMS + buck + charger IC):  REJECTED — engineering risk
  without payoff at v1; commercial bank already integrates the same
  protections.
H0 (single-cell hobby BMS desk demo):       REJECTED — does not deliver
  Pi 5 5V/5A peak, no clean-shutdown story.

Cell chemistry assumed (not SKU-locked): Li-Ion (whatever cells the
  commercial bank uses internally; cell-level chemistry is not
  exposed to us by the bank).

Nominal pack voltage / capacity range:
  internal cells: 3.7 V Li-Ion (bank vendor's choice)
  bank PD output: USB-C-PD profiles (5/9/12/15/20 V); we use the 5V/5A
                  profile for the Pi 5
  capacity envelope: ≥20 Ah / ≥95 Wh @ 3.7 V nominal

Buck topology to 5V/5A:
  NO custom buck designed by us. The bank's USB-C-PD output IC
  delivers 5V/5A directly to the Pi 5's USB-C-PD input; the Pi 5 PMIC
  handles internal rails. No TP4056, no BQ24074, no external PMIC.

Energy budget arithmetic (worst-case envelope per substrate-spike close):
  typical (Pi 5 8 GB + 7" DSI + Dragino HAT active LoRa RX):
                                   ~9 W → 95 Wh / 9 W ≈ 10.5 h runtime
  peak   (above + WiFi assoc + CoT emit):
                                   ~12 W × 0.80 effective → ≈ 6.3 h
  worst-case (still hits ≥4 h target with margin):
                                   ≈ 8 h at sustained 12 W
  charge time, near-empty → 80% via 87 W magnetic-pogo input:
                                   ~70-80 min (95 Wh × 0.80 / 87 W /
                                   0.85 PD-conversion)

Charging input:
  [SUPERSEDED 2026-05-14 — no in-shell charging path in v1]
  external connector type:    NONE. Recharging is external only:
                              open battery service door → remove
                              power bank → charge bank via its
                              own integrated USB-C cable to any
                              wall adapter → return bank → close
                              door.
  rating:                     n/a (handled by whatever PD source
                              the operator plugs the bank's own
                              cable into; bank rates its own input).
  internal path:              n/a (no enclosure-side charging path).
  cable accessory shipped:    NONE. Bank's integrated USB-C cable
                              is the only charging cable; user
                              provides the wall adapter.

Power output path:
  power bank's USB-C-PD output → internal USB-C cable → Pi 5 USB-C-PD
  input @ 5V/5A → Pi 5 PMIC powers display backlight + Dragino HAT +
  any USB peripherals.

Protections (handled by the commercial bank unless noted):
  per-cell BMS:                handled by bank (internal)
  balancing:                   handled by bank (internal)
  inrush limit:                handled by bank's PD controller +
                               Pi 5 PMIC
  over-current fuse:           handled by bank + Pi 5 PMIC
  NTC cold-charge cutoff:      NOT exposed by typical PD power banks
                               — accepted as out-of-envelope; see
                               operating-envelope caveat below.
  over-temp shutdown:          handled by bank (internal)
  OV / UV cutoff:              handled by bank (internal)
  reverse-polarity:            n/a (USB-C is keyed; magnetic-pogo
                               vendor handles polarity)
  USB-C ESD:                   handled by bank's USB-C input stage
                               when external charger is connected
                               externally; no in-shell USB-C surface
                               in v1.

Signal contract:
  [SUPERSEDED 2026-05-14 — POWER_GOOD removed; BATTERY_STATE /
   CHARGE_STATE removed from firmware surface entirely.]
  POWER_GOOD       — REMOVED. No external charger input exists in
                     v1, so "external power present" is not a state
                     the SBC can read. Implication for CoT/TAK gate:
                     dropped from the gate predicate (see top-of-
                     section amendment block).
  BATTERY_STATE    — REMOVED from firmware signal surface. Operator-
                     visible only via the bank's own LED count during
                     service. Not consumed by any firmware task in v1.
  CHARGE_STATE    — REMOVED from firmware signal surface. Charging
                     happens off-device on the bench; no in-firmware
                     "charging / charged / fault" inference path
                     exists in v1.
  THERMAL_STATE    — type: NOT exposed by the bank. Pi 5 SoC thermal
                            is readable via /sys/class/thermal — that
                            is a separate signal owned by the runtime
                            spike, not this one.
                     consumed by: runtime spike thermal task.
  SHUTDOWN_REQUEST — type: derived locally on the Pi 5 — when VBUS
                            from the bank starts to droop under load
                            (bank near-empty), Pi 5 firmware low-VBUS
                            detection raises the signal and systemd
                            initiates graceful shutdown. Implementation
                            detail owned by runtime-task-architecture-
                            spike close.
                     consumed by: systemd via a small monitor daemon.
  Power button     — debounced momentary switch on a Pi 5 GPIO; tied
                     into systemd reboot/poweroff target. Standard
                     integration; no design specials here.

Clean-shutdown approach (v1 default):
  (a) Pi 5 firmware low-VBUS detection → graceful systemd shutdown
      — adopted.
  (c) accept unclean tail — adopted as fallback. Rely on SQLite WAL
      durability + a read-only Yocto rootfs partition layout to make
      "unclean shutdown does not corrupt the filesystem" the system-
      level guarantee.
  (b) external small UPS HAT (~30 s buffer) — REJECTED for v1 (extra
      hardware, extra failure mode, the (a)+(c) combo is sufficient
      for handheld use).

Runtime measurement method:
  rig:           USB-C inline power meter on the magnetic-pogo input
                 (charge side); USB-C inline power meter on the
                 bank → Pi 5 cable (output side); thermistor or
                 IR thermometer on the heat-spreader.
  loads:         (idle) Pi 5 booted, kiosk idle, LoRa RX listening
                 (typical) above + walkers + PMTiles redraw at native
                          refresh
                 (peak)   above + WiFi monitor active + CoT/TAK
                          emitting (when ADR-016 lands)
  capture path:  USB power meters log to CSV via PC during bench
                 runs; v1 does not need an in-Pi power-data table.
                 Promotion of power data into a SQLite table is a
                 v2 ask routed to the runtime spike, not this one.

Service:
  [SUPERSEDED 2026-05-14 — battery service door promoted to the only
   regular access path; clamshell-open path retired for routine
   battery service.]
  battery access:     battery service door open → power bank slides
                      out / charged bank slides in → close door.
                      Main clamshell stays sealed except for non-
                      battery service events (e.g. SD card recovery,
                      mainboard service).
  hot-swap:           NOT supported in v1 (Pi 5 powers down during
                      swap; operationally acceptable for shift-change
                      swap, not mid-incident).
  separate door:      MANDATORY (promoted from optional 2026-05-08
                      verdict) — own gasket + 2× M3 captive screws,
                      sealed. Geometry owned by
                      spikes/gateway-handheld-enclosure-spike.md.

Cross-spike implications recorded:
  tak-cot-integration:
      [SUPERSEDED 2026-05-14] Export gate is now
      "WiFi + manual opt-in" (2 inputs). POWER_GOOD has been removed
      from the gate predicate because no external charger input
      exists in v1. Bank-empty conditions are still caught at the
      file-system level by the Pi 5 low-VBUS clean-shutdown path
      (SHUTDOWN_REQUEST below) — no CoT/TAK-side change is needed
      for graceful tail behaviour.
  gateway runtime tasks:
      power_monitor task reads Pi 5 VBUS voltage from PMIC and
      raises SHUTDOWN_REQUEST when VBUS droops past threshold.
      [SUPERSEDED 2026-05-14] No external-VBUS-presence GPIO/ADC
      input — that signal does not exist in v1. Daemon design owned
      by runtime-task-architecture-spike close.
  enclosure (battery placement / vent / service):
      [SUPERSEDED 2026-05-14] 119.9 × 73.4 × 31.4 mm bank envelope
      (Anker A1689 class) [CORRECTED 2026-05-14 — Anker official
      spec; anker.com/eu-en/products/a1689; was 154 × 62 × 30 mm in
      2026-05-08 verdict] + MANDATORY battery service door (promoted
      from optional). NO magnetic-pogo bulkhead. Internal divider
      plate separates display+Pi+HAT compartment from battery
      compartment. Geometry owned by
      spikes/gateway-handheld-enclosure-spike.md 2026-05-14
      amendment.
  substrate (5V/5A peak):
      Pi 5 8 GB worst-case envelope (substrate-spike close) confirmed
      compatible with the 87 W bank output via standard USB-C PD;
      no external buck.
  production-concerns §2 (cold-charge):
      promoted to v1 scope as a USER-FACING OPERATIONAL CAVEAT —
      "do not charge below 0°C" — because the commercial bank does
      NOT expose a cold-charge NTC cutoff. Documented in handover
      notes (production-concerns close); not solved in firmware.
  production-concerns §4 (clean-shutdown rootfs / SQLite):
      promoted to v1 scope and addressed by the (a)+(c) combo above.

Operating-envelope caveats accepted (written here, not buried):
  - cold charge:           bank does not have NTC cutoff. Operational
                           rule: do not charge below 0°C. Visible in
                           user-facing handover notes.
  - peak-while-charging:   [SUPERSEDED 2026-05-14 — no in-shell
                           charging in v1] Charging happens off-
                           device on the bench. The device runs from
                           the bank until depleted; clean shutdown
                           on low VBUS catches the tail. Shift-
                           change swap is the operational pattern,
                           not run-while-charging.
  - bank state visibility: bank's internal SoC / cell health / thermal
                           NOT readable by the Pi 5 in v1. Operator
                           checks the bank's LED indicator during
                           service; firmware infers via VBUS-droop.
  - hot-swap:              not supported. Pi 5 powers down during
                           battery swap. Operationally acceptable in
                           the SARCOM use case (hut staff swap on
                           shift change, not mid-incident).

Not implemented in this spike: part selection (procurement ticket),
                                magnetic-pogo SKU choice (procurement
                                ticket), enclosure mechanical layout
                                (enclosure-spike CAD), clean-shutdown
                                daemon code (runtime-task-architecture-
                                spike close), PCB design, BOM commit.

Follow-ups filed:
  (1) enclosure-spike close — consumes 119.9 × 73.4 × 31.4 mm
      [CORRECTED 2026-05-14 — Anker official spec;
      anker.com/eu-en/products/a1689] bank envelope, magnetic-pogo
      bulkhead, optional battery-service door question.
  (2) runtime-task-architecture-spike close — formalises the
      SHUTDOWN_REQUEST signal sequencing and the low-VBUS systemd-
      graceful-shutdown daemon.

Next action: enclosure-spike close + runtime-task-architecture-spike
             close. Procurement of the bank itself rolls into the
             bom.md update commit alongside the substrate-spike close
             outputs.
```

## Cross-references

- `production-concerns.md` §2 (cold-charging Li-Ion) and §4 (clean-shutdown filesystem behavior) — promoted from "post-v1 risk register" to v1 scope.
- `decisions/ADR-004-gateway-platform.md` — does not currently address power.
- `spikes/gateway-handheld-substrate-spike.md` — peak-load profile.
- `spikes/gateway-handheld-enclosure-spike.md` — battery envelope, ventilation, service door.
- `spikes/gateway-runtime-task-architecture-spike.md` — power_monitor task.
- `spikes/tak-cot-integration-spike.md` — POWER_GOOD as the export gate.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
