---
title: "Open risks and uncertainties after the 2026-05-13 gateway-v1 CAD session"
date: 2026-05-13
type: dev-log
session-trigger: "End-of-session risk register requested by Pieter; CAD skeleton complete, pre-extrusion checkpoint"
---

## What this captures

This dev-log records open risks and uncertainties accumulated during the 2026-05-13 CAD session that built the gateway-v1 sketch skeleton (22 user parameters, 5 components, 9 sketches, all landscape, all fully constrained, bulkheads placed, gasket grooves drawn, boss layout rationalised, bezel retargeted). It is NOT a TODO list and NOT a procurement spec. It is a register of "things assumed but not verified" + "things deferred" + "things waiting on input or empirical data". Future sessions should read this before extruding any feature or placing any order against the design.

## Substantive design decisions still open

- **C1 — total depth 75 mm vs the spike-close spec of 45–55 mm.** `front_depth (40) + rear_depth (35) = 75 mm`. The 45–55 mm range in the `gateway-handheld-enclosure-spike` close (2026-05-08) was a hand-wave estimate, not a stack-up calculation. Empirical minimum for the chosen stack — Pi 5 + the passive heat-spreader stack committed in the enclosure spike-close §Decision (1 mm thermal pad + 30 × 30 × 8 mm Al block + 1.5 mm AlMg3 sheet recessed in the rear shell; no fan, no active cooler) + Dragino HAT + Pi Touch Display 2 + polycarbonate window + Anker A1689 power bank + ASA walls — was estimated here at roughly 70–75 mm, but that estimate itself underweighted the HAT envelope. See [`2026-05-14-c1-depth-stackup-arithmetic.md`](2026-05-14-c1-depth-stackup-arithmetic.md) for the cited-per-row stack-up that puts the real depth at ~85–100 mm. The 2026-05-14 follow-up reframes C1 as a spec-language correction (update the spike-close depth field) rather than an architecture or cooling decision; the passive heat-spreader path stays, the substrate stays, the depth number on the spec is what changes. A cardboard mockup is still useful, but at ~90 mm rather than 75 mm.

- **Battery-door intent — 75 × 35 mm slide-out cap.** Pieter chose "keep as is" on 2026-05-13 but flagged that he cannot personally judge whether a 75 × 35 mm aperture is service-friendly for repeated battery swaps with a 154 × 62 × 30 mm Anker A1689 sliding axially through it. Geometrically the aperture clears the bank's 62 × 30 cross-section, so slide-out is feasible. Real-world ergonomics are untested. Revisit after first physical print plus one battery-swap rehearsal.

- **Divider cable pass-through position at (0, 0).** `front-shell/divider-horizontal` has its USB-C pass-through circle (radius 6 mm) at the geometric centre of the enclosure. This was a default chosen during the divider prompt; whether it matches the actual cable routing from the power bank's USB-C-PD output to the Pi 5's USB-C-PD input depends on where each port physically sits inside the front and rear compartments. Detail-design pending.

## Vendor / SKU placeholders that drive geometry

- **Magnetic-pogo charging connector — 25 W (5 V / 5 A) rating.** The bulkhead is drawn as a 25 × 15 mm rectangle (`pogo_w` × `pogo_h`) in `front-shell/Sketch1`. Specific vendor / module not sourced. Most consumer magnetic-USB cables on the EU market handle 1–2 A; Pi 5 at full load draws up to 5 A. Industrial magnetic charging connectors (Würth REDOX class, similar) handle higher current but cost more. Until a specific module is identified, the 25 × 15 mm footprint is a placeholder — the actual cutout shape and depth will shift once a part is chosen.

- **Gore PolyVent SKU and exact bore.** `polyvent_bore = 11 mm` is a midpoint of the Gore PolyVent SMT-series range (10–13 mm). Specific Gore PolyVent SMT-class part number not chosen; the final bore could shift ±2 mm. Geometry will resize at procurement.

- **IP67 sealed power button — APEM IRR/IZP class assumed.** `button_bore = 16 mm` reflects an M16 thread, common for IP67 momentary buttons in the €5–8 range. Actual vendor part not sourced; if a different SKU is chosen (different thread spec, different mounting depth), the bore may need adjustment.

- **SMA-female bulkhead — 6.35 mm literal.** B1's bore in `front-shell/Sketch1` is drawn with the literal `6.35 mm / 2` dimension. SMA is a standard, so the bore is geometrically fixed; the parametric chain is intentionally bypassed here. No risk per se — just an inconsistency in the parametric chain worth flagging.

## Material / thermal / sealing assumptions untested

- **AlMg3 1.5 mm × 80 × 60 mm heat-spreader thermal performance.** Sized to dissipate Pi 5 ~5 W typical / ~7–8 W peak via passive thermal coupling to ambient through the sealed ASA rear shell. The 80 × 60 footprint is a rough thermal-resistance estimate. Real-world performance under sustained peak load, in a sealed shell, with the Anker A1689 nearby (potential thermal cross-talk) — not measured. Possible failure mode: sustained throttling under load.

- **Buna-N 2 mm cord-stock gasket in printed groove under thermal cycling.** Gasket material plus groove geometry (1 mm radial width per side, 2 mm depth) chosen per typical hobbyist FDM IP65 practice. Compression-set behaviour over weeks-to-months of temperature cycling in a sealed enclosure with a Pi 5 inside — not measured. Possible failure mode: gradual loss of seal compression, IP65 drift.

- **ASA print IP65 quality on a real hobbyist FDM printer.** Spike close named ASA for the UV plus thermal envelope. Printer + filament + layer height + post-processing all influence actual IP65 outcome. First print will likely NOT be IP65 without iteration on layer height, perimeter count, wall ironing, or post-print sealant. Expect 2–3 print iterations to dial in.

- **DRM/KMS 90° rotation for Pi Touch Display 2 (portrait → landscape).** Native portrait orientation of Pi Touch Display 2 needs a kernel/userspace rotation to display landscape. Assumed to work via standard Yocto / kernel cmdline parameter. Not bench-tested on Pi 5 + this specific display.

## Geometric findings worth tracking

- **Bezel clamping pattern asymmetric.** After the 2026-05-13 corner-pitch fix (Option B — short-edge ±52.5 bosses dropped) and the bezel retarget to (±70, ±53.5), the bezel's 4 corner screws now cluster on the long edges. Clamping force pattern is no longer symmetric across all four quadrants. For IP65 sealing of the polycarbonate window this should be adequate — the bezel's own rigidity spreads the load — but if the window leaks at the short-edge midpoints in a rain test, the bezel attachment scheme needs revisiting (more bezel screws, or a different boss layout).

- **Gasket groove offset curves on rounded corners.** The groove's outer plus inner offset curves (`d40 = 0.5 mm`, `d41 = 0.5 mm + gasket_width / 2 = 1.5 mm`) inherit rounded corners with r = 7.5 mm and r = 6.5 mm respectively. The shapes are geometrically correct for following a rounded enclosure, but the extrude operation hasn't been performed yet and may surface unexpected solver issues — particularly where the groove approaches the bulkhead cutouts on the perimeter.

## Process / tooling caveats

- **Earlier spike-closes from 2026-05-08 are shallow.** The substrate, power-architecture, and enclosure spike-closes that this CAD work consumes (commits `e68db96`, `bd5505b`, `87f6dcb`) were closed on a small chat-Q&A round, not on the slow research process documented in `docs/spike-rules.md`. CLAUDE.md captured this on 2026-05-08 (the "Process discipline" subsection). The 75 mm depth conflict is one concrete instance where the spike's spec was not empirically grounded. Other assumptions baked into those closes may also need revision when full stack-up arithmetic is done.

- **Fusion 360 MCP bridge (`ndoo/fusion360-mcp-bridge`) is alpha.** Single commit, ~7 weeks old. The `fusion_screenshot` path is broken against Fusion 2702.x; PRs #1 and #2 cover the fix but neither is merged. If the bridge breaks on a future Fusion or Claude Code update, this CAD workflow stops until the bridge is patched or replaced. Autodesk's official Fusion MCP is the documented fallback if a paid Fusion subscription is acquired.

- **No extrudes performed yet.** The next major work block is sketches → solid features (extrudes for shell walls, cuts for display window + bulkheads, recess for heat-spreader pocket, bezel and battery-door assembly). Risk that some sketches won't extrude cleanly on first attempt — particularly the gasket-groove offset geometry and the rectangular pogo cutout that sits tangent to the bottom envelope edge. Plan for 1–2 sessions of extrude debugging.

## Next session pickup

Next session should: (a) decide the C1 depth path with a fresh head, optionally after a cardboard mockup of 180 × 120 × 75 mm to feel ergonomically; (b) if extrudes start, do them feature-by-feature with a re-audit after each to catch solver issues early; (c) revisit the shallow 2026-05-08 spike-closes if the C1 decision reveals other unverified assumptions baked into them. Do NOT touch procurement of any physical component until the design file produces a clean exported BOM with finalised vendor SKUs.
