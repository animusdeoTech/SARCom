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

## Rust doc sources

Confirmed firmware lane: **Lane A — bare-metal `no_std`**, target `xtensa-esp32s3-none-elf`, `esp-hal` + Embassy. Lane B (`esp-idf-hal`, `esp-idf-svc`, FreeRTOS, `xtensa-esp32s3-espidf`) is **prohibited**. Any example or doc page that imports an `esp-idf-*` crate is wrong-lane and must be discarded.

Risky crates — `lora-phy`, `embassy-executor` / `embassy-time` / `embassy-sync`, `esp-hal`, `walkers`, `egui` / `eframe`, `nmea` / `nmea0183` — must be looked up from the configured source in the table **before drafting code**, not after.

Source order:

1. **`cargo doc -p <crate>`** when the crate is pinned in `Cargo.lock` and `target/doc/<crate>/` exists — version-exact, offline, fastest.
2. **Context7 MCP** for crates marked Context7 in the table below.
3. **`github` MCP** for the crates the table routes there (`lora-phy`, `nmea`, walkers PMTiles specifics) — Context7 does not cover these.
4. **`rust-analyzer` / `cargo check` / compiler diagnostics** are the **validation and repair layer**, used *after* a draft exists. They are not a doc-lookup source and must not be the first thing consulted.
5. **Brave Search is never the source for Rust API docs.** If all configured sources fail, record the failure (in this file or a follow-up note) before falling back.

Per-crate first lookup (verified 2026-05-06):

| Crate | First source | Notes |
|---|---|---|
| `lora-phy` (lora-rs) | `github` MCP → `lora-rs/lora-rs` (`examples/`, `src/`) | Context7 has **no entry**. `/lora-rs/lora-rs` returns "not found"; `resolve-library-id` returns irrelevant LoRA-AI projects. **Reject anything from the archived `embassy-rs/lora-phy`.** |
| `embassy-executor`, `embassy-time`, `embassy-sync` | Context7 `/embassy-rs/embassy` | Lane A only. Cross-check against `esp-hal` examples for correct timer-driver wiring on ESP32-S3. |
| `esp-hal` | Context7 `/esp-rs/esp-hal` | Lane A confirmed. Version-pinned snapshots also exist (`/websites/espressif_projects_rust_esp-hal_1_0_0_*`) for matching a specific `Cargo.lock`. **Never `esp-idf-hal`.** |
| `walkers` | Context7 `/podusowski/walkers` for basics, `github` MCP → `podusowski/walkers` for PMTiles | Context7 surfaces only the `HttpTiles` + OpenStreetMap path. PMTiles tile-source API is not indexed — read `walkers/examples/` and `walkers/src/sources/` directly. |
| `egui` / `eframe` | Context7 `/emilk/egui` | Pin the version to whatever `eframe` resolves to in `Cargo.lock`; widget API moves between minor versions. |
| `nmea` / `nmea0183` | `github` MCP → `AeroRust/nmea` (README + `src/lib.rs`) | Context7 indexes only the **C++ Arduino** library under the same name — wrong language. Switch to `cargo doc -p <crate>` once selected and pinned. |
| Hardware datasheets | `resources/datasheets/` (when populated) | Deep chip-level reading deferred to bring-up. |

**`lora-phy` preflight is mandatory.** Before editing any file that initialises, transmits, receives, performs CAD on, or configures the LoRa radio via `lora-phy`:

- Read `resources/docs/lora-phy-preflight.md`.
- Run the slash command `/rust_lora_phy_preflight` (defined in `.claude/commands/`).
- Run `scripts/check-lora-phy-docs.ps1` for current pin/rustdoc status.
- Produce the **preflight statement** (template lives in the preflight file) before any code change touches radio paths.
- `rust-analyzer` and `cargo check` remain validation/repair only.

### Rust doc lookup protocol

Before writing or modifying Rust code that touches one of the risky crates, Claude must:

1. Identify the crate(s) involved.
2. State the selected source: local `cargo doc` if available; otherwise the configured source from the table above.
3. Consult that source.
4. Only then draft code.
5. After drafting, use `rust-analyzer` / `cargo check` / compiler diagnostics as the validation and repair layer.

If the configured source is missing, stale, wrong-language, or wrong-lane, do not guess. Record the failure and switch to the fallback listed in the table.

## Rust API notes

Patterns no doc lookup will catch on its own:

- **Embassy task model.** `#[embassy_executor::task]` functions cannot take generic parameters and must not be invoked as a normal `async fn`. Spawn them via `spawner.spawn(task_fn(arg)).unwrap()`. For the entry point and timer/executor setup, use the version-matched `esp-hal` + Embassy example for ESP32-S3 — do not invent the macro name from memory. The exact macro (e.g. `#[esp_hal::main]`, `#[esp_rtos::main]`, or another) varies between `esp-hal` releases and is verified at the source consulted via the protocol above.
- **`esp-hal` peripheral ownership.** Peripherals are typestate singletons consumed on use. The SX1262 SPI bus + chip-select GPIO must be passed through explicitly into `lora-phy`'s `RadioKind` impl — there is no global access. Constructing two SPI masters from the same `peripherals.SPI2` is a compile error.
- **`lora-phy` `RadioKind` impl.** Ships with chip-specific structs (`Sx126x`, `Sx127x` and their config types) and a `RadioError` associated type. SX1262 (tag, relay) and SX1276 (gateway HAT) share the trait; only the config and pin wiring differ. Verify the trait surface against the `lora-rs/lora-rs` repo before drafting — context7 cannot.
- **Lane boundary, hard.** No `esp-idf-hal`, no `esp-idf-svc`, no `std::thread` on MCU, no FreeRTOS, no `xtensa-esp32s3-espidf` target. If a generated snippet imports any of those, discard and re-source from Lane A.

## Rust doc lookup smoke test

Before any serious firmware or UI implementation session, Claude should run four dry-run stubs to prove the doc-lookup behaviour from `## Rust doc sources` actually works. The stubs are **not committed** and **not production code** — they are a behaviour check.

The four dry runs:

1. Embassy task skeleton with `Spawner` and `#[embassy_executor::task]`.
2. `esp-hal` SPI ownership skeleton for the SX1262 — peripheral take, SPI master configuration, chip-select GPIO.
3. `lora-phy` radio init / send skeleton — `RadioKind` impl wiring for SX1262.
4. `walkers` PMTiles tile-source skeleton — local file source plugged into a `Map`.

For each dry run, Claude must state in plain text:

- Which crate was touched.
- Which doc source was consulted (Context7, `github` MCP, or local `cargo doc`).
- Whether any wrong-lane (`esp-idf-*`) or wrong-language (Arduino C++, etc.) source was rejected during the lookup.

The smoke test is about proving the lookup behaviour, not about producing usable stubs. If a configured source is unavailable, the dry run is recorded as a failure with the reason — guessing past it is not allowed.

## What "done" looks like for v1

Dot moves on the 7" touchscreen map as Pieter walks around his garden carrying a tag. Relay on a wooden pole in the garden, solar-powered, rebroadcasting packets. Gateway stores POSITION reports in `tag_reports` (SQLite) on its local Yocto Linux install and draws them live with a native Rust GUI. Zero internet required at any point. See [ARCHITECTURE.md §15](ARCHITECTURE.md) for the full v1 acceptance criteria.
