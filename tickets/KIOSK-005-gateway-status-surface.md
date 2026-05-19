---
id: KIOSK-005
title: "Gateway status surface — DEFERRED FROM v1a"
status: deferred-from-v1a
type: implementation-ticket
opened: 2026-05-18
deferred: 2026-05-19
---

# KIOSK-005 — Gateway status surface — DEFERRED FROM v1a

## Status: deferred — no v1a implementation

This ticket is **deferred from v1a** per the v1a UI data-model collapse decision recorded in [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md).

## Why it was deferred

The collapse decision (2026-05-19) holds that the v1a UI data model mirrors the protocol primitive: every node is one uniform `NodeData` derived from POSITION packets, with kind-distinction reduced to an inventory lookup (`HashMap<u8, NodeKind>`) for icon glyph + colour. The detail surface (KIOSK-004) is one layout for any selected node.

Gateway-self status (battery percentage, charging state, RTC freshness, UI render-tick liveness) is **knowledge the gateway has about itself locally**, not knowledge derived from POSITION packets the network surfaces. Pieter's locked principle, verbatim 2026-05-19:

> "het enige 'extra' dat KAN komen in de detail view is dat van uzelf als ge een gateway zijt, meer niet, en dat doen we nu momenteel ook NIET."

The door is left open ("KAN komen") for a future ticket to add a gateway-self detail surface that renders battery / RTC / process liveness. **v1a does not do this.** In v1a, the gateway appears in the sidebar and detail surface as just another node, with `NodeData.last_seen_secs = 0` (always-now, because it is the receiver) and no extra chrome.

## Source-of-truth for the deferral

- [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md) — the architecture decision driving this deferral.
- [`decisions/ADR-013-multi-hop-flood-via-packet-id.md`](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) — one packet type (POSITION); no per-kind wire distinction.
- [`tickets/KIOSK-003-sidebar-row-redesign.md`](KIOSK-003-sidebar-row-redesign.md) — sidebar row format uniform across node kinds; gateway row renders as `● {label}` with no `last_seen` suffix (gateway is local).
- [`tickets/KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md) — single detail layout for any selected node; gateway detail does not surface battery / RTC / charging in v1a.

## What v1a does instead

- Sidebar: gateway row appears with kind-icon (`■` green) + label, no status suffix. Same row format as every other node, with `last_seen` elision branching on `inventory.kind == Gateway` (presentation-layer one-line branch, not a data-model split). See KIOSK-003 scope.
- Detail surface: gateway header + `last frame · — (local)` row + lat/lon. No battery, no charging, no RTC, no render-tick liveness. See KIOSK-004 scope.
- `tools/sarcom-kiosk-lab/src/data.rs` `NodeData` carries no `battery_pct` / `charging` / `rtc_valid` fields for gateway. Adding them is the future ticket's job, not v1a's.

## What does NOT carry forward from the original draft

The original draft of this ticket (deferred 2026-05-19) proposed:
- Extending the data model with `GatewayData.battery_pct` and `GatewayData.charging`.
- Rendering a battery indicator in the sidebar gateway row.
- A `ui` render-tick liveness dot.
- An honest-unknowns discipline for unknown battery + render-tick state.

All four are out of v1a scope. They remain valid design ideas for a future gateway-self detail surface ticket. None of them block v1a.

## What this entry does NOT change

- ADR-007 (read-only UI) — unaffected.
- ADR-013 (one packet type) — already says gateway-self status is not in POSITION; this deferral aligns with it.
- KIOSK-001 / KIOSK-003 / KIOSK-004 / KIOSK-006 / KIOSK-008 v1a scope — the only thing affected is that `RTC ok` chrome and similar gateway-self indicators are out (already done in this session).
- v2+ deferred lanes — coverage telemetry (ADR-013 §10), no-fix uncertainty disc (SPIKE-002 reject), GPS-less localisation, drone overlay — unaffected.

## Future work, not v1a

A future ticket — e.g. "GW-001: gateway-self detail surface" or similar — can revisit gateway battery / RTC / process liveness as a dedicated detail surface that the gateway opens for **itself only** (not in the same uniform-detail-for-any-node flow). When that ticket is opened, this file can be left as a deferral record or be retitled. Until then this file is the deferred-from-v1a stub.
