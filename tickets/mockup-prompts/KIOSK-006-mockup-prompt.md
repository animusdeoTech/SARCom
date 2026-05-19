# Claude CLI prompt — KIOSK-006 mockup (SOS alerting: persistent strip + background pulse)

You are producing a single SVG mockup that illustrates the implementation
target for KIOSK-006. Output one SVG file at
`UX/mockups/KIOSK-006-mockup.svg` and one short markdown rationale at
`UX/mockups/KIOSK-006-mockup.md`. **No feature code changes.**

## Read first

- [`CLAUDE.md`](../../CLAUDE.md)
- [`decisions/ADR-007-touchscreen-primary-ui.md`](../../decisions/ADR-007-touchscreen-primary-ui.md) — especially line 46 (no alert acknowledgement flow)
- [`decisions/ADR-010-sos-encoding.md`](../../decisions/ADR-010-sos-encoding.md) — tag-side SOS button and buzzer
- [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../../tools/sarcom-kiosk-lab/src/app.rs) — current bottom-strip SOS render branch
- [`tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`](../../tools/sarcom-kiosk-lab/src/map/markers.rs) — SOS pulse ring on tag marker
- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`](../../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) — three-layer render stack (basemap + hillshade + OSM overlays, in z-order)
- [`tools/sarcom-kiosk-lab/src/map/region.rs:25-71`](../../tools/sarcom-kiosk-lab/src/map/region.rs) — Overlay enum (osm + hillshade variants)
- [`dev-log/2026-05-16-lidar-overlay-implementation.md`](../../dev-log/2026-05-16-lidar-overlay-implementation.md) — implementation context for the LIDAR hillshade
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs`](../../tools/sarcom-kiosk-lab/src/ui/palette.rs)
- [`tickets/KIOSK-006-sos-alerting.md`](../KIOSK-006-sos-alerting.md) — **the current ticket text is the spec — follow it.** Single design: persistent bottom strip only, no banner, no acknowledge, no dismiss, per the strict-ADR closure of SPIKE-001.
- KIOSK-002 is deferred from v1a per `tickets/README.md:53, 85` — **do not carry the crosshair-at-centre or strip coord-readout patterns from the original KIOSK-002 prompt into this mockup.** The strip is the SOS surface only.

## Hard constraints

1. **Single design.** No banner. No acknowledge button anywhere in the kiosk under any condition. No dismiss affordance. No new surface.
2. **NO layout change to the strip when SOS activates.** Strip height stays at current 24px regardless of state. `DISTRESS` text size stays at current 13pt — no growth. Only the strip's *fill colour* and *text content* change when SOS becomes active. The background pulse is a separate aesthetic enhancement.
3. **The strip is full-width SOS state when active.** No left/right zone split. No coord-readout zone alongside SOS. The strip is the SOS surface per `tickets/KIOSK-006-sos-alerting.md:33, 56-64` — coord readouts are out of scope (KIOSK-002 deferred per `tickets/README.md:53, 85`).
4. **Non-SOS strip state retains existing `read-only · PMTiles · zoom N` rendering** from `tools/sarcom-kiosk-lab/src/app.rs:283-344` unchanged. No coord readout anywhere in v1a. Do not invent a substitute non-SOS surface; the existing chrome is the existing chrome.
5. **`ack at tag` wording preserved** at the right end of the strip text — directs the operator that acknowledgement happens at the tag (ADR-010 tag-side SOS button), not at the gateway. Shortened from `ack at the tag` per `tickets/KIOSK-006-sos-alerting.md:42, 47`.
6. **Map chrome budget:** scale bar + compass rose only. No zoom +/−, no fit-all, no home, no clear-UI, no other floating buttons. **No crosshair** (KIOSK-002 deferred).
7. **Lab fixture size:** 800×480 per `tools/sarcom-kiosk-lab/README.md:53`. Annotate as ADR-015-pending.
8. **Use only palette constants from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.**

## The design to render

The SVG must contain **two stacked 800×480 panels** showing the strip in two SOS scenarios.

### Map area composition (applies to both panels)

**Map area renders three layers, in z-order:**

1. **PMTiles basemap** — Protomaps dark style, line-art on dark slate (roads, water, buildings, landuse, boundaries, places). Source: `pmtiles_map.rs:61, 73, 106`.
2. **LIDAR-derived hillshade raster** — DHMV-II DSM baked into a raster PMTiles archive, rendered at transparency 0.5 above the basemap so terrain relief is visible without washing out the line art. Source: `pmtiles_map.rs:25, 64-74, 107-113`.
3. **OSM XML overlays** — Overpass-fetched + hand-drawn overlays, painted in declaration order inside the `Map::show` closure. Source: `pmtiles_map.rs:115-125`, schema at `region.rs:25-71`.

Sim markers (relay orange cross, gateway green square outline, tag coloured circles per state, no-fix ghost dashed ring) paint on top of all three layers — and tag-2 carries the SOS pulse ring per `markers.rs:216-227` on top of its base dot.

**Map area SVG fidelity:**

The SVG is wireframe fidelity, not photorealism. The three-layer basemap is suggested as follows:

- **Layer 1 (basemap):** dark slate background (`MAP_BG` from `tools/sarcom-kiosk-lab/src/ui/palette.rs`), with light line-art suggesting roads + water + landuse (thin pale strokes, not full Protomaps fidelity).
- **Layer 2 (hillshade):** soft grayscale terrain shading patch covering most of the map area, suggesting elevation relief. Render as a translucent gradient or noise texture at ~30-50% opacity over the basemap. Annotate as `LIDAR hillshade · transparency 0.5 · per pmtiles_map.rs:107-113`.
- **Layer 3 (OSM overlays):** light additional line-work or area shapes painted over the hillshade — distinguishable from the basemap line-art. Annotate as `OSM XML overlays · Overpass + hand-drawn · per pmtiles_map.rs:115-125`.

Do not attempt pixel-fidelity reproduction of the actual tile rendering. The goal is "a reviewer looking at this knows it's basemap + hillshade + OSM overlays, not just basemap." If you have to choose between suggesting all three layers and clean readability, suggest all three even if rough — the absence of hillshade or OSM overlay is the failure mode this cleanup is fixing.

### Panel A — single SOS (tag-2 in distress)

- Map: three-layer render per the **Map area composition** subsection above; `MultiTag` scenario with tag-2 in SOS.
- Compass rose top-left, scale bar bottom-left (per KIOSK-001). No crosshair.
- tag-2 rendered with SOS pulse ring per `markers.rs:216-227`.
- Sidebar (rendered abstractly / dim) on the right.
- Bottom strip, 24px tall, **full-width** red fill `Color32::from_rgb(160, 28, 28)` per `app.rs:245`:
  - Strip text spans the full width: `DISTRESS · tag-2 · 42s · ack at tag` in 13pt bold white. Exact format per `tickets/KIOSK-006-sos-alerting.md:42, 82`. `flags.SOS=1`, `since {wall}`, and the `last frame` prefix on the age are all removed; `ack at tag` (no "the") is the closing token.
  - Strip background has a slow pulse/blink animation on the fill saturation. In the SVG, render as a static red strip with a margin annotation describing the animation, OR use SVG `<animate>` on the fill attribute for visible pulse (preferred).
  - **No left/right zone split. No coord-readout zone.** The strip is the SOS surface, full width.

### Panel B — multi-SOS scenario (hypothetical: tag-2 + tag-5 both SOS, tag-5 most recent)

- Same map and chrome as Panel A but with BOTH tag-2 AND tag-5 rendered with SOS pulse rings on the map (per `markers.rs:216-227`).
- Bottom strip identical structurally to Panel A (full-width red):
  - Strip text: `DISTRESS · tag-5 · 8s · ack at tag` (most-recent SOS only — tag-5, not stacked).
- Margin note in the SVG: `strip shows most-recent SOS only · both markers pulse on the map · multi-SOS budget is a v2 concern per ADR-014 / ARCHITECTURE.md §13 lines 559-595`.

## Pulse/blink spec (rationale must describe explicitly)

- Background fill pulses between full saturation (`rgb(160,28,28)`) and ~70% saturation (`rgb(112,20,20)`) over a ~1.2s period.
- Synchronised with the map marker's pulse-ring period at `markers.rs:216-227` for visual coherence.
- In SVG, an `<animate attributeName="fill" values="rgb(160,28,28);rgb(112,20,20);rgb(160,28,28)" dur="1.2s" repeatCount="indefinite" />` on the strip rectangle is acceptable.
- The pulse is **aesthetic on the existing strip** — it does NOT add a surface or change the strip's structural layout.

## Annotation requirements

- Annotate the strip text in both panels: `13pt bold white · current size, no growth per KIOSK-006:33, 80`
- Annotate that the strip is full-width SOS state: `no zone split · no coord readout in v1a · strip is the SOS surface · per KIOSK-006:33, 56-64`
- Annotate the strip background pulse: `slow pulse synchronised with map marker pulse-ring · 1.2s period · aesthetic only, no new surface`
- Annotate the `ack at tag` text: `direct to tag-side acknowledge per ADR-010 · NO kiosk-side button · ADR-007:46 holds`
- In Panel B, annotate: `multi-SOS · strip shows most-recent only · all SOS markers pulse on map · v2 concern per ADR-014`
- Margin note: `no banner · no acknowledge button · no dismiss affordance · single design`

## Rationale markdown content

- Scenario summary (the two panels).
- Per-element source-of-truth citations (`path:line`).
- An "Aesthetic vs structural" section explicitly stating that the pulse is aesthetic on the existing strip and does not constitute a new surface under ADR-007.
- A "What is NOT rendered" section listing: banner, acknowledge button, dismiss affordance, height growth, text-size growth — and citing ADR-007:46 + KIOSK-006 scope.
- A "what a reviewer verifies" closing section:
  - Strip height unchanged (24px) regardless of SOS state
  - DISTRESS text size unchanged (13pt)
  - No banner, no acknowledge button, no dismiss affordance anywhere
  - Multi-SOS strip shows most-recent only; all SOS markers pulse on the map
  - SOS strip is **full-width** — no zone split, no coord readout (KIOSK-002 deferred per `tickets/README.md:53, 85`)
  - Non-SOS strip state retains existing `read-only · PMTiles · zoom N` rendering from `app.rs:283-344` unchanged
  - Exact strip text format: `DISTRESS · {label} · {age} · ack at tag` (no "the" in "ack at tag"; per `KIOSK-006:42`; no `last frame` prefix on the age)
  - No crosshair on the map

## Non-goals

- No top-anchored banner.
- No acknowledge button.
- No dismiss affordance, no "are you sure" prompts.
- No protocol downlink.
- No DB write of dismissal state.
- No height growth of the strip during SOS.
- No text-size growth of `DISTRESS`.
- No SMS / email / external alerts.
- No floating buttons on the map.
- No changes to sidebar layout beyond what KIOSK-003 already specifies.
