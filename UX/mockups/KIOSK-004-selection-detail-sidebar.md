# KIOSK-004 — Selection → recenter + sidebar-replacement detail — v1a mockup rationale

Single-design strict-ADR mockup. Three stacked 800×480 lab fixtures
showing the per-node-type compact detail surface for tag, relay, and
gateway selection. Detail view **replaces** the sidebar list in place;
no overlay; no slide-in; no tap-outside dismiss; 150 ms eased pan
recenter. Map areas in all three panels surface the three-layer
basemap composition per
`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`. SVG at
`UX/mockups/KIOSK-004-selection-detail-sidebar.svg`.

## Scenario summary (three panels)

**Panel A — tag-3 selected, no-fix.** Tag-3 rendered on the map as a
ghost at `last_valid_fix_pos` with a dashed outer ring + faded fill per
`tools/sarcom-kiosk-lab/src/map/markers.rs:265-302`, plus an amber
dashed selection halo. Other markers dimmed. Map performs a 150 ms
eased pan onto `tag_visible_pos` (one-shot, no follow mode). Detail
surface's most load-bearing element is the `LAST FIX · 8 m ago`
framing label scoping the lat/lon rows below it to the last valid fix —
**not** the sentinel current `pos` — per `KIOSK-004:50, 91`.

**Panel B — relay-2 selected, overdue.** Sidebar replacement renders
the compact relay detail. RelayData carries only `node_id`, `label`,
`pos`, and `last_seen_secs` per `tools/sarcom-kiosk-lab/src/data.rs:153-160`.
Compact detail: state strip, three key-value rows, no flags, no
battery. `NOT SHOWN` block names absent fields with a citation.

**Panel C — gw-0 selected, healthy.** Most-compact gateway detail.
GatewayData base carries `node_id`, `label`, `pos` per
`tools/sarcom-kiosk-lab/src/data.rs:163-167`; `KIOSK-005` extends with
`battery_pct: Option<u8>` and `charging: Option<bool>`. Mockup renders
the KIOSK-005-extended shape with concrete values (78%, charging yes).

In every panel the back-to-list row at the top is the **only**
dismissal affordance. Tap-outside on the map does **NOT** dismiss.
Bottom strip remains the existing read-only PMTiles rendering per
`app.rs:283-344`.

## Three-layer basemap composition (all three panels)

Each panel's map area surfaces three layers per
`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`:

| Layer | Source | Visible in SVG as |
|---|---|---|
| **L1 · PMTiles basemap** (Protomaps dark) | `pmtiles_map.rs:61, 73, 106` | Dark slate + thin pale line-art (roads, water, landuse) + grid pattern |
| **L2 · LIDAR hillshade** (DHMV-II DSM, raster PMTiles) | `pmtiles_map.rs:25, 64-74, 107-113` | Translucent grayscale gradient + soft ridge lines over the basemap |
| **L3 · OSM XML overlays** (Overpass + hand-drawn) | `pmtiles_map.rs:115-125`, schema `region.rs:25-71` | Building polygons, hand-drawn footpath, distinguishable from L1 line-art |
| **Markers** | `markers.rs` | Bright glyphs on top of all three layers |

Hillshade transparency is 0.5 per the current default at
`pmtiles_map.rs:108-112` (tunable 0.35–0.7).

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| Sidebar replacement, not overlay, no slide-in | `KIOSK-004:24, 28, 30, 76, 88` |
| `← Back to list` is the only dismiss | `KIOSK-004:28, 77, 92` |
| 150 ms eased pan, one-shot, no zoom change | `KIOSK-004:35-36, 87` |
| Recenter target for hikers: `tag_visible_pos` | `tools/sarcom-kiosk-lab/src/map/markers.rs:32-38` |
| Recenter target for relay/gateway: `pos` | `KIOSK-004:35` |
| Per-node-type compact fields, no N/A placeholders | `KIOSK-004:23, 38-67, 80, 91, 117` |
| Tag detail fields (TagData) | `tools/sarcom-kiosk-lab/src/data.rs:131-150` |
| Relay detail fields (RelayData) | `tools/sarcom-kiosk-lab/src/data.rs:153-160` |
| Gateway detail fields (GatewayData base) | `tools/sarcom-kiosk-lab/src/data.rs:163-167` |
| Gateway battery + charging extension | `KIOSK-005-gateway-status-surface.md:33-48` |
| `LAST FIX · {age}` framing for no-fix tag | `KIOSK-004:50, 91` |
| Ghost marker rendering at `last_valid_fix_pos` | `tools/sarcom-kiosk-lab/src/map/markers.rs:265-302` |
| Relay overdue threshold > 3600 s | `tools/sarcom-kiosk-lab/src/data.rs:42-48` |
| `format_age_or_unavailable` honoured | `tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26` |
| Selection enum (from KIOSK-003) | `KIOSK-003-sidebar-row-redesign.md:43-56` |
| Three-layer render stack | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Overlay schema | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Hillshade implementation context | `dev-log/2026-05-16-lidar-overlay-implementation.md` |
| Map chrome budget = scale bar + compass rose | `tickets/README.md:26` + `KIOSK-001:16, 31` |
| No coord readout / no crosshair (KIOSK-002 deferred) | `tickets/README.md:30, 53, 71, 85` |
| ADR-007 read-only invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |
| 800×480 lab fixture (ADR-015-pending) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:4-19` |

## Per-node-type fields table

| Detail row | Tag (A) | Relay (B) | Gateway (C) |
|---|---|---|---|
| `← Back to list` | ✓ | ✓ | ✓ |
| Header: `NODE` + `<label>` | ✓ AMBER | ✓ AMBER | ✓ GREEN |
| State strip | `⚠ NO FIX  gps_valid=false` | `⚠ OVERDUE  POSITION 65 m` | `● HEALTHY  RTC ok` |
| `last frame age` | ✓ `30 s` | — | — |
| `LAST FIX · {age}` framing | ✓ scopes lat/lon | — | — |
| `last fix lat / lon` | ✓ | — | — |
| `last frame` | — | ✓ `POSITION 65 m` | — |
| `lat / lon` (current) | — | ✓ | ✓ |
| `battery` | ✓ `ok` | — | ✓ `78%` (KIOSK-005-ext) |
| `charging` | — | — | ✓ `yes` (KIOSK-005-ext) |
| Flags pills | ✓ | — | — |
| `NOT SHOWN` block | RSSI/SNR/hop count | flags/battery | frame age |

`—` cells are deliberately absent — **not** rendered as `N/A`.

## What is NOT rendered

- **No overlay, slide-in, popover, banner, modal.** `KIOSK-004:24, 28, 30, 76`.
- **No tap-outside dismiss.** `KIOSK-004:28, 77, 92`.
- **No follow-mode lock.** Recenter is one-shot. `KIOSK-004:79`.
- **No new data fields** on `TagData` / `RelayData` / `GatewayData`.
- **No invented RSSI / SNR / hop count.** Named in NOT SHOWN block.
- **No write actions.** `KIOSK-004:78, 115`.
- **No floating map buttons.**
- **No crosshair.** KIOSK-002 deferred.
- **No sentinel current `pos`** in tag-3 detail — only the last valid fix.

## What a reviewer verifies

- Detail view **fully replaces** the sidebar list — no surface overlays the map.
- **`← Back to list`** is the only dismissal affordance.
- **Relay** and **gateway** detail layouts omit fields their data
  models do not carry.
- **tag-3 ghost** at `last_valid_fix_pos` with dashed ring + faded
  fill; framed `LAST FIX · 8 m ago` heading in the detail.
- **Map recenter** is a smooth 150 ms eased pan.
- **Three-layer basemap** rendered in all three panels with
  distinguishable L1/L2/L3 anchors (basemap line-art, hillshade
  gradient, OSM overlays).
- The KIOSK-005 gateway battery/charging extension renders in Panel C.

## Open questions for Pieter

1. **Panel C battery value when KIOSK-005 has NOT yet landed.** The
   mockup renders concrete values to show the extended shape.
   `KIOSK-004:65, 70` allows omitting the rows. If a placeholder is
   preferred, the text must be `BAT unknown` per `KIOSK-005:46, 79`.
2. **Tag-3 selection halo colour.** Mockup uses `AMBER` (matches the
   no-fix state).
3. **Gateway `pos` rendering for handheld-carry deployments.**
   `KIOSK-004:66, 125` — mockup shows the pos for v1a garden test.
