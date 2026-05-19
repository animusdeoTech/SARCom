# Claude CLI prompt — KIOSK-004 mockup (selection → recenter + sidebar-replacement detail)

You are producing a single SVG mockup that illustrates the implementation
target for KIOSK-004. Output one SVG file at
`UX/mockups/KIOSK-004-mockup.svg` and one short markdown rationale at
`UX/mockups/KIOSK-004-mockup.md`. **No feature code changes.**

## Read first

- [`CLAUDE.md`](../../CLAUDE.md)
- [`decisions/ADR-007-touchscreen-primary-ui.md`](../../decisions/ADR-007-touchscreen-primary-ui.md)
- [`tools/sarcom-kiosk-lab/src/data.rs:123-150`](../../tools/sarcom-kiosk-lab/src/data.rs) — uniform `NodeData` shape (lines 123-142) + `SimState { nodes, inventory }` (lines 145-150) per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`. One layout for any node kind; kind-glyph + colour from the inventory map.
- [`tools/sarcom-kiosk-lab/src/map/markers.rs`](../../tools/sarcom-kiosk-lab/src/map/markers.rs) — especially `tag_visible_pos` helper at lines 32-38 and the ghost-marker rendering at lines 265-302
- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125`](../../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) — three-layer render stack (basemap + hillshade + OSM overlays, in z-order)
- [`tools/sarcom-kiosk-lab/src/map/region.rs:25-71`](../../tools/sarcom-kiosk-lab/src/map/region.rs) — Overlay enum (osm + hillshade variants)
- [`dev-log/2026-05-16-lidar-overlay-implementation.md`](../../dev-log/2026-05-16-lidar-overlay-implementation.md) — implementation context for the LIDAR hillshade
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs`](../../tools/sarcom-kiosk-lab/src/ui/palette.rs)
- [`tickets/KIOSK-004-selection-detail-panel.md`](../KIOSK-004-selection-detail-panel.md) — **the current ticket text is the spec — follow it.** Single design: sidebar-replacement detail view, no overlay, no tap-outside dismiss, per the strict-ADR closure of SPIKE-001.
- Prior umbrella mockup at `UX/mockups/v1a-operator-map-mockup.svg` — the left panel (strict-ADR sidebar replacement) is the layout precedent to mirror.

## Hard constraints

1. **Single design.** Detail view REPLACES the sidebar list in place. No slide-in panel, no overlay, no popover, no modal, no banner.
2. **Back-to-list via the top row only.** No tap-outside dismissal. The sidebar IS the surface; the operator returns to the list via the explicit `← Back to list` row.
3. **Map recenter is a smooth pan, 150ms duration. No zoom change.** One-shot — not follow-mode. Recenter targets `tag_visible_pos` for hikers (see `markers.rs:32-38`) and `pos` for relay / gateway.
4. **Uniform detail layout for any node kind.** Per the post-collapse data model, every selected node renders the SAME field set from `NodeData`; rows that don't apply (e.g. `last fix` framing on a tag with `gps_valid=true`) are simply absent — not `N/A` placeholders. Icon glyph + colour come from inventory.kind lookup, not from a per-kind layout branch.
5. **Map chrome budget:** scale bar + compass rose only. No zoom +/−, no fit-all, no home, no clear-UI, no other floating buttons.
6. **Lab fixture size:** 800×480 per `tools/sarcom-kiosk-lab/README.md:53`. Annotate as ADR-015-pending.
7. **Use only palette constants from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.**

## The design to render

The SVG must contain **three stacked 800×480 panels**, one per node type, so the per-type compact detail layout is visible.

### Map area composition (applies to all three panels)

**Map area renders three layers, in z-order:**

1. **PMTiles basemap** — Protomaps dark style, line-art on dark slate (roads, water, buildings, landuse, boundaries, places). Source: `pmtiles_map.rs:61, 73, 106`.
2. **LIDAR-derived hillshade raster** — DHMV-II DSM baked into a raster PMTiles archive, rendered at transparency 0.5 above the basemap so terrain relief is visible without washing out the line art. Source: `pmtiles_map.rs:25, 64-74, 107-113`.
3. **OSM XML overlays** — Overpass-fetched + hand-drawn overlays, painted in declaration order inside the `Map::show` closure. Source: `pmtiles_map.rs:115-125`, schema at `region.rs:25-71`.

Sim markers (relay orange cross, gateway green square outline, tag coloured circles per state, no-fix ghost dashed ring) paint on top of all three layers, **plus** the selection halo per panel.

**Map area SVG fidelity:**

The SVG is wireframe fidelity, not photorealism. The three-layer basemap is suggested as follows:

- **Layer 1 (basemap):** dark slate background (`MAP_BG` from `tools/sarcom-kiosk-lab/src/ui/palette.rs`), with light line-art suggesting roads + water + landuse (thin pale strokes, not full Protomaps fidelity).
- **Layer 2 (hillshade):** soft grayscale terrain shading patch covering most of the map area, suggesting elevation relief. Render as a translucent gradient or noise texture at ~30-50% opacity over the basemap. Annotate as `LIDAR hillshade · transparency 0.5 · per pmtiles_map.rs:107-113`.
- **Layer 3 (OSM overlays):** light additional line-work or area shapes painted over the hillshade — distinguishable from the basemap line-art. Annotate as `OSM XML overlays · Overpass + hand-drawn · per pmtiles_map.rs:115-125`.

Do not attempt pixel-fidelity reproduction of the actual tile rendering. The goal is "a reviewer looking at this knows it's basemap + hillshade + OSM overlays, not just basemap." If you have to choose between suggesting all three layers and clean readability, suggest all three even if rough — the absence of hillshade or OSM overlay is the failure mode this cleanup is fixing.

### Panel A — Tag detail (tag-3 selected, no-fix scenario)

- Sidebar (320px wide, full height) shows tag-3's detail per the strict-ADR sidebar-replacement layout in the prior umbrella mockup's left panel.
- Top row (≥48px tall): `← Back to list` with sub-text `(tag-3 selected — tap to return)`. Tap target ≥48px.
- Header block: `NODE` small label + `tag-3` large (e.g. 22pt) in `AMBER`, + sub-label `hiker · last fix 8 m ago`.
- State strip: `AMBER` `⚠ NO FIX  gps_valid=false`.
- Key-value rows (monospace, large enough to read at 2m glance):
  - `last frame age` → `30 s`
  - **`LAST FIX · 8 m ago`** — framing label spanning the next two rows in `AMBER`, per `tickets/KIOSK-004-selection-detail-panel.md:50, 91` (last valid fix coordinates only; sentinel current `pos` is NOT rendered as a useful coordinate)
  - `last fix lat` → `50.92338° N` (under the LAST FIX frame)
  - `last fix lon` → `5.41862° E` (under the LAST FIX frame)
  - `battery` → `ok` (in `GREEN`)
- Flags row: three pills/chips for `SOS · 0`, `GPS_VALID · 0` (highlighted in `AMBER` because invalid is the alert state), `BATT · 0`.
- `NOT SHOWN` block listing `RSSI / SNR / hop count` with the rationale `kiosk-lab data model does not carry these fields (data.rs:131-150)`.
- Map area:
  - Compass rose top-left, scale bar bottom-left (per KIOSK-001).
  - tag-3 rendered as **ghost at `last_valid_fix_pos`** with dashed outer ring + faded fill per `markers.rs:265-302`, with framing sub-label `tag-3 · last fix (8m ago)`.
  - Selection halo around tag-3 (subtle amber dashed circle) to show selection.
  - Other markers (tag-1, tag-2, etc.) shown at reduced prominence.

### Panel B — Relay detail (relay-2 selected, overdue)

- Sidebar renders the **same uniform layout as Panel A**. The only differences are the kind-glyph + colour (relay: `✚` ORANGE from inventory) and which NodeData fields end up populated.
- Top row: `← Back to list`.
- Header block: `NODE` small label + kind-glyph `✚` + `relay-2` large (e.g. 22pt) in `ORANGE`.
- State strip: `AMBER` `⚠ OVERDUE  65 m (> 3600 s)` — drop the `last frame` / `POSITION` prefix per the no-prefix discipline; the state strip itself is the timestamp context.
- Key-value rows:
  - `lat` → (relay-2's lat)
  - `lon` → (relay-2's lon)
  - No `last frame` row (age carried by the state strip).
  - No `last fix` framing (relay has `gps_valid=true`; current pos IS the last fix).
- **`NOT SHOWN` block IDENTICAL to Panel A: `RSSI · SNR · hop count — per ADR-013 §10 (reception-log v2+ deferral)`.** This is the load-bearing demonstration — the same NOT SHOWN block reads against ANY node, not against the relay specifically. No `flags / battery — relay data model does not carry these fields` reasoning; relay-specific absences are sim-fixture gaps, not protocol closures.
- Map: same chrome (compass rose + scale bar only); relay-2 marked as selected (amber halo around the relay cross).

### Panel C — Gateway detail (REMOVED)

Gateway-self detail (battery / RTC / render-tick liveness) is deferred from v1a per `tickets/KIOSK-005-gateway-status-surface.md` (deferred stub). In v1a a gateway selection renders the SAME uniform layout as Panel A / B — kind-glyph `■` GREEN + label, state strip `● HEALTHY`, `last frame · — (local)` row (or row omitted entirely — implementer choice), `lat` / `lon` from NodeData.pos, identical `NOT SHOWN` block. No battery / charging / RTC rows.

If you're producing this mockup with three panels, the gateway panel becomes a third uniform-layout demo (no extra chrome). If you're producing it with two panels, omit the gateway entirely and surface the "gateway renders the same layout" claim in the annotation margin instead.

## Map recenter annotation

In each panel, add a small arrow + margin note: `smooth pan 150ms onto selected node's tag_visible_pos (markers.rs:32-38) for no-fix tag, else NodeData.pos · one-shot, no follow-mode`.

## Annotation requirements

- Annotate the `← Back to list` row in each panel.
- Annotate the **uniformity** of the layout across kinds: the layout is the same template for tag / relay / gateway; only kind-glyph + colour come from inventory.kind lookup. NOT SHOWN block is identical across panels (protocol-closure, not kind-specific absence).
- Annotate the tag-3 ghost-marker framing in Panel A: `rendered at last_valid_fix_pos · NOT at sentinel current pos · per markers.rs:265-302`.
- Margin note: `no slide-in panel · no overlay · sidebar IS the detail surface · per ADR-007 strict reading · single design`.

## Rationale markdown content

- Scenario summary (the three panels, one per node type).
- Per-element source-of-truth citations (`path:line`).
- A "Uniform fields table" showing which detail rows appear for any node, with a small column noting when a row is populated (always / only when `!gps_valid` / only when `battery_low`), citing `data.rs:123-142` (NodeData).
- A "what a reviewer verifies" closing section:
  - Detail view fully replaces sidebar list — no overlay over the map
  - `Back-to-list` is the only dismissal affordance (no tap-outside)
  - Relay and gateway selections render the SAME uniform layout as the tag selection (kind-glyph + colour differ; field set is the same template; absent rows are absent, not `N/A`)
  - tag-3 ghost rendered at last_valid_fix_pos with explicit `LAST FIX · {age}` framing per `KIOSK-004:50, 91`
  - Map recenter is smooth 150ms pan, one-shot, no zoom change
  - Map chrome = compass rose + scale bar only (no floating buttons; no crosshair — KIOSK-002 deferred)

## Non-goals

- No slide-in panel, no popover, no overlay, no modal.
- No tap-outside dismissal.
- No follow-mode lock; one-shot recenter only.
- No new data fields on `NodeData`.
- No gateway-self chrome (battery / charging / RTC / render-tick). Deferred per KIOSK-005.
- No invented RSSI/SNR — listed in NOT SHOWN with rationale only.
- No write actions, no commissioning trigger, no waypoint create.
- No floating buttons on the map.
- No N/A placeholder rows for absent fields.
