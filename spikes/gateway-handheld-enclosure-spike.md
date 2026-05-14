---
title: "Spike — Handheld gateway 3D-printed waterproof enclosure (Fusion 360)"
status: closed
type: spike
timebox: 1 day
opened: 2026-05-06
closed: 2026-05-08
amended: 2026-05-14
---

# Spike: Handheld gateway 3D-printed waterproof enclosure

## 2026-05-14 partial supersession — magnetic-pogo charging bulkhead retired

The **magnetic-pogo charging connector is removed from the bulkhead inventory and from §Decision.** The gateway no longer has any external charging input in the enclosure wall. Recharging is fully external: open the battery service door, remove the power bank, charge the bank via its own integrated USB-C cable, return it. The battery service door is promoted from optional to **mandatory** (it is now the only regular access path).

The mirrored power-side change lives in `spikes/gateway-handheld-power-architecture-spike.md` 2026-05-14 amendment. The pending ADR-016 CoT/TAK export gate language drops the "external power" input and becomes **"WiFi + manual opt-in"**.

What stays unchanged: LoRa SMA bulkhead (top edge centred), IP67 sealed power button (side edge), Gore PolyVent membrane (rear shell). IP65 target stays. ASA material commitment stays. Internal layout (Pi 5 + Dragino HAT on back of Pi Touch Display 2, Anker A1689 in separate compartment) stays. Passive heat-spreader path (1 mm thermal pad → 30 × 30 × 8 mm Al block → AlMg3 sheet) stays.

See also `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`.

## Closed 2026-05-08

**Verdict — H1.** IP65 (rain + dust, no immersion) is the committed v1 target, on a single-material ASA clamshell with a Buna-N 2 mm cord-stock gasket, a 3 mm polycarbonate display window, off-the-shelf threaded SMA + IP67 power button, ~~magnetic-pogo charging (no panel-mount USB-C)~~ **[SUPERSEDED 2026-05-14 — no in-shell charging in v1; see top-of-file supersession]**, and a Gore PolyVent rear membrane. H0 (IP54 + pouch policy) is rejected for v1.

**v1 build path is the 7" landscape variant only.** Form factor: ~180 × 120 mm front face, **~85–100 mm depth** (spec-language correction 2026-05-14 per [`dev-log/2026-05-14-c1-depth-stackup-arithmetic.md`](../dev-log/2026-05-14-c1-depth-stackup-arithmetic.md) — the prior "~45–55 mm" figure was a hand-wave that did not stack-up the Dragino HAT envelope; the corrected number is what the chosen substrate + HAT + display + Anker A1689 + passive heat-spreader path actually require. No architecture, cooling, or substrate change). Single LoRa SMA bulkhead centered on the top edge; **no GNSS bulkhead** (Dragino HAT L80-M39's onboard 15 × 15 × 4 mm patch antenna fires up through the ASA shell back/top — defensible because ADR-011 keeps DS3231 RTC primary, GPS opportunistic). The 5" portrait variant is parked as a v2 follow-up spike — **no parametrised dual-front design is produced here.**

**Acceptance gate.** Terril Waterschei in measurable rain, no water ingress. Failing the rain test does NOT silently demote to IP54; it triggers a rework loop on the gasket compression + window seal, not a scope reduction.

**Material commitment: ASA throughout.** Single material across shell halves, bezel, internal mounts, internal dividers — to keep thermal-expansion behavior uniform across mating surfaces (mismatched-CTE seams leak).

**Internal layout.** Pi 5 + Dragino HAT stack mounted to the back of the Pi Touch Display 2 (7") via the display's own M2.5 standoffs (display + Pi assembly is one rigid unit, attached to the front shell via the bezel). Anker A1689-class power bank (**119.9 × 73.4 × 31.4 mm** `[CORRECTED 2026-05-14 — Anker official spec; anker.com/eu-en/products/a1689; was "~155 × 60 × 30 mm" in 2026-05-08 verdict, consumed from power-architecture-spike's own hand-wave dims]`) lives in a separate compartment behind/below the Pi+HAT+display unit, divided by an internal ASA wall. **Bank orientation in the rear compartment:** long axis (119.9 mm) along device X, width axis (73.4 mm) along device Y, thickness axis (31.4 mm) along device Z. This places the battery service door on the **+X face** of the rear shell (perpendicular to the bank's long axis, so the bank slides out axially along its 119.9 mm long axis through the 73.4 × 31.4 cross-section aperture). The 30 mm of X-axis slack in the rear compartment (interior 171 mm minus bank 119.9 mm) is an open architectural question — see `dev-log/2026-05-14-anker-dims-and-gate-propagation.md` §(a). Pi 5 SoC thermal path: 1 mm thermal pad on SoC → 30 × 30 × 8 mm aluminum heat-spreader block → thermal paste → AlMg3 1.5 mm sheet (80 × 60 mm) recessed into a rear-shell pocket. **Optional separate battery-service door is INCLUDED** (own Buna-N O-ring + 2× M3 captive screws) — keeps the main clamshell sealed for non-battery service. (Door promoted from optional to mandatory in the 2026-05-14 amendment at top of file.)

**Bulkhead inventory locked:** 1× LoRa SMA (top edge centred), 1× IP67 sealed power button (side edge), ~~1× magnetic-pogo charging connector (opposite side or bottom edge)~~ **[SUPERSEDED 2026-05-14 — removed]**, 1× Gore PolyVent (rear shell, off the grip surface), 1× mandatory battery service door (own Buna-N O-ring + 2× M3 captive screws — promoted from optional 2026-05-14; the only regular access path). Zero GNSS SMA, zero SD service slot (v1 SD access requires opening the main clamshell — acceptable for prototype service interval), zero commissioning-trigger button (magnet+reed equivalent is a v2 consideration), zero in-shell charging connector.

**Drop tolerance.** Chamfered or radiused corners in the v1 print. Optional silicone bumper sleeve as a v2 polish item.

Named follow-up: **Fusion 360 modelling ticket** — the actual CAD work. Build a print-1-test-1 ASA prototype before committing to colour/finish. The ticket is not written in this commit.

Decisions captured below in the §Decision note.

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
- Heat path for Pi 5 in a sealed shell cannot dissipate peak load without exceeding cell-pack thermal limits — fall back to CM5 / Zero 2W in the substrate spike, or accept thermal throttling under peak as a v1 constraint. (Pi 4 retired 2026-05-07.)
- Display window seal cannot pass capacitive touch through the chosen window thickness — change material (e.g. thinner glass insert) or accept a press-fit window with a non-touch UI fallback (pivot away from touchscreen as the only input — would require ADR-007 amendment, flag explicitly).

## Fallback / next action

- If H1 holds: write the Fusion 360 modelling ticket with the design brief from this spike. Build a print-1-test-1 prototype before committing to colour/finish.
- If H0 (IP65 unachievable): adopt IP54 + pouch policy for v1; revisit IP65 in v2 once a print profile + post-processing workflow exists.

## Decision note

```
Date: 2026-05-08

IP target chosen: IP65 (rain + dust, no immersion).
  Reason: hand-holdable SAR-adjacent use case demands rain tolerance;
  IP67 is over-spec for a handheld that does not get submerged and
  forces gasket-rope + harder display-window seal; IP54 (H0) is
  unacceptable for unaided field carry. Acceptance gate: Terril
  Waterschei rain test, no ingress. Failing the gate triggers a
  rework loop on gasket compression + window seal — NOT a silent
  demotion to IP54.

Material chosen: ASA throughout (single material across shell halves,
  bezel, internal mounts, internal dividers).
  Reason: outdoor UV stability, thermal-cycle stability across the
  -10°C to +60°C envelope, and matched CTE on every mating surface
  (mismatched-CTE seams leak when the device cycles temperature).
  PETG rejected for v1 (UV degradation under sustained sun load,
  brittle in winter cold). PLA explicitly out (sun deformation).
  SLA/resin out for the shell (brittle); acceptable for non-load
  detail like button caps if needed.

Form factor (v1, 7" landscape variant only):
  front face:         ~180 × 120 mm
  depth:              ~85 - 100 mm   (corrected 2026-05-14 per
                                       dev-log/2026-05-14-c1-depth-
                                       stackup-arithmetic.md; original
                                       "~45 - 55 mm" was a hand-wave
                                       that did not stack-up the
                                       Dragino HAT envelope. No
                                       architecture, cooling, or
                                       substrate change.)
  orientation:        landscape
  display:            Raspberry Pi Touch Display 2 (7", 22-pin DSI),
                      720×1280 native portrait rotated to landscape
                      via DRM/KMS at the display layer.
  CAD modules:        ONE design produced in v1 — 7" landscape.
  5" portrait variant: parked as a v2 follow-up spike. NO parametrised
                      dual-front design produced here.

Sealing strategy:
  main seam:
    type:           clamshell with cut Buna-N 2 mm cord-stock gasket
                    in a printed groove (2 mm wide × 2 mm deep,
                    square or D-shape profile), glued in with a thin
                    layer of structural silicone (Dow 3145 or
                    equivalent).
    compression:    M3 stainless screws every 30 - 40 mm around the
                    perimeter, into ASA-printed bosses with M3 brass
                    heat-set inserts (Ruthex-class).
    vendor / spec:  Buna-N (NBR) cord-stock at hardware-store
                    grade is acceptable; 70-Shore-A. Specific cord
                    SKU is procurement-ticket detail.

  display window:
    material:       3 mm polycarbonate (Lexan 9034 grade or
                    equivalent UV-stable PC). NOT acrylic (PMMA
                    cracks under impact), NOT 3D-printed (not
                    optically clear, not flat enough).
    seal:           1 - 2 mm Buna-N O-ring in a printed groove on the
                    bezel + structural-silicone bond at the perimeter
                    as a redundant layer.
    bezel attach:   bezel attaches to the front shell via 4× M3
                    corner screws + heat-set inserts.
    touch:          capacitive touch passes through 3 mm PC at the
                    sensitivity Pi Touch Display 2 supports —
                    confirm during print-1-test-1 prototype.

  power button:
    type:           off-the-shelf IP67 sealed momentary button
                    (APEM IRR / IZP class or equivalent, ~€5-8
                    each).
    mounting:       threaded mount through shell wall with built-in
                    O-ring; specific vendor SKU at procurement.
    quantity:       1 (power only). NO commissioning-trigger button
                    in v1.

  LoRa SMA:
    bulkhead spec:  off-the-shelf threaded SMA-female bulkhead with
                    built-in O-ring (Amphenol or equivalent,
                    ~€2-3). Threads into the shell via printed
                    thread + M-lock-nut from inside.
    location:       top edge, centred.

  GPS SMA:
    bulkhead spec:  N/A — no GPS bulkhead in v1.
    reason:         L80-M39 onboard 15×15×4 mm patch antenna on the
                    Dragino HAT fires up through the ASA shell
                    back/top. ADR-011 keeps DS3231 RTC primary,
                    GPS opportunistic, so no-external-GPS is
                    defensible. If field testing shows the patch
                    cannot get sky view through the shell, the
                    follow-up is to add an external GPS SMA — that
                    is a v2 amendment, not a v1 blocker.

  USB-C charging:
    [SUPERSEDED 2026-05-14 — no in-shell charging path in v1]
    type:           NONE. No panel-mount USB-C, no magnetic-pogo,
                    no charging input in the enclosure wall at all.
    spec:           n/a.
    rating:         n/a.
    location:       n/a.
    accessory:      NONE. Recharging is external only: open the
                    battery service door → remove the power bank
                    → charge the bank via its own integrated USB-C
                    cable to any wall adapter → return the bank →
                    close the door.
    reason:         eliminates one IP65 sealing surface, one daily-
                    cycled connector, one BOM accessory category,
                    and removes the POWER_GOOD signal from the
                    firmware contract (no external charger presence
                    to detect). Mirror of the 2026-05-14 amendment
                    in gateway-handheld-power-architecture-spike.md.

  vent membrane:
    type:           Gore PolyVent (or equivalent ePTFE adhesive
                    membrane), ~€5/piece.
    location:       rear shell, away from gasket grooves and the
                    display window, off the grip surface.
    mounting:       printed boss with a 6 - 8 mm hole; adhesive-
                    mount membrane.
    reason:         pressure equalisation during temperature cycles
                    so the seal does not suck moist air in when the
                    device cools down, AND condensation-prevention
                    inside the shell.

Internal layout:
  substrate envelope (Pi 5 + Dragino HAT Z-stack + Pi Touch Display 2 (7")):
                    Pi 5 PCB  85 × 56 mm
                    HAT Z-stack ~25 - 30 mm above Pi PCB
                    Display footprint follows the Pi Touch Display 2
                    (7") back-mounting standoffs.
                    Pi+HAT mounts to the BACK of the display via the
                    display's own M2.5 standoffs — display + Pi
                    assembly is ONE RIGID UNIT, attached to the
                    front shell via the bezel.

  battery envelope (Anker A1689-class power bank, per power-arch close):
                    119.9 × 73.4 × 31.4 mm [CORRECTED 2026-05-14 —
                    Anker official spec; anker.com/eu-en/products/
                    a1689; was 154 × 62 × 30 mm in 2026-05-08
                    verdict]. Separate compartment behind / below
                    the Pi+HAT+display unit, divided by an internal
                    ASA wall.

  total internal volume target:
                    bounded by the form factor above (~180 × 120 ×
                    45-55 mm minus shell wall thickness ~3 mm and
                    bezel/divider plate volumes). Exact mm³ in CAD.

  heat path:
                    Pi 5 SoC → 1 mm thermal pad → 30 × 30 × 8 mm
                    aluminum heat-spreader block → thermal paste →
                    AlMg3 1.5 mm sheet (80 × 60 mm) recessed into a
                    rear-shell pocket, glued OR screwed in with a
                    thermal-paste interface to ambient. NO active
                    cooler. NO fan. NO vent for cooling (the Gore
                    PolyVent is for pressure equalisation, NOT
                    convective cooling).

  condensation:
                    Gore PolyVent membrane handles humidity exchange.
                    No silica-gel pocket in v1 — vent is sufficient.
                    Revisit if condensation observed during field
                    test.

  strain relief:
                    IPEX 1.0 → SMA pigtail from the Dragino HAT
                    routed to the central top-edge SMA bulkhead;
                    immobilised at the IPEX end with a small
                    printed clamp or a hot-glue spot. Per
                    production-concerns.md §3.

  service path for power bank:
                    SEPARATE BATTERY-SERVICE DOOR included — own
                    Buna-N O-ring + 2× M3 captive screws. Main
                    clamshell stays sealed for non-battery service.

Bulkhead inventory:
  [AMENDED 2026-05-14 — magnetic-pogo entry removed; battery
   service door promoted from optional to mandatory.]
  LoRa SMA:                 1× — top edge, centred. Mandatory.
  GPS SMA:                  0 — explicitly NOT included (L80-M39
                            patch antenna onboard).
  USB-C charging:           0 — no in-shell charging path in v1.
  magnetic-pogo charging:   0 — RETIRED 2026-05-14. (Was: 1×
                            mandatory in 2026-05-08 verdict.)
  power button:             1× — IP67 sealed, side edge. Mandatory.
  battery service door:     1× — own Buna-N O-ring + 2× M3 captive
                            screws. MANDATORY (promoted from
                            optional 2026-05-14). Only regular
                            access path.
  commissioning button:     0 — deferred. Magnet+reed-switch
                            equivalent considered for v2.
  SD service slot:          0 — explicitly NOT included for v1.
                            SD access requires opening the main
                            clamshell. Acceptable for prototype
                            service interval.
  vent membrane:            1× — Gore PolyVent on rear shell.

Battery serviceability:
  user-replaceable via the dedicated battery-service door. Power
  bank slides out / new one slides in. Hot-swap NOT supported in v1
  (Pi 5 powers down during swap — per power-architecture-spike close).

Drop tolerance / bumpers in v1:
  chamfered or radiused corners in the v1 print. Optional silicone
  bumper sleeve as a v2 polish item.

Operating-procedure caveats accepted (written here, not buried):
  - rain handling at chosen IP rating:
      IP65 is the design target. Acceptance gate is the Terril
      Waterschei rain test before declaring v1 acceptance. NOT
      tested for immersion. Operator handover note: do NOT submerge.
  - heat envelope under peak load:
      passive heat-spreader to rear AlMg3 plate. Sustained peak
      load (Pi 5 8 GB + LoRa RX + WiFi monitor + CoT/TAK emit)
      may throttle the Pi 5 toward ~80°C junction. Acceptable for
      the SARCOM workload (mostly RX + render; peak is rare).
      Validated empirically during the v0.6 substrate empirical
      test pass per substrate-spike close.
  - service intervals:
      Buna-N gasket inspection at every battery swap; gasket
      replacement after 2 years OR on visible degradation
      (cracking, hardening, deformation). Vent membrane
      replacement every 5 years OR after observed condensation
      inside the shell.
  - GPS sky-view through ASA shell:
      L80-M39 patch antenna fires up through 3 mm ASA. If field
      tests show the patch cannot fix in the shell, escalate to
      v2 amendment for an external GPS SMA. Not a v1 blocker.

Cross-spike implications recorded:
  substrate envelope:
      Pi 5 (any RAM, empirical pick at v0.6) + Dragino HAT +
      Pi Touch Display 2 (7", 22-pin DSI) consumed unchanged from
      substrate-spike close.
  power architecture:
      119.9 × 73.4 × 31.4 mm Anker A1689-class power bank envelope
      [CORRECTED 2026-05-14 — Anker official spec; anker.com/eu-en/
      products/a1689; was 154 × 62 × 30 mm in 2026-05-08 consumed
      figure] + magnetic-pogo bulkhead consumed unchanged from
      power-architecture-spike close. Battery-service door QUESTION
      from that spike: ANSWERED HERE — yes, separate door included.
      (Both magnetic-pogo bulkhead and door-optional clause are
      further amended 2026-05-14; see top-of-file supersession.)
  runtime tasks:
      power-button GPIO debouncing + low-VBUS shutdown daemon
      remain owned by runtime-task-architecture-spike close.
      Mechanical surfaces here; firmware behavior there.
  production-concerns §3 (IPEX strain relief):
      promoted to v1 active. Printed clamp or hot-glue spot at
      the IPEX end of the LoRa pigtail; CAD ticket implements.
  ADR-007 (read-only touchscreen):
      preserved literally — the polycarbonate window is an
      input-passthrough surface, the touchscreen below remains
      the only UI. No physical input affordances on the shell
      compete with the touchscreen.

Not implemented in this spike: CAD geometry, print profiles,
                                prototype fabrication, vendor SKU
                                pinning, BOM commit, ADR edits.

Follow-up filed: Fusion 360 modelling ticket — the actual CAD
                 work. Build a print-1-test-1 ASA prototype before
                 committing to colour/finish. Ticket NOT written
                 in this commit.

Next action: open the Fusion 360 modelling ticket. Procurement
             of the SMA bulkhead, IP67 power button, magnetic-pogo
             connector, Gore PolyVent, Buna-N cord-stock, M3 heat-
             set inserts + screws, structural silicone, polycarbonate
             sheet, and AlMg3 sheet rolls into the bom.md update
             commit alongside the substrate + power-architecture
             outputs.
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
