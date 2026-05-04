---
title: "Spike — Reliable docs access for Rust firmware and UI work"
status: open
type: spike
timebox: 0.5 day
---

# Spike: Reliable docs access for Rust firmware and UI work

## Why this spike exists

We are about to write Rust across four codebases: tag firmware, relay firmware, gateway receiver, and kiosk UI. Several key crates are niche enough that Claude's training data is incomplete, outdated, or mixed with a deprecated predecessor. Without reliable doc access, code generation devolves into API hallucination and repeated corrective web searches.

The goal is not air-gapped offline docs. The goal is a reproducible, low-friction setup where Claude consults a known-good source for each crate instead of guessing. Context7 MCP is acceptable even though it requires a network connection — it is reliable and structured, which is what matters.

## Pre-requisite research question: confirm the firmware lane

Before any doc work, confirm which compilation lane this project uses for tag and relay firmware. The two lanes are mutually exclusive — mixing docs or examples between them produces code that does not compile.

**Lane A — bare-metal (`no_std`):**
`esp-hal` + Embassy (`embassy-executor`, `embassy-time`, `embassy-sync`, `embassy-hal-integration`) + `xtensa-esp32s3-none-elf` target. No OS, no heap allocator by default, no `std`. This is what ARCHITECTURE.md and CLAUDE.md specify.

**Lane B — ESP-IDF (`std`):**
`esp-idf-hal` + `esp-idf-svc` + FreeRTOS underneath + `xtensa-esp32s3-espidf` target. Provides `std`, threads, and an RTOS. Different crate names, different APIs, different build system.

The project has committed to Lane A (ADR-001, CLAUDE.md). Confirm this is still the intended lane and record it explicitly in the decision note so there is no ambiguity when selecting doc sources. Any example code or API snippet sourced from Lane B examples must be rejected.

## Hypotheses

**H1 (good enough):** Context7 MCP plus targeted CLAUDE.md additions for crates Context7 misses or covers poorly gives Claude enough API accuracy to write compilable first drafts without web searches across the six priority crates.

**H0 (not good enough):** Context7 coverage for the niche crates is too thin or stale, and the fallback options are too high-maintenance to sustain. Firmware writing proceeds with on-demand web searches and manual correction.

## Timebox

**Half a day hard stop.**

```
0.5 h — confirm firmware lane; record in decision note
0.5 h — test Context7 MCP: add to Claude Code config, probe each of the 6 target crates
0.5 h — for crates Context7 misses: evaluate fallback doc sources (see §3)
0.5 h — draft CLAUDE.md additions for non-obvious patterns
0.5 h — write the decision note and per-crate source table
```

## Target crates (priority order)

| Priority | Crate | Why risky |
|---|---|---|
| 1 | `lora-phy` (lora-rs/lora-rs) | Active development; archived predecessor (`embassy-rs/lora-phy`) still indexed — training data likely stale or mixed |
| 2 | `embassy-executor`, `embassy-time`, `embassy-sync` | Task model has non-obvious constraints; `#[embassy_executor::task]` forbids generics, wrong signatures compile but panic |
| 3 | `esp-hal` | Typestate peripheral ownership; Lane A only — must not be confused with `esp-idf-hal` (Lane B) |
| 4 | `walkers` | Small crate; PMTiles local-file API unlikely to be in training data |
| 5 | `egui` / `eframe` | Popular but fast-moving; pin version and verify against docs before writing custom widgets |
| 6 | `nmea` / `nmea0183` | Small crate; hallucination risk for struct fields and parser return types |

`rusqlite` and `tokio` are well-covered in training data and are low risk. Exclude from doc setup unless problems surface.

## What to verify

### 0. Firmware lane confirmation
- Read `esp-hal` and `embassy-hal-integration` crate READMEs; confirm they target `xtensa-esp32s3-none-elf` (Lane A), not `xtensa-esp32s3-espidf` (Lane B)
- Confirm the project does not use `esp-idf-hal`, `esp-idf-svc`, or any crate that links against ESP-IDF
- Record in the decision note: "Lane A confirmed" or "Lane B confirmed" — no ambiguity

### 1. Context7 MCP
- Add Context7 to Claude Code's MCP config
- For each Priority 1–6 crate: ask Context7 to resolve the crate by name, check that it returns the correct repo (`lora-rs/lora-rs` not `embassy-rs/lora-phy`) and a version consistent with recent Cargo.lock
- For `esp-hal`: confirm the docs are for Lane A (bare-metal) — Context7 may index both `esp-hal` and `esp-idf-hal`; they must not be conflated
- Record: which crates Context7 covers reliably, which it misses or returns stale / wrong-lane docs for

### 2. Fallback doc sources (only for crates Context7 misses)

Options to evaluate, cheapest first:

**a) `cargo doc` with dependencies**
`cargo doc --no-deps` only builds docs for the local crate — it does not generate docs for dependencies. To get docs for a specific dependency: `cargo doc -p lora-phy` (builds that crate and its deps, writes to `target/doc/`). Evaluate: does `target/doc/lora_phy/` give complete, browsable HTML that Claude can be pointed at? Is it version-locked to Cargo.lock? Yes on both counts — this is the most reliable local source for any crate in the dependency tree.

**b) docs.rs rustdoc JSON**
docs.rs publishes pre-built rustdoc JSON for every crate version at `https://docs.rs/crate/{name}/{version}/json`. Evaluate: can this JSON be fetched once, stored in `resources/docs/{crate}.json`, and queried by a small script to extract struct/trait/method signatures as markdown? Re-generate whenever Cargo.lock changes. Acceptable if fetch + extract takes < 5 min.

**c) Vendored source + rust-analyzer**
`cargo vendor` copies all dependency sources into `vendor/`. rust-analyzer can index this. Primarily useful once the workspace exists; see §3.

Prefer (a) for niche crates — it requires no network after the initial `cargo fetch`, is version-exact, and produces human-readable HTML. Option (b) is useful for generating compact markdown summaries.

### 3. LSP tool (rust-analyzer) as validation and repair tooling
The `LSP` tool in Claude Code connects to rust-analyzer. Its role here is **not** to replace upfront doc access — it is to catch and repair hallucinations after code is drafted. With the workspace Cargo.toml in place and dependencies fetched, rust-analyzer can resolve types, surface mismatched method signatures, and flag missing trait impls in real time.

Evaluate: with a minimal workspace stub (Cargo.toml declaring the target crates, empty `src/lib.rs`), does rust-analyzer activate usefully without the full firmware code being written? If yes, LSP becomes a first-session validation tool. If it requires substantially complete code to be useful, it is a later-phase tool.

This does not replace Context7 or local cargo doc — it is the repair layer after generation.

### 4. CLAUDE.md additions
Some things no lookup tool will catch because they are patterns, not API surface:

- **Embassy task model:** `#[embassy_executor::task]` functions cannot take generic parameters and must not be called as normal `async fn`. Document the correct `spawner.spawn(task_fn(arg))` pattern and the `#[main]` entry point.
- **`esp-hal` peripheral ownership:** peripherals are consumed on use and must be explicitly passed through. Document the ownership pattern for the SPI bus + SX1262 chip select.
- **`lora-phy` `RadioKind` impl:** document which trait methods are mandatory vs. defaulted, the `RadioError` associated type, and the config structs for SX1262 vs. SX1276.
- **Lane boundary:** CLAUDE.md must explicitly state that `esp-idf-hal`, `esp-idf-svc`, FreeRTOS threading, and `xtensa-esp32s3-espidf` are out of scope for firmware. This prevents Claude from importing Lane B examples.

These go into `CLAUDE.md` regardless of the MCP outcome.

## Hardware datasheet inventory (out of scope for this spike)

Deep reading of chip datasheets is deferred to the bring-up phase. Record here where truth will live so Claude knows where to look when chip-level work starts:

| Document | What it covers | Where it will live |
|---|---|---|
| SX1262 datasheet | Register map, LoRa modulation params, SPI protocol | `resources/datasheets/SX1262_datasheet.pdf` |
| SX1276 datasheet | Same for the Dragino HAT radio | `resources/datasheets/SX1276_datasheet.pdf` |
| UC6580 GNSS datasheet / NMEA guide | GNSS module on the Wireless Tracker V2; NMEA sentence set | `resources/datasheets/UC6580_datasheet.pdf` |
| Heltec Wireless Tracker V2 schematic | Pin mapping, power rails, GNSS enable GPIO, SX1262 SPI pins | `resources/datasheets/heltec_wireless_tracker_v2_schematic.pdf` |
| DS3231 datasheet | I²C register map, alarm config, aging offset | `resources/datasheets/DS3231_datasheet.pdf` |
| Dragino LoRa GPS HAT schematic | SX1276 SPI pins on the Pi header, GPS wiring | `resources/datasheets/dragino_lora_gps_hat_schematic.pdf` |

Add these to `resources/datasheets/` before bring-up work starts. Reference them in CLAUDE.md with a note that chip-level questions go there first before a web search.

## Pass criteria

- Firmware lane confirmed and recorded unambiguously
- Context7 (or fallback) resolves accurate API docs for Priority 1–3 crates without a web search
- CLAUDE.md contains `## Rust API notes` covering the four patterns above and the lane boundary rule
- CLAUDE.md contains `## Doc sources` with a per-crate first-lookup table (see deliverable below)
- The setup takes < 15 minutes to reproduce on a fresh Claude Code session — it is config and CLAUDE.md, not manual ritual

## Fail criteria

Any one of:
- Context7 resolves `lora-phy` to the archived `embassy-rs/lora-phy` repo
- Context7 returns Lane B (`esp-idf-hal`) docs for `esp-hal` queries
- All fallback doc options require > 5 min to regenerate after a version bump
- No source can provide `lora-phy` `RadioKind` trait method list without a web search

## Fallback options

**Option A — `cargo doc -p <crate>` + CLAUDE.md**
For each niche crate, run `cargo doc -p <crate>` once after `Cargo.lock` is pinned; HTML lands in `target/doc/`. CLAUDE.md points Claude at the local path. No MCP. No scripts. Version-exact. Re-run when deps change. Cheap, correct, low-tech.

**Option B — Context7 + CLAUDE.md gaps**
Add Context7 for broad coverage; fall back to Option A only for crates Context7 misses or gets wrong. Lower friction day-to-day; slightly higher setup cost.

**Option C — Context7 + docs.rs JSON extraction script**
Context7 for well-covered crates; a project script (`scripts/gen-docs.sh`) fetches rustdoc JSON from docs.rs and extracts compact markdown summaries for the niche crates into `resources/docs/`. Re-run on `Cargo.lock` changes. Best coverage-to-effort ratio if the script is < 50 lines.

## Deliverable

A `## Rust doc sources` section added to `CLAUDE.md` containing:

1. The confirmed firmware lane (Lane A / bare-metal) and the explicit prohibition on Lane B crates
2. A per-crate first-lookup table:

| Crate | First source | Notes |
|---|---|---|
| `lora-phy` | [TBD by spike] | Confirm `lora-rs/lora-rs`, not archived repo |
| `embassy-*` | [TBD by spike] | Lane A only |
| `esp-hal` | [TBD by spike] | Lane A only; not `esp-idf-hal` |
| `walkers` | [TBD by spike] | |
| `egui` / `eframe` | [TBD by spike] | |
| `nmea` / `nmea0183` | [TBD by spike] | |
| Hardware datasheets | `resources/datasheets/` | Deep read deferred to bring-up |

3. The non-obvious API patterns (embassy task model, esp-hal peripheral ownership, lora-phy RadioKind impl, lane boundary rule)

## Decision note template

```
Date:
Firmware lane: A (bare-metal, no_std) / B (ESP-IDF, std)
Context7 coverage: [per-crate: ok / stale / wrong lane / missing]
Fallback chosen: A / B / C / none needed
CLAUDE.md sections added: Rust doc sources / Rust API notes
Per-crate source table: [filled in]
Next action:
```
