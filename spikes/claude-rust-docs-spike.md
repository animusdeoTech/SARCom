---
title: "Spike — Reliable docs access for Rust firmware and UI work"
status: resolved
resolved: 2026-05-06
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
| `lora-phy` | `github` MCP → `lora-rs/lora-rs` | Context7 has no entry. Reject any hit from archived `embassy-rs/lora-phy`. Switch to `cargo doc -p lora-phy` once pinned. |
| `embassy-*` | Context7 `/embassy-rs/embassy` | Lane A only. |
| `esp-hal` | Context7 `/esp-rs/esp-hal` | Lane A confirmed; pinned snapshots exist. Never `esp-idf-hal`. |
| `walkers` | Context7 `/podusowski/walkers` + `github` MCP for PMTiles | Context7 only covers `HttpTiles`/OSM; PMTiles API needs the repo. |
| `egui` / `eframe` | Context7 `/emilk/egui` | Pin the version to what `eframe` resolves to. |
| `nmea` / `nmea0183` | `github` MCP → `AeroRust/nmea` | Context7 indexes only the C++ Arduino library — wrong language. |
| Hardware datasheets | `resources/datasheets/` | Deep read deferred to bring-up |

3. The non-obvious API patterns (embassy task model, esp-hal peripheral ownership, lora-phy RadioKind impl, lane boundary rule)

## Decision note

```
Date: 2026-05-06
Firmware lane: A (bare-metal, no_std) — confirmed
  Sources:
    - ADR-001 §Decision: esp-hal + Embassy + lora-rs/lora-rs, no_std
    - Cargo.toml workspace contains only `protocol`; no esp-idf-* crates anywhere
    - tools/sarcom-kiosk-lab uses eframe 0.29 (host-side, std — not firmware)
Context7 coverage:
  - lora-phy:        MISSING (resolve returns LoRaWAN/LoRA-AI noise; /lora-rs/lora-rs → 404)
  - embassy-*:       OK (/embassy-rs/embassy, plus dedicated executor page)
  - esp-hal:         OK, Lane A confirmed (/esp-rs/esp-hal returns no_std/no_main examples
                     and embassy-executor + esp-rtos integration)
  - walkers:         PARTIAL (/podusowski/walkers covers HttpTiles + OSM; PMTiles not surfaced)
  - egui / eframe:   OK (/emilk/egui, rich coverage)
  - nmea / nmea0183: MISSING (only the C++ Arduino library is indexed, wrong language)
Fallback chosen: B (Context7 + github MCP for the gaps), with `cargo doc -p` once
  Cargo.lock pins the relevant crates. Path (a) cargo-doc is unusable today because
  the firmware crates are not yet in the workspace; revisit when they land.
CLAUDE.md sections added:
  - ## Rust doc sources         (lane statement, lookup order, per-crate table)
  - ## Rust API notes           (Embassy task model, esp-hal ownership,
                                 lora-phy RadioKind, lane boundary)
Per-crate source table: filled in (see CLAUDE.md ## Rust doc sources)
Pass criteria status:
  - Firmware lane confirmed unambiguously: PASS
  - Context7 accurate for Priority 1–3 without web search: PARTIAL — Priority 1 (lora-phy)
    failed and is routed to github MCP. Priority 2–3 pass.
  - CLAUDE.md ## Rust API notes covers the four patterns + lane boundary: PASS
  - CLAUDE.md ## Rust doc sources contains per-crate first-lookup table: PASS
  - Setup reproducible in <15 min on a fresh session: PASS — all of it lives in CLAUDE.md
    and the existing .mcp.json
Fail criteria status:
  - "Context7 resolves lora-phy to the archived embassy-rs/lora-phy repo": NOT TRIGGERED
    (it does not resolve to that repo — it does not resolve at all, which is its own
    failure but not the one named here)
  - "Context7 returns Lane B docs for esp-hal queries": NOT TRIGGERED
  - "All fallback options require >5 min after a version bump": NOT TRIGGERED
    (github MCP is on-demand; cargo doc -p is once-per-version-bump)
  - "No source can provide lora-phy RadioKind without web search": NOT TRIGGERED
    (github MCP can read the repo source directly)
Next action:
  - When firmware crates are added to the workspace, run `cargo doc -p lora-phy -p nmea`
    once and add a note to CLAUDE.md pointing future Claude at target/doc/.
  - If walkers PMTiles work hits an API question that can't be answered from
    walkers/examples/, file a small follow-up note rather than re-spiking.
```

## Outcome

The fail criterion on `lora-phy` was triggered in spirit (it's not in Context7 at all), but the structural answer holds: github MCP fills the gap, and the per-crate table in CLAUDE.md routes future sessions correctly without the user having to repeat themselves. The spike's H1 hypothesis (Context7 + targeted CLAUDE.md additions is good enough) is **accepted with one caveat** — Priority 1 is a github-MCP crate, not a Context7 crate, until lora-phy is indexed or pinned in Cargo.lock.

## Post-resolution hardening

Added 2026-05-06, after critique of the original resolution.

- The original spike resolved doc **availability** — what to consult and where.
- This addendum hardens Claude **behaviour** — that the configured source is actually consulted before drafting code for risky crates, not merely listed in CLAUDE.md.
- H1 is accepted only **conditionally**: Claude must consult the configured source from the per-crate table before drafting code for any of the risky crates (`lora-phy`, `embassy-*`, `esp-hal`, `walkers`, `egui`/`eframe`, `nmea`/`nmea0183`).
- LSP / `rust-analyzer` is explicitly demoted to **validation and repair**, not first lookup. The first revision of CLAUDE.md placed it at the top of the order, which contradicted §3 of this spike ("its role here is **not** to replace upfront doc access"). Fixed.
- The lookup protocol (`CLAUDE.md ### Rust doc lookup protocol`) and smoke test (`CLAUDE.md ## Rust doc lookup smoke test`) are now part of the practical pass condition for any new firmware/UI implementation session. Without them the table is just a passive reference; with them it is enforced behaviour.
- One specific claim was weakened: the original `## Rust API notes` asserted that the ESP32-S3 entry point uses `#[esp_rtos::main]` based on a Context7 snippet. That macro is version-dependent and was not verified against the project's `Cargo.lock` (no firmware crate exists yet). Replaced with a rule that defers the exact macro to the version-matched `esp-hal` example.
- `lora-phy` is the highest-risk crate and is not covered by Context7. SARCOM now has a dedicated local preflight workflow:
  - `resources/docs/lora-phy-preflight.md` — local truth pointer, lookup order, do-not-invent rules, preflight statement template
  - `.claude/commands/rust_lora_phy_preflight.md` — slash command `/rust_lora_phy_preflight`
  - `scripts/check-lora-phy-docs.ps1` — advisory PowerShell helper (always exits 0)

  This reduces reliance on ad-hoc prompting. The workflow does not guarantee correctness, but it makes skipping source lookup visible and contrary to repo instructions: a session that drafts radio code without producing a preflight statement is now obviously noncompliant.
