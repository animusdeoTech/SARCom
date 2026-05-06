---
title: "ADR-004: Gateway platform — RPi + Dragino HAT + Yocto"
status: accepted
date: 2026-04-22
type: adr
tags: [decision, gateway, rpi, dragino, yocto, kiosk]
---

# ADR-004: Gateway platform — Raspberry Pi + Dragino HAT + Yocto

**Status:** Accepted
**Date:** 2026-04-22

## Context

In this project (per [ADR-008](ADR-008-no-cloud-no-downlink.md) and [ADR-007](ADR-007-touchscreen-primary-ui.md)), the gateway is **also** the operator workstation. It is not a transit node between the field and some cloud. It must:

- Receive LoRa packets via SX1276 (Dragino LoRa/GPS HAT, SPI)
- Persist to local SQLite ([ADR-009](ADR-009-database-sqlite.md))
- Render a live read-only map on a 7" DSI touchscreen ([ADR-005](ADR-005-map-and-ui.md))
- Boot cleanly, auto-launch the kiosk UI, survive power cycles with no human attention
- Have zero cloud dependency

Hardware on hand (see [../hardware/desk-inventory.md](../hardware/desk-inventory.md) — planned; for now see this ADR and [ARCHITECTURE.md](../ARCHITECTURE.md)): 3× Raspberry Pi (~6 years old, likely Pi 3B+ or early Pi 4), 3× Dragino LoRa/GPS HAT (SX1276), 1× Raspberry Pi 7" DSI touchscreen.

## Decision

- **Hardware:** Raspberry Pi (3B+ or 4, whichever has healthy ports and a working SD slot) + Dragino LoRa/GPS HAT (SX1276) + Raspberry Pi 7" DSI touchscreen
- **OS: Yocto** (custom image), from day one — no Raspbian / Raspberry Pi OS / Debian intermediate step
- **Software:** one Rust binary doing LoRa RX + DB writes + kiosk UI

## Consequences

- **Yocto upfront cost is paid now, once.** Payoff: a stripped, deterministic, single-purpose image; the kernel and rootfs pinned by commit; reproducible rebuilds. For an appliance that might run in a hut for years, this is the right baseline, and doing bring-up on Raspbian then re-porting to Yocto would mean paying the integration tax twice.
- **Target triple:** `aarch64-unknown-linux-gnu` (Pi 4) or `armv7-unknown-linux-gnueabihf` (Pi 3B+). Cross-compile from the dev laptop.
- **Rust in Yocto** via `meta-rust` layer, Rust version pinned in the recipe.
- **Single binary.** One Tokio async runtime, one process. LoRa RX loop, SQLite writer, and kiosk UI all in-process. No IPC, no dbus gymnastics. See [../software/repo-layout.md](../software/repo-layout.md) (planned; for now see [ARCHITECTURE.md §17](../ARCHITECTURE.md)).
- **Systemd unit** `lora-sar.service`: `Restart=always`, autostarts on boot, runs as a non-root user with membership in the spi/gpio groups. The DSI touchscreen renders from first pixel.
- **Dragino pin numbering gotcha.** Dragino docs use WiringPi numbering, not BCM. Verify against the physical 40-pin header and the Rust GPIO crate (`rpi-pal`, the maintained fork of the archived `rppal` — same API, drop-in, Pi 5 compatible; swap recorded in `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`) before writing SPI/interrupt glue.
- **Bent pins on the in-hand HATs:** straighten with precision tweezers before seating; documented in [../hardware/gateway-assembly.md](../hardware/gateway-assembly.md) (planned; for now see this ADR and [TODO.md](../TODO.md)).
- **No HTTP server in v1.** The Pi exposes no network-facing endpoints. SSH is available for dev and disabled on production images.

## Alternatives considered

- **Raspbian for bring-up, Yocto later.** The 2026-04-19 stance. Rejected: two integration passes, and the Yocto-specific weirdness (device tree overlays for the DSI screen, SPI enable, user groups) is where most of the work is anyway — punting it doesn't avoid it.
- **Buildroot instead of Yocto.** Smaller, simpler, faster to first boot. Genuine alternative. Rejected on preference for Yocto's layer ecosystem and metadata model. Worth revisiting if Yocto turns into a blocker.
- **A laptop as gateway.** The hut scenario is "an appliance sits on a shelf." Laptop is the wrong shape.
- **Pi 5.** Not on hand; Pi 3B+/4 is sufficient. Revisit if thermals or RAM become an issue.
