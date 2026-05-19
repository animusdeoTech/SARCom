# KIOSK-008 — Map marker + track rendering (rationale)

Mockup at `UX/mockups/KIOSK-008-marker-track.svg`. Single-design v1a;
two stacked 800×480 lab fixtures rendering the baseline-vs-selection
contrast.

## Scenario summary

Both panels render the `MultiTag` data fixture analogue (tag-1 fresh,
tag-2 SOS, tag-3 no-fix, tag-4 stale, relay-1, gw-0). The single
diff between panels is selection state:

- **Panel A — `Selection::None`.** Baseline three-fix tail for every
  eligible tag (`gps_valid=true` and not in SOS state). SOS tag-2
  renders current dot + pulse ring only. No-fix tag-3 renders the
  ghost only. Stale tag-4 renders tail + current dot in
  `freshness_color(Stale)` = rgb(180, 83, 9).
- **Panel B — `Selection::Node(_)`.** tag-1's three-fix tail is
  replaced by the full polyline through every fix in `NodeData.track`
  (`tools/sarcom-kiosk-lab/src/data.rs:134`), rendered in tag colour
  BLUE at opacity 0.5. tag-2, tag-3, tag-4 are unchanged — only the
  selected tag's track-render swaps.

## Per-element source-of-truth citations

| Element | Source-of-truth |
|---|---|
| Three-layer map composition (basemap / hillshade / OSM) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| L1 PMTiles basemap (Protomaps dark) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:61, 73, 106` |
| L2 LIDAR hillshade @ ~0.5 transparency | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:25, 64-74, 107-113` |
| L3 OSM XML overlays | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:115-125`, `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Tag-tail dots paint inside same `Map::show` closure, after basemap layers, before current dots | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:127+, 161-202`; `tickets/KIOSK-008-marker-track-rendering.md:63-68` |
| Current-position dot (8 px filled circle) | `tools/sarcom-kiosk-lab/src/map/markers.rs:194-262`, specifically the `circle_filled` at `markers.rs:229` and the PMTiles inline `pmtiles_map.rs:194` |
| Per-state colour for tail + current dot | `tools/sarcom-kiosk-lab/src/ui/palette.rs:4-30`; `tag_display_color` / `freshness_color` at `tools/sarcom-kiosk-lab/src/map/markers.rs:40-48` |
| `NodeData.track` polyline data | `tools/sarcom-kiosk-lab/src/data.rs:134` |
| Visible-position helper (selects sentinel vs last-valid for ghost) | `tools/sarcom-kiosk-lab/src/map/markers.rs:32-38` |
| Selection halo around selected current dot | `tools/sarcom-kiosk-lab/src/map/markers.rs:231-237` |
| SOS pulse ring | `tools/sarcom-kiosk-lab/src/map/markers.rs:216-227` |
| No-fix ghost (faded fill + dashed outer ring + label) | `tools/sarcom-kiosk-lab/src/map/markers.rs:265-302` |
| Selection enum (`Selection::None` / `Selection::Node(usize)`) | `tickets/KIOSK-003-sidebar-row-redesign.md` post-collapse three-question check section |
| Recenter on selection (150 ms eased pan to `tag_visible_pos`) | `tickets/KIOSK-004-selection-detail-panel.md:36, 87` |
| Legacy `draw_tracks` 1 px dashed-segment helper (unchanged) | `tools/sarcom-kiosk-lab/src/map/markers.rs:102-114` |
| Strict ADR-007 chrome budget (compass rose + scale bar only) | `tickets/README.md:14-33, 26`; `tickets/KIOSK-001-map-scale-north.md:16, 31` |
| No crosshair / no coord readout (KIOSK-002 deferred) | `tickets/README.md:53, 85` |
| No uncertainty disc (SPIKE-002 closed reject) | `tickets/README.md:46`; `tickets/SPIKE-002-nofix-uncertainty-disc-semantics.md` |
| ADR-007 read-only UI invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |

## Per-state behaviour

Mirrors `tickets/KIOSK-008-marker-track-rendering.md:86-91`.

| Tag state | Current dot | Three-fix tail | Full track on selection |
|---|---|---|---|
| Normal (`gps_valid=true`, fresh) | yes, per-state colour from `freshness_color` (`markers.rs:40-48`) | yes, per-state colour with positional fade | yes |
| SOS (`sos=true`, `gps_valid=true`) | yes, RED + SOS pulse ring (`markers.rs:216-227`) | **NO tail** (Decisions pinned #4) | yes |
| Stale (`gps_valid=true`, past stale threshold) | yes, `freshness_color(Stale)` = rgb(180, 83, 9) per `palette.rs:23-30` | yes, same per-state colour with positional fade | yes (the existing track is what it is) |
| No-fix (`gps_valid=false`) | ghost at `last_valid_fix_pos` (`markers.rs:265-302`) | **no tail** — ghost is the entire indicator | **open question** (see Open questions for Pieter) |

## Decisions pinned

Cross-references the matching section in
`tickets/KIOSK-008-marker-track-rendering.md:198-222`.

- **#4 SOS-state tail = NO.** SOS tag renders current dot + SOS pulse
  ring only. No three-fix tail. The pulse ring at
  `tools/sarcom-kiosk-lab/src/map/markers.rs:216-227` is the
  attention-grabbing surface; an extra red trail would compete with
  it. Reflected in Panel A and Panel B tag-2: no tail in either
  panel.
- **#7 Selection polyline colour = tag colour at opacity 0.5.**
  Identity is preserved across multiple selections; a neutral colour
  would confuse "whose path is this" when multiple tags are on the
  map. Reflected in Panel B tag-1 polyline rendered as BLUE at
  opacity 0.5.
- **#8 Tail vs current-dot colour parity from `freshness_color`.**
  Both the tail and the current dot for a given tag use the
  per-state colour returned by `freshness_color` at
  `tools/sarcom-kiosk-lab/src/map/markers.rs:40-48`. KIOSK-008 also
  bundles a fix to the operator-facing PMTiles inline draw at
  `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:173-179` so the
  current dot's colour matches the tail's colour per state (the
  inline draw is currently BLUE-only — an honesty-discipline bug
  fixed in passing per acceptance criterion #11). Reflected in
  tag-4: tail dots and current dot both render in rgb(180, 83, 9).

## Mockup-render-only choices

Five concrete design values are surfaced by the ticket as open
questions (`tickets/KIOSK-008-marker-track-rendering.md:224-244`).
The mockup picks one value per question **for the render only**.
These are **NOT implementer locks** — Pieter pins them at review.

- **Tail fade triple:** newest 1.0, middle 0.6, oldest 0.3
  (linear-ish; symmetric reading at 2 m glance).
- **Tail-dot size:** 4 px (current-position dot is 8 px at
  `markers.rs:229` / `pmtiles_map.rs:194`).
- **Connector line between tail dots:** included as a faint
  per-state-colour line at opacity 0.3. Communicates direction;
  fades behind dot density at high tag count.
- **No-fix tag selection:** ghost only — no polyline rendered for
  historical fixes prior to `last_valid_fix_pos`. The ghost is the
  entire indicator (per the prompt). Alternative is to render the
  polyline up to the last valid fix.
- **Selection polyline stroke-width:** 2 px (vs tail-dot 4 px
  diameter, current-dot 16 px diameter). Reads as "slightly thicker
  than baseline tail."

## Open questions for Pieter

Reproduced verbatim from
`tickets/KIOSK-008-marker-track-rendering.md:228-244` (the surviving
Risks list, after Decisions pinned #4 / #7 / #8 closed three of the
original eight). Pieter has not decided these.

- **Fade opacity values for the three-fix tail.** Concrete numbers —
  e.g. newest 1.0, middle 0.6, oldest 0.3, or some other curve.
  Implementer-choice within Pieter's intent? Or pin a specific
  triple now?
- **Tail-dot size.** Current dot is 8 px (`circle_filled` at
  `markers.rs:229` and `pmtiles_map.rs:194`). Tail dots should be
  smaller — 3 px? 4 px? Visually distinct from current dot at 2 m
  glance.
- **Connector line between tail dots.** Faint line connecting the
  three-fix tail dots, or pure dots only? Trade-off: line makes
  direction more obvious; dots-only is cleaner at high tag density.
- **No-fix tag selection.** When a no-fix tag is selected, does the
  full polyline render for the historical fixes that DO exist
  (everything up to `last_valid_fix_pos`)? Or stays ghost-only?
- **Selection polyline thickness.** "Slightly thicker than baseline
  tail" — concrete `stroke-width` value? Currently undefined.

## What a reviewer verifies

- tag-1 baseline three-fix tail visible in Panel A (BLUE per-state
  colour, dots fading newest → oldest, faint connector line).
- tag-2 (SOS) has NO tail in either panel — only current dot +
  pulse ring (Decisions pinned #4).
- tag-3 (no-fix) ghost has NO tail in either panel — faded ghost at
  `last_valid_fix_pos` with dashed outer ring is the entire
  indicator.
- tag-4 (stale) tail dots and current dot both render in
  `freshness_color(Stale)` = rgb(180, 83, 9) — per-state colour
  consistency (Decisions pinned #8).
- tag-1 in Panel B shows a full polyline through `NodeData.track`
  replacing its three-fix tail; rendered in BLUE at opacity 0.5
  (Decisions pinned #7); selection halo (+2 px white ring) on the
  current dot per `markers.rs:231-237`.
- tag-3 / tag-4 in Panel B still show their baseline rendering;
  tag-2 still shows only the pulse-ring (no tail).
- Map chrome in both panels = compass rose + scale bar only. No
  zoom +/−, no fit-all, no home, no clear-UI, no other floating
  buttons. No crosshair.
- Three-layer basemap (PMTiles basemap + LIDAR hillshade + OSM
  overlays) rendered in both panels with L1/L2/L3 anchor callouts.
- A 150 ms eased recenter arrow on Panel B shows the pan onto
  `tag_visible_pos` per `markers.rs:32-38` and
  `tickets/KIOSK-004-selection-detail-panel.md:36, 87`.
- Five "Mockup-render-only" values are annotated as such — none of
  them are implementer-locked.
