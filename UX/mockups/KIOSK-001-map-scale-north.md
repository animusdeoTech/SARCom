# KIOSK-001 — Map spatial aids (scale bar + compass rose) — v1a mockup rationale

Single-design strict-ADR mockup. Renders the map chrome budget for v1a
per `tickets/README.md:26` and `tickets/KIOSK-001-map-scale-north.md:16,
31`, **plus** the three-layer basemap composition per
`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`. SVG at
`UX/mockups/KIOSK-001-map-scale-north.svg`.

## Scenario summary

The 800×480 lab fixture shows the kiosk in its default PMTiles render
path. The map area surfaces all three layers from the actual render
stack, in z-order: Protomaps-dark basemap line-art (Layer 1), LIDAR
DHMV-II hillshade at transparency 0.5 (Layer 2), and OSM XML overlays
in declaration order (Layer 3). Sim markers (tag-1 normal, relay-1
healthy, gw-0 healthy) paint on top, suggesting a minimal
`MultiTag`-style scene — markers are deliberately schematic since this
mockup is about chrome + basemap, not marker fidelity. The sidebar is
abstracted; sidebar detail is owned by
`tickets/KIOSK-003-sidebar-row-redesign.md`.

Two chrome elements are surfaced as load-bearing:

1. **Compass rose** in the top-left of the map area, ~80×80 region,
   carrying N/E/S/W cardinals + 30° tick graduations + a north-up
   pointer + an explicit `N🔒` lock badge.
2. **Scale bar** in the bottom-left of the map area, fixed 80 px wide,
   dual-tone alternating, labelled at 250 m for the garden-scale lab
   fixture.

The existing bottom strip is rendered in its non-SOS state per
`tools/sarcom-kiosk-lab/src/app.rs:283-344` (`read-only · PMTiles ·
zoom 15`), extended with the dim attribution `© OpenStreetMap ·
DHMV-II LIDAR · PMTiles · zoom 15` to acknowledge both source tilesets.
**No coordinate readout** anywhere; KIOSK-002 is deferred from v1a per
`tickets/README.md:53, 85`.

## Three-layer basemap composition

Per `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` the kiosk's
PMTiles render path stacks three layers; the SVG surfaces all three
distinguishably at wireframe fidelity (not photorealism).

| Layer | Source | Render | Visible in SVG as |
|---|---|---|---|
| **L1 · PMTiles basemap** (Protomaps dark) | `pmtiles_map.rs:61, 73, 106` | `Map::with_layer(..., 1.0)` | Dark slate `MAP_BG` + thin pale line-art (roads/water/landuse) + grid fine pattern |
| **L2 · LIDAR hillshade** (DHMV-II DSM, raster PMTiles) | `pmtiles_map.rs:25, 64-74, 107-113` | `Map::with_layer(hs, 0.5)` | Translucent grayscale gradient + soft "contour" ridge lines covering most of the map area |
| **L3 · OSM XML overlays** (Overpass + hand-drawn) | `pmtiles_map.rs:115-125`, schema `region.rs:25-71` | `OsmMap::draw_with_projector` inside `Map::show` closure | Building footprint polygons (Overpass-style), hand-drawn footpath (later in declaration order paints on top), forest landuse polygon — all distinguishable from L1 line-art |
| **Sim markers** | `markers.rs` (relay cross, gateway square, tag circle, ghost ring) | painted last inside the same closure | Bright marker glyphs on top of everything |

The hillshade transparency of 0.5 is the current default at
`pmtiles_map.rs:108-112`; the comment there notes tunable range 0.35–0.7.

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| Compass rose top-left, 30° ticks, N-up locked, subtle HUD | `tickets/KIOSK-001-map-scale-north.md:25, 27, 31` |
| Compass rose elevates the legacy `"N"` text label | `tools/sarcom-kiosk-lab/src/map/mod.rs:261-268` |
| Scale bar bottom-left, fixed 80 px, dual-tone, snap step set | `tickets/KIOSK-001-map-scale-north.md:24, 42` |
| Scale-bar distance derivation | `tickets/KIOSK-001-map-scale-north.md:68` + `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:42-44` |
| Three-layer render stack (z-order) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Basemap layer init (Protomaps style + tile source) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:61, 73, 106` |
| Hillshade layer (DHMV-II DSM raster) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:25, 64-74, 107-113` |
| OSM overlays render inside `Map::show` closure | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:115-125` |
| Overlay schema (osm + hillshade tagged enum) | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Hillshade implementation context | `dev-log/2026-05-16-lidar-overlay-implementation.md` |
| Bottom strip `read-only · PMTiles · zoom N` render path | `tools/sarcom-kiosk-lab/src/app.rs:283-344` |
| ADR-007 read-only invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |
| 800×480 lab fixture (ADR-015-pending substrate) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:4-19` |
| Strict-ADR map chrome budget = scale bar + compass rose only | `tickets/README.md:26` + `KIOSK-001:16, 31` |
| KIOSK-002 deferred (no coord readout in v1a) | `tickets/README.md:30, 53, 71, 85` |

## Map chrome budget

**Rendered:**

- Compass rose with cardinal labels + 30° tick graduations + N-lock
  badge — top-left of map area.
- Scale bar, fixed 80 px wide, dual-tone — bottom-left of map area.
- Tile attribution (`© OpenStreetMap · DHMV-II LIDAR · PMTiles ·
  zoom 15`) and existing `read-only · PMTiles · zoom 15` strip
  text — bottom edge.

**Explicitly NOT rendered (per `tickets/README.md:26`,
`KIOSK-001:16, 31, 79`):**

- Zoom `+` button.
- Zoom `−` button.
- Fit-all button.
- Home button.
- Clear-UI / sidebar-toggle button.
- Any other floating button or tap target on the map.
- Crosshair at map centre (KIOSK-002 coord-readout pattern, deferred).
- Coordinate readout anywhere on the surface.
- No-fix uncertainty disc (SPIKE-002 closed reject per
  `tickets/README.md:46`).

The interaction model is walkers' native pan + pinch-to-zoom.

## Implementer-choice notes (within the ticket's intent)

Three details inside the ticket's "implementer chooses" envelope are
shown at one concrete value in the mockup:

- **Compass rose tick density: 30°.** `KIOSK-001:25` allows 30° **or**
  45°. The mockup renders 30°.
- **Scale-bar snap step: 250 m.** `KIOSK-001:24` lists the superset
  `{50m, 100m, 250m, 500m, 1km, 2km}`.
- **N-lock glyph `N🔒`.** `KIOSK-001:25` says north-up is fixed; the
  glyph is the mockup's explicit affordance.

All three surfaced as open questions for Pieter.

## What a reviewer verifies

- Scale bar is **80 px wide**, dual-tone, labelled with a value drawn
  from `{50m, 100m, 250m, 500m, 1km, 2km}` per `KIOSK-001:24`.
- Compass rose is **top-left** of the map area with **30° tick
  graduations** and an explicit N-lock indicator.
- **No floating buttons anywhere on the map.**
- Tile attribution is **present and dim** along the bottom edge,
  acknowledging both OSM and DHMV-II LIDAR sources.
- Map area shows **three distinguishable layers**: basemap line-art
  (L1), hillshade gradient (L2), OSM XML overlays (L3). Annotations
  L1/L2/L3 in the panel correspond to right-column citations.
- Lab fixture is **800×480** annotated as **ADR-015-pending** per
  `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36`.
- Visual weight of chrome surfaces is **subtle HUD** (TEXT_DIM for
  lines, TEXT_BRIGHT only on the N cardinal + pointer) — does not
  compete with markers.
- Every annotation in the right column corresponds to a numbered
  callout (1–4) or layer anchor (L1/L2/L3) inside the panel.

## Open questions for Pieter

1. **Compass rose tick density** — 30° (mockup choice) or 45°?
   `KIOSK-001:25` allows either.
2. **Scale-bar snap step set** — keep the superset
   `{50m, 100m, 250m, 500m, 1km, 2km}` from `KIOSK-001:24` for v1a,
   or narrow to a smaller set for the garden-scale fixture?
3. **`N🔒` lock glyph** — add the explicit glyph requirement to
   `KIOSK-001` (the mockup shows it), or drop the glyph and let the
   fixed rose imply the lock?
4. **Hillshade transparency** — the mockup renders ~0.55 effective
   opacity to suggest the layer visibly at wireframe scale. The
   actual render uses 0.5 with a tunable range 0.35–0.7 per
   `pmtiles_map.rs:108-112`. No action needed unless the on-device
   visual review wants the wireframe-scale opacity tuned for review
   parity.
