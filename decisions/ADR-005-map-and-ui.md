---
title: "ADR-005: Map & UI — native Rust, no web"
status: accepted
date: 2026-04-22
supersedes: "ADR-005 (2026-04-19): MapLibre GL JS"
type: adr
tags: [decision, ui, map, kiosk, rust]
---

# ADR-005: Map & UI — native Rust kiosk, no browser

**Status:** Accepted
**Date:** 2026-04-22
**Supersedes:** the 2026-04-19 "MapLibre GL JS" decision.

## Context

The kiosk at the mountain hut renders a live map showing tags and relays. Hard constraints:

- Must work with zero internet — the hut scenario is "snowstorm hits, WiFi is gone, relays on poles are still running, map still shows whoever the gateway can currently hear"
- Must boot fast and recover cleanly — this is an appliance
- Must render smoothly on a 7" DSI touchscreen (1024 × 600 typical), with touch pan/pinch zoom
- Must stay consistent with the Rust-everywhere stance of [ADR-001](ADR-001-firmware-language.md)
- Map tiles pre-bundled on the Pi (offline) — no runtime tile downloads

## Decision

**Native Rust GUI. No browser, no web stack.**

Starting bet: **`egui` + [`walkers`](https://github.com/podusowski/walkers)** (slippy-map widget for egui). Rationale: `egui` is the most mature immediate-mode GUI in Rust, runs on `winit` + `wgpu`, compiles fast, and `walkers` already supports local offline tile sources with touch pan/zoom.

Final library pick is allowed to shift after a small prototype — `iced + custom canvas` and `slint + custom widget` remain acceptable fallbacks if `egui`/`walkers` doesn't hold up under load or on Pi graphics drivers. The non-negotiable part is "native, not a browser."

**Offline tiles: PMTiles.** `walkers` documents native support for `.pmtiles` as its local/offline tile source — a single mmap-friendly file, no background tile server, no SQLite tile reader to write. Generate one PMTiles archive of the relevant area once (e.g. `planetiler` → `.pmtiles`, or convert an existing `.mbtiles` via `pmtiles convert`) and bundle it into the Yocto image. Updating tiles means rebuilding the image. A small spike at kiosk bring-up must verify that `walkers` can render the chosen PMTiles file on the Pi's GPU before any other kiosk work lands; if that spike fails, fall back to a custom `TilesManager` that reads from MBTiles (SQLite) — but the default path and the one the BOM/image recipe target is PMTiles.

## Consequences

- **No Chromium, no React, no TypeScript, no Tauri-wrapping-HTML, no npm, no build step outside Cargo.**
- **No MapLibre, no Leaflet, no OpenLayers.** All are web-first and would require running a browser engine.
- **PMTiles pipeline:** one-time generation step during Yocto image build. Documented in [../operations/](../operations/) (planned; currently lives in this ADR until the folder is split out).
- **Phone-friendly access is deferred but not closed off.** If a future v2 wants "operator pulls out phone, connects to gateway's WiFi AP, sees a read-only map," the cleanest path is a small Rust HTTP server (`axum`) that serves a static snapshot over the LAN. The data layer (SQLite) is already shaped for that — we just don't build the HTTP surface in v1.
- **No auth, no login, no settings screen** — see [ADR-007](ADR-007-touchscreen-primary-ui.md).
- **Tradeoff acknowledged:** no pre-built cartography, no free drop-in web mapping stack. The offline-first constraint is the hard requirement and it forces this shape.

## Alternatives considered

- **MapLibre GL JS in a Chromium kiosk** (the 2026-04-19 decision). Rejected: requires Chromium on a Pi for one appliance use case, violates Rust-everywhere, and the offline vector-tile story needs a local tile server anyway — more moving parts.
- **Leaflet.** Simpler than MapLibre, same browser-dependency objection.
- **Desktop GIS (QGIS).** Vastly overpowered; wrong shape for an appliance.
- **Terminal UI with text map.** Rejected on taste; staff think spatially, a rendered map is the correct representation.
