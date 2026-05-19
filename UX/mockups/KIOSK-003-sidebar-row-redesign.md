# KIOSK-003 — Sidebar row redesign — v1a mockup rationale

Single-design strict-ADR mockup. Renders the new sidebar row format per
`tickets/KIOSK-003-sidebar-row-redesign.md:29-39`, sticky `DISTRESS`
section per :41, ≥48 px touch targets per :40, 72, selectable
relay+gateway rows per :42, 67, 74, and full-row selected tint per :77.
SVG at `UX/mockups/KIOSK-003-sidebar-row-redesign.svg`.

This mockup's focus is the sidebar. The map area is rendered as an
abstracted block, labelled to acknowledge the three layers that live
underneath at runtime per `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`
(PMTiles basemap + LIDAR DHMV-II hillshade + OSM XML overlays).

## Scenario summary

The 800×480 lab fixture shows the `MultiTag` scenario extended to
exercise every row format. Map area is rendered as a greyed/abstracted
block with a four-line label naming the actual render stack (basemap
→ LIDAR hillshade → OSM XML overlays + citation). KIOSK-001 owns the
full map chrome render; this mockup deliberately keeps the map flat
so reviewer attention lands on the sidebar.

Sidebar (320 px wide, full panel height) shows:

- **Sticky `DISTRESS` section** (header `DISTRESS · 2`) pinned at top:
  - `🔴 SOS · tag-2 · 42 s` (red, bold) — **selected**, full-row tint
  - `⚠ tag-3 · NO FIX · last fix 8 m` (amber)
- **Scrollable `NODES · 5` section**, mission-first sort preserved:
  - `● tag-1  ·  12 s ago` (normal)
  - `● tag-4 · stale · 12 m` (dim)
  - `● tag-5 · very stale · 24 m` (very dim)
  - `● tag-6 · 18 s  ·  🔋 BATT` (battery-low, AMBER `🔋 BATT` token
    co-located on the primary line)
  - `● relay-1 · POSITION 14 m`
  - `● gw-0 · RTC ok` (green)

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| One-line row format per node state | `tickets/KIOSK-003-sidebar-row-redesign.md:29-39` |
| Very-stale row variant | `KIOSK-003:34` |
| Battery-low: `🔋 BATT` icon + suffix together on primary line | `KIOSK-003:35, 131` |
| Sticky `DISTRESS` section above scrollable list | `KIOSK-003:41, 73` |
| All rows ≥48 px touch target | `KIOSK-003:40, 72, 17` |
| Mission-first sort preserved | `tools/sarcom-kiosk-lab/src/ui/sidebar.rs:57-84` + `KIOSK-003:41` |
| Selectable hiker rows (existing) | `tools/sarcom-kiosk-lab/src/ui/sidebar.rs:31-34` |
| Selectable relay + gateway rows (KIOSK-003 scope change) | `KIOSK-003:42, 67, 74` |
| Full-row selected tint | `KIOSK-003:77` + existing tint at `src/ui/sidebar.rs:93-97` |
| `format_age_or_unavailable` for relative-time strings | `tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26` |
| No counter footer card | `KIOSK-003:24, 67, 132` |
| Three-layer map render stack (when not abstracted) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Overlay schema | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Hillshade implementation context | `dev-log/2026-05-16-lidar-overlay-implementation.md` |
| ADR-007 read-only invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |
| 800×480 lab fixture (ADR-015-pending) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:4-19` |
| KIOSK-004 detail surface (no overlay) | `tickets/KIOSK-004-selection-detail-panel.md:24, 28, 76` |
| No coord readout in v1a (KIOSK-002 deferred) | `tickets/README.md:30, 53, 71, 85` |

## Row-format table (reproduced from `KIOSK-003:29-39`)

| Node state | Row primary text | Colour |
|---|---|---|
| Normal hiker | `● tag-1  ·  12 s ago` | `TEXT_BRIGHT` label, `TEXT_DIM` age |
| SOS hiker | `🔴 SOS · tag-2 · 42 s` | `RED`, bold |
| No-fix hiker | `⚠ tag-3 · NO FIX · last fix 8 m` | `AMBER` |
| Stale hiker | `● tag-4 · stale · 12 m` | `TEXT_DIM` |
| Very-stale hiker | `● tag-5 · very stale · 24 m` | `TEXT_DIM` (very dim) |
| Battery-low hiker | append `🔋 BATT` (icon + suffix together) on the primary line | tint the `🔋 BATT` token `AMBER` |
| Relay healthy | `● relay-1 · POSITION 14 m` | `TEXT_BRIGHT` |
| Relay overdue (>3600 s) | `⚠ relay-1 · POSITION 65 m` | `AMBER` |
| Gateway healthy | `● gw-0 · RTC ok` | `GREEN` |
| Gateway RTC invalid | `⚠ gw-0 · RTC unset` | `AMBER` |

## What is NOT rendered

- **No counter footer card.** `KIOSK-003:24, 67, 132`.
- **No swipe gestures, no long-press actions on rows.** `KIOSK-003:64`.
- **No popover, no overlay, no detail view.** That is KIOSK-004.
- **No floating map buttons.**
- **No coord readout anywhere.** KIOSK-002 deferred.

## What a reviewer verifies

- Each row is **≥48 px tall** (48 px per row + 22 px section headers).
- **Sticky `DISTRESS` section** is structurally separate from the
  scrollable `NODES` section.
- **Selection treatment is visible on tag-2**: full-row background
  tint extends across the full 320 px width.
- **Battery-low shows `🔋 BATT`** (icon + suffix together) on the
  primary line.
- **Mission-first sort preserved**.
- **Very-stale row present** as a distinct format from `stale`.
- **No counter footer card.**
- **Relay and gateway rows are selectable.**
- Map area's abstract label names the **three layers underneath**
  (basemap + LIDAR hillshade + OSM XML overlays per
  `pmtiles_map.rs:82-125`), so the reviewer doesn't mistake the
  abstract for "PMTiles only".

## Open questions for Pieter

1. **Sticky-section label** — `DISTRESS` (mockup), `ALERTS`, or
   `ACTIVE DISTRESS` per `KIOSK-003:129`?
2. **`🔋 BATT` glyph specifically** — emoji vs vector battery glyph.
