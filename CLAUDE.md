---
title: "CLAUDE.md — LLM onboarding"
status: living
type: meta
tags: [onboarding, meta, claude]
---

# CLAUDE.md

Onboarding note for any Claude instance entering this project — chat, Claude Code CLI, Obsidian vault plugin, Cowork, whatever. Read this first.

## What this project is

LoRa-based Search & Rescue telemetry network. GPS tags → solar LoRa relays → Pi gateway with a 7" touchscreen showing a live read-only map. For mountain hut staff. **Fully offline. No cloud. No phone app. No downlink. Pure uplink.** If connectivity exists, the system ignores it; if it doesn't, nothing changes.

For the full picture see [ARCHITECTURE.md](ARCHITECTURE.md).

## Do NOT re-open these decisions without explicit reason

These are **Accepted** in `decisions/`, dated 2026-04-22 (ADR-001 through ADR-009), 2026-04-24 (ADR-010, ADR-011), 2026-04-25 (ADR-012), and 2026-04-26 (ADR-013, ADR-014). ADR-012 is **superseded in part** by ADR-013/014 — its v1a/v1b split, tag buzzer, and non-goals list survive; its role enum, SIGHTING, RELAY_INFO and three-table schema do not. Arguing accepted decisions again wastes time. If context genuinely changed, write a new ADR that supersedes the old one; do not re-litigate inline.

- **Firmware / gateway / UI language: Rust, end to end.** Tag firmware, relay firmware, gateway receiver, kiosk UI — all Rust. No C, no C++, no Zephyr, no Python runtime, no JavaScript, no TypeScript, no React, no npm in production SARCOM runtime code. Archived or local UX exploration tools (under `tools/`) may use npm, but do not introduce new npm tooling unless explicitly requested. See [ADR-001](decisions/ADR-001-firmware-language.md).
- **Tag hardware:** Heltec Wireless Tracker V2 (ESP32-S3 + SX1262 + UC6580 GNSS). Not the WiFi LoRa 32 V4 Expansion Kit — that was the earlier pick and is explicitly superseded. See [ADR-002](decisions/ADR-002-tag-hardware.md).
- **Relay hardware:** Heltec Wireless Tracker V2 + Heltec Solar Kit for Dev-board. Note the Solar Kit default bracket does NOT fit the Tracker V2 form factor — mounting uses adhesive PCB standoffs + 3M VHB tape. See [ADR-003](decisions/ADR-003-relay-hardware.md).
- **Gateway OS:** Yocto Linux from day one. Not Raspbian. Not Raspberry Pi OS. See [ADR-004](decisions/ADR-004-gateway-platform.md).
- **Kiosk UI:** native Rust GUI. `egui` + `walkers` is the starting bet; `iced` and `slint` are the fallbacks. Offline tiles are **PMTiles**, not MBTiles (`walkers` supports `.pmtiles` natively). No browser, no WebView, no Chromium, no MapLibre, no Leaflet, no Tauri. See [ADR-005](decisions/ADR-005-map-and-ui.md).
- **Relay GNSS:** on the board (UC6580), used only during commissioning and maintenance. OFF during normal forwarding. BLE maintenance CLI is **v1 (not v0, not v2+)** — you cannot deploy a sealed solar relay without a way to verify it is alive on-site. See [ADR-006](decisions/ADR-006-relay-has-gnss.md).
- **Touchscreen is the only UI:** read-only map, no login, no modals, no settings screens, no CRUD. See [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md).
- **No cloud, no downlink.** No internet-hosted server. No REST API. No WebSocket. No phone app. No commands from gateway to tag. Pure uplink. No NTP — not even if WiFi is available. See [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md).
- **Database:** SQLite (WAL mode), single file. Not PostgreSQL. Not DuckDB. Not a K/V store. Dedup is recent-window (24 h), not a permanent UNIQUE index. `seq_nr` is `u32` on the wire. See [ADR-009](decisions/ADR-009-database-sqlite.md).
- **SOS encoding:** same band as heartbeat (868.1 MHz, sub-band M), SOS is a flag bit in `POSITION`, cadence is jittered. No separate SOS frequency in v1 — it created a phase-lock bug. See [ADR-010](decisions/ADR-010-sos-encoding.md).
- **Gateway time source:** DS3231 I²C RTC module with CR2032 coin cell is primary; Dragino HAT Quectel L80-M39 GPS/PPS is opportunistic. No NTP. See [ADR-011](decisions/ADR-011-gateway-time-source.md).
- **v1 protocol is multi-hop on a single channel.** One packet type: POSITION. Loop prevention is dedup on `(node_id, seq_nr)`, 60 s expiry. No FORWARD envelope. No path arrays. No per-hop RSSI/SNR on the wire. No SIGHTING. No RELAY_INFO. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- **No wire-level role enum.** Node presentation (hiker / relay / drone) is gateway config in `nodes.toml`, keyed on `node_id`. If a future change wants a role byte, it must justify what receiver behaviour depends on it. See [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- **Coverage / science telemetry is a separate layer, deferred to a future ADR.** Per-hop RSSI/SNR coverage data will live in a reception-log layer designed when v1 forwarding is working. Don't put analytics into the forwarding protocol. See [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- **Every protocol change updates the duty-cycle budget table ([ARCHITECTURE.md](ARCHITECTURE.md) §13).** A change without a budget update is incomplete. See [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md).
- **v1 splits into v1a (single-relay garden test) and v1b (drone-pod overlay, two-relay chained test).** v1b is gated on v1a passing. From [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md), preserved through the ADR-013 rollback.
- **Tag-side buzzer for SOS only.** Relays do not have buzzers as search beacons — that draws the searcher to the wrong location. From [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md), preserved.

If a suggestion starts with "let's just add a small web dashboard" or "let's use Python for the gateway" or "let's use React for the map" or "let's put SOS on a separate frequency for more range" or "let's just NTP the clock when WiFi is around" or "let's let the gateway compute the relay's position" or "let's auto-detect aerial role from altitude" or "let's add a homing beacon" or "let's add direction finding" or "let's put a buzzer on the relay" or "let's put SOS on the aviation distress band" or "let's add a FORWARD envelope with path tracking" or "let's record per-hop RSSI in the packet" or "let's add a role byte to distinguish hiker from relay" or "let's split into CH_TAG and CH_FWD channels" or "let's defer multi-hop to v2" — stop. That door is closed. Re-read the ADRs.

## Always read these first, in this order

1. `README.md` — project status at a glance, folder structure
2. `CLAUDE.md` — this file
3. `decisions/README.md` — ADR index
4. `ARCHITECTURE.md` — the system in one document
5. `TODO.md` — what's being worked on right now
6. Whichever file is relevant to the current task

## Tone and working style (Pieter)

- Writes in a mix of English and Belgian Dutch. Responses in either are fine; defaults to English for durable docs, switches freely in chat.
- Terse, direct, swears when frustrated. This is not hostility — it's normal communication. Respond in kind (professionally-direct) rather than apologising or over-hedging.
- Hates "fastest time to market" shortcuts framed as engineering advice. Wants high-quality decisions with reasoning visible, not defaults dressed up as choices. If you recommend the obvious easy thing, say so and justify it.
- Hates AI sludge that makes things more complicated than they need to be. If a choice is simple, say it's simple and move on. If a choice is hard, show the tradeoff.
- Values physical plug-and-play. Will pay for a board that drops into a solar kit rather than 3D-print a bracket. Will NOT skimp on software correctness.
- Physical deployment is 50% of the project. The enclosure/mount/power story is a first-class concern, not a v2 polish item.
- Is building this for himself, learning, and portfolio. Not selling anything. Quality > speed.

## Tools this project uses

- **Language:** Rust everywhere. `no_std` on MCU (tag, relay), `std` on the gateway/kiosk. Python only for one-off scripts.
- **MCU firmware:** `esp-hal`, Embassy async executor, [`lora-phy`](https://github.com/lora-rs/lora-rs) (from `lora-rs/lora-rs` — the older `embassy-rs/lora-phy` is archived), an NMEA parser (likely `nmea0183` or similar). Nightly Rust for the `xtensa-esp32s3-none-elf` target (via `espup` / `esp-rs`).
- **Gateway / kiosk:** Rust on Yocto Linux, `linux-embedded-hal` for SPI to the Dragino SX1276, the **same `lora-phy` crate** (it supports SX127x as well as SX126x — no separate gateway radio driver to write), `rppal` for GPIO, `gpsd` + `chrony` for opportunistic GPS-disciplined time, `tokio` for async, `rusqlite` or `sqlx` for SQLite.
- **UI:** native Rust GUI, `egui` + `walkers` (starting bet). Offline **PMTiles** bundled in the Yocto image.
- **Shared crates:** one `protocol` crate (`no_std` with optional `std`, used by tag, relay, gateway) and one `persistence` crate (`std`-only, owns the SQLite schema and queries, used by the gateway binary).
- **Build:** Cargo workspace. Binaries live under `firmware/{tag,relay}/` and `gateway/` (the kiosk is a module inside the gateway binary, not a separate crate). `espflash` for flashing. Yocto (`meta-rust` layer) for the Pi image. SCP for deploy during bring-up.
- **Relevant skills:** `docx`/`pdf` only if the user explicitly asks for a printed report. There is no frontend-design work to do — we're not building a web frontend.

## What "done" looks like for v1

Dot moves on the 7" touchscreen map as Pieter walks around his garden carrying a tag. Relay on a wooden pole in the garden, solar-powered, rebroadcasting packets. Gateway stores POSITION reports in `tag_reports` (SQLite) on its local Yocto Linux install and draws them live with a native Rust GUI. Zero internet required at any point. See [ARCHITECTURE.md §15](ARCHITECTURE.md) for the full v1 acceptance criteria.
