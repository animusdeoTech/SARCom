---
title: "C1 depth — cited stack-up arithmetic, and a finding the 2026-05-13 hand-wave underestimated by ~15-25 mm"
date: 2026-05-14
type: dev-log
session-trigger: "C1 path decision from 2026-05-13 risk register; ran the stack-up math before mockup."
---

## Purpose

The 2026-05-13 CAD session closed at `front_depth (40) + rear_depth (35) = 75 mm` and parked the C1 decision for a "rested session, ideally after a cardboard mockup of 180×120×75 mm." That entry also flagged that "Pi stack + display ≈ 34 mm" was a hand-wave, not stack-up arithmetic, and that the 2026-05-08 spike-closes feeding the design were closed on a short Q&A round.

This file runs the arithmetic with cited per-layer dimensions before any mockup gets taped, because if the cited stack-up disagrees with 70-75 mm by a lot, the mockup dimensions themselves are wrong.

## Method

Each row below is one mechanical layer. Source is one of: spike-close text (`path` cited), official datasheet (URL cited), or **HAND-WAVE** (explicitly flagged, not in either source). Numbers in mm.

The layout is the one committed in `spikes/gateway-handheld-enclosure-spike.md` §Decision: "Pi 5 + Dragino HAT stack mounted to the back of the Pi Touch Display 2 via the display's own M2.5 standoffs — display + Pi assembly is ONE RIGID UNIT, attached to the front shell via the bezel." Front compartment holds the display+Pi+HAT assembly + heat path; rear compartment holds the Anker A1689 power bank.

## Front compartment Z-stack (outer face → divider)

| # | Layer | mm | Source |
|---|---|---|---|
| 1 | ASA front shell wall around bezel | 3.0 | **HAND-WAVE** — enclosure spike says "shell wall thickness ~3 mm"; no stack-up justification |
| 2 | Polycarbonate display window | 3.0 | `spikes/gateway-handheld-enclosure-spike.md` §Decision "display window: 3 mm polycarbonate" |
| 3 | Air gap window→display glass | 0.5 | **HAND-WAVE** — not specified anywhere |
| 4 | Pi Touch Display 2 (7") assembly | 15 (14.92 excl. protective film) | [Pi Touch Display 2 product brief PDF](https://datasheets.raspberrypi.com/display/touch-display-2-product-brief.pdf); confirmed against [Pi docs Touch Display 2 dimensions table](https://www.raspberrypi.com/documentation/accessories/touch-display-2.html) "7-inch depth: 15 mm" |
| 5 | Display mounting boss / standoff to Pi PCB | ~6-15 | **HAND-WAVE** — Pi docs §Step 4 only say "Align the four corner stand-offs of your Raspberry Pi with the four mounting points on the back of the Touch Display 2"; boss height not published. Depends on Pi orientation (see §Orientation note below) |
| 6 | Pi 5 PCB | 1.6 | Pi standard PCB stack — well known |
| 7 | Pi 5 GPIO 40-pin header above PCB top | 8.5 | Pi standard 40-pin male header height — well known |
| 8 | Dragino HAT envelope (HAT PCB + tallest component, seated on GPIO header) | 25 | Dragino LoRa GPS HAT User Manual v1.0 §1.8 "Size: 60mm × 53mm × 25mm" — [PDF](https://www.dragino.com/downloads/downloads/LoRa-GPS-HAT/LoRa_GPS_HAT_UserManual_v1.0.pdf). Substrate spike's "HAT Z-stack ~25-30 mm above Pi PCB" is consistent if the 25 mm is HAT-bottom-of-mating-connector to HAT-tallest-component |
| 9 | Clearance HAT-top → front-compartment divider face | ~2 | **HAND-WAVE** — not specified |
| | **Front compartment subtotal** | **~64.6** (best case, row 5 = 6 mm) to **~74.6** (worst case, row 5 = 15 mm) | |

Heat path (parallel, not additive): Pi 5 SoC (on Pi top side, +1.6 mm) → 1 mm thermal pad → 30×30×8 mm Al block → thermal paste → 1.5 mm AlMg3 sheet recessed in **rear** shell. The Al block lives in the same Z-volume as the HAT (different X-Y position; HAT sits over the GPIO-header edge, block sits over the SoC) — it does NOT add to the front compartment depth in principle, but the AlMg3 sheet is in the **rear** shell pocket, so the heat path crosses the divider. This is itself unresolved (see C1.4 below).

## Rear compartment Z-stack (divider → outer back face)

| # | Layer | mm | Source |
|---|---|---|---|
| A | Divider wall (ASA, printed) | ~1.5-2 | **HAND-WAVE** — not specified |
| B | Clearance divider → power bank | ~1-2 | **HAND-WAVE** |
| C | Anker A1689 power bank (short axis) | 30 | `spikes/gateway-handheld-power-architecture-spike.md` §Closed 2026-05-08 "154 × 62 × 30 mm" |
| D | Clearance power bank → AlMg3 pocket | 1-3 | **HAND-WAVE** — also conflicts with heat-spreader pocket placement |
| E | AlMg3 1.5 mm sheet in rear-shell pocket | 1.5 | enclosure spike §Decision |
| F | ASA rear shell wall | 3.0 | **HAND-WAVE** (same as row 1) |
| | **Rear compartment subtotal** | **~38-41.5** | |

## What the arithmetic says about C1

| Path (from 2026-05-13 dev-log) | Front needed (interior) | Rear needed (interior) | Total external | Reality vs path |
|---|---|---|---|---|
| (a) accept 75 mm | 40 - 3 wall = 37 | 35 - 3 wall = 32 | 75 | **doesn't fit.** Front needs ~58-68 mm interior; only 37 available. Rear is OK. |
| (b) trim front to 37 mm, zero margin | 34 | 32 | 72 | **doesn't fit by ~24-34 mm in front.** "Zero margin" estimate was based on the 34 mm hand-wave for "Pi stack + display"; real number is ~58-68 mm |
| (c) redesign to ≤65 mm | ≤32 | ≤30 | 65 | **physically impossible** with current substrate + display + HAT-on-back-of-display layout. |

The 2026-05-13 entry's "Pi stack + display ≈ 34 mm" omits the Dragino HAT envelope entirely. Re-reading: the line is "Pi stack + display" — if "Pi stack" was meant to exclude the HAT, the 34 mm is plausible (15 display + ~6 boss + 1.6 Pi + 8.5 GPIO + few mm slack ≈ 33 mm). With the 25 mm HAT envelope added, the actual front-compartment requirement is ~58-68 mm.

## So what's actually true about the C1 problem

The C1 issue is **a spec-language miss in the enclosure spike-close, not an architecture problem.** The 70–75 mm estimate in the 2026-05-13 dev-log was built bottom-up from the rear compartment (Anker + walls ≈ 36 mm) plus the "Pi stack + display ≈ 34 mm" hand-wave, with no HAT envelope counted. The Dragino HAT envelope is 25 mm per its v1.0 datasheet (and the substrate spike's own "HAT Z-stack ~25–30 mm above Pi PCB" matches). Add it in honestly and the real device depth is ~85–100 mm.

That is fine. This is a prototype, the device is a "doosje" by intent (see CLAUDE.md "values physical plug-and-play / 50% of the project"), and ~90 mm depth is a reasonable handheld prototype envelope — not a dimension that justifies rearchitecting the substrate, the cooling, or the HAT. The fix is in the spec language of `spikes/gateway-handheld-enclosure-spike.md` §Form factor and §Closed, where "depth: ~45 - 55 mm" should be replaced with the real number plus a reference to this stack-up.

A cardboard mockup is still useful, but at 180 × 120 × ~90 mm rather than 180 × 120 × 75 mm.

## Layout / orientation uncertainty (Pi-on-back-of-display)

Row 5 of the front stack is the biggest single uncertainty. Two plausible orientations, with very different stack consequences:

- **Orientation X** — Pi 5 BOTTOM side (USB-A, Ethernet) faces the display. Requires the display→Pi standoff to clear bottom-side connectors, which are ~13.5 mm tall (USB-A 3.0, RJ45). Standoff ≈ 14-15 mm. HAT stacks freely on Pi 5 top (GPIO header) toward the back. This is the configuration that *allows* the HAT-on-Pi-on-display stack to physically exist. Front stack: ~70+ mm.

- **Orientation Y** — Pi 5 TOP side (GPIO header) faces the display. Standoff only needs to clear GPIO header (8.5 mm). Cleaner mechanically and the conventional Pi-on-Touch-Display-1 mount. But the GPIO header is now *between* Pi and display — a Dragino HAT cannot stack on it without going *through* the display. HAT would need to be relocated (40-pin ribbon to a side-mounted HAT board), changing the whole internal layout. Front stack: ~30 mm (no HAT) + side-compartment volume for HAT.

The enclosure spike close does not resolve which orientation it intends. The 2026-05-13 CAD session implicitly chose Orientation X (HAT stacks on Pi behind display), but did not stack-up its z-cost.

## Things this exercise also surfaces

- **C1.4 — heat path crosses the divider.** Al block sits on Pi 5 SoC (front compartment); AlMg3 sheet is in rear shell pocket (rear compartment). Either the divider has a cut-out for the thermal column, or the AlMg3 sheet moves to the front-compartment back wall (no longer "rear shell pocket"), or the heat path needs a flexible thermal coupling. Not addressed in the enclosure spike close.
- **Boss height for Pi-on-Display-2 is not published.** Step 4 of the Pi docs just says "align with the standoffs." A real number requires either measuring the display (we don't have one yet), buying the display, or asking RPi support. This is a hard input the spike close didn't acquire.
- **Two un-cited 3 mm wall assumptions.** Front + rear ASA walls in the spike's "~3 mm" framing are not derived from print-quality requirements, IP65 wall-thickness recommendations, or structural calc. They're typical-FDM defaults.
- **Anker A1689 clearance assumption.** Power bank cannot be jammed against the divider or rear AlMg3 sheet without thermal coupling between Pi heat path and the lithium pack. The 1-3 mm clearance numbers above are hand-waved.

## Provisional implications (not decisions)

Real device depth in the orientation-X stack is ~85–100 mm. This is a prototype enclosure; ~90 mm is fine. The substrate stays (Pi 5 + Dragino HAT + Pi Touch Display 2), the cooling stays passive (heat-spreader pad + Al block + AlMg3 sheet, no fan), the Anker A1689 stays as the power source. **What changes is the spec language in `spikes/gateway-handheld-enclosure-spike.md`** §Form factor and §Closed: "depth: ~45 - 55 mm" gets replaced with the real number (~85–100 mm, pending the orientation-X vs Y resolution below) plus a back-ref to this stack-up. This is a spec-language correction, not an architecture decision.

The orientation question (Pi-TOP vs Pi-BOTTOM facing the display — see §Layout / orientation uncertainty above) is a separate concrete unknown that does need an answer before extrudes, because it affects where the HAT mounting hardware lives and how the DSI ribbon routes. It does not affect the depth-spec correction.

## What this does NOT do

- Does not commit any number to ADR-015 or ADR-017.
- Does not touch the substrate, the HAT, the cooling, or the power source. Those stay as the spike-closes already commit.
- Does not touch Fusion 360 yet. Extrudes still need the orientation question (Pi-TOP vs Pi-BOTTOM facing display) resolved.

## Next session pickup

1. **Update `spikes/gateway-handheld-enclosure-spike.md`** §Closed and §Form factor: change "depth: ~45 - 55 mm" → "depth: ~85–100 mm (per `dev-log/2026-05-14-c1-depth-stackup-arithmetic.md`)". Mark as spec-language correction, no architecture change.
2. **Resolve Orientation X vs Y** by measuring or sourcing the actual Pi-on-Touch-Display-2 boss height + confirming HAT-on-back-of-display geometry. This is the remaining real blocker for extrudes.
3. **Cardboard mockup** at 180 × 120 × ~90 mm when convenient — useful but no longer in the critical path.
4. **Proceed to extrudes** once orientation is settled and the depth spec is updated.

## Cross-refs

- `dev-log/2026-05-13-gateway-v1-cad-session-risks.md` — the entry this one extends (C1 specifically).
- `spikes/gateway-handheld-enclosure-spike.md` §Decision — internal layout text being checked.
- `spikes/gateway-handheld-substrate-spike.md` §Decision — option 2 (USB SX1276) lives there.
- `spikes/gateway-handheld-power-architecture-spike.md` §Closed 2026-05-08 — Anker A1689 154×62×30 mm.
- Pi Touch Display 2 product brief — https://datasheets.raspberrypi.com/display/touch-display-2-product-brief.pdf
- Pi Touch Display 2 docs page (dimensions table + mounting step 4) — https://www.raspberrypi.com/documentation/accessories/touch-display-2.html
- Dragino LoRa GPS HAT v1.0 User Manual §1.8 dimensions — https://www.dragino.com/downloads/downloads/LoRa-GPS-HAT/LoRa_GPS_HAT_UserManual_v1.0.pdf
