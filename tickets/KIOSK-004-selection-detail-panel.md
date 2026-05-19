---
id: KIOSK-004
title: "Selection → recenter + sidebar-replacement detail surface"
status: ready-for-review
type: implementation-ticket
opened: 2026-05-18
adr007-variant-dependency: closed — SPIKE-001 closed strict; sidebar replacement only
---

# KIOSK-004 — Selection → recenter + sidebar-replacement detail surface

## Problem statement

Once [`KIOSK-003-sidebar-row-redesign.md`](KIOSK-003-sidebar-row-redesign.md) collapses each sidebar row to a single operational line, the operator needs a way to see the rest of the per-node detail (full coordinates, last-valid-fix age separately when no-fix, battery flag, SOS flag, ui_kind, label, node_id, self-ann age for relays). Selecting a row should also recenter the map on that node's last-known position — currently tapping a hiker row at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:31-34`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) only toggles the row's background tint without moving the camera.

Tapping a marker on the map should drive the same selection. Today the click-to-select logic in the legacy paths at [`tools/sarcom-kiosk-lab/src/map/mod.rs:224-245`](../tools/sarcom-kiosk-lab/src/map/mod.rs) sets `self.selected_tag` but does not recenter; the PMTiles path at [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) delegates the full map area to walkers and does not currently have marker tap-handling at all (markers are drawn by the lab's painter; gestures go to walkers).

## User story

*As a SAR operator, I want tapping a node — in the list or on the map — to recenter the map on its last-known fix and show me full per-node detail in one gesture.*

## Scope

Per [`SPIKE-001-adr007-informational-overlays.md`](SPIKE-001-adr007-informational-overlays.md)'s closure (strict ADR-007 retained for v1a) and Pieter's v1a UX posture, the detail surface is **sidebar replacement only**. No overlay, no slide-in panel, no popover, no tap-outside dismiss.

### Selection-driven sidebar replacement

- Selection (set via tap on a sidebar row from KIOSK-003 OR via tap on a map marker) **replaces the sidebar list content** with a sidebar-replacement detail view. A **`← Back` row / button** at the top returns to the list. **No tap-outside dismiss** — return is always via the explicit back affordance.
- The detail view occupies the full 320 px sidebar width — design the layout to read at 2 m glance per [`decisions/ADR-007-touchscreen-primary-ui.md:21`](../decisions/ADR-007-touchscreen-primary-ui.md). This is a first-class detail surface, not a cramped sidebar inset.
- No overlay. No slide-in panel. No surface overlays the map.
- Detail layout and flags display follow the current v1a operator-map mockup style at [`UX/mockups/v1a-operator-map-mockup.md`](../UX/mockups/v1a-operator-map-mockup.md).

### Map recenter

- Map performs a **one-shot pan-only recenter** (no zoom change, no follow-mode) onto the selected node's `tag_visible_pos` (existing helper at [`tools/sarcom-kiosk-lab/src/map/markers.rs:32-38`](../tools/sarcom-kiosk-lab/src/map/markers.rs)) for hikers, or `pos` for relay / gateway.
- **Smooth recenter at 150 ms** (eased pan). Snap is acceptable as a fallback if walkers' camera API does not support animated pan, but the target is a 150 ms eased pan.

### Per-node compact detail layouts

**Per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`**: the UI data model is one uniform `NodeData` struct. The detail surface renders the same field set for any selected node, regardless of inventory kind. Icon glyph + colour for the header come from `inventory.get(node_id)` — not from a per-kind layout branch.

### Detail layout (uniform across node kinds)

All fields are read from the selected `NodeData` at [`tools/sarcom-kiosk-lab/src/data.rs`](../tools/sarcom-kiosk-lab/src/data.rs).

- **Back row / button** (top): returns selection to `Selection::None`.
- **Node header** (large): `{kind-icon} {label}` — icon glyph + colour from inventory lookup.
- **State strip** (one line, full-width tint): worst-priority state derived from POSITION-derived flags — `🔴 DISTRESS` if `sos`, `⚠ NO FIX` if `!gps_valid`, `⚠ STALE` or `● HEALTHY` from `freshness_color(state)`. No `RTC ok` / `RTC unset` — gateway-self status is deferred from v1a per `tickets/KIOSK-005-gateway-status-surface.md` (deferred stub).
- **Key/value rows** (compact two-column):
  - `last frame` — `format_age_or_unavailable(last_seen_secs)` per [`tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26`](../tools/sarcom-kiosk-lab/src/ui/mod.rs). For gateway nodes (`inventory.kind == Gateway`), `last_seen_secs` is sentinel-zero (gateway is local); render the row as `last frame · — (local)` rather than `0 s`.
  - `last fix` — `format_age_or_unavailable(last_valid_fix_age_secs)`, rendered when distinct from `last_seen`, **including the no-fix case** where it scopes the lat/lon below.
  - `lat` / `lon`:
    - When `gps_valid = true`: derived from current `pos`.
    - **When `gps_valid = false`**: derived from `last_valid_fix_pos` and framed by the `last fix · {age}` line above. **Do not show the sentinel / current invalid coordinate as if it were useful.**
  - `battery` — only when `battery_low == true`, rendered as `🔋 low` (AMBER). Not rendered as `ok` when false; absence carries the signal.
- **No `NOT SHOWN` block for sim-fixture gaps.** A `NOT SHOWN` block is reserved for protocol-level closures (e.g. RSSI/SNR/hop count per ADR-013 §10 reception-log v2+ deferral) and for gateway-local-concept-N/A cases. Absent fields on a relay because the v1a fixture doesn't populate them are **not** a NOT SHOWN reason — they're a presentation artifact of the same uniform layout reading a sparse `NodeData`.

### Common

- All age fields honour `format_age_or_unavailable` at [`tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26`](../tools/sarcom-kiosk-lab/src/ui/mod.rs).
- Back row / button returns selection to `Selection::None`.
- **No RSSI / SNR / hop count.** Wire protocol does not carry them (ADR-013 §10 closes reception-log telemetry as v2+ deferral); the UI data model follows the wire. Mockup may carry a `NOT SHOWN` block citing ADR-013 §10; do not add fields.

## Non-goals

- **No overlay surface of any kind.** No slide-in panel. No popover. No banner. No modal.
- **No tap-outside dismiss.** Return to the list is via the Back row / button only.
- **No write surface.** No edit, no save, no waypoint set, no commissioning trigger (commissioning has its own dedicated spike at [`spikes/ble-gateway-ui-flow-spike.md`](../spikes/ble-gateway-ui-flow-spike.md), out of scope here).
- **No lock-follow mode.** Recenter is one-shot, not continuous. Lock-follow is a separate operator preference and a separate later ticket if needed.
- **No new data fields** on `NodeData`. Use the post-collapse fields per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`.
- **No gateway-self status surface in v1a.** Battery / charging / RTC / render-tick liveness for the gateway are deferred per `tickets/KIOSK-005-gateway-status-surface.md`.
- **No protocol changes.**
- **No casual use of the word "modal"** in code comments or operator-facing copy. Use *sidebar-replacement detail view*, or *bottom strip* (existing chrome).

## Acceptance criteria

1. Tapping any sidebar row OR any map marker sets the same `Selection` value (the enum introduced in KIOSK-003).
2. Map pans to selected node's visible position via a **150 ms eased pan** — for tags use `tag_visible_pos` per [`tools/sarcom-kiosk-lab/src/map/markers.rs:32-38`](../tools/sarcom-kiosk-lab/src/map/markers.rs) (last_valid_fix_pos when `!gps_valid`, else current pos); relays and gateway use `pos` directly. Snap is acceptable only as a fallback if walkers' camera API does not support animated pan.
3. Sidebar list content is **replaced in place** by the detail view; no surface overlays the map.
4. The detail view's layout uses the full 320 px sidebar width and is legible at 2 m glance.
5. Detail view shows the per-node compact fields listed in Scope, in the mockup-aligned layout. Relative-time strings render via `format_age` in `tools/sarcom-kiosk-lab/src/ui/mod.rs`.
6. **No-fix tag detail shows the last valid fix coordinates framed as `LAST FIX · {age}`.** The current sentinel / invalid coordinate is **not** rendered as a useful coordinate.
7. The Back row / button returns selection to `Selection::None`. **Tap-outside on the map does NOT dismiss the detail.**
8. Switching scenarios clears selection (preserved from existing `switch_scenario` at [`tools/sarcom-kiosk-lab/src/app.rs:99-107`](../tools/sarcom-kiosk-lab/src/app.rs)).
9. A code comment cites the SPIKE-001 closure and the strict-ADR posture (sidebar replacement, no overlay, no tap-outside dismiss).

## Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Switch to `MultiTag` scenario.
3. Tap tag-2 (SOS) in the sidebar. Map pans (eased, ~150 ms) to tag-2's position. Sidebar list is replaced by tag-2's full detail.
4. Tap tag-3's no-fix ghost marker on the map. Detail view updates; shows `LAST FIX · {age}` with last valid fix coordinates. Current sentinel is not shown.
5. Tap relay-1 in the sidebar. Map pans to relay; detail view shows the uniform NodeData layout — same fields as tag-1's detail view (rows that don't apply, e.g. `last fix` when relay has `gps_valid=true`, are simply absent — not rendered as `N/A` placeholders).
6. Tap the gateway row. Detail view shows the same uniform layout. `last frame` row reads `— (local)` since the gateway is the receiver. No `RTC ok` / battery / charging chrome (deferred per KIOSK-005).
7. Tap the Back row / button. Detail view dismisses; sidebar list returns.
8. Tap on empty map area while detail view is shown. Detail view does NOT dismiss (no tap-outside dismiss).
9. Drag-to-reposition a marker (existing behaviour at [`tools/sarcom-kiosk-lab/src/map/markers.rs:78-98`](../tools/sarcom-kiosk-lab/src/map/markers.rs)) still works; drag is distinguished from tap by movement-threshold (Risk).
10. `cargo test` passes including the existing scenario-based tests.

## Likely files / modules touched

- [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) — already touched by KIOSK-003; this ticket adds the sidebar-replacement detail view as a branch when `selection != Selection::None`
- [`tools/sarcom-kiosk-lab/src/app.rs`](../tools/sarcom-kiosk-lab/src/app.rs) — sidebar panel composition at lines 347-356
- [`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs) — marker tap-detection (the PMTiles path does not currently have it); recenter via walkers' `MapMemory` (`zoom()` exposed at line 42-44, but the centre needs setter exposure too); eased pan helper
- [`tools/sarcom-kiosk-lab/src/map/mod.rs:224-245`](../tools/sarcom-kiosk-lab/src/map/mod.rs) — existing legacy-path click-to-select extended to drive the new `Selection` enum and the recenter call
- [`tools/sarcom-kiosk-lab/src/map/markers.rs:50-76`](../tools/sarcom-kiosk-lab/src/map/markers.rs) — `find_closest` currently used by the drag system; selection re-uses it
- New module candidate: `tools/sarcom-kiosk-lab/src/ui/detail_panel.rs`

## Risks / open questions

- **Drag-vs-tap disambiguation.** A short tap = select; movement > threshold = drag. Current drag system at [`tools/sarcom-kiosk-lab/src/map/mod.rs:167-189`](../tools/sarcom-kiosk-lab/src/map/mod.rs) uses `response.drag_started()` / `dragged()` which already distinguishes; the new tap-to-select needs to fall through cleanly when drag fires.
- **Recenter in PMTiles vs legacy.** Legacy paths use `view_offset` ([`tools/sarcom-kiosk-lab/src/app.rs:33`](../tools/sarcom-kiosk-lab/src/app.rs)). PMTiles uses walkers' `MapMemory` ([`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:26`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)). Two different camera systems. Confirm walkers exposes a `center_at(Position)` or equivalent, and an eased-pan path or a way to interpolate via repeated frame updates.
- **150 ms eased pan feasibility.** If walkers does not expose camera interpolation, an in-app frame-driven interpolation (set centre per frame for 150 ms) is the fallback. If even that is non-trivial, instant snap is the acceptance-criteria-permitted fallback.
- **Tap on marker in PMTiles**: walkers owns the touch events when the PMTiles path is delegated inside `pm.show()` ([`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:80+`](../tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs)). The lab's painter draws markers over the walkers area but does not currently intercept clicks there. May require restructuring how the painter is allocated, or layering a transparent overlay painter on top.
- **Gateway coordinates "if meaningful".** Whether the gateway has a meaningful position depends on the deployment shape. For v1a single-garden, the gateway is roughly fixed; show `NodeData.pos`. For mobile-handheld carry scenarios, the position may change. Implementer renders the field; later operator feedback decides whether it should ever be omitted.

## Confidence

**Medium.** Selection plumbing extension is straightforward; the gesture integration with walkers, the 150 ms eased recenter, and the no-tap-outside-dismiss enforcement are the load-bearing risks.

## Dependencies

- **Depends on** [`KIOSK-003-sidebar-row-redesign.md`](KIOSK-003-sidebar-row-redesign.md) — provides the `Selection::Node(usize)` enum that this ticket consumes.
- **No dependency on** [`KIOSK-005-gateway-status-surface.md`](KIOSK-005-gateway-status-surface.md) — KIOSK-005 is deferred from v1a per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`. The v1a detail surface is one uniform layout for any node kind; the gateway selection renders the same NodeData fields the layout shows for tag and relay (no special gateway-self chrome). Earlier draft cited a soft dependency on `GatewayData.battery_pct` / `charging` extensions; both the dependency and those struct extensions are retracted.
- **SPIKE-001 closed strict** — no variant decision is pending; sidebar replacement is the only design.
