---
title: "v1a UI data model collapses TagData/RelayData/GatewayData → NodeData + inventory map"
date: 2026-05-19
status: decided
type: dev-log
---

# v1a UI data model collapses TagData / RelayData / GatewayData → `NodeData` + inventory

## Decision

The kiosk-lab UI data model collapses the three subtype-split node structs (`TagData`, `RelayData`, `GatewayData` at `tools/sarcom-kiosk-lab/src/data.rs:127-164`) into a single uniform `NodeData` struct that carries the POSITION-derived fields available for any node in the network. Node-kind distinction (tag / relay / gateway) is reduced to an **inventory mapping** held alongside the node list: `HashMap<u8, NodeKind>` (gateway-internal table, modelled in v1a sim, will be SQLite-backed in the gateway binary).

`SimState` shape:

```rust
pub struct SimState {
    pub nodes: Vec<NodeData>,        // every node, regardless of kind
    pub inventory: HashMap<u8, NodeKind>,
}

pub enum NodeKind { Tag, Relay, Gateway }

pub struct NodeData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
    pub state: NodeState,
    pub last_seen_secs: f32,
    pub gps_valid: bool,
    pub sos: bool,
    pub battery_low: bool,
    pub track: Vec<[f32; 2]>,
    pub sightings: Vec<SightingEntry>,
    pub last_valid_fix_pos: Option<[f32; 2]>,
    pub last_valid_fix_age_secs: Option<f32>,
}
```

The UI never branches on `NodeKind` for **field selection**. It branches on inventory lookup **only** for icon glyph + colour assignment. Selection state is a single `Selection::Node(usize)` (index into `sim.nodes`) — no per-kind variants. Detail surface is one layout for any selected node, populated from `NodeData`.

The one explicit loophole Pieter named: the gateway itself, in a hypothetical gateway-self detail view, MAY surface fields it knows locally (DS3231 RTC, on-board battery sensor) that no POSITION packet carries. **v1a does not implement this** — the gateway appears in the kiosk as just another node, with `NodeData.last_seen_secs = 0.0` (always-now, because it is the receiver) and no extra chrome.

## Why this rationale

Pieter, 2026-05-19, after spotting that `RelayData.self_ann_age_secs` reified a "self-announce frame" that doesn't exist on the wire (see `dev-log/2026-05-19-self-ann-subtyping-fetish-third-instance.md`), traced the same failure mode one layer deeper: not just the field name, but the struct split itself.

Verbatim:

> "ik besef me net dat we in de UI de subtype fetish (my bad) ook laten doortrekken want in de v1 is er geen netto plus aan speciale detail views te maken voor relay tag en gw. alles is hetzelfde, alles gewoon een node in het netwerk dat position packets kan broadcasten als het tag firmware draait en position packets repeatet als het relay firmware draait. thats it. de ui mag dat onderscheid relay en tag nie maken vanuit het datamodel perspectief. dat is hetzelfde. het enige wat de gateway gaat doen is een inventaris tabel bijhouden van node-ids -> tag of relay of gateway mapping, maar dat soort onderscheid laten doorpropageren naar de detail views en het onderliggende data model is ook fout."

> "ik wil dat de protocol truth gereflecteerd wordt in het data model van de ui, punt aan de lijn, geen uitzondering, het enige onderscheid dat we maken is via de inventaris sqlite tabel om icoontjes en kleurtjes toe te wijzen aan elke subtype."

The principle, locked: **the UI data model mirrors the protocol primitive (POSITION)**. Tag firmware broadcasts POSITION. Relay firmware broadcasts POSITION + forwards others' POSITION. Gateway receives. The network has one packet type per ADR-013; the UI data has one node shape per this entry.

## What changes

**Code (`tools/sarcom-kiosk-lab/`):**

- `src/data.rs` — delete `TagData`, `RelayData`, `GatewayData`. Add `NodeData`, `NodeKind`. `SimState` carries `Vec<NodeData>` + `HashMap<u8, NodeKind>`. Scenario builders (`normal`, `sos`, `stale`, `no_fix`, `multi_tag`, `sos_no_fix`) updated to populate both.
- `src/ui/sidebar.rs` — single `render_node_row` replaces `render_hiker_row` + `render_relay_row` + `render_gateway_row`. Icon glyph + colour selected from inventory lookup. Gateway-specific suffix elision (no `last_seen` rendering when `node_id` maps to `Gateway` since the gateway is local) is a one-line presentation branch, not a data branch.
- `src/map/markers.rs` — `draw_tags` / `draw_relay` / `draw_gateway` collapse to one `draw_nodes` iterating `Vec<NodeData>`, dispatching icon glyph from inventory.
- `src/map/pmtiles_map.rs`, `src/map/mod.rs`, `src/ui/edit_panel.rs`, `src/app.rs`, `src/ui/header.rs` — update call sites.
- `src/app.rs` — `Selection` enum becomes `{ None, Node(usize) }` (one variant — the prior split into `Tag(_) / Relay(_) / Gateway` was itself a subtype-fetish artifact).
- `Cargo.lock` — only if HashMap requires a new import elsewhere.

**Tickets:**

- `tickets/KIOSK-003-sidebar-row-redesign.md` — state-row table reorganised by node-state (fresh / aging / stale / SOS / no-fix / battery-low), not by node-kind. Selection enum collapsed. Three-question check resolution updated.
- `tickets/KIOSK-004-selection-detail-panel.md` — drop per-kind subsections (Tag / Relay / Gateway); one detail layout for any selected node.
- `tickets/KIOSK-005-gateway-status-surface.md` — deferred from v1a (gateway-self status is the loophole that v1a does not implement).

**Mockups (`UX/mockups/`):**

- `KIOSK-003-sidebar-row-redesign.svg` + `.md` — state-row table collapsed.
- `KIOSK-004-selection-detail-sidebar.svg` + `.md` — three panels → one panel design + concise note that the same layout serves tag / relay / gateway with icon/colour from inventory.
- `KIOSK-005-gateway-status.svg` + `.md` — converted to deferred stub (same shape as `KIOSK-002-mockup-prompt.md`).
- `KIOSK-001` / `KIOSK-006` / `KIOSK-008` — sidebar rows already RTC-stripped (earlier this session); confirm uniform format.

**Memory:**

- `memory/feedback_no_subtyping_fetish.md` — extend mental-model lock: "UI data model mirrors POSITION primitive; inventory is the only kind-distinction." Already updated for the field-name instance; extending for the struct-collapse decision.

## What stays the same

- **ADR-013** — already says this. The wire protocol has one packet type. This entry doesn't change the protocol; it removes the UI layer's pretending otherwise.
- **`tickets/SPIKE-001`** — already closed strict; the per-ticket mockups already single-design. This entry compounds (collapsing further within the single-design posture).
- **`tickets/SPIKE-002`** — no-fix uncertainty disc reject; unaffected.
- **`tickets/KIOSK-002`** — deferred from v1a; unaffected.
- **`tickets/KIOSK-007`** — doc cleanup; this entry is a SOURCE of further README cleanup.
- **`tickets/KIOSK-008`** — track + selection polyline; the rendering logic stays the same but now keys on `NodeData.track` for any selected node (relays and gateway typically have empty tracks; the polyline renders if and only if `>=2` points exist).
- **`decisions/ADR-007`** — read-only UI; this entry stays inside that. No new write surfaces.
- **`dev-log/2026-05-19-gateway-rx-timestamps-as-position-field.md`** — gateway-RX-fills-timestamps decision unchanged. POSITION timestamps still fill at gateway RX; that's where every node's `last_seen_secs` is derived.

## Cross-references

- `decisions/ADR-013-multi-hop-flood-via-packet-id.md` — one packet type (POSITION); no wire-level role enum.
- `decisions/ADR-007-touchscreen-primary-ui.md` — read-only UI invariant.
- `dev-log/2026-05-19-self-ann-subtyping-fetish-third-instance.md` — the third instance that triggered this deeper trace.
- `dev-log/2026-05-16-osm-overlay-collapse-subtypes.md` — second instance, structural subtyping.
- `memory/feedback_no_subtyping_fetish.md` — three-instance catalogue + mental-model lock.
