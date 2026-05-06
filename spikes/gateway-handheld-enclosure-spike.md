---
title: "Spike — Handheld gateway 3D-printed waterproof enclosure (Fusion 360)"
status: open
type: spike
timebox: 1 day
opened: 2026-05-06
---

# Spike: Handheld gateway 3D-printed waterproof enclosure

## Why this spike exists

The pivot ships the gateway in a **custom Fusion 360 designed and 3D-printed waterproof handheld shell**. None of the existing ADRs cover enclosure mechanicals; ADR-003 covers the relay's OEM Solar Kit shell only. The enclosure is now a first-class deliverable — IP rating, materials, gasket seal, display window, button membrane, antenna bulkheads, USB-C bulkhead, internal heat path, condensation, battery serviceability are all unanswered.

Per `production-concerns.md` §3, IPEX strain relief is a real risk on the relay; the gateway has the same problem on its LoRa SMA path *and* possibly its GPS SMA path. Per §4, SD-card clean-shutdown was already a concern on mains; battery operation moves it to daily.

This spike scopes the mechanical envelope, the IP target, and the major design-decision categories so the Fusion 360 work has a brief.

## Hypothesis / research question

**H1.** A handheld gateway can hit **IP65** in v1 (rain + dust, no immersion) using a single-gasket clamshell with a printed-in gasket groove, an O-ring seal around the display window, threaded SMA bulkheads with built-in O-rings, a USB-C panel-mount-with-cap, and ASA or PETG with weather-stable colorant. **IP67** (immersion to 1 m, 30 min) is a stretch goal — pushes from O-ring to gasket-rope and complicates the display window and the USB-C port.

**H0.** Custom-printed waterproofing at hobbyist precision misses IP65 due to layer-line porosity / flatness tolerance; v1 ships at IP54 (splash) and bag-or-pouch handling is added to the operating procedure for rain.

## Scope fence

- **No CAD modelling.** Spike output is a design brief + decision note, not Fusion 360 geometry.
- **No print profile decisions.** Material recommendations live here; print orientation, layer height, infill belong to the print-profile follow-up.
- **No final dimensions.** Internal envelope is bounded by `gateway-handheld-substrate-spike.md` (SBC + HAT + display + battery footprint); this spike consumes that bounding box, doesn't redo it.
- **No tag enclosure.** Tag is a separate spike (`tag-handheld-enclosure-spike.md`).
- **No relay enclosure changes.** Relay sticks with the OEM Solar Kit shell per ADR-003.
- **No production injection-mould or post-printing finishing decisions** — those are post-v1.

## What to verify

### IP rating target

- **IP65** (rain + dust, no immersion): hand-holdable with reasonable rain tolerance; matches the SAR-adjacent use case where the operator may be outdoors. Working v1 target.
- **IP67** (immersion to 1 m, 30 min): stretch goal. Forces rope gasket, all-metal bulkheads with proper O-rings, more attention to the display window seal, and likely a vent membrane (Gore PolyVent or equivalent) to handle pressure changes during seal compression.
- **IP54** (splash): fallback. Acceptable if H0 wins.

### Materials

- **PETG**: easy to print, decent UV resistance, brittle in cold. Default candidate for first prototype.
- **ASA**: better UV + thermal performance, harder to print. Stronger choice for outdoor exposure.
- **PLA**: explicitly out — UV degrades, deforms in sun-loaded enclosure.
- **SLA / resin print**: fine surface finish, water-tight at the wall but expensive and brittle. Useful for a non-load-bearing detail (e.g. button caps); not the shell.
- **Inserts:** brass heat-set inserts for any threaded fastening point that opens/closes more than once (battery service, possible firmware-recovery access).

### Sealing strategy (each is a concrete design choice)

- **Main shell seam:** clamshell with printed gasket groove + closed-cell foam rope OR liquid silicone gasket OR cut Buna-N gasket. Pick one and justify.
- **Display window:** O-ring + transparent insert (acrylic or PETG sheet) bonded with structural silicone OR thermoformed window with a flange. Touchscreen capacitive layer behind window — verify the chosen window thickness still passes touch.
- **Buttons:** silicone membrane bonded to a printed bezel, OR sealed tactile switches with O-ring (e.g. APEM IP67 series equivalent). At minimum: power button, possibly a "commissioning trigger" button (hardware-side analog of the magnet+reed-switch pattern from ADR-006 for relays).
- **SMA bulkheads:** factory-sealed bulkhead-mount SMA connectors with built-in O-ring. Threaded into a heat-set insert in the shell wall. One mandatory (LoRa); optional second (GPS) if GPS antenna is external.
- **USB-C charging port:** panel-mount USB-C connector with rubber cap (tethered to the shell so it doesn't get lost), OR magnetic-Pogo charging with no exposed connector at all (cleaner seal, more BOM cost). Decide.
- **Vent / pressure-equalisation:** Gore PolyVent or equivalent — small adhesive PTFE membrane that lets pressure equalise without letting water in. **Mandatory at IP67.** Recommended at IP65 to prevent the seal sucking in moist air during a temperature drop.

### Internal layout / mechanicals

- **Internal envelope** consumed from the substrate spike: SBC + HAT + display + battery + bulkheads + cable bend radius.
- **Heat path for Pi 5 in a sealed shell:** active-cooler + duct that exhausts to a heat-sink-style internal mass, OR aluminum back-plate as the heat sink (Pi 5 thermal pad → aluminum plate epoxied or screwed to the rear shell). Sealed enclosures *do not* run open-air-cooler well; document the chosen heat path.
- **Condensation:** combination of (a) the vent membrane equalising humidity, (b) silica gel pack in a serviceable pocket, (c) keeping the PCB above the lowest internal point so any liquid water collects below it. Pick the approach.
- **Battery service:** is the cell pack user-replaceable (door + heat-set screws + foam pillow), or sealed (battery dies → ship back / open the shell as a service event)? This decision interacts with the cold-charge / service-life discussion in `gateway-handheld-power-architecture-spike.md`.
- **Strain relief on internal pigtails:** IPEX → SMA pigtail from HAT to bulkhead must be immobilised at the IPEX end (per `production-concerns.md` §3). Print a small clamp or a glue spot.
- **Drop tolerance / corner protection:** chamfered or radiused corners; optional silicone bumpers or a printed soft-shell layer for v2.

### Bulkhead inventory

Working list, refined during the spike:
- 1× LoRa SMA (mandatory)
- 0–1× GPS SMA (optional, depends on whether GPS antenna lives inside the shell with a sky-window or terminates at a bulkhead)
- 1× USB-C charging (or Pogo, see above)
- 1× power button
- 0–1× commissioning button (or replaced by a magnet+reed-switch through-shell input — a reed sensor inside, magnet held against the case from outside)
- 0–1× SD card service slot (sealed, only opened during firmware recovery)

### Cross-spike implications (record, don't solve)

- `gateway-handheld-substrate-spike.md`: physical envelope; this spike consumes it.
- `gateway-handheld-power-architecture-spike.md`: battery service door, vent for cold-charge thermal management, USB-C location.
- `gateway-runtime-task-architecture-spike.md`: power-button / commissioning-button GPIO debouncing belongs to firmware, not here.
- `production-concerns.md` §3 (IPEX strain relief) and §4 (clean shutdown — already promoted in power spike).

## Pass criteria

- IP target chosen (65 / 67 / 54-fallback) with reasoning.
- Material chosen (PETG vs ASA vs other) with reasoning, including UV / thermal envelope.
- Sealing strategy named for each surface: main seam, display window, buttons, SMA, USB-C, vent.
- Internal layout sketch (text + dimensions): substrate envelope, battery envelope, heat path, condensation strategy, strain-relief approach.
- Bulkhead inventory committed (count + type).
- Cross-spike implications recorded.

## Fail criteria

- IP65 cannot be reached at hobby-FDM print precision in v1 — drop to IP54 explicitly and update the operating-procedure note ("rain protection requires a bag/pouch").
- Heat path for Pi 5 in a sealed shell cannot dissipate peak load without exceeding cell-pack thermal limits — fall back to Pi 4 / CM5 / Zero 2W in the substrate spike, or accept thermal throttling under peak as a v1 constraint.
- Display window seal cannot pass capacitive touch through the chosen window thickness — change material (e.g. thinner glass insert) or accept a press-fit window with a non-touch UI fallback (pivot away from touchscreen as the only input — would require ADR-007 amendment, flag explicitly).

## Fallback / next action

- If H1 holds: write the Fusion 360 modelling ticket with the design brief from this spike. Build a print-1-test-1 prototype before committing to colour/finish.
- If H0 (IP65 unachievable): adopt IP54 + pouch policy for v1; revisit IP65 in v2 once a print profile + post-processing workflow exists.

## Decision note template

```
Date:
IP target chosen: 65 / 67 / 54 — reason:
Material chosen: PETG / ASA / other — reason:

Sealing strategy:
  main seam:        type:                              vendor / spec:
  display window:   type:                              spec:
  buttons:          type (membrane / sealed switch):   how many:
  LoRa SMA:         bulkhead spec:
  GPS SMA:          bulkhead spec / N/A:
  USB-C:            panel-mount-with-cap / Pogo / other:
  vent membrane:    Gore PolyVent / other / none — reason:

Internal layout:
  substrate envelope (from substrate spike): __ × __ × __ mm
  battery envelope:                          __ × __ × __ mm
  total internal volume target:              __ mm³
  heat path:        active cooler+duct / aluminum back-plate / other:
  condensation:     vent + desiccant / vent only / desiccant only / other:
  strain relief:    printed clamp / hot-glue / other:

Bulkhead inventory:
  LoRa SMA:                yes
  GPS SMA:                 yes / no — reason:
  USB-C charging:          yes / Pogo / other:
  power button:            yes
  commissioning button:    yes / via magnet+reed / no — reason:
  SD service slot:         yes / no — reason:

Battery serviceability:
  user-replaceable / sealed — reason:

Drop tolerance / bumpers in v1: yes / no / v2:

Operating-procedure caveats accepted (write here, do not silently bury):
  - rain handling at chosen IP rating: ___
  - heat envelope under peak load: ___
  - service intervals (gasket replace / battery): ___

Cross-spike implications recorded:
  substrate envelope:   ___
  power architecture:   ___
  runtime tasks:        ___
  production-concerns §3 (IPEX strain relief): ___

Not implemented in this spike: CAD geometry, print profiles, prototype fabrication.

Next action:
```

## Cross-references

- `decisions/ADR-003-relay-hardware.md` — Solar Kit shell for the relay; *not* the gateway model; cited for contrast.
- `decisions/ADR-004-gateway-platform.md` — does not currently address mechanicals.
- `decisions/ADR-007-touchscreen-primary-ui.md` — read-only touchscreen; this spike must preserve that constraint when picking the window/button strategy.
- `production-concerns.md` §3 (IPEX) and §4 (clean shutdown).
- `spikes/gateway-handheld-substrate-spike.md` — physical envelope source.
- `spikes/gateway-handheld-power-architecture-spike.md` — battery placement, vent, service door.
- `spikes/tag-handheld-enclosure-spike.md` — sibling for the tag's separate Fusion 360 shell.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
