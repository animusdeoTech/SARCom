---
id: KIOSK-008
title: "Map marker + track rendering"
status: ready-for-review
type: implementation-ticket
opened: 2026-05-19
adr007-variant-dependency: none
---

# KIOSK-008 — Map marker + track rendering

## Problem statement

The kiosk's PMTiles render path currently draws tag dots (filled
circles per state colour) and the no-fix ghost marker, but **does not
render any track history** beyond what the legacy `MapMode::FakeGrid`
may have shown. Tag data carries `track: Vec<[f32; 2]>` per
[`tools/sarcom-kiosk-lab/src/data.rs:134`](../tools/sarcom-kiosk-lab/src/data.rs),
but nothing in [`pmtiles_map.rs`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)
consumes it. The inline tag draw at
[`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:161-202`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)
renders the current-position dot only and does not call the existing
legacy helper at [`tools/sarcom-kiosk-lab/src/map/markers.rs:102-114`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
(`draw_tracks` — a 1 px dashed-segment renderer used by the
non-default `MapMode::FakeGrid` / `MapMode::OsmVector` paths via
[`tools/sarcom-kiosk-lab/src/map/markers.rs:186-203`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
`draw_tags`).

For SAR operator-need ("hoe gaat het met de mensen waar ik
verantwoordelijk voor ben? wie is er allemaal en waar zijn ze?" per
[`tickets/README.md`](README.md) v1a posture, lines 14-33), a single
dot per tag answers "where are they right now" but not "in welke
richting bewegen ze en hoe snel." A small fix-tail per tag answers
both at glance. A full track on demand (via selection) lets the
operator confirm route history when investigating a specific person.

This ticket is **ADR-007-independent**. Track rendering is on-map
content, not an overlay surface; no modal / popover / banner tension
per [`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../decisions/ADR-007-touchscreen-primary-ui.md).

## User story

*As a SAR operator at the gateway, I want to see at glance where each
tag is, in which direction it's moving, and roughly how fast — and I
want the option to see the full route history for a specific person
without leaving the map.*

## Scope

### Baseline track rendering (always on, every tag with `gps_valid=true`)

- Each tag renders its current dot **plus its three most recent fixes
  as small dots with fade-out**.
- Fade-out: newest fix at full tag colour, older fixes dimmer
  (concrete opacity values are an open question — see Risks).
- Three-fix tail size is smaller than the current-position dot
  (current dot is 8 px filled per [`markers.rs:229`](../tools/sarcom-kiosk-lab/src/map/markers.rs)).
- A thin connector line between the fixes is acceptable but optional
  (implementer chooses based on at-2 m-glance readability — see open
  question).
- When the tag has fewer than three fixes in `track`, render whatever
  exists; no padding, no placeholder.
- Tails are rendered **inside the same `Map::show` closure** as the
  current marker draw at
  [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:127+`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs),
  after the three basemap layers (per `pmtiles_map.rs:82-125` z-order)
  and before the current-position dots, so the current dot always
  paints on top of its own tail.

### Selection-driven full track (consumes `Selection::Node(usize)` from KIOSK-003)

- When a tag is selected, render the **full track polyline** for that
  tag connecting every fix in `NodeData.track` (line 134).
- Style: **tag colour at opacity 0.5** (Decisions pinned #7 —
  identity preserved across multiple selections), slightly thicker
  stroke than the baseline tail (concrete stroke-width is an open
  question — see Risks).
- The selected tag's baseline three-fix tail is **replaced** by the
  full polyline for the duration of the selection (no double-render).
- Other tags continue to render their three-fix tail unchanged.
- On selection clear (`Selection::None`), the full polyline disappears;
  the three-fix tail returns.

### Per-state behaviour

| Tag state | Current dot | Three-fix tail | Full track on selection |
|---|---|---|---|
| Normal (`gps_valid=true`, fresh) | yes, per-state colour from `freshness_color` per [`markers.rs:40-48`](../tools/sarcom-kiosk-lab/src/map/markers.rs) (see Decisions pinned: KIOSK-008 also fixes the PMTiles inline draw at `pmtiles_map.rs:173-179` to use `freshness_color`) | yes, current-colour fade | yes |
| SOS (`sos=true`, `gps_valid=true`) | yes, RED + SOS pulse ring per [`markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs) | **NO tail** (Decisions pinned #4 — pulse ring is the SOS surface; an extra red trail would compete with it) | yes |
| Stale (`gps_valid=true`, last_seen past stale threshold per `freshness_for_tag`) | yes, GREY / dim per `freshness_color` | yes, GREY fade | yes (the existing track is what it is) |
| No-fix (`gps_valid=false`) | ghost at `last_valid_fix_pos` per [`markers.rs:265-302`](../tools/sarcom-kiosk-lab/src/map/markers.rs) | **no tail** — ghost is the entire indicator; track to before last-valid-fix would mislead | **OPEN QUESTION** (see Risks): does selection show the polyline up to the last valid fix? |

### Markers (unchanged)

The current marker rendering (relay orange cross, gateway green square
outline, tag colour-coded circles per state, no-fix ghost dashed ring)
is **preserved as-is**. This ticket only adds track-tail +
selection-polyline; it does not redesign existing markers.

## Non-goals

- No track for relay / gateway. Only tags.
- No track-history pruning logic in this ticket. Whatever ends up in
  `NodeData.track` is what gets rendered. (Pruning, retention policy,
  etc. are out of scope.)
- No animation. Static render. The newest-to-oldest fade is positional,
  not temporal.
- No "follow the latest track-point" camera mode. Selection still does
  the one-shot pan per
  [`tickets/KIOSK-004-selection-detail-panel.md:36`](KIOSK-004-selection-detail-panel.md).
- No track-aware annotations on the map (no labels like "moving south
  at 1.2 m/s"). Direction + speed are visual-inference only.
- No protocol or data-model changes. Uses existing `NodeData.track`
  field at `data.rs:134`.
- No uncertainty disc on no-fix tags.
  [`tickets/SPIKE-002-nofix-uncertainty-disc-semantics.md`](SPIKE-002-nofix-uncertainty-disc-semantics.md)
  closed reject per [`tickets/README.md:46`](README.md).
- No tail or polyline coloured by age relative to the wall clock — the
  fade is positional ordering only.

## Acceptance criteria

1. PMTiles render path draws three-fix baseline tail for every tag
   with `gps_valid=true` and ≥1 fix in `track`.
2. Tail fades from current-fix colour (full opacity) to oldest visible
   fix (open-question opacity values — see Risks).
3. Current-position dot always paints on top of its own tail.
4. Selecting a tag (`Selection::Node(_)` from
   [`tickets/KIOSK-003-sidebar-row-redesign.md`](KIOSK-003-sidebar-row-redesign.md))
   replaces that tag's three-fix tail with the full polyline through
   `NodeData.track`.
5. Other tags retain their three-fix tail when one tag is selected.
6. Deselecting (`Selection::None`) returns the selected tag to
   baseline three-fix tail behaviour.
7. No-fix tags render the ghost marker only (no baseline tail).
   Selection of a no-fix tag's behaviour resolved per Risks open
   question.
8. SOS tag renders current dot + pulse ring only — **no three-fix
   tail** (Decisions pinned #4). Pulse ring per
   [`markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
   is the SOS surface; a red trail would visually compete with it.
9. No regression in current marker rendering at
   [`markers.rs:194-302`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
   nor in the legacy `draw_tracks` helper at
   [`markers.rs:102-114`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
   used by the `MapMode::FakeGrid` / `MapMode::OsmVector` paths.
10. Existing tests pass; new tests cover the baseline-tail +
    selection-polyline branches.
11. PMTiles inline tag draw at
    [`pmtiles_map.rs:173-179`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)
    is updated to use `freshness_color` from
    [`markers.rs:40-48`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
    so tail and current dot both render per-state colour
    (BLUE/RED/GREY) — Decisions pinned #8.

## Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Default `MultiTag` scenario: tag-1 (normal) shows a current dot
   plus three smaller dots trailing in its movement direction.
3. Tap tag-1 in the sidebar: sidebar replaces with detail (KIOSK-004
   behaviour), and on the map the three-fix tail is replaced by a
   full polyline connecting every fix in `track`.
4. Tap another tag (or back-to-list): the polyline for tag-1
   disappears; the three-fix tail returns.
5. Switch to `Sos`: SOS tag shows current dot + pulse ring only — no
   three-fix tail (Decisions pinned #4).
6. Switch to `SosNoFix`: no-fix tag renders ghost only (no tail), per
   Risks resolution.
7. Switch to `Stale`: stale tag shows GREY-dim tail driven by
   `freshness_color`. Verify the current-dot colour also matches
   (Decisions pinned #8).
8. `cargo test --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
   passes (any new tests this ticket adds).

## Likely files / modules touched

- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) —
  primary site; new track-tail + selection-polyline draw inside the
  `Map::show` closure after the basemap layers and before / interleaved
  with the current marker draw at lines 161-202. Needs to read the
  current `Selection` state from app-level (the inline draw does not
  currently take a `Selection` argument; signature widening expected).
- [`tools/sarcom-kiosk-lab/src/map/markers.rs`](../tools/sarcom-kiosk-lab/src/map/markers.rs) —
  possibly extract a `draw_tag_tail` and `draw_tag_full_track` helper
  for reuse with the legacy paths; current marker logic at lines
  194-302 stays intact. The existing `draw_tracks` helper at
  lines 102-114 is a different visual treatment (1 px dashed segments)
  and is retained for the legacy paths unchanged.
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs`](../tools/sarcom-kiosk-lab/src/ui/palette.rs) —
  possibly add a `TAIL_FADE_*` helper if interpolating opacities
  cleanly is awkward inline; otherwise reuse existing tag colours
  with multiplied alpha via
  `egui::Color32::from_rgba_unmultiplied(r, g, b, a)`.
- New test file likely required for the baseline-tail and
  selection-polyline rendering paths.

## Decisions pinned

Three of the original eight Risks open questions have been pinned by
Pieter. Source: the STEP-6 cleanup prompt (2026-05-19) that bundled
clock-invalid removal with these pinnings.

- **#4 SOS-state tail: NO.** SOS tag renders current dot + pulse ring
  only. No three-fix tail. The pulse ring per
  [`markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
  is the attention-grabbing surface; an extra red trail competes with
  it. Reflected in the per-state behaviour table above (SOS row =
  "NO tail") and acceptance criterion #8.
- **#7 Selection polyline colour: tag colour at opacity 0.5.**
  Identity wins; a neutral colour confuses "whose path is this" when
  multiple tags are on the map. Reflected in the Scope's
  Selection-driven full track style spec.
- **#8 Tail vs current-dot colour parity: fix-in-passing.** KIOSK-008
  also updates
  [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:173-179`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)
  to call `freshness_color` from
  [`tools/sarcom-kiosk-lab/src/map/markers.rs:40-48`](../tools/sarcom-kiosk-lab/src/map/markers.rs)
  so the current dot's colour matches the tail's colour per state.
  This is a pre-existing honesty-discipline bug fix bundled into
  KIOSK-008 (BLUE-only inline draw lies about freshness state).
  Reflected in acceptance criterion #11.

## Risks / open questions

Five concrete design values remain **Pieter has not decided**. Each
item is surfaced for the per-ticket mockup review, not pre-filled.

- **Fade opacity values for the three-fix tail.** Concrete numbers —
  e.g. newest 1.0, middle 0.6, oldest 0.3, or some other curve.
  Implementer-choice within Pieter's intent? Or pin a specific triple
  now?
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

## Confidence

**Medium.** The baseline tail is mechanically straightforward (loop
over last three fixes, render with decreasing opacity). The
selection-polyline read of the `Selection` enum is a small extension
to the current marker draw, but the PMTiles inline tag draw at
`pmtiles_map.rs:161-202` does not currently take a `Selection`
argument — signature widening to plumb `Selection` from the app-level
into the render path is the main mechanical cost. The load-bearing
risks are the open-question design values; once Pieter pins them,
implementation is small.

## Dependencies

- **Depends on**
  [`tickets/KIOSK-003-sidebar-row-redesign.md`](KIOSK-003-sidebar-row-redesign.md)
  — provides the `Selection` enum that selection-polyline consumes.
- **Soft sibling of**
  [`tickets/KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md)
  — KIOSK-004 owns the sidebar-replacement detail surface; KIOSK-008
  owns the map-render side of the same selection. Both consume
  `Selection`.
- **No dependency on**
  [`tickets/KIOSK-001-map-scale-north.md`](KIOSK-001-map-scale-north.md)
  — can land in either order; chrome and tracks are independent.
- **SPIKE-001 closed strict** per
  [`tickets/README.md:45`](README.md) — track-on-map is not an
  overlay; no ADR-007 tension.
- **SPIKE-002 closed reject** per
  [`tickets/README.md:46`](README.md) — no uncertainty disc; ghost
  marker stays as point + dashed ring.
