# KIOSK-004 — Selection → recenter + sidebar-replacement detail — v1a mockup rationale

Single-design strict-ADR mockup. Two stacked 800×480 lab fixtures
demonstrating the **uniform detail layout** that renders for any
selected node. Per
`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`, the UI
data model is one `NodeData` shape; the detail surface is one layout;
icon glyph + colour come from the inventory map
(`HashMap<u8, NodeKind>`), not from a per-kind layout branch.

SVG at `UX/mockups/KIOSK-004-selection-detail-sidebar.svg`.

## Scenario summary (two panels — same layout)

**Panel A — tag-3 selected, no-fix.** Tag-3 rendered as a ghost at
`last_valid_fix_pos` with a dashed outer ring + faded fill per
`tools/sarcom-kiosk-lab/src/map/markers.rs:265-302`, plus an amber
dashed selection halo. Other markers dimmed. Map performs a 150 ms
eased pan onto `tag_visible_pos` (one-shot, no follow mode). The
detail surface renders the **uniform layout**: back row → node header
(BLUE `●` from inventory) → state strip → key/value rows → NOT SHOWN
block. The no-fix-specific render-path adds the `LAST FIX · 8 m ago`
framing line scoping the lat/lon rows below to `last_valid_fix_pos`
— **not** the sentinel current `pos` — per `KIOSK-004:50, 91`.

**Panel B — relay-2 selected, overdue.** Relay-2 rendered as the
ORANGE cross marker with an amber selection halo. Map performs the
same 150 ms eased pan onto the relay's `pos`. The detail surface
renders the **identical uniform layout** as Panel A — back row → node
header (ORANGE cross from inventory) → state strip → key/value rows →
NOT SHOWN block. The only field-level differences: ORANGE icon (vs
BLUE), `⚠ OVERDUE · 65 m (> 3600 s)` state strip (vs `⚠ NO FIX`),
no `LAST FIX` framing (relay-2 has `gps_valid=true`, current pos IS
the last fix), `NOT SHOWN` block **identical** to Panel A.

**The NOT SHOWN block being identical across both panels is the
load-bearing demonstration.** It is reserved for protocol-level
closures (RSSI/SNR/hop count per ADR-013 §10) that apply to any node
emitting POSITION packets. Not a relay-specific block. Not a
sim-fixture-gap rationalisation.

**Gateway detail (formerly Panel C) is removed.** Gateway-self status
(battery, RTC, render-tick liveness) is deferred from v1a per
`tickets/KIOSK-005-gateway-status-surface.md` (deferred stub). In
v1a a gateway node renders the **same uniform detail layout**:
header (`■` GREEN from inventory + label), state strip `● HEALTHY`,
no `last frame` row (gateway is the receiver — sentinel-zero
`last_seen_secs` is not a meaningful age, so the row is simply
absent), lat/lon from `NodeData.pos`. No extra gateway-only chrome.

In every panel the back-to-list row at the top is the **only**
dismissal affordance. Tap-outside on the map does **not** dismiss.
Bottom strip remains the existing read-only PMTiles rendering per
`app.rs:283-344`.

## Three-layer basemap composition (both panels)

Each panel's map area surfaces three layers per
`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`:

| Layer | Source | Visible in SVG as |
|---|---|---|
| **L1 · PMTiles basemap** (Protomaps dark) | `pmtiles_map.rs:61, 73, 106` | Dark slate + thin pale line-art + grid pattern |
| **L2 · LIDAR hillshade** (DHMV-II DSM @ 0.5 transparency) | `pmtiles_map.rs:25, 64-74, 107-113` | Translucent grayscale gradient + soft ridge lines |
| **L3 · OSM XML overlays** (Overpass + hand-drawn) | `pmtiles_map.rs:115-125`, schema `region.rs:25-71` | Building polygons distinguishable from L1 line-art |
| **Markers** | `markers.rs` | Bright glyphs on top of all three layers |

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| Uniform detail layout for any node kind | `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md` · `tickets/KIOSK-004-selection-detail-panel.md` (post-collapse) |
| Inventory map (`HashMap<u8, NodeKind>`) | `tools/sarcom-kiosk-lab/src/data.rs` (post-refactor) |
| Sidebar replacement, not overlay, no slide-in | `KIOSK-004` (post-collapse) |
| `← Back to list` is the only dismiss | `KIOSK-004` |
| 150 ms eased pan, one-shot | `KIOSK-004` · `tools/sarcom-kiosk-lab/src/map/markers.rs:32-38` |
| Recenter target for no-fix tag: `tag_visible_pos` | `markers.rs:32-38` |
| Recenter target for other nodes: `NodeData.pos` | `KIOSK-004` |
| `LAST FIX · {age}` framing for no-fix node | `KIOSK-004:50, 91` |
| Ghost rendering at `last_valid_fix_pos` | `tools/sarcom-kiosk-lab/src/map/markers.rs:265-302` |
| Relay overdue threshold > 3600 s | `tools/sarcom-kiosk-lab/src/data.rs:42-48` |
| `format_age_or_unavailable` | `tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26` |
| Selection enum (`Selection::Node(usize)`) | `KIOSK-003-sidebar-row-redesign.md` (post-collapse) |
| Three-layer render stack | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Overlay schema | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Map chrome budget = scale bar + compass rose | `tickets/README.md:26` + `KIOSK-001:16, 31` |
| ADR-007 read-only invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |
| 800×480 lab fixture (ADR-015-pending) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:4-19` |
| Gateway-self status deferred from v1a | `tickets/KIOSK-005-gateway-status-surface.md` (deferred stub) |
| ADR-013 §10 reception-log v2+ deferral | `decisions/ADR-013-multi-hop-flood-via-packet-id.md` §10 |

## Uniform-layout field table — same template, different NodeData values

Every node renders this template. Rows that don't apply for the
selected node are **absent**, not `N/A` placeholders. The kind-icon
glyph + colour come from inventory lookup.

| Row | Source field | Renders when |
|---|---|---|
| Back row | (UI chrome) | always |
| Header: kind-icon + `{label}` | inventory.kind + `NodeData.label` | always |
| State strip | derived: `sos` → DISTRESS, `!gps_valid` → NO FIX, freshness-bucket → STALE/HEALTHY/OVERDUE | always |
| `last fix` | `NodeData.last_valid_fix_age_secs` | only when `!gps_valid` (otherwise current `pos` IS the last fix); scopes the lat/lon rows below |
| `LAST FIX · {age}` scoping label | (UI chrome) | only when `!gps_valid` |
| `lat` / `lon` | `NodeData.pos` if `gps_valid` else `NodeData.last_valid_fix_pos` | when a position exists |
| `🔋 BATT` token | `NodeData.battery_low` | only when `battery_low == true` |
| `NOT SHOWN` block (RSSI/SNR/hop count) | (protocol closure annotation) | always — cites ADR-013 §10 |

`—` and "absent" cells are **deliberately not rendered**, not rendered as `N/A`.

## What is NOT rendered

- **No overlay, slide-in, popover, banner, modal.** `KIOSK-004` (post-collapse).
- **No tap-outside dismiss.** Back row is the only dismissal.
- **No follow-mode lock.** Recenter is one-shot.
- **No per-kind layout branch.** Inventory.kind drives icon glyph + colour only.
- **No new NodeData fields.** Use the post-collapse `data.rs` shape.
- **No gateway-self chrome** (battery, RTC, render-tick). Deferred per KIOSK-005.
- **No `NOT SHOWN` entries for sim-fixture gaps.** A NodeData field that v1a sim happens not to populate for a given kind is **not** a NOT SHOWN reason — the row is simply absent. NOT SHOWN is reserved for protocol-level closures.
- **No invented RSSI / SNR / hop count.** Named in NOT SHOWN block (protocol closure).
- **No write actions.**
- **No floating map buttons.**
- **No crosshair.** KIOSK-002 deferred.
- **No sentinel current `pos`** rendered for a `!gps_valid` node — only `last_valid_fix_pos` under the `LAST FIX` framing.

## What a reviewer verifies

- Detail view **fully replaces** the sidebar list in both panels — no surface overlays the map.
- **`← Back to list`** is the only dismissal affordance.
- **The layout is identical between Panel A and Panel B.** Only the NodeData field values differ. The same template renders. The kind-icon glyph + colour are the only kind-specific differences (BLUE `●` vs ORANGE `✚`).
- **NOT SHOWN block is identical** between Panel A and Panel B — proof that it cites a protocol closure, not a kind-specific data-model gap.
- **tag-3 ghost** at `last_valid_fix_pos` with dashed ring + faded fill in Panel A; `LAST FIX · 8 m ago` framing scopes lat/lon.
- **Relay-2** in Panel B renders the same uniform layout — no relay-specific NOT SHOWN block, no relay-specific layout chrome.
- **Map recenter** is a smooth 150 ms eased pan in both panels.
- **Three-layer basemap** rendered in both panels with distinguishable L1/L2/L3 anchors.
- **No `RTC ok` / `RTC unset` / battery / charging / render-tick chrome anywhere** — gateway-self status is deferred per KIOSK-005.

## Open questions for Pieter

1. **State-strip wording.** Mockup uses `⚠ NO FIX` / `⚠ OVERDUE` / `● HEALTHY`. Exact text + when each fires is a presentation detail.
2. **Tag-3 selection halo colour.** Mockup uses AMBER (matches the no-fix state). Could also be a state-neutral white halo.
3. **Gateway last-seen rendering.** Mockup omits the `last frame` row entirely for gateway (sentinel-zero `last_seen_secs` is not meaningful). The post-collapse ticket allows either rendering; current choice is omission to match the no-prefix discipline applied across the rest of the UI.
