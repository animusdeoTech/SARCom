# KIOSK-005 — Gateway status surface — DEFERRED FROM v1a (no mockup)

## Status: deferred — no mockup produced

This mockup is **deferred from v1a along with its parent ticket** per
the v1a UI data-model collapse decision recorded in
[`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md).

The original mockup proposed rendering gateway-self status (battery
percentage, charging state, RTC status, `ui` render-tick liveness) in
the sidebar `gw-0` row. Per the post-collapse principle:

- The v1a UI data model is one uniform `NodeData` derived from
  POSITION packets that any node broadcasts.
- The kind-distinction (tag / relay / gateway) is reduced to an
  inventory lookup (`HashMap<u8, NodeKind>`) used for icon glyph +
  colour only.
- The gateway-self status surface (battery / charging / RTC /
  render-tick) is **local knowledge** the gateway has about itself,
  not POSITION-derived. It is the one explicit loophole Pieter named
  ("het enige 'extra' dat KAN komen in de detail view is dat van
  uzelf als ge een gateway zijt"), but it is **NOT v1a**.

## Source-of-truth for the deferral

- [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md) — architecture decision driving this deferral.
- [`tickets/KIOSK-005-gateway-status-surface.md`](../../tickets/KIOSK-005-gateway-status-surface.md) — parent ticket, now a deferred-from-v1a stub.
- [`tickets/KIOSK-003-sidebar-row-redesign.md`](../../tickets/KIOSK-003-sidebar-row-redesign.md) — sidebar gateway row in v1a renders as `● {label}` with no status suffix.
- [`tickets/KIOSK-004-selection-detail-panel.md`](../../tickets/KIOSK-004-selection-detail-panel.md) — detail surface in v1a is one uniform layout; no gateway-specific battery / charging / RTC fields.

## What v1a does instead

- The gateway appears in the kiosk sidebar and detail surface as just another node (same uniform layout per KIOSK-003 / KIOSK-004).
- `NodeData.last_seen_secs` is sentinel-zero for the gateway (gateway is the receiver; no POSITION packets received from itself). The sidebar renders the row as `● {label}` (no age suffix) when `inventory.kind == NodeKind::Gateway`.
- No `RTC ok` / `RTC unset` / battery indicator / `ui` render-tick dot / `BAT unknown` chrome anywhere.

## What would unblock a future KIOSK-005 (out of scope here)

A future ticket — e.g. "GW-001: gateway-self detail surface" — would add the gateway's local self-knowledge to its own detail view. That ticket would:

- Define a `GatewayLocalState` struct (or extension to `NodeData` gated on `inventory.kind == Gateway`) carrying battery, charging, RTC freshness, render-tick liveness.
- Specify the rendering branch in the detail surface: when the selected node is `inventory.kind == Gateway`, render the uniform layout + the gateway-local extras.
- Honour the honest-unknowns discipline (`BAT unknown` when the battery sensor is not yet implemented; `ui` dot pulsing when the render loop ticks, not when the LoRa RX task is healthy).

None of that is v1a's job.

## What this entry does NOT change

- ADR-007 (read-only UI) — unaffected.
- ADR-013 (one packet type) — already says gateway-self is not in POSITION; this deferral aligns with it.
- The existing v1a per-ticket mockups under `UX/mockups/` (KIOSK-001/003/004/006/008 + umbrella) — already RTC-stripped earlier in this session.
- v2+ deferred lanes (coverage telemetry, no-fix uncertainty disc, GPS-less localisation, drone overlay) — unaffected.
