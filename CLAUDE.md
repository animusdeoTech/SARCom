---
title: "CLAUDE.md — LLM onboarding"
status: living
type: meta
tags: [onboarding, meta, claude]
---

# CLAUDE.md

Onboarding note for any Claude instance entering this project — chat, Claude Code CLI, Obsidian vault plugin, Cowork, whatever. Read this first.

## What this project is

LoRa-based Search & Rescue telemetry network. GPS tags → solar LoRa relays → handheld Rust gateway with a touchscreen showing a live read-only map. Carried by hut staff or rescue-adjacent operators; the mountain hut is one possible deployment site, not the only one. **Local-first. No cloud. No phone app. No inbound network. Pure uplink on the LoRa side.** Outbound LAN-bounded CoT/TAK export is a v1 feature gated on WiFi + manual opt-in (pending ADR-016); when any gate input is false the export path is silent and nothing else changes.[^pivot][^gate-2026-05-14]

For the full picture see [ARCHITECTURE.md](ARCHITECTURE.md).

[^pivot]: 2026-05-06 form-factor pivot — see [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md). Three new ADRs proposed: ADR-015 (handheld substrate + form factor; supersedes-in-part ADR-004; refines-in-part ADR-005/006/007), ADR-016 (base-mode export gate; supersedes-in-part ADR-008), ADR-017 (custom 3D-printed waterproof enclosures for gateway and tag; refines-in-part ADR-002). The accepted ADR ledger below reads as it does today; supersession headers will land when those ADRs are written.

## Current hardware inventory (as of 2026-05-16)

**Nothing is in hand.** Every ADR'd or spike-closed hardware pick is paper-decided only. No Pi 5. No Pi Touch Display 2. No Heltec Wireless Tracker V2 boards. No Heltec Solar Kit. No Dragino HAT. No DS3231 RTC. No enclosure, no battery, no display panel, no antennas. The three Pi 4 Model B units mentioned under ADR-004 are retired (see [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md)) and not in usable condition.

**Procurement is Pieter's call, not Claude's. Pieter's wallet is not a planning input.** Paper-decided ≠ buy-committed. An ADR that picks a hardware part is a technical commitment to *that* part *if and when* it gets bought; it is not a commitment to buy it, and it is not a schedule. Claude does not recommend purchases. Claude does not push toward procurement. Claude does not assume future purchases as the basis for current work. Claude does not structure plans around eventual hardware as a foregone conclusion. Claude does not write "ordering now would unblock X" or "the cheapest path to validating this is to buy Y." Pieter spends his money when he is convinced — not when Claude is convinced, not when a spike is convinced, not when a plan needs it. Every plan must work with the hardware actually in hand (nothing) or admit upfront that it cannot, and stop there.

Any spike phase or implementation step that calls for **deploy on / flash / measure on / photograph** physical hardware is **deferred-pending-procurement** until the inventory line above changes. Stop asking whether a board is in the house; the answer is no. When procurement starts, this section is updated; until then, treat every hardware-dependent step as out-of-scope for the work at hand and surface it as a deferred-line in the relevant spike's decision note, not as a halt-and-ask-the-user gate.

## Do NOT re-open these decisions without explicit reason

These are **Accepted** in `decisions/`, dated 2026-04-22 (ADR-001 through ADR-009), 2026-04-24 (ADR-010, ADR-011), 2026-04-25 (ADR-012), and 2026-04-26 (ADR-013, ADR-014). ADR-012 is **superseded in part** by ADR-013/014 — its v1a/v1b split, tag buzzer, and non-goals list survive; its role enum, SIGHTING, RELAY_INFO and three-table schema do not. Arguing accepted decisions again wastes time. If context genuinely changed, write a new ADR that supersedes the old one; do not re-litigate inline.

- **Firmware / gateway / UI language: Rust, end to end.** Tag firmware, relay firmware, gateway receiver, kiosk UI — all Rust. No C, no C++, no Zephyr, no Python runtime, no JavaScript, no TypeScript, no React, no npm in production SARCOM runtime code. Archived or local UX exploration tools (under `tools/`) may use npm, but do not introduce new npm tooling unless explicitly requested. See [ADR-001](decisions/ADR-001-firmware-language.md).
- **Tag hardware:** Heltec Wireless Tracker V2 (ESP32-S3 + SX1262 + UC6580 GNSS). Not the WiFi LoRa 32 V4 Expansion Kit — that was the earlier pick and is explicitly superseded. See [ADR-002](decisions/ADR-002-tag-hardware.md).
- **Relay hardware:** Heltec Wireless Tracker V2 + Heltec Solar Kit for Dev-board. Note the Solar Kit default bracket does NOT fit the Tracker V2 form factor — mounting uses adhesive PCB standoffs + 3M VHB tape. See [ADR-003](decisions/ADR-003-relay-hardware.md).
- **Gateway OS:** Yocto Linux from day one. Not Raspbian. Not Raspberry Pi OS. See [ADR-004](decisions/ADR-004-gateway-platform.md).
- **Kiosk UI:** native Rust GUI. `egui` + `walkers` is the starting bet; `iced` and `slint` are the fallbacks. Offline tiles are **PMTiles**, not MBTiles (`walkers` supports `.pmtiles` natively). No browser, no WebView, no Chromium, no MapLibre, no Leaflet, no Tauri. See [ADR-005](decisions/ADR-005-map-and-ui.md).
- **Relay GNSS:** on the board (UC6580), used only during commissioning and maintenance. OFF during normal forwarding. BLE maintenance CLI is **v1 (not v0, not v2+)** — you cannot deploy a sealed solar relay without a way to verify it is alive on-site. See [ADR-006](decisions/ADR-006-relay-has-gnss.md).
- **Touchscreen is the only UI:** read-only map, no login, no modals, no settings screens, no CRUD. See [ADR-007](decisions/ADR-007-touchscreen-primary-ui.md).
- **No cloud, no downlink.** No internet-hosted server. No REST API. No WebSocket. No phone app. No commands from gateway to tag. Pure uplink on the LoRa side. No NTP — not even if WiFi is available. See [ADR-008](decisions/ADR-008-no-cloud-no-downlink.md). (Pending ADR-016 splits the wording into four categories — no inbound surface, no internet-bound calls, no cloud-hosted dependency, *and* outbound LAN multicast/unicast under explicit gate. Categories one through three stay closed; the fourth is the new addition for base-mode CoT/TAK export. The NTP door stays closed under ADR-011 regardless.)
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

Note: outbound LAN-bounded CoT/TAK export under the pending ADR-016 gate (WiFi + manual opt-in)[^gate-2026-05-14] is **not** the same shape as a "small web dashboard" or an NTP call. It is read-only, outbound-only, RFC1918 / link-local / multicast-only, and silent unless all gate inputs are true. The dashboard / cloud / phone-app / NTP doors remain closed.

> **Pi 4 substrate is retired (2026-05-07).** All three on-hand Pi 4 Model B units tested out of order; substrate is now Pi 5 (variant TBD per [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md)). Do NOT propose a Pi 4 power-on test. Do NOT list Pi 4 as a substrate candidate. See [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md).

## Always read these first, in this order

1. `README.md` — project status at a glance, folder structure
2. `CLAUDE.md` — this file
3. `decisions/README.md` — ADR index
4. `ARCHITECTURE.md` — the system in one document
5. `TODO.md` — what's being worked on right now
6. Whichever file is relevant to the current task

## Spike-writing protocol

When writing or updating any file under `spikes/`, read these first:

1. `docs/spike-rules.md` — definition + hard rules + executor test
2. `docs/spike-template.md` — copy as the starting skeleton
3. The three canonical exemplar spikes named in the rules file

A spike that does not match the canonical shape is rewritten before commit. The hard "no" list (no IC selection, no vendor lock-in, no euro figures stated as answers, etc.) is non-negotiable. Named parts / vendors appear ONLY as candidates with comparison criteria, never as the choice. Cost envelope is relative at open; exact euro only at close if cited, measured, or candidate-pending. Every concrete claim is either a candidate-pending-comparison or cites repo-local `path:line` evidence (produced via `rg -n` / `git grep -n`). External datasheets, measured values, and calculator outputs are valid evidence at close. Claims that cannot be sourced go in a top-of-file "To verify before close" block, flagged low-confidence.

The spike's output is a commitment to one named follow-up: implementation ticket, ADR write, procurement-decision ticket backed by its own ranked shortlist, smaller re-scoped spike, or a decomposition follow-up that lists sub-spikes. The spike does not decide architecture.

## Tone and working style (Pieter)

- Writes in a mix of English and Belgian Dutch. Responses in either are fine; defaults to English for durable docs, switches freely in chat.
- Terse, direct, swears when frustrated. This is not hostility — it's normal communication. Respond in kind (professionally-direct) rather than apologising or over-hedging.
- Hates "fastest time to market" shortcuts framed as engineering advice. Wants high-quality decisions with reasoning visible, not defaults dressed up as choices. If you recommend the obvious easy thing, say so and justify it.
- Hates AI sludge that makes things more complicated than they need to be. If a choice is simple, say it's simple and move on. If a choice is hard, show the tradeoff.
- Values physical plug-and-play. Will pay for a board that drops into a solar kit rather than 3D-print a bracket. Will NOT skimp on software correctness.
- Physical deployment is 50% of the project. The enclosure/mount/power story is a first-class concern, not a v2 polish item.
- Is building this for himself, learning, and portfolio. Not selling anything. Quality > speed.
- **Dev machine: Windows home PC (desktop).** Not a laptop. The cross-compile / kiosk spike / espflash / Yocto-cross / `cargo check` workflow runs on a Windows desktop. References in the doc set to "the dev laptop" or "on a laptop first" are stale and being retired — see [`dev-log/2026-05-08-dev-machine-correction.md`](dev-log/2026-05-08-dev-machine-correction.md) (this retirement).

## Schema-extension discipline — the subtyping-fetish check

Before proposing any new enum variant, schema `kind` value, wire-level type byte, or struct subtype, Claude (especially in reviewer / orchestrator role) must answer these three questions explicitly, in the proposal itself:

1. **Is the distinction operator-visible or consumer-visible at runtime, or is it only provenance / origin?** If it's only provenance, it's an **attribute**, not a type.
2. **Does an existing type already use an attribute-pattern for this category?** Mirror it. Hillshade overlays use `kind = "hillshade"` + `source = "dhmv_ii_dsm_1m"`. Relay roles live in `nodes.toml` keyed on `node_id`, not as a wire-level role byte (`decisions/ADR-013-multi-hop-flood-via-packet-id.md`). If a pattern exists, use it.
3. **Does the new variant force every downstream consumer (renderer, parser, dispatcher, doc-section, firmware byte-pack) into an extra branch / argument / case / wire byte?** If yes, the distinction must be *categorically* worth that cost — not just notionally distinct.

If question 1 = "provenance only", or question 2 = "yes, a pattern exists", or question 3 = "yes, but not categorically", the answer is an attribute on an existing type, not a new type. Redesign before proposing.

Two prior instances where this check was needed and caught late by Pieter:

- **Nearly added `drone_relay` / `fixed_relay` as wire-level subtypes in POSITION packets.** The correct shape is one `relay` role with presentation / configuration attributes in `nodes.toml` per ADR-013. Had this landed, every firmware layer (tag, relay, gateway) would have paid a per-packet decode-branch cost forever, plus a wire byte that encoded nothing functionally distinct.
- **Shipped `osm_overpass` as a peer enum variant of `osm` in the region overlay schema** (2026-05-16). Caught after one bake. Collapsed to one `kind = "osm"` with `source = "file" | "overpass"` per `dev-log/2026-05-16-osm-overlay-collapse-subtypes.md`. Every consumer (region.rs, app.rs, pmtiles_map.rs, mod.rs, fetch-region.ps1, terril-waterschei region.toml, README, QUICKSTART) had gained a parallel branch / argument / doc-section for what is operator-invisible provenance.

If a type addition lands without the three-question check visibly run, the filter wasn't applied. Surface the check in the proposal — don't push the watchdog burden on Pieter. Shifting the failure cost to the operator is the same failure mode in disguise.

## Tools this project uses

- **Language:** Rust everywhere. `no_std` on MCU (tag, relay), `std` on the gateway/kiosk. Python only for one-off scripts.
- **MCU firmware:** `esp-hal`, Embassy async executor, [`lora-phy`](https://github.com/lora-rs/lora-rs) (from `lora-rs/lora-rs` — the older `embassy-rs/lora-phy` is archived), an NMEA parser (likely `nmea0183` or similar). Nightly Rust for the `xtensa-esp32s3-none-elf` target (via `espup` / `esp-rs`).
- **Gateway / kiosk:** Rust on Yocto Linux, `linux-embedded-hal` for SPI to the Dragino SX1276, the **same `lora-phy` crate** (it supports SX127x as well as SX126x — no separate gateway radio driver to write), `rpi-pal` for GPIO (the maintained fork of the archived `rppal`; same API, drop-in, Pi 5 compatible — see [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md)), `gpsd` + `chrony` for opportunistic GPS-disciplined time, `tokio` for async, `rusqlite` or `sqlx` for SQLite. Substrate (Pi class + display class + battery + enclosure) is open pending ADR-015.
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
| `walkers` | Context7 `/podusowski/walkers` for basics, `github` MCP → `podusowski/walkers` for PMTiles | Context7 surfaces only the `HttpTiles` + OpenStreetMap path. PMTiles tile-source API lives at `walkers/src/pmtiles.rs` (crate root, behind the `pmtiles` feature flag) — read that file plus `walkers/examples/` directly. The `walkers/src/sources/` directory only contains HTTP-based tile providers (Mapbox, OSM, Geoportal, OpenFreeMap); PMTiles is NOT there. |
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

Dot moves on the handheld gateway's touchscreen map as Pieter walks around his garden carrying a tag. Relay on an off-the-shelf plastic tripod (per `spikes/physical-fabrication-brief-spike.md`), solar-powered, rebroadcasting packets. Gateway stores POSITION reports in `tag_reports` (SQLite) on its local Yocto Linux install and draws them live with a native Rust GUI. Zero internet required at any point on the LoRa side; outbound CoT/TAK export is silent unless WiFi + manual opt-in are both present (pending ADR-016).[^gate-2026-05-14] See [ARCHITECTURE.md §15](ARCHITECTURE.md) for the full v1 acceptance criteria. Display class, substrate, and enclosure shape are open pending ADR-015 / ADR-017.

[^gate-2026-05-14]: The pending-ADR-016 export gate was re-scoped from three inputs ("WiFi + external power + manual opt-in") to two inputs ("WiFi + manual opt-in") on 2026-05-14 after the magnetic-pogo charging input was dropped from the v1 gateway. With no in-shell charging path, the SBC cannot read "external power present" as a signal, so it cannot be a gate input. See [`dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md) for the originating decision, [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md) for the propagation pass, and `spikes/gateway-handheld-power-architecture-spike.md` + `spikes/tak-cot-integration-spike.md` + `spikes/gateway-runtime-task-architecture-spike.md` 2026-05-14 amendments for the spike-level changes.
