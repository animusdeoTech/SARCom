---
title: "KIOSK-005 mockup prompt — DEFERRED FROM v1a"
status: deferred-from-v1a
type: mockup-prompt-stub
opened: 2026-05-18
deferred: 2026-05-19
superseded-by: KIOSK-003 (sidebar row) + KIOSK-004 (detail surface)
---

# KIOSK-005 mockup prompt — DEFERRED FROM v1a

## Status: deferred — no mockup produced

This mockup prompt is **deferred from v1a along with its parent ticket** per the v1a UI data-model collapse decision recorded in [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md).

The original prompt body specified rendering gateway-self status (battery percentage, charging state, RTC freshness, `ui` render-tick liveness, `BAT unknown` honest-unknowns chrome) in the sidebar `gw-0` row, plus an extended `GatewayData` struct with `battery_pct: Option<u8>` + `charging: Option<bool>`. All of that is **out of v1a scope**.

## Why it was deferred

The v1a UI data model is one uniform `NodeData` (post-collapse 2026-05-19). Kind-distinction (tag / relay / gateway) is reduced to an inventory map (`HashMap<u8, NodeKind>`) for icon glyph + colour only. Gateway-self status is **local knowledge the gateway has about itself**, not POSITION-derived. It's the one explicit loophole Pieter named ("het enige 'extra' dat KAN komen in de detail view is dat van uzelf als ge een gateway zijt"), but it is **NOT v1a**.

`GatewayData` no longer exists as a separate struct. The `battery_pct` / `charging` data-model extensions the original prompt cited are not in the post-collapse `NodeData`. The mockup-md target (`UX/mockups/KIOSK-005-gateway-status.md`) is a deferred stub matching this prompt.

## Source-of-truth for the deferral

- [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md) — architecture decision driving this deferral.
- [`tickets/KIOSK-005-gateway-status-surface.md`](../KIOSK-005-gateway-status-surface.md) — parent ticket, now a deferred-from-v1a stub.
- [`tickets/KIOSK-003-sidebar-row-redesign.md`](../KIOSK-003-sidebar-row-redesign.md) — sidebar gateway row in v1a renders as `● {label}` with no status suffix.
- [`tickets/KIOSK-004-selection-detail-panel.md`](../KIOSK-004-selection-detail-panel.md) — detail surface in v1a is one uniform layout; no gateway-specific battery / charging / RTC fields.
- [`UX/mockups/KIOSK-005-gateway-status.md`](../../UX/mockups/KIOSK-005-gateway-status.md) — the deferred mockup-md stub.

## Where v1a gateway rendering lives instead

- **Sidebar row** (KIOSK-003): the gateway row appears as `● gw-0` with the inventory-assigned green-square icon, no status suffix. The same row template renders every node; gateway only differs in the elision of the `last_seen` line (the gateway is local; `last_seen_secs = 0` sentinel).
- **Detail surface** (KIOSK-004): when the gateway row is selected, the same uniform detail layout renders — `last frame · — (local)` reflects the local-receiver case, no battery / charging / RTC chrome.

## When to re-open

A future ticket — e.g. "GW-001: gateway-self detail surface" — would add the gateway's local self-knowledge (battery / charging / RTC freshness / render-tick liveness) to its own detail view. That ticket would:

- Define a `GatewayLocalState` struct (or extension to `NodeData` gated on `inventory.kind == Gateway`) carrying the local-only fields.
- Specify the rendering branch in the detail surface for `inventory.kind == Gateway`.
- Honour the honest-unknowns discipline (`BAT unknown` when the battery sensor isn't yet implemented; `ui` dot pulsing when the render loop ticks, not when the LoRa RX task is healthy).

None of that is v1a's job.

## What this stub does NOT change

- The deferred mockup-md at [`UX/mockups/KIOSK-005-gateway-status.md`](../../UX/mockups/KIOSK-005-gateway-status.md) is the canonical landing for this prompt and matches this stub.
- No SVG is produced. No KIOSK-005 mockup output is emitted by the orchestrator at [`tickets/mockup-prompts/00-RUN-ALL.md`](00-RUN-ALL.md) — the orchestrator skips this prompt the same way it skips KIOSK-002.

## Idempotency

The orchestrator at `00-RUN-ALL.md` should:
- **NOT** process this prompt (skip-line, same shape as KIOSK-002).
- **NOT** include `UX/mockups/KIOSK-005-gateway-status.svg` or `.md` in the allowed-writes whitelist.
- **NOT** include KIOSK-005 in the per-ticket reminders.

The mockup-md stub at `UX/mockups/KIOSK-005-gateway-status.md` is the post-deferral artifact; do not regenerate it from this prompt.
