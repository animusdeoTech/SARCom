---
title: "ADR-007: Touchscreen is the only UI"
status: accepted
date: 2026-04-22
type: adr
tags: [decision, ui, kiosk, touchscreen]
---

# ADR-007: The 7" touchscreen is the only UI

**Status:** Accepted
**Date:** 2026-04-22

## Context

Mountain hut staff are not software users. They are hut staff. Any UI they have to learn is an obstacle. The kiosk will be wall- or shelf-mounted, and someone will glance at it every 30 minutes at best.

Constraints:

- Staff do not log in, configure, sign up, accept cookies, or dismiss modals
- Staff do not run a web browser, type URLs, or deal with update prompts
- The UI must be understandable at 2 m in varying light
- Language: English, Dutch, German, French, Italian are all plausible — so visual-first, minimal text

## Decision

**The 7" DSI touchscreen is the only UI. It renders a live read-only map showing tags and relays. No other UI surfaces exist in v1.**

What the UI does:

- Shows each tag as a dot on the map, colour-coded by freshness
- Shows each relay as a pole icon at its commissioned position
- Indicates staleness visually (fresh = solid dot; stale = faded; very stale = greyed-out with a "last seen" timestamp)
- Flashes the marker red if SOS is active on a tag
- Supports touch pan and pinch zoom
- Auto-recovers from power cycles with no prompts

What the UI **does not** do:

- No login, no user accounts, no role management
- No modals, no dialogs, no "are you sure?" prompts
- No settings screen, no CRUD on tags or relays
- No remote dashboard, no web interface, no network-facing API — see [ADR-008](ADR-008-no-cloud-no-downlink.md)
- No data entry of any kind
- No alert acknowledgement flow

## Consequences

- **No auth system in v1.** Nothing to log into.
- **The kiosk is a single fullscreen Rust process.** Launched by systemd after the display is up. Restart-on-crash is instant.
- **Read-only data flow** at the application layer: LoRa RX thread writes to SQLite; UI thread reads from SQLite (WAL mode). No shared mutable state in app code beyond SQLite's ACID guarantees.
- **Operator-adjustable parameters** (region tiles, default zoom) live in a config file reached by plugging in a keyboard and running a CLI — not in any UI screen.
- **No network surface.** The kiosk binary exposes no HTTP endpoint, no WebSocket. Phone-friendly map view is v2+ (see [ADR-005](ADR-005-map-and-ui.md)).

## Alternatives considered

- **Web app on staff's phone or laptop.** Rejected: WiFi may not exist, staff may not own the device, URL typing is a friction point.
- **Rich multi-screen dashboard with login.** Rejected: solving a problem nobody has.
- **Text-only terminal.** Rejected: a map is genuinely the right representation here.
