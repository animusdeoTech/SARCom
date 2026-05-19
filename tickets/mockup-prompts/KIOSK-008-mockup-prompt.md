# Claude CLI prompt — KIOSK-008 mockup (map marker + track rendering: three-fix tail baseline + selection-driven full polyline)

You are producing a single SVG mockup that illustrates the
implementation target for KIOSK-008. Output one SVG file at
`UX/mockups/KIOSK-008-marker-track.svg` and one short markdown
rationale at `UX/mockups/KIOSK-008-marker-track.md`. **No feature code
changes.** No edits to any other doc.

## Read first

- [`CLAUDE.md`](../../CLAUDE.md)
- [`decisions/ADR-007-touchscreen-primary-ui.md`](../../decisions/ADR-007-touchscreen-primary-ui.md) — read-only UI invariant; "What the UI does not do" at lines 38-46
- [`tools/sarcom-kiosk-lab/src/data.rs:123-142`](../../tools/sarcom-kiosk-lab/src/data.rs) — `NodeData` including `track: Vec<[f32; 2]>` at line 134, `last_valid_fix_pos` at line 138, `last_valid_fix_age_secs` at line 141 (post-collapse per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`)
- [`tools/sarcom-kiosk-lab/src/map/markers.rs`](../../tools/sarcom-kiosk-lab/src/map/markers.rs) — current marker rendering (read in full); especially:
  - `tag_visible_pos` helper at lines 32-38
  - `tag_display_color` / `freshness_color` at lines 40-48
  - Legacy `draw_tracks` 1 px dashed-segment helper at lines 102-114 (different visual treatment from KIOSK-008's tail design; retained for legacy paths)
  - Tag-dot rendering + SOS pulse ring + selection outline at lines 194-262
  - No-fix ghost at lines 265-302
- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-180`](../../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) — three-layer render stack (basemap + LIDAR hillshade + OSM overlays, in z-order) and inline tag draw at lines 161-202 (does NOT currently consume `tag.track`)
- [`tools/sarcom-kiosk-lab/src/map/region.rs:25-71`](../../tools/sarcom-kiosk-lab/src/map/region.rs) — Overlay enum (osm + hillshade variants)
- [`dev-log/2026-05-16-lidar-overlay-implementation.md`](../../dev-log/2026-05-16-lidar-overlay-implementation.md) — implementation context for the LIDAR hillshade
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs`](../../tools/sarcom-kiosk-lab/src/ui/palette.rs)
- [`tickets/KIOSK-008-marker-track-rendering.md`](../KIOSK-008-marker-track-rendering.md) — the ticket this mockup illustrates
- [`tickets/KIOSK-001-map-scale-north.md`](../KIOSK-001-map-scale-north.md) — sibling: chrome ticket; KIOSK-008 splits marker / track work out
- [`tickets/KIOSK-003-sidebar-row-redesign.md`](../KIOSK-003-sidebar-row-redesign.md) — provides the `Selection` enum that selection-polyline consumes
- [`tickets/KIOSK-004-selection-detail-panel.md`](../KIOSK-004-selection-detail-panel.md) — sibling: KIOSK-004 owns the sidebar-replacement detail surface, KIOSK-008 owns the map-render side of the same selection

## Hard constraints

1. **Single design.** ADR-007 is the spec; no overlays, popovers, modals, banners, or acknowledgement flows. Per `tickets/README.md:14-33`.
2. **Map chrome budget for v1a is strict.** Scale bar + compass rose only. No zoom +/−, no fit-all, no home, no clear-UI, no other floating button. **No crosshair** (KIOSK-002 deferred per `tickets/README.md:53, 85`).
3. **Three-layer basemap composition** per `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` (see "Map area composition" below). Apply the standard composition block + SVG-fidelity guidance to all three panels.
4. **Lab fixture size.** Render at 800×480 per `tools/sarcom-kiosk-lab/README.md:53`. Annotate as ADR-015-pending; substrate is open per `README.md:36`.
5. **Cite `path:line` for every concrete claim** in the rationale markdown.
6. **Use only palette constants from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.** Tag-tail dots and selection-polyline lines use tag colour with multiplied alpha; no new palette entries.
7. **Per-state colour consistency (Decisions pinned #8 in `tickets/KIOSK-008-marker-track-rendering.md`).** Tails and current dots both use the per-state colour from `tag_display_color` / `freshness_color` per `markers.rs:40-48`. The mockup MUST show the current dot and tail rendering in the same colour for each tag (BLUE for normal/fresh, RED for SOS, GREY for stale, ORANGE for aging if exercised). KIOSK-008's implementation bundles a fix to the operator-facing PMTiles inline draw at `pmtiles_map.rs:173-179` (previously BLUE-only) — the mockup reflects this fix.
8. **SOS tail = NO (Decisions pinned #4).** The SOS tag renders the current dot + pulse ring only. No three-fix tail. The pulse ring is the SOS surface; a red trail would compete with it.
9. **Selection polyline colour = tag colour at opacity 0.5 (Decisions pinned #7).** Identity wins over neutral colour; this is no longer a mockup-render-only choice.
10. **Do not invent design values Pieter has not decided.** Five open questions remain (per the ticket's Risks section): fade-opacity triple, tail-dot size, connector-line presence, no-fix selection behaviour, polyline thickness. Pick a concrete value FOR THE MOCKUP RENDER ONLY, surface every choice in the rationale's "Open questions for Pieter" section, and flag it as mockup-render-only rather than implementer-locked.

## The design to render

The SVG must contain **two stacked 800×480 panels** so the
baseline-vs-selection contrast is visible.

### Map area composition (applies to both panels)

**Map area renders three layers, in z-order** (per
`pmtiles_map.rs:82-125`):

1. **PMTiles basemap** — Protomaps dark style, line-art on dark slate (roads, water, buildings, landuse, boundaries, places). Source: `pmtiles_map.rs:61, 73, 106`.
2. **LIDAR-derived hillshade raster** — DHMV-II DSM baked into a raster PMTiles archive, rendered at transparency 0.5 above the basemap so terrain relief is visible without washing out the line art. Source: `pmtiles_map.rs:25, 64-74, 107-113`.
3. **OSM XML overlays** — Overpass-fetched + hand-drawn overlays, painted in declaration order inside the `Map::show` closure. Source: `pmtiles_map.rs:115-125`, schema at `region.rs:25-71`.

KIOSK-008 content (track tails and selection polylines) paints
**inside the same `Map::show` closure** after the three basemap
layers, before the current-position dots and the SOS pulse rings. Sim
markers (current-position dots, no-fix ghost, relay orange cross,
gateway green square outline) paint on top of all four.

**Map area SVG fidelity:**

The SVG is wireframe fidelity, not photorealism. The three-layer
basemap is suggested as follows:

- **Layer 1 (basemap):** dark slate background (`MAP_BG` from `tools/sarcom-kiosk-lab/src/ui/palette.rs`), with light line-art suggesting roads + water + landuse (thin pale strokes).
- **Layer 2 (hillshade):** soft grayscale terrain shading patch at ~30-50% opacity over the basemap. Annotate as `LIDAR hillshade · transparency 0.5 · per pmtiles_map.rs:107-113`.
- **Layer 3 (OSM overlays):** light additional line-work or area shapes painted over the hillshade — distinguishable from the basemap line-art. Annotate as `OSM XML overlays · Overpass + hand-drawn · per pmtiles_map.rs:115-125`.

Do not attempt pixel-fidelity reproduction of the actual tile
rendering. The goal is "a reviewer looking at this knows it's basemap
+ hillshade + OSM overlays plus tag tails + selection polyline, not
just basemap and dots."

### Panel A — `MultiTag` scenario, no selection (baseline)

- Three-layer basemap composition rendered per the standard above.
- **tag-1 (normal, `gps_valid=true`):** current dot in BLUE at 8 px,
  plus three smaller fix-dots trailing in its movement direction.
  Newest tail-dot at full BLUE saturation, older tail-dots dimmer
  (fade triple is mockup-choice; surface in open questions). Optional
  faint connector line between the three tail-dots (also
  mockup-choice; surface).
- **tag-2 (SOS, `gps_valid=true`):** current dot in RED + SOS pulse
  ring per `markers.rs:216-227`. **NO three-fix tail** (Decisions
  pinned #4 — pulse ring is the SOS surface; a red trail would
  compete with it).
- **tag-3 (no-fix, `gps_valid=false`):** ghost marker at
  `last_valid_fix_pos` per `markers.rs:265-302` (faded fill + dashed
  outer ring + `NO FIX · {age}` label). **No tail.** The ghost is the
  entire indicator.
- **tag-4 (stale, `gps_valid=true`, last_seen past stale threshold):**
  current dot in GREY / dim, plus three GREY-dim tail-dots fading.
  Tail colour matches current-dot colour via `freshness_color` per
  Decisions pinned #8.
- **relay-1:** orange cross marker, unchanged from current rendering.
- **gw-0:** green square outline, unchanged.
- Compass rose top-left, scale bar bottom-left (per KIOSK-001).
- Bottom strip: existing `read-only · PMTiles · zoom 15` per
  `app.rs:283-344`.

### Panel B — same `MultiTag` scenario, tag-1 selected

Same fixture and basemap composition as Panel A. The single visible
diff is the track-rendering for tag-1:

- **tag-1 baseline three-fix tail is REPLACED by a full polyline**
  through every fix in `NodeData.track` (line 134). Style: **tag colour
  (BLUE) at opacity 0.5** (Decisions pinned #7 — pinned, no longer
  mockup-render-only), slightly thicker stroke than the baseline tail.
  Concrete stroke-width is mockup-choice; surface.
- tag-1 still renders its current-position dot on top of the polyline
  (selection outline per `markers.rs:231-237` adds a thin white halo
  ring around the dot at +2 px radius).
- **tag-2, tag-3, tag-4 retain their baseline three-fix tail behaviour
  from Panel A unchanged.** Other tags do not re-render polylines when
  one tag is selected.
- Map recenter: the mockup may render an arrow or `Δ` showing the
  150 ms eased pan onto `tag_visible_pos` per `markers.rs:32-38` and
  `KIOSK-004:36, 87` — same convention as the KIOSK-004 mockup. The
  sidebar in Panel B may be abstracted (KIOSK-004 owns the
  sidebar-replacement detail surface; this mockup is about the
  map-render side).

## Annotation requirements

In the SVG, add small marginal callouts for:

- The three-fix tail on tag-1 in Panel A: `three-fix tail · newest →
  oldest fade · per KIOSK-008 scope · concrete fade values =
  mockup-render-only (see open questions)`.
- The SOS tag-2 in Panel A: `SOS · current dot + pulse ring only · NO
  tail · per KIOSK-008 Decisions pinned #4 · markers.rs:216-227`.
- The no-fix tag-3 in Panel A: `no-fix ghost only · NO tail · per
  KIOSK-008 Per-state behaviour table · markers.rs:265-302`.
- The stale tag-4 in Panel A: `stale · GREY tail + GREY current dot ·
  per-state colour from freshness_color · Decisions pinned #8`.
- The full polyline on tag-1 in Panel B: `full track polyline ·
  NodeData.track · tag colour (BLUE) at opacity 0.5 · stroke-width =
  mockup-render-only · replaces three-fix tail during selection ·
  Decisions pinned #7`.
- The other tags in Panel B: `other tags retain three-fix tail when
  one tag is selected · per KIOSK-008 AC#5`.
- The three basemap layers in each panel (L1 / L2 / L3 anchors).

## Rationale markdown content

The `.md` file must include:

- Scenario summary (the two panels).
- Per-element source-of-truth citations (`path:line`) for each
  rendered element (basemap layers, tag dots, tail dots, polyline,
  ghost marker, SOS pulse ring, selection outline, recenter convention).
- A "Per-state behaviour" table mirroring the one in
  `tickets/KIOSK-008-marker-track-rendering.md` so the reviewer can
  cross-reference (with SOS row = "NO tail" pinned, no clock-invalid
  row).
- A "Decisions pinned" subsection naming the three pinned decisions
  (#4 SOS tail = NO; #7 polyline colour = tag colour @ 0.5; #8
  freshness_color consistency) and citing the KIOSK-008 ticket's
  matching subsection.
- A "what a reviewer verifies" closing section:
  - tag-1 baseline three-fix tail visible in Panel A.
  - tag-2 (SOS) has NO tail, only current dot + pulse ring.
  - tag-3 (no-fix) ghost has NO tail.
  - tag-4 (stale) tail + current dot both render in GREY (per-state
    colour consistency).
  - tag-1 in Panel B shows a full polyline replacing its three-fix
    tail; tag-3 / tag-4 still show their baseline rendering; tag-2
    still shows only pulse-ring (no tail).
  - Map chrome = compass rose + scale bar only (no floating buttons,
    no crosshair).
  - Three-layer basemap rendered in both panels.
- A "Mockup-render-only choices" section explicitly listing the
  five remaining open-question design values the mockup picked one
  of, with a note that these are NOT implementer locks.
- An "Open questions for Pieter" section reproducing the ticket's
  surviving Risks list verbatim:
  - Fade opacity values for the three-fix tail.
  - Tail-dot size.
  - Connector line between tail dots.
  - No-fix tag selection behaviour.
  - Selection polyline thickness.

## Non-goals

- No track for relay / gateway. Tags only.
- No track-history pruning, retention, animation, follow-mode,
  speed-vector labels, time-coloured tracks.
- No protocol or data-model changes.
- No uncertainty disc on no-fix tags (SPIKE-002 closed reject).
- No floating map buttons (zoom, fit, home, clear-UI).
- No crosshair (KIOSK-002 deferred from v1a per
  `tickets/README.md:53, 85`).
- No coord readout anywhere.
- No banner, no popover, no modal, no acknowledgement flow.
- No marketing polish; wireframe fidelity sufficient.
