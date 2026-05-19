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
- **Scrollable `NODES · 4` section**, mission-first sort preserved:
  - `● tag-1  ·  12 s` (normal)
  - `● tag-4 · stale · 12 m` (dim)
  - `● tag-6 · 18 s  ·  🔋 BATT` (battery-low, AMBER `🔋 BATT` token
    co-located on the primary line)
  - `✚ relay-1 · 14 m` (orange cross)
  - `■ gw-0` (green square)

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| One-line row format per node state | `tickets/KIOSK-003-sidebar-row-redesign.md:29-39` |
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

## Row-format table — uniform across node kinds

Row format is **state-driven**, not kind-driven. The same template renders for every node; the inventory map (`HashMap<u8, NodeKind>`) provides the icon glyph + colour only. Per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`.

| Node state | Row primary text | Colour | Sticky section? |
|---|---|---|---|
| Fresh | `{glyph} {label}  ·  {age}` | label by kind, `TEXT_DIM` age, glyph from `freshness_color(Fresh)` | no |
| Aging | `{glyph} {label}  ·  {age}` | label by kind, `TEXT_DIM` age, glyph from `freshness_color(Aging)` | no |
| Stale | `{glyph} {label} · stale · {age}` | `TEXT_DIM` | no |
| SOS | `🔴 SOS · {label} · {age}` | `RED`, bold | **yes** (DISTRESS) |
| No-fix | `⚠ {label} · NO FIX · last fix {age}` | `AMBER` | **yes** (DISTRESS) |
| Battery-low | append `🔋 BATT` (icon + suffix together) on the primary line — token tinted `AMBER` | compatible with any state above | inherits |

Note: the timestamp itself carries no `last` / `POSITION` prefix; the line context (sidebar node row) makes the meaning unambiguous. The `last fix` framing on no-fix rows remains because it scopes the lat/lon below to a known past position, not a current sentinel.

**Inventory-driven icon glyph + colour** (presentation only — no data-model branch):
- `NodeKind::Tag` → `●` filled dot, `TEXT_BRIGHT` label colour
- `NodeKind::Relay` → `✚` cross, `ORANGE` label colour
- `NodeKind::Gateway` → `■` square, `GREEN` label colour; `{age}` suffix elided (gateway is local, `last_seen_secs = 0`)

For the gateway specifically, the row reads `■ gw-0` (with the inventory-assigned glyph + colour; no age). No `RTC ok` / `RTC unset` chrome — gateway-self status is deferred per `tickets/KIOSK-005-gateway-status-surface.md`.

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
- **No `last` / `POSITION` prefix on timestamps** — line context carries the meaning.
- **Kind-specific glyphs**: tag `●` / relay `✚` / gateway `■` matching map markers.
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
