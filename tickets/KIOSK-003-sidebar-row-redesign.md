---
id: KIOSK-003
title: "Sidebar row redesign — one operational line, sticky alerts, 48 px rows, selectable relay/gateway"
status: ready-for-review
type: implementation-ticket
opened: 2026-05-18
adr007-variant-dependency: none
---

# KIOSK-003 — Sidebar row redesign

## Problem statement

The current sidebar at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) has four operator-relevant problems:

1. **High info density at uniform visual weight.** Each hiker row renders SOS banner (lines 106-127), name with state dot (lines 129-138), last-seen wall+age line (lines 165-179), coordinates line (lines 180-188), GPS_VALID=0 sentinels message (lines 140-163 path), last-fix-age line, and BATT LOW (lines 191-199) — all in 10-11 pt dim-grey monospace at roughly the same weight.
2. **Touch targets too small.** Rows use `Margin::symmetric(10, 5)` at line 102 plus 10-11 pt text → roughly 30-40 px row height. Standard touch-target minimum is 44-48 px.
3. **Relay and gateway rows are non-interactive.** Hiker rows accept clicks at lines 31-34 (`resp.clicked()` → toggles `self.selected_tag`). Relay row at lines 205-266 and gateway row at lines 268-302 do not implement click handling at all — operator cannot select them.
4. **No sticky alert section.** Mission-first ordering at lines 57-84 sorts SOS / no-fix to the top, but they scroll with the list. In Multi-Tag scenarios (`ScenarioKind::MultiTag` at [`tools/sarcom-kiosk-lab/src/data.rs:287-338`](../tools/sarcom-kiosk-lab/src/data.rs)) the alerts can scroll off-screen if the operator scrolls.

This ticket is **ADR-007-independent**. None of the changes introduce an overlay, popover, dialog, write affordance, acknowledgement flow, or any other ADR-007-touching surface. The sidebar remains read-only; row taps are select-only (and the detail-panel that consumes that selection is a separate ticket, [`KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md)).

## User story

*As a SAR operator, I want each row in the node list to tell me one operationally-actionable fact at a glance, with critical alerts always visible, so I can scan the list in 2 seconds rather than reading it.*

## Scope

- **Collapse each row to one primary line.** Per state:
  - Normal hiker: `● tag-1  ·  12 s ago`
  - SOS hiker: `🔴 SOS · tag-2 · 42 s` (red, bold)
  - No-fix hiker: `⚠ tag-3 · NO FIX · last fix 8 m` (amber)
  - Stale hiker: `● tag-4 · stale · 12 m` (dim)
  - Very-stale hiker: `● tag-5 · very stale · 24 m` (very dim)
  - Battery-low hiker: appended as ` · BATT` to the primary line
  - Relay healthy: `● relay-1 · self-ann 14 m`
  - Relay overdue (past 3600 s per [`tools/sarcom-kiosk-lab/src/data.rs:42-48`](../tools/sarcom-kiosk-lab/src/data.rs)): `⚠ relay-1 · self-ann 65 m` (amber)
  - Gateway healthy: `● gw-0 · RTC ok`
  - Gateway with invalid RTC: `⚠ gw-0 · RTC unset` (amber)
- **Bump row minimum height to 48 px.** Update `Margin::symmetric(10, 5)` at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:102, 213, 270`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) (three frames) to deliver it.
- **Pin SOS-and-NoFix rows in a sticky alert section** above the scrollable list area. The mission-first sort logic at lines 57-84 is preserved as the fallback for the scrolling list (so non-alert rows stay sorted by hiker-priority).
- **Make relay and gateway rows selectable.** Both currently render a `Frame` without `.interact(Sense::click())` (relay at line 212-265, gateway at line 269-301). Add click handling that toggles selection consistent with hiker selection at lines 31-34.
- **Lift `selected_tag: Option<usize>` to a richer `Selection` enum.** Current type at [`tools/sarcom-kiosk-lab/src/app.rs:13`](../tools/sarcom-kiosk-lab/src/app.rs) only addresses hiker indices. Proposed shape:
  ```rust
  enum Selection {
      None,
      Tag(usize),    // index into sim.tags (Vec<TagData>)
      Relay(usize),  // index into a future Vec<RelayData>; today the lab has a singular
                     // `relay: RelayData` at data.rs:172, so the value is always Relay(0).
                     // Shape anticipates v1b (two-relay chained test per CLAUDE.md) without
                     // forcing the SimState widening in this ticket.
      Gateway,       // SARCOM v1 has one gateway per deployment per ARCHITECTURE.md;
                     // no index needed.
  }
  ```
  `Relay(usize)` matches the `Tag(usize)` pattern (positional index into the relevant Vec) and is forward-compatible with the v1b multi-relay scenario. The data-model widening (`sim.relay: RelayData` → `sim.relays: Vec<RelayData>`) is **out of scope for this ticket** and belongs in whatever ticket introduces multi-relay sim scenarios. Until then, the only valid relay-selection value is `Relay(0)`, dispatched against `sim.relay` directly.

  Lift touches every callsite: [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:30-34`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs), [`tools/sarcom-kiosk-lab/src/app.rs:81, 102, 138`](../tools/sarcom-kiosk-lab/src/app.rs), [`tools/sarcom-kiosk-lab/src/map/markers.rs:194-203`](../tools/sarcom-kiosk-lab/src/map/markers.rs) (`draw_tags` signature takes `selected_tag: Option<usize>`).
- **Multi-line per-row detail is removed from the sidebar entirely.** Full coordinates, last-valid-fix-age line, GPS_VALID=0 sentinels message, BATT LOW separate line, lat/lon, ui_kind — all move to KIOSK-004's detail panel.

## Non-goals

- **Not in scope:** detail-panel render — that is [`KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md) and depends on SPIKE-001's variant decision.
- **Not in scope:** swipe gestures or long-press actions on rows (gloves + risk; not v1a).
- **Not in scope:** new data fields on `TagData` / `RelayData` / `GatewayData`. Current shape at [`tools/sarcom-kiosk-lab/src/data.rs:131-167`](../tools/sarcom-kiosk-lab/src/data.rs) suffices for the one-line rows.
- **Not in scope:** new visual palette entries — existing `RED` / `AMBER` / `ORANGE` / `GREY` / `GREEN` / `TEXT_DIM` / `TEXT_BRIGHT` at [`tools/sarcom-kiosk-lab/src/ui/palette.rs:8-19`](../tools/sarcom-kiosk-lab/src/ui/palette.rs) suffice.
- **Not in scope:** counters footer card (it is mentioned in [`tools/sarcom-kiosk-lab/README.md:56`](../tools/sarcom-kiosk-lab/README.md) but I did not see it implemented in the current `sidebar.rs`; the README is stale on multiple points per KIOSK-007).

## Acceptance criteria

1. Each scenario in `ScenarioKind::all()` ([`tools/sarcom-kiosk-lab/src/data.rs:75-85`](../tools/sarcom-kiosk-lab/src/data.rs)) renders rows that match the one-line-per-row table above.
2. Row height (any row, any scenario) is ≥48 px when measured against egui's row-rect.
3. In the `MultiTag` scenario, both the SOS tag (tag-2) and the no-fix tag (tag-4) appear in the sticky-alert section at the top of the sidebar. Scrolling the rest of the list does not displace them.
4. Tapping the relay row toggles selection to `Selection::Relay(0)` (only valid relay index in current sim per the data-model note above); tapping the gateway row toggles to `Selection::Gateway`; tapping a hiker row toggles to `Selection::Tag(i)`. Re-tapping the selected row clears selection (consistent with current behaviour at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:31-34`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs)).
5. Selection state survives the existing scenario-switch reset at [`tools/sarcom-kiosk-lab/src/app.rs:102`](../tools/sarcom-kiosk-lab/src/app.rs) clearing to `Selection::None` — same as today's `self.selected_tag = None`.
6. `format_age_or_unavailable` at [`tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26`](../tools/sarcom-kiosk-lab/src/ui/mod.rs) continues to be honoured for every relative-time string in the rows — no new callsite emits "X ago" against `clock_valid=false`. The lock-in test at [`tools/sarcom-kiosk-lab/src/ui/mod.rs:48-53`](../tools/sarcom-kiosk-lab/src/ui/mod.rs) still passes.
7. Visual selection treatment (existing background tint at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:93-97`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs)) is extended to relay and gateway rows.
8. Existing map-side selection rendering (selected-tag white outline at [`tools/sarcom-kiosk-lab/src/map/markers.rs:231-237`](../tools/sarcom-kiosk-lab/src/map/markers.rs)) continues to work for `Selection::Tag(_)`. Relay / Gateway selection visuals on the map are out of scope for this ticket (KIOSK-004 problem).

## Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Cycle through each scenario via the header combobox ([`tools/sarcom-kiosk-lab/src/ui/header.rs:24-39`](../tools/sarcom-kiosk-lab/src/ui/header.rs)). For each, verify the row content matches the one-line table.
3. Switch to `MultiTag`. Confirm tag-2 (SOS) and tag-4 (no-fix) are pinned at top in the sticky-alert section.
4. Tap each row type: hiker, relay, gateway. Visual selection treatment appears on each.
5. Tap a row, then switch scenarios; selection clears (existing `switch_scenario` behaviour).
6. Switch to `Clock Invalid`; all relative-time strings show "time unavailable" via `format_age_or_unavailable`.
7. `cargo test --manifest-path tools\sarcom-kiosk-lab\Cargo.toml` — existing tests pass (especially `unavailable_when_clock_invalid` at [`tools/sarcom-kiosk-lab/src/ui/mod.rs:48-53`](../tools/sarcom-kiosk-lab/src/ui/mod.rs) and `sos_no_fix_scenario_has_ghost_data` at [`tools/sarcom-kiosk-lab/src/data.rs:518-526`](../tools/sarcom-kiosk-lab/src/data.rs)).

## Likely files / modules touched

- [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) — primary rewrite of `render_hiker_row` (lines 86-203), `render_relay_row` (205-266), `render_gateway_row` (268-302); add sticky-alert section render; per-row click handling for relay/gateway
- [`tools/sarcom-kiosk-lab/src/app.rs`](../tools/sarcom-kiosk-lab/src/app.rs):
  - line 13: `selected_tag: Option<usize>` → `selection: Selection`
  - line 81: default in `new()`
  - line 102: reset in `switch_scenario`
  - line 138: reset in `load_layout`
- [`tools/sarcom-kiosk-lab/src/map/markers.rs`](../tools/sarcom-kiosk-lab/src/map/markers.rs):
  - line 186-203 `draw_tags` signature change from `selected_tag: Option<usize>` to `selection: &Selection`
  - line 215-222 update map-render call site

## Risks / open questions

### Three-question check — `Selection` enum (esp. `Relay(usize)`)

Per [CLAUDE.md "Schema-extension discipline — the subtyping-fetish check"](../CLAUDE.md), run the three questions explicitly before adding a new typed variant:

1. **Operator-visible?**
   Yes. Selection controls which node is highlighted, recentered, and shown in the detail surface. Relay and gateway selection are operator-visible UI state.
2. **Existing attribute pattern?**
   There is no existing single attribute-pattern that spans Tag / Relay / Gateway selection. Existing state is `selected_tag: Option<usize>`, which only covers hikers. Since tags and relays live in different data structures, a typed UI-runtime enum is clearer than inventing a shared role/kind field. This remains UI state only and does not add a wire-protocol role byte or reopen ADR-013.
3. **Blast radius?**
   Small but real: replaces `selected_tag: Option<usize>` with a richer enum and touches sidebar selection, marker selected rendering, scenario reset, and KIOSK-004 detail/recenter logic. Five callsites; each is a one-line change. The compiler will catch every miss.

Final enum shape:

```rust
enum Selection {
    None,
    Tag(usize),
    Relay(usize),
    Gateway,
}
```

### Other risks / open questions

- **`Relay(usize)` indexing matches `Tag(usize)`** (positional index into the relevant Vec), anticipating the v1b multi-relay scenario per CLAUDE.md without forcing the `SimState` widening here. Whatever later ticket introduces multi-relay sim scenarios is the right place to extend `sim.relay: RelayData` → `sim.relays: Vec<RelayData>` and start dispatching `Relay(i)` against the vec; until then the enum carries a 0 placeholder. A longer-term question (out of scope for v1a) is whether the eventual addition of drone-relays per [`spikes/airborne-positioning-overlay-spike.md`](../spikes/airborne-positioning-overlay-spike.md) collapses Tag / Relay / Gateway / Drone into a single `Node(NodeId)` selection keyed via `nodes.toml.ui_kind` — that is a category-shape decision for a separate spike, not this ticket.
- **Sticky-alert section label.** "ALERTS"? "ACTIVE DISTRESS"? Pick at mockup review (see [`CLAUDE-DESIGN-PROMPT-v1a-operator-map.md`](CLAUDE-DESIGN-PROMPT-v1a-operator-map.md)).
- **48 px rows × 7 nodes on `MultiTag` + 2 in sticky section** = potentially overflowing on 800×480 minus header strip. Verify in the lab. If overflow, the scrollable list is the spill area and that's fine; sticky section stays pinned.
- **Battery-low display location.** Inline as ` · BATT` suffix on the primary line is compact but loses prominence. Consider a small icon instead.
- **Counter footer card.** The kiosk-lab README at [`tools/sarcom-kiosk-lab/README.md:56`](../tools/sarcom-kiosk-lab/README.md) describes a `▼ NODES (n)` / `▼ NO FIX (n)` / counters-footer arrangement that does not exist in the current `sidebar.rs`. KIOSK-007 (doc cleanup) is the right place to reconcile this — this ticket should not silently re-implement what the README claims.

## Confidence

**High.** Pure UI rewrite over an understood, fully-cited data model. Selection-enum lift is mechanical.

## Dependencies

- None for this ticket directly.
- **Blocks** [`KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md), which consumes the new `Selection` enum.
