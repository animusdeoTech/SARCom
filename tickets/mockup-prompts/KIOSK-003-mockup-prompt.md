# Claude CLI prompt — KIOSK-003 mockup (sidebar row redesign)

You are producing a single SVG mockup that illustrates the implementation
target for KIOSK-003. Output one SVG file at
`UX/mockups/KIOSK-003-mockup.svg` and one short markdown rationale at
`UX/mockups/KIOSK-003-mockup.md`. **No feature code changes.**

## Read first

- [`CLAUDE.md`](../../CLAUDE.md)
- [`decisions/ADR-007-touchscreen-primary-ui.md`](../../decisions/ADR-007-touchscreen-primary-ui.md)
- [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs`](../../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) — current sidebar
- [`tools/sarcom-kiosk-lab/src/data.rs:131-167`](../../tools/sarcom-kiosk-lab/src/data.rs) — TagData / RelayData / GatewayData
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs`](../../tools/sarcom-kiosk-lab/src/ui/palette.rs)
- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`](../../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) — three-layer render stack (basemap + hillshade + OSM overlays, in z-order)
- [`tools/sarcom-kiosk-lab/src/map/region.rs:25-71`](../../tools/sarcom-kiosk-lab/src/map/region.rs) — Overlay enum (osm + hillshade variants)
- [`dev-log/2026-05-16-lidar-overlay-implementation.md`](../../dev-log/2026-05-16-lidar-overlay-implementation.md) — implementation context for the LIDAR hillshade
- [`tickets/KIOSK-003-sidebar-row-redesign.md`](../KIOSK-003-sidebar-row-redesign.md)

## Hard constraints

1. **Single design.** No overlays, no popovers. Per `tickets/README.md:14-33` and the strict-ADR closure of SPIKE-001.
2. **Sidebar 320px wide, full panel height.**
3. **All rows minimum 48px tall** for touch targets.
4. **Lab fixture size:** 800×480 (map 480px wide on left, sidebar 320px wide on right). The map can be rendered as a greyed/abstracted block — focus of THIS mockup is the sidebar. When abstracting, label the block to indicate the three layers exist underneath: `map area abstracted; full render is basemap + LIDAR hillshade + OSM overlays per pmtiles_map.rs:82-125`. Do not flatten the map's description to "PMTiles line-art" — that's only the bottom layer.
5. **Mission-first sort preserved** for the scrollable list per the existing logic at `tools/sarcom-kiosk-lab/src/ui/sidebar.rs:57-84`.
6. **No counter footer card.** The stale README description at `tools/sarcom-kiosk-lab/README.md:56` is dropped from v1a scope; do not render it.
7. **Map chrome budget** (when the map is rendered, even abstractly): scale bar + compass rose only. No zoom buttons.
8. **Use only palette constants from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.**

## The design to render

### Sidebar layout, top to bottom

**1. Sticky `DISTRESS` section** (present only when ≥1 SOS or no-fix node):

- Section header bar (~20px tall) with label `DISTRESS` in `RED` or `AMBER` per palette, against a dark band (`PANEL_BG` darker variant).
- Rows pinned here for tags in SOS or no-fix state.
- Each row ≥48px tall.
- Rows do not scroll with the rest of the list.

**2. Scrollable `NODES` section:**

- Section header (~20px tall) with label `NODES · n` where n = count of non-alert nodes.
- Mission-first sort preserved.
- Each row ≥48px tall.

### Row format — one operational line per row

Row format is **uniform across node kinds** per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`. Kind-distinction is icon glyph + colour from inventory map only (tag `●`, relay `✚`, gateway `■`). Timestamps render bare (no `last` / `POSITION` prefix); line context disambiguates. The `last fix` framing on no-fix rows remains because it scopes the lat/lon to a known past position, not a current sentinel.

| Node state | Row primary text format | Colour |
|---|---|---|
| Fresh tag | `● tag-1  ·  12 s` | `TEXT_BRIGHT` label, `TEXT_DIM` age |
| SOS tag | `🔴 SOS · tag-2 · 42 s` | `RED`, bold |
| No-fix tag | `⚠ tag-3 · NO FIX · last fix 8 m` | `AMBER` |
| Stale tag | `● tag-4 · stale · 12 m` | `TEXT_DIM` |
| Battery-low | append `🔋 BATT` (icon + suffix together) on the primary line — both signals carry, the icon for glance, the text for unambiguous readout | tint the `🔋 BATT` token `AMBER` |
| Relay healthy | `✚ relay-1 · 14 m` | `ORANGE` label |
| Relay overdue (>3600s) | `⚠ relay-1 · 65 m` | `AMBER` |
| Gateway | `■ gw-0` | `GREEN`; no age suffix (gateway is local) |

### Selection treatment

- **Full-row background tint** when selected (use a subtle highlight from the existing palette — e.g. `PANEL_BG` lightened by ~15%, or a derived translucent tint).
- The selected row's text remains the same colour; only the row background changes.
- Show **one selected row** in the mockup (e.g. tag-2 selected) so the affordance is visible.

### Selectable rows

- All rows are selectable, including relay and gateway rows (this is the KIOSK-003 scope change from the current sidebar).

### Battery-low display detail

- Both icon and suffix together on the primary line: `🔋 BATT` (small battery icon glyph + suffix text), tinted `AMBER`. Per `tickets/KIOSK-003-sidebar-row-redesign.md:35`.
- Both signals carry: the icon for at-a-glance scanning, the text for unambiguous readout. Co-located on the primary line — not split across primary line + row's right edge.

## Scenario

Render the `MultiTag` scenario from `tools/sarcom-kiosk-lab/src/data.rs`, extended for this mockup so each row-format variant is visible:

- tag-1 (Fresh, last seen 12 s)
- tag-2 (SOS, last seen 42 s) — pinned in `DISTRESS` section
- tag-3 (No-fix, last fix 8 m) — pinned in `DISTRESS` section
- tag-4 (Stale, 12 m)
- relay-1 (healthy, 14 m) — orange ✚ cross glyph
- gw-0 (gateway, no age suffix) — green ■ square glyph

Show **tag-2 as the selected row** (visible full-row tint).

Optionally add tag-6 with battery-low so the `🔋 BATT` icon + suffix token is visible somewhere in the rendering (on the primary line, not split to the row edge).

## Annotation requirements

- Annotate the sticky `DISTRESS` section header and one of its rows.
- Annotate the scrollable `NODES` section header.
- Annotate the selected-row treatment (full-row tint).
- Annotate the battery-low display: `🔋 BATT · icon + suffix together on the primary line · per KIOSK-003:35`.
- Margin note: `no counter footer card; deferred per KIOSK-003 scope · all rows ≥48px touch target`

## Rationale markdown content

- Scenario summary.
- Per-element source-of-truth citations (`path:line`).
- A "Row-format table" reproducing the table above for reference.
- A "what a reviewer verifies" closing section:
  - Each row is ≥48px tall
  - Sticky `DISTRESS` section is structurally separate from scrollable `NODES` section
  - Selection treatment is visible on one row (full-row tint)
  - Battery-low shows `🔋 BATT` (icon + suffix together) on the primary line
  - Mission-first sort preserved
  - No counter footer card

## Non-goals

- No counter footer card.
- No swipe gestures, no long-press actions on rows.
- No popover, no overlay, no detail view (that is KIOSK-004).
- No new palette colours — reuse existing `RED`/`AMBER`/`ORANGE`/`GREY`/`GREEN`/`TEXT_DIM`/`TEXT_BRIGHT`.
- No changes to the map area beyond rendering it as a greyed block.
- No floating buttons on the map.
