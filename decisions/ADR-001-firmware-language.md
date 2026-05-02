---
title: "ADR-001: Rust everywhere"
status: accepted
date: 2026-04-22
type: adr
tags: [decision, language, rust, firmware, gateway, ui]
---

# ADR-001: Rust everywhere — firmware, gateway, and kiosk UI

**Status:** Accepted
**Date:** 2026-04-22

## Context

This project has four software surfaces:

1. Tag firmware (ESP32-S3, bare-metal with async runtime)
2. Relay firmware (ESP32-S3, bare-metal with async runtime)
3. Gateway receiver + persistence (Linux on RPi, SPI to SX1276)
4. Kiosk UI (Linux on RPi, 7" DSI touchscreen — live read-only map)

Requirements:

- Shared wire protocol between tag, relay, and gateway (one source of truth for `PacketType`, `PositionPayload`, CRC, sentinels)
- Single language across the stack — fewer context switches, no duplicated types, fewer "which side is wrong?" debug sessions
- Quality over time-to-first-blink — this is a learning and portfolio project, not a ship-by-Friday product
- Runs offline by default — no cloud toolchains, no npm, no web bundles
- Native performance on the Pi kiosk (not a browser engine)

## Decision

**Rust everywhere. No C, no C++, no Python, no TypeScript/JavaScript on device.**

- Tag + Relay firmware: Rust + [`esp-hal`](https://github.com/esp-rs/esp-hal) + [Embassy](https://embassy.dev/) + [`lora-phy`](https://github.com/lora-rs/lora-rs) (from the `lora-rs/lora-rs` repo — the older `embassy-rs/lora-phy` is archived and redirects there), `no_std`
- Gateway receiver: Rust + [`linux-embedded-hal`](https://github.com/rust-embedded/linux-embedded-hal) (SPI to SX1276 on the Dragino HAT) + the same [`lora-phy`](https://github.com/lora-rs/lora-rs) crate (it supports SX126x *and* SX127x; no separate "gateway radio driver" to write) + `tokio` async runtime
- Kiosk UI: Rust native (see [ADR-005](ADR-005-map-and-ui.md)) — no Chromium, no React, no Tauri-wrapping-HTML
- Shared `protocol` crate: `no_std` with an optional `std` feature for host-side test tooling; used by tag, relay, gateway receiver, and kiosk UI
- Shared `persistence` crate: `std`-only, owns the SQLite schema and queries (see [ADR-009](ADR-009-database-sqlite.md)); used by the gateway binary

Python is permitted for *host-side* scripts only — simulation, test vector generation, log analysis. Never on device.

## Consequences

- Single Cargo workspace holds every binary. Types live in one crate, used by all four surfaces.
- Nightly Rust is required for the `xtensa-esp32s3-none-elf` target (Xtensa backend is not upstream in stable rustc). Pin a specific nightly snapshot for reproducibility.
- Wireless Tracker V2 is not yet upstream in `esp-hal`. We will write and publish a `heltec-wireless-tracker-v2-bsp` crate. This is the planned portfolio contribution.
- Cross-compilation: developer laptop compiles ESP32-S3 binaries via `espup`-managed toolchain, and Pi binaries via `aarch64-unknown-linux-gnu` (Pi 4) or `armv7-unknown-linux-gnueabihf` (Pi 3B+).
- The kiosk UI is not in a browser. MapLibre/Leaflet/OpenLayers are off the table. See [ADR-005](ADR-005-map-and-ui.md).
- No JavaScript runtime on the Pi. No Node. No npm.
- BSP work and Yocto integration both have upfront costs. We accept them because the long-term story (one language, one toolchain, reproducible image) is worth it for an appliance.

## Alternatives considered

- **C++/Arduino on MCU, Python/FastAPI on Pi, React on frontend.** The default in this space. Rejected: three languages, no shared wire-type safety, 300 MB of Chromium on a Pi to render five markers.
- **Zephyr + C (or Rust) on MCU.** See [../archive/zephyrOS_study.md](../archive/zephyrOS_study.md). Real research; conclusion was that Zephyr's tooling (west, DT, Kconfig, sysbuild) doesn't pay off for a two-binary project on one SoC family.
- **ESP-IDF + C on MCU.** Mature tooling, but gives up type safety across the wire and the shared-crate story collapses.
- **Rust on MCU, Go on gateway, Svelte on UI.** Technically fine, operationally annoying. Shared-types argument wins.
