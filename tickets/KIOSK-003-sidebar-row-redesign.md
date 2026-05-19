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

- **Collapse each row to one primary line.** Row format is **uniform across node kinds** — per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`, the UI data model is one `NodeData` shape; the only kind-distinction is icon glyph + colour from the inventory map (`HashMap<u8, NodeKind>`). Rows differ by node **state** (POSITION-derived), not by node kind. Timestamps render bare (no `last` / `POSITION` prefix); the line context makes the meaning unambiguous. The `last fix` framing on no-fix rows remains because it scopes the lat/lon to a known past position, not a current sentinel.
  - Fresh: `{glyph} {label}  ·  {age}` — glyph + label colour from inventory.kind (tag `●` TEXT_BRIGHT / relay `✚` ORANGE / gateway `■` GREEN), state-bullet colour from `freshness_color(Fresh)` per [`tools/sarcom-kiosk-lab/src/map/markers.rs:40-48`](../tools/sarcom-kiosk-lab/src/map/markers.rs).
  - Aging: `{glyph} {label}  ·  {age}` — bullet colour from `freshness_color(Aging)`.
  - Stale: `{glyph} {label} · stale · {age}` — `TEXT_DIM`.
  - SOS: `🔴 SOS · {label} · {age}` — `RED`, bold. Lives in the sticky-alert section.
  - No-fix: `⚠ {label} · NO FIX · last fix {age}` — `AMBER`. Lives in the sticky-alert section.
  - Battery-low: append `🔋 BATT` token to the primary line (icon + suffix together); `AMBER` tint on the token only — both signals carry, the icon for glance, the text for unambiguous readout. Compatible with any state above.
  - **Gateway is a node-kind in the inventory, not a category-special row.** The gateway is local (it is the receiver), so its `NodeData.last_seen_secs` is sentinel-zero. **Sidebar formatting branch:** when `inventory.get(node_id) == Some(NodeKind::Gateway)`, the row renders as `■ {label}` with no age suffix. This is presentation (one-line branch on inventory.kind), not a data-model split. No `RTC ok` / `RTC unset` chrome — gateway-self status is deferred from v1a per `tickets/KIOSK-005-gateway-status-surface.md` and `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`.
- **Bump row minimum height to 48 px.** Update `Margin::symmetric(10, 5)` at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:102, 213, 270`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) (three frames) to deliver it.
- **Pin SOS-and-NoFix rows in a sticky alert section** above the scrollable list area. The mission-first sort logic at lines 57-84 is preserved as the fallback for the scrolling list (so non-alert rows stay sorted by hiker-priority).
- **Make relay and gateway rows selectable.** Both currently render a `Frame` without `.interact(Sense::click())` (relay at line 212-265, gateway at line 269-301). Add click handling that toggles selection consistent with hiker selection at lines 31-34.
- **Lift `selected_tag: Option<usize>` to a `Selection` enum.** Current type at [`tools/sarcom-kiosk-lab/src/app.rs:13`](../tools/sarcom-kiosk-lab/src/app.rs) only addresses hiker indices. **Per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`, Selection has one Node variant** (no per-kind split):
  ```rust
  enum Selection {
      None,
      Node(usize),  // index into sim.nodes (Vec<NodeData>)
  }
  ```
  Selecting a node is selecting a node, regardless of kind. The detail surface (KIOSK-004) reads the selected `NodeData` and the inventory map looks up the kind for icon/colour assignment. **Earlier drafts of this ticket proposed `Tag(usize) / Relay(usize) / Gateway` per-kind variants** — that draft itself re-instantiated the subtype-fetish in UI state and is retracted. See the dev-log entry above.

  Lift touches every callsite that owned the old `selected_tag: Option<usize>` field — `app.rs` (field + `new()` default + `switch_scenario` reset + `load_layout` reset), `ui/sidebar.rs` (row click handler + `is_sel` check), `map/mod.rs` (click-to-select handler + `draw_tags` call). Internal function signatures (e.g. `markers::draw_tags`) can stay positional-index based; `Selection::idx()` unwraps for the call site.
- **Multi-line per-row detail is removed from the sidebar entirely.** Full coordinates, last-valid-fix-age line, GPS_VALID=0 sentinels message, BATT LOW separate line, lat/lon, ui_kind — all move to KIOSK-004's detail panel.

## Non-goals

- **Not in scope:** detail-panel render — that is [`KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md) and depends on SPIKE-001's variant decision.
- **Not in scope:** swipe gestures or long-press actions on rows (gloves + risk; not v1a).
- **Not in scope:** new data fields on `NodeData`. Current shape (post-collapse per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`) suffices for the one-line rows.
- **Not in scope:** new visual palette entries — existing `RED` / `AMBER` / `ORANGE` / `GREY` / `GREEN` / `TEXT_DIM` / `TEXT_BRIGHT` at [`tools/sarcom-kiosk-lab/src/ui/palette.rs:8-19`](../tools/sarcom-kiosk-lab/src/ui/palette.rs) suffice.
- **Not in scope:** counters footer card (it is mentioned in [`tools/sarcom-kiosk-lab/README.md:56`](../tools/sarcom-kiosk-lab/README.md) but I did not see it implemented in the current `sidebar.rs`; the README is stale on multiple points per KIOSK-007).

## Acceptance criteria

1. Each scenario in `ScenarioKind::all()` ([`tools/sarcom-kiosk-lab/src/data.rs:75-85`](../tools/sarcom-kiosk-lab/src/data.rs)) renders rows that match the one-line-per-row table above.
2. Row height (any row, any scenario) is ≥48 px when measured against egui's row-rect.
3. In the `MultiTag` scenario, both the SOS tag (tag-2) and the no-fix tag (tag-4) appear in the sticky-alert section at the top of the sidebar. Scrolling the rest of the list does not displace them.
4. Tapping any row (hiker / relay / gateway) toggles selection to `Selection::Node(i)` where `i` is the index of that node in `sim.nodes`. Re-tapping the selected row clears selection back to `Selection::None` (consistent with current behaviour at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:31-34`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs)).
5. Selection state survives the existing scenario-switch reset at [`tools/sarcom-kiosk-lab/src/app.rs:102`](../tools/sarcom-kiosk-lab/src/app.rs) clearing to `Selection::None` — same as today's `self.selected_tag = None`.
6. Visual selection treatment is a **full-row background tint** (extending the existing background tint at [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:93-97`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) to the row's full width), applied to hiker, relay, and gateway rows. The same full-row tint treatment is also acceptable for sticky-alert / distress rows where it improves at-a-glance prominence.
7. Existing map-side selection rendering (selected-node white outline at [`tools/sarcom-kiosk-lab/src/map/markers.rs:231-237`](../tools/sarcom-kiosk-lab/src/map/markers.rs)) works for any `Selection::Node(_)`, regardless of inventory.kind. Map-side recenter on the selected node is KIOSK-004's surface.

## Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Cycle through each scenario via the header combobox ([`tools/sarcom-kiosk-lab/src/ui/header.rs:24-39`](../tools/sarcom-kiosk-lab/src/ui/header.rs)). For each, verify the row content matches the one-line table.
3. Switch to `MultiTag`. Confirm tag-2 (SOS) and tag-4 (no-fix) are pinned at top in the sticky-alert section.
4. Tap each row type: hiker, relay, gateway. Visual selection treatment appears on each.
5. Tap a row, then switch scenarios; selection clears (existing `switch_scenario` behaviour).
6. `cargo test --manifest-path tools\sarcom-kiosk-lab\Cargo.toml` — existing tests pass (especially `sos_no_fix_scenario_has_ghost_data` in `tools/sarcom-kiosk-lab/src/data.rs`).

## Likely files / modules touched

- [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) — primary rewrite of `render_hiker_row` (lines 86-203), `render_relay_row` (205-266), `render_gateway_row` (268-302); add sticky-alert section render; per-row click handling for relay/gateway
- [`tools/sarcom-kiosk-lab/src/app.rs`](../tools/sarcom-kiosk-lab/src/app.rs):
  - line 13: `selected_tag: Option<usize>` → `selection: Selection`
  - line 81: default in `new()`
  - line 102: reset in `switch_scenario`
  - line 138: reset in `load_layout`
- [`tools/sarcom-kiosk-lab/src/map/markers.rs`](../tools/sarcom-kiosk-lab/src/map/markers.rs):
  - `draw_tags(painter, sim, selected_node: Option<usize>, t, view)` — call site at `src/map/mod.rs` passes `self.selection.idx()` (the post-lift `Selection` enum's unwrap helper). The function signature stays positional-index-based; the named `Selection` enum lives at app-state level.

## Risks / open questions

### Three-question check — `Selection` enum

Per [CLAUDE.md "Schema-extension discipline — the subtyping-fetish check"](../CLAUDE.md), run the three questions explicitly before adding a new typed variant. **An earlier draft of this section concluded that a per-kind enum (`Tag(usize) / Relay(usize) / Gateway`) was correct.** That conclusion was wrong — it perpetuated subtype-fetish into UI state. See `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md` for the correction.

Rerun:

1. **Operator-visible?**
   Selection is operator-visible — it highlights, recentres, and triggers the detail surface. But **the kind-distinction** (tag vs relay vs gateway) is operator-visible only for **icon glyph + colour**, not for any UI behaviour that branches on kind. Icon/colour comes from inventory lookup, not from the Selection value.
2. **Existing attribute pattern?**
   Yes. Per the post-collapse data model, all nodes live in `sim.nodes: Vec<NodeData>` indexed positionally — same shape as the prior `sim.tags: Vec<TagData>`. Selection should index the same vec.
3. **Blast radius?**
   Single-variant `Selection::Node(usize)` is mechanically equivalent to the old `Selection::Tag(usize)` plus inventory dispatch at render time. Five callsites; each is a one-line change. The compiler catches every miss.

Final enum shape:

```rust
enum Selection {
    None,
    Node(usize),
}
```

Where the kind of the selected node is `sim.inventory[&sim.nodes[i].node_id]`.

### Other risks / open questions

- **`Selection::Node(usize)` indexes `sim.nodes`** (single Vec covering tags, relays, gateway). The v1b multi-relay scenario per CLAUDE.md adds new `NodeData` entries to the same vec; no enum widening needed. Drone-relays per [`spikes/airborne-positioning-overlay-spike.md`](../spikes/airborne-positioning-overlay-spike.md) are likewise just another `NodeKind` value in the inventory.
- **Sticky-alert section label.** "ALERTS"? "ACTIVE DISTRESS"? Pick at the per-ticket mockup review (see the per-ticket mockup plan in [`tickets/README.md`](README.md)).
- **48 px rows × 7 nodes on `MultiTag` + 2 in sticky section** = potentially overflowing on 800×480 minus header strip. Verify in the lab. If overflow, the scrollable list is the spill area and that's fine; sticky section stays pinned.
- **Battery-low display.** Decided: small battery icon **plus** suffix text together (see Scope). Both signals carry without forcing a separate row.
- **Counter footer card.** Decided: **no counter footer card for v1a.** The kiosk-lab README at [`tools/sarcom-kiosk-lab/README.md:56`](../tools/sarcom-kiosk-lab/README.md) describes a `▼ NODES (n)` / `▼ NO FIX (n)` / counters-footer arrangement that does not exist in the current `sidebar.rs`. [`KIOSK-007-doc-cleanup.md`](KIOSK-007-doc-cleanup.md) removes the stale README claim; this ticket does not add the footer.

## Confidence

**High.** Pure UI rewrite over an understood, fully-cited data model. Selection-enum lift is mechanical.

## Dependencies

- None for this ticket directly.
- **Blocks** [`KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md), which consumes the new `Selection` enum.
