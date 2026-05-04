---
title: "TODO — ordered backlog"
status: living
type: backlog
tags: [todo, backlog]
---

# TODO

Ordered by blocking dependency, not by calendar. Top = work on this next. If something is blocked, it lives in "Blocked" at the bottom until unblocked.

As of 2026-04-26 the ADR ledger runs 001–014. ADR-012 was superseded in part on 2026-04-26 by ADR-013 (multi-hop flood via packet_id dedup — one wire packet type, no role enum, no SIGHTING) and ADR-014 (duty-cycle budget table as mandatory protocol gate). The v1a/v1b split, tag SOS buzzer, and non-goals list from ADR-012 survive. Next phase is **ordering hardware** and **standing up the Rust workspace** while it ships. Phase boundaries below map 1:1 to the v0 / v0.5 / v1a / v1b acceptance gates in [ARCHITECTURE.md §15](ARCHITECTURE.md).

## Right now (this week)

- [x] Finalise [bom.md](bom.md) with Heltec DE cart sanity-checked against ADR-002 / ADR-003 / ADR-011 SKUs
- [ ] **Place the Heltec order** — per the "Cart sanity-check" list in [bom.md](bom.md): 3× Wireless Tracker V2 (EU 863–928 MHz variant; 2 tag + 1 relay), 1× Solar Kit for Dev-board, **1× IPEX1.0 → SMA female bulkhead pigtail**, **1× 868 MHz external SMA antenna** (for the relay's Solar Kit bulkhead), **1× 868 MHz SMA stubby antenna** (for the Dragino HAT). Per [ADR-002](decisions/ADR-002-tag-hardware.md) and [ADR-003](decisions/ADR-003-relay-hardware.md).
- [ ] **Place the parallel (Amazon / Tinytronics) order** — **6× Samsung INR18650-25R** or equivalent (2 tag + 2 relay + 2 spares; see [bom.md](bom.md) §Batteries), **1× DS3231 RTC module + 1× CR2032 coin cell** (per [ADR-011](decisions/ADR-011-gateway-time-source.md)), **1× small piezo active buzzer** (~€1; tag SOS audible cue per [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md)), M2.5 self-adhesive PCB standoffs + 3M VHB tape + M2.5×6 screws (relay mounting workaround per [ADR-003](decisions/ADR-003-relay-hardware.md) §Consequences), M2.5 brass standoff + screw kit (Pi + HAT + touchscreen).
- [ ] Place the desk-hygiene order: PH0 + PH00 precision screwdriver set, 40-pin M/F Dupont jumper set, fine tweezers, 2× quality USB-C data cables (not charge-only), 1× powered USB hub, 3× High Endurance microSD 32–64 GB, 1× CAT6 Ethernet, 1× official Pi PSU (verify 5V/3A USB-C vs 5V/2.5A micro-USB against the Pi model), Pi heatsink kit, USB current meter.
- [ ] Order the wooden pole + stainless hose clamps for the garden relay (local hardware store).
- [ ] Write a one-page desk-inventory note: which Pi model each of the 3 units is, which HAT has which bent pins, what's actually missing.

## Doc review findings — close before writing firmware

Six things the review flagged that are genuinely not yet in the docs or the lab. No ADR changes. No implementation. Just spec gaps and one stale line.

- [ ] **Staleness thresholds are wrong.** The kiosk-lab and UI mockups use ad-hoc values (e.g. "stale if > 2 min"). Normal heartbeat cadence is 300–330 s, so a healthy tag would flag stale before the next expected frame. Fix: derive thresholds from heartbeat cadence — fresh = < 330 s, aging = < 660 s, stale = < 22 min, very stale = > 22 min. SOS mode separate: SOS stale = > 3 min (expected every 45–60 s). Document in [ARCHITECTURE.md §11](ARCHITECTURE.md); fix constants in `tools/sarcom-kiosk-lab/src/data.rs`. **Kiosk-lab side fixed 2026-05-04** — `freshness_for_tag` / `freshness_for_relay` in `tools/sarcom-kiosk-lab/src/data.rs` use cadence-derived thresholds with unit tests. `ARCHITECTURE.md §11` still needs the matching update before this can be closed.

- [ ] **No-fix / SOS edge case is architecturally underspecified.** When a tag transitions from valid-fix to no-fix (e.g. SOS entry in a canyon), the architecture says it goes to the side list — but it does not say what happens to the existing map marker. Operator needs to see the last valid fix as a ghost/dashed marker, clearly labelled "last valid fix [time]", while the SOS banner and "no current fix" state show alongside it. Problem is: the UI read-model needs two distinct query results per node: `latest_report` (most recent row regardless of GPS state) and `latest_valid_fix` (most recent row with GPS_VALID=1). Define both in [ARCHITECTURE.md §11](ARCHITECTURE.md) before kiosk UI code hardens. **Kiosk-lab now has a `SOS + No Fix` mockup scenario (2026-05-04)** with ghost marker at last valid fix and dual-age display in sidebar; the architecture text is still open.

- [ ] **Relay runtime duty-cycle enforcement has no firmware spec.** [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) defines the design-time budget table. What the relay firmware should actually *do* when TX pressure approaches 1% is not written down anywhere. Needs a concrete rule in [ARCHITECTURE.md §9](ARCHITECTURE.md): rolling 1-hour airtime budget counter; SOS packets retry until queue expiry; heartbeat retransmits drop on budget overflow; self-announce drops first. Without this, firmware will implement something ad-hoc and inconsistent with the budget table.

- [x] **Clock-invalid scenario missing from kiosk-lab.** ~~[ADR-011](decisions/ADR-011-gateway-time-source.md) requires a "clock not set" banner and suppression of all relative-time strings when RTC is unavailable at boot. The kiosk-lab has Normal / SOS / Stale / No Fix / Multi-Tag scenarios but nothing for this state. Add a `ClockInvalid` scenario: header shows "RTC NOT SET", all "X ago" strings replaced with "time unavailable", map still renders markers from DB ordering.~~ Done 2026-05-04: `ScenarioKind::ClockInvalid` added; `SimState.clock_valid: bool` flag suppresses every relative-time string via `format_age_or_unavailable`; header shows `⚠ RTC NOT SET` + `CLOCK INVALID` in place of wall time.

- [ ] **README "Code: Not yet written" is now inaccurate.** The repo has `tools/sarcom-kiosk-lab`, `UX/`, `scripts/`. Fix the status row to: "Production firmware/gateway code not yet written. UX and tooling code exists under `tools/`."

- [ ] **Archived mockup-studio TOML export silently drops nodes.** `tomlExport.ts` filters on `['tag', 'relay', 'gateway']` — misses `hiker` and `drone-relay`. Tool is archived so don't fix the code; add one line to `tools/sarcom-mockup-studio/ARCHIVED.md` that export output is stale and should not be used as config input.

## While hardware is in transit (1–2 weeks)

- [ ] Stand up the Cargo workspace per [ARCHITECTURE.md §17](ARCHITECTURE.md). Crates (library-only): `protocol` (no_std, with optional `std` feature), `persistence` (std), `heltec-wireless-tracker-v2-bsp`. Binaries: `firmware/tag/`, `firmware/relay/` (both Xtensa), `gateway/` (aarch64 or armv7; the kiosk is a module at `gateway/src/ui/`, not a separate crate). `cargo check` green on all targets. **In progress: workspace root + `crates/protocol` done (2026-05-04). Remaining: `persistence`, BSP stubs, firmware/gateway crate skeletons.**
- [ ] Write [ARCHITECTURE.md §13](ARCHITECTURE.md) (duty-cycle budget table) as the source of truth before any forwarding code is written. Confirm single-tag SOS rebroadcast fits within 1%. Document the canonical airtime calculator parameters (SF10 / 125 kHz / CR 4/5 / preamble 8 / explicit header / CRC on / LDRO off). See [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md).
- [x] Write the `protocol` crate: ONE packet type — **POSITION** (16 B payload / 22 B frame, `seq_nr: u32 BE`, layout per [ARCHITECTURE.md §7](ARCHITECTURE.md) / [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) §3). **CRC-16/CCITT-FALSE** (poly 0x1021, init 0xFFFF, no reflect, xorout 0x0000; check value 0x29B1 for "123456789"). Canonical vectors frozen: heartbeat `A5 01 01 10 01 00 00 00 2A 01 1C 16 78 07 08 16 4B B5 07 37 **29 04**`; SOS `…03… **89 B7**`. Pure relay decision logic + 32-entry ring-buffer SeenCache (u32 timestamps, 60 s expiry, oldest-evict). 22 tests pass; clippy -D warnings clean. Done 2026-05-04. Note: SOS jitter/duty-cycle tests in the `tag` crate are still pending (hardware not yet started).
- [ ] Define the gateway's `nodes.toml` schema: `node_id` → `label` + `ui_kind` (`hiker` / `relay` / `drone-relay`). Document in [ARCHITECTURE.md §11](ARCHITECTURE.md). Per [ADR-013 §9](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- [ ] Pick and pin `esp-hal`, Embassy, and `lora-phy` versions. Use `lora-phy` **from `lora-rs/lora-rs`** (the `embassy-rs/lora-phy` repo is archived). Record exact commit hashes for reproducibility. Note `lora-phy` will be used on the **gateway** too — it supports SX127x as well as SX126x.
- [ ] Install the `esp-rs` / `espup` toolchain for the `xtensa-esp32s3-none-elf` target; verify a minimal blink builds.
- [ ] First Yocto image experiment: build a minimal image with `meta-raspberrypi` + `meta-rust`, boot it on one of the Pis, verify the touchscreen comes up. Add a device-tree overlay for `i2c-rtc,ds3231` and wire `hwclock --hctosys` into the boot path per [ADR-011](decisions/ADR-011-gateway-time-source.md).
- [ ] Kiosk UI spike: `egui` + `walkers` hello-world rendering a **local `.pmtiles` archive** (per [ADR-005](decisions/ADR-005-map-and-ui.md)) on a laptop first, then on the Pi. If PMTiles does not render on the Pi's GPU stack, the fallback is writing a custom MBTiles tile provider — this must be resolved before any other kiosk work lands.
- [ ] `persistence` crate first draft: single core table `tag_reports` matching [ARCHITECTURE.md §10](ARCHITECTURE.md) per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) — `seq_nr` as u32-in-INTEGER, **no permanent UNIQUE dedup index**, recent-window dedup via `INSERT ... WHERE NOT EXISTS` keyed by `(node_id, seq_nr)`. (Tags and relays both produce rows here; presentation distinguished by `nodes.toml`.) A future reception-log table for per-hop RSSI/SNR coverage analysis is explicitly out of v1 — see [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

## v0 — when hardware arrives (three nodes talking on the desk)

Matches [ARCHITECTURE.md §15 v0](ARCHITECTURE.md). Goal: gateway stdout shows a parsed POSITION frame from the tag via the relay, CRC-verified.

- [ ] First-boot smoke test on a Wireless Tracker V2 over USB-C — verify `espflash board-info` sees it, flash a blink.
- [ ] Tag bring-up: NMEA from UC6580 reaches the MCU; LoRa TX fires sentinel frames on 868.1 MHz; packet visible on an SDR or second Tracker V2 in RX.
- [ ] Relay bring-up: RX from tag on 868.1 MHz, validation (MAGIC / VER / TYPE / LEN / CRC-16/CCITT-FALSE), seen_cache lookup on `(node_id, seq_nr)`, byte-identical rebroadcast on the same channel. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) §5.
- [ ] Gateway bring-up: RX from relay on the Dragino HAT (via `lora-phy` on SX1276), parse, stdout print — **no DB yet** at v0.
- [ ] DS3231 RTC wired to the Pi I²C bus; verify the kernel reads it across a power cycle and the coin-cell path works.
- [ ] Mount the relay: adhesive PCB standoffs + 3M VHB in the Solar Kit enclosure, IPEX1.0→SMA pigtail from the Tracker V2 LoRa port to the Solar Kit bulkhead, external 868 MHz antenna fitted, solar lead into the Solar Kit charge controller, 18650 pack to Tracker V2 battery input, seal the enclosure.
- [ ] Straighten the Dragino HAT pins, fit the HAT to the chosen Pi, mount the Pi to the touchscreen back panel.

## v0.5 — gateway writes to SQLite, kiosk renders a marker

Matches [ARCHITECTURE.md §15 v0.5](ARCHITECTURE.md). Goal: a dot appears on the touchscreen for every packet the gateway receives, rendered from a hardcoded valid test coordinate (`GPS_VALID=1`) — never from sentinel values.

- [ ] Gateway writes to `tag_reports` using the `persistence` crate; recent-window dedup on `(node_id, seq_nr)` works correctly (retransmits inside 24 h suppressed; wrap-after-reset accepted).
- [ ] Kiosk module in the gateway binary reads SQLite and draws a marker. Verify that sentinel-coordinate frames go to the "no fix" side list and **never** produce a map marker.
- [ ] Clock-not-set banner renders when the RTC is absent at boot.
- [ ] `nodes.toml` is loaded at boot; `node_id` → `label` + `ui_kind` controls the marker glyph (hiker dot / pole icon / drone icon).

## v1a — single-relay garden test (multi-hop protocol, single hop physically deployed)

Matches [ARCHITECTURE.md §15 v1a](ARCHITECTURE.md). Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md): one wire packet type (POSITION), packet_id dedup, no role byte. Tag + 1 relay + gateway physically; protocol multi-hop-capable from day one.

- [ ] Garden deployment: relay on the wooden pole, gateway indoors near the window, tag in the pocket with real GNSS.
- [ ] Relay rebroadcasts received POSITIONs by `packet_id`, dedup'd locally on `(node_id, seq_nr)` with 60 s expiry.
- [ ] Gateway dedup'd writes to `tag_reports`.
- [ ] Kiosk shows the dot moving as the user walks, with correct "last seen X ago" timestamps from the RTC-disciplined clock; node icon and label come from `nodes.toml`.
- [ ] 72-hour unattended solar soak on the relay; still rebroadcasting at the end of the window.
- [ ] Tag emits a buzzer pulse pattern on entering SOS state and falls silent on exit. Bench-verify before garden test.
- [ ] Deliberate SOS trigger (bench switch if the button isn't wired yet) flips marker to red within `DISTRESS_WINDOW` and clears correctly.
- [ ] Duty-cycle behaviour matches [ARCHITECTURE.md §13](ARCHITECTURE.md) within ±10% measured.
- [ ] Entire stack has never touched the internet during the test.

## v1b — two-relay chained garden test (drone-pod overlay)

Matches [ARCHITECTURE.md §15 v1b](ARCHITECTURE.md). Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md): a 3rd Tracker V2 with a drone-pod `node_id` (`ui_kind = "drone-relay"` in `nodes.toml`) — same relay firmware, different node_id, no separate "AERIAL" build.

**Do not begin v1b firmware work before v1a hard gates pass.** This is the structural anti-creep gate.

- [ ] Flash the 3rd Wireless Tracker V2 with the same relay firmware, just a different `node_id`.
- [ ] Add the drone-pod `node_id` to `nodes.toml` with `ui_kind = "drone-relay"`.
- [ ] Optional barometer (BMP280 or BME280) wired on I²C for altitude observation; if absent, GNSS altitude is the fallback.
- [ ] Drone-pod 1S LiPo (~500–800 mAh) wired to the Tracker V2 battery input, independent of the drone's flight battery. Simple under-mount via zip-ties or velcro.
- [ ] Garden test: tag placed in a deliberate blind spot the gateway and the paal-relay both cannot hear directly (verified: gateway alone sees no packets); drone airborne overhead bridges the gap.
- [ ] Both relays rebroadcast; loop prevention exercised by a deliberate test where both relays hear each other and the seen_cache catches the loop (`DUP` log line on the second hearing of each `packet_id`).
- [ ] Kiosk renders the drone-pod with the drone icon at its current self-announced position; tag dot appears via the chain.
- [ ] Duty-cycle behaviour at the relays matches [ARCHITECTURE.md §13](ARCHITECTURE.md) within ±10% measured.

## Housekeeping debt (do any time)

- [x] Archive `zephyrOS_study.md` (Zephyr is ruled out in ADR-001)
- [x] Archive `product-roadmap.md` (calendar roadmap replaced by this TODO)
- [x] Add `tools/sarcom-mockup-studio/` — browser-based drag-and-drop mockup tool (Vite + React + tldraw). Not the production kiosk. Run with `cd tools\sarcom-mockup-studio && npm install && npm run dev`.
- [ ] Consider splitting `ARCHITECTURE.md` into `architecture/{system-overview,protocol,operational-modes,non-goals}.md` once it becomes unwieldy. Today, one file is fine.
- [ ] Pick repo name, logo, GitHub org for the code

## Blocked

- [ ] SOS button wiring — waiting on a decision for which GPIO on the Tracker V2 + button type (tactile momentary vs. latched vs. magnetic)
- [ ] Commissioning trigger mechanism — leading candidate is magnet + reed switch (works through sealed IP67 enclosure), final choice deferred to relay bring-up
- [ ] Field deployment in real mountains — blocked on garden v1a passing (v1b drone overlay desirable but not required for mountain field deployment)

## Deferred (v2+)

Explicitly NOT in v1 scope. Listed here so they don't rot in open tabs.

- **Reception-log / coverage-analysis layer.** Per-hop RSSI/SNR for who-heard-what coverage science. Designed when v1 forwarding is working and the analysis question is concrete. Not part of v1 protocol. See [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- BLE maintenance CLI on the relay (service engineer stands next to the pole with a phone/laptop, reads battery mV / RX count / last RSSI, triggers a fresh commissioning broadcast)
- Phone-friendly read-only map view (a local HTTP server on the gateway, reached from a phone on the hut's WiFi)
- Cloud sync of the SQLite tag_reports to a Postgres backend
- Downlink control (gateway → tag commands): cadence change, SOS escalation
- HMAC per packet, key management, tile auth
- Multi-gateway topology
- OTA firmware updates
- FreeRTOS / advanced power management
