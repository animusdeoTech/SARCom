# KIOSK-006 — SOS alerting (persistent strip + background pulse) — v1a mockup rationale

Single-design strict-ADR mockup. Two stacked 800×480 lab fixtures
showing the persistent SOS bottom strip in single-SOS and multi-SOS
scenarios. Strip is **full-width** SOS state when active; **no
zones**, **no coord readout**, **no banner**, **no acknowledge button
(non-goal)**, **no dismiss affordance**. Map areas render the
three-layer basemap composition per
`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`. SVG at
`UX/mockups/KIOSK-006-sos-strip.svg`.

## Scenario summary (two panels)

**Panel A — single SOS (tag-2).** Map shows tag-2 with red SOS pulse
ring per `tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`, over the
three-layer basemap (basemap line-art + LIDAR hillshade gradient + OSM
XML overlays). Other markers (tag-1, tag-4, relay-1, gw-0) rendered at
reduced prominence. Sidebar abstracted with the sticky DISTRESS section
showing `🔴 SOS · tag-2 · 42 s`. Bottom strip spans the **full 800 px
fixture width**, 24 px tall (height unchanged from current per
`tickets/KIOSK-006-sos-alerting.md:33, 80`), red fill
`Color32::from_rgb(160, 28, 28)` per
`tools/sarcom-kiosk-lab/src/app.rs:245`. Strip text in 13 pt bold
white, single line, exactly:

```
DISTRESS · tag-2 · 42s · ack at tag
```

Background pulses between `rgb(160,28,28)` and `rgb(112,20,20)` over
1.2 s, synchronised with the marker pulse-ring beat (SVG `<animate>`
in both elements).

**Panel B — multi-SOS (tag-2 + tag-5; tag-5 most recent).** Two
markers carry SOS pulse rings on the three-layer basemap. Sidebar
sticky DISTRESS shows both. Strip shows **most-recent only**:
`DISTRESS · tag-5 · 8s · ack at tag`. No stacking, no
carousel.

## Three-layer basemap composition (both panels)

| Layer | Source | Visible in SVG as |
|---|---|---|
| **L1 · PMTiles basemap** (Protomaps dark) | `pmtiles_map.rs:61, 73, 106` | Dark slate + line-art (roads/water/landuse) |
| **L2 · LIDAR hillshade** (DHMV-II DSM) | `pmtiles_map.rs:25, 64-74, 107-113` | Translucent grayscale gradient + ridge lines |
| **L3 · OSM XML overlays** (Overpass + hand-drawn) | `pmtiles_map.rs:115-125`, schema `region.rs:25-71` | Building polygons + hand-drawn footpath |
| **Markers + SOS pulse ring** | `markers.rs:216-227` | Bright SOS marker glyphs on top |

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| Strip is the SOS surface; persistent bottom strip only | `KIOSK-006:29, 31-35` |
| Strip height unchanged (24 px); text size unchanged (13 pt) | `KIOSK-006:33, 80` |
| **Full-width** SOS state, no zone split, no coord-readout zone | `KIOSK-006:33, 56-64` |
| Strip text exact format `DISTRESS · {label} · {age} · ack at tag` | `KIOSK-006:42, 82` |
| `ack at tag` (no "the") | `KIOSK-006:42, 47, 114` |
| `flags.SOS=1` and `since {wall}` removed | `KIOSK-006:21, 45-46, 82` |
| Red fill `Color32::from_rgb(160, 28, 28)` | `tools/sarcom-kiosk-lab/src/app.rs:245` |
| Marker SOS pulse-ring | `tools/sarcom-kiosk-lab/src/map/markers.rs:216-227` |
| Strip pulse synchronised with marker pulse-ring | `KIOSK-006:34, 81, 84, 111` |
| Multi-SOS shows most-recent only | `KIOSK-006:51-54, 76, 86` |
| Multi-tag SOS is a v2 concern (duty-cycle cap) | `KIOSK-006:51-53` + `ARCHITECTURE.md` §13 lines 559-595 |
| Acknowledgement happens at the tag (ADR-010) — non-goal at gateway | `decisions/ADR-010-sos-encoding.md` + `KIOSK-006:16, 47` + `ADR-007:46` |
| `format_age_or_unavailable` honoured | `tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26` |
| Three-layer render stack | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Overlay schema | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Hillshade implementation context | `dev-log/2026-05-16-lidar-overlay-implementation.md` |
| Non-SOS strip retains existing `read-only · PMTiles · zoom N` | `tools/sarcom-kiosk-lab/src/app.rs:283-344` |
| No coord readout (KIOSK-002 deferred) | `tickets/README.md:30, 53, 71, 85` |
| 800×480 lab fixture (ADR-015-pending) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:8-19` |

## Aesthetic vs structural

The strip background pulse is **aesthetic on the existing strip** —
it does NOT add a new surface, does NOT change the strip's
structural layout, and does NOT change strip height or text size.
Per `KIOSK-006:35`: "The strip remains legible at every phase of the
pulse; the operator should not have to wait for a peak to read the
text."

## What is NOT rendered

- **NO banner.** `KIOSK-006:58, 63, 67, 84`.
- **NO acknowledge button** (non-goal) anywhere under any condition.
  `KIOSK-006:59, 67, 83` + `ADR-007:46`.
- **NO dismiss button.** `KIOSK-006:60, 67, 86`.
- **NO modal.** `KIOSK-006:61, 67, 75`.
- **NO popover.** `KIOSK-006:62, 67`.
- **NO tap targets on the strip.** `KIOSK-006:64`.
- **NO height growth.** `KIOSK-006:33, 80, 89`.
- **NO text-size growth.** `KIOSK-006:33, 80, 90`.
- **NO left/right zone split.** Full-width SOS state.
- **NO coord-readout zone alongside SOS.** KIOSK-002 deferred.
- **NO crosshair.** Same reason.
- **NO protocol downlink** (ADR-008). `KIOSK-006:68`.
- **NO DB write of acknowledgement state.** `KIOSK-006:69`.
- **NO SMS / email / external notification.** `KIOSK-006:71`.
- **NO audible alert.** Deferred to SPIKE-003. `KIOSK-006:72`.
- **NO multi-SOS UI** (stacked rows, carousel, queue). `KIOSK-006:76, 86`.
- **NO no-fix uncertainty disc.** SPIKE-002 closed reject. `KIOSK-006:73`.

## What a reviewer verifies

- **Strip height unchanged** (24 px) regardless of SOS state.
- **DISTRESS text size unchanged** (13 pt).
- **No banner, no acknowledge button** (non-goal), **no dismiss
  affordance** anywhere.
- **Multi-SOS strip shows most-recent only** (tag-5 in Panel B);
  both markers continue to pulse on the map.
- **SOS strip is full-width** (800 px) — no zone split, no coord
  readout (KIOSK-002 deferred).
- **Non-SOS strip state** retains existing
  `read-only · PMTiles · zoom N` rendering unchanged.
- **Exact strip text format**: `DISTRESS · {label} · {age} ·
  ack at tag` — `ack at tag` (no "the"). No `last frame` prefix
  on the age.
- **No crosshair** on the map.
- **Three-layer basemap** rendered in both panels with distinguishable
  L1/L2/L3 anchors.
- **Strip pulse** synchronised with marker pulse-ring (both at 1.2 s).

## Open questions for Pieter

1. **`read-only` token in the strip text.** Mockup drops it; ticket
   `KIOSK-006:49, 113` allows either choice.
2. **Strip pulse coordination strategy.** Shared time source vs
   independent `<animate>`; `KIOSK-006:111` allows either.
3. **Strip width on the actual existing code path.** Mockup renders
   800 px full-fixture-width; current `app.rs:230-282` may render
   only over the 480 px map area. Both consistent with `KIOSK-006:33`.
