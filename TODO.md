---
title: "TODO — ordered backlog"
status: living
type: backlog
tags: [todo, backlog]
---

# TODO

Ordered by blocking dependency, not by calendar. Top = work on this next. If something is blocked, it lives in "Blocked" at the bottom until unblocked.

As of 2026-04-26 the ADR ledger runs 001–014. ADR-012 was superseded in part on 2026-04-26 by ADR-013 (multi-hop flood via packet_id dedup — one wire packet type, no role enum, no SIGHTING) and ADR-014 (duty-cycle budget table as mandatory protocol gate). The v1a/v1b split, tag SOS buzzer, and non-goals list from ADR-012 survive. Phase boundaries below map 1:1 to the v0 / v0.5 / v1a / v1b acceptance gates in [ARCHITECTURE.md §15](ARCHITECTURE.md).

**2026-05-06 form-factor pivot.** Gateway is now a handheld portable Rust device with a touchscreen, battery + USB-C charging, custom 3D-printed waterproof enclosure, and an ADR-016-gated outbound LAN CoT/TAK export path. Three new ADRs proposed (ADR-015 substrate + form factor; ADR-016 export gate; ADR-017 enclosures). Until those land, items below cite "(pending ADR-015)" / "(pending ADR-016)" parentheticals where the pre-pivot text named a fixed-kiosk substrate or banned outbound network calls categorically. See [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md) for the per-file edit checklist this section folds in, and the open-spikes block immediately below for the active research lanes.[^pivot]

[^pivot]: 2026-05-06 form-factor pivot — see [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md). Three new ADRs proposed: ADR-015 (handheld substrate + form factor; supersedes-in-part ADR-004; refines-in-part ADR-005/006/007), ADR-016 (base-mode export gate; supersedes-in-part ADR-008), ADR-017 (custom 3D-printed waterproof enclosures for gateway and tag; refines-in-part ADR-002).

## Right now (this week)

- [x] Finalise [bom.md](bom.md) with Heltec DE cart sanity-checked against ADR-002 / ADR-003 / ADR-011 SKUs
- [x] **Heltec order #110639 shipped 2026-05-05** — 10× Wireless Tracker V2 (EU 863–870 MHz variant) + 2× Solar Kit for Dev-board, ETA roughly 2026-05-19 → 2026-06-02, $403.40 via Bancontact (per [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md)). Pigtails + antennas were also on the cart; verify on receipt.
- [ ] **Place the parallel (Amazon / Tinytronics) order** — **6× Samsung INR18650-25R** or equivalent (2 tag + 2 relay + 2 spares; see [bom.md](bom.md) §Batteries), **1× DS3231 RTC module + 1× CR2032 coin cell** (per [ADR-011](decisions/ADR-011-gateway-time-source.md)), **1× small piezo active buzzer** (~€1; tag SOS audible cue per [ADR-012](decisions/ADR-012-node-roles-and-sighting-semantics.md) — buzzer survives the ADR-013 rollback), M2.5 self-adhesive PCB standoffs + 3M VHB tape + M2.5×6 screws (relay mounting workaround per [ADR-003](decisions/ADR-003-relay-hardware.md) §Consequences), M2.5 brass standoff + screw kit (Pi + HAT + touchscreen).
- [ ] Place the desk-hygiene order: PH0 + PH00 precision screwdriver set, 40-pin M/F Dupont jumper set, fine tweezers, 2× quality USB-C data cables (not charge-only), 1× powered USB hub, 3× High Endurance microSD 32–64 GB, 1× CAT6 Ethernet, 1× official Pi PSU (verify 5V/3A USB-C vs 5V/2.5A micro-USB against the Pi model), Pi heatsink kit, USB current meter.
- [ ] **Relay tripod + Solar Kit adapter** — off-the-shelf plastic tripod, selection owned by [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md). Wooden-pole + Fusion-360 designed-pole approaches retired 2026-05-08 — see [`dev-log/2026-05-08-relay-mount-tripod-decision.md`](dev-log/2026-05-08-relay-mount-tripod-decision.md).
- [x] Pi 4 substrate tested out of order 2026-05-07; substrate moves to Pi 5 per [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md).
- [ ] **Inspect the 3× Dragino HAT silkscreen revisions** for the GPIO 25 CS-routing defect flagged in [`spikes/gateway-rx-bringup-spike.md`](spikes/gateway-rx-bringup-spike.md) B1. Record per-unit rev in `hardware/pi{1,2,3kiosk}/specs.md` (per dev-log D3). The HATs are physically intact per `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`; only the Pi 4 hosts were dead.
- [ ] Write a one-page desk-inventory note: Pi model + HAT rev + bent-pin status + missing parts per unit.

## Carry-over voor volgende CAD sessie (2026-05-15+)

Geconsolideerde blocker-lijst na de 2026-05-14 sessies. Lees eerst de drie dev-logs van die dag in chronologische volgorde voor context: [`c1-depth-stackup-arithmetic`](dev-log/2026-05-14-c1-depth-stackup-arithmetic.md) → [`pogo-drop-and-shell-extrudes`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md) → [`anker-dims-and-gate-propagation`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md). Pickup morgen: lees in die volgorde, dan deze lijst.

In prioriteit-volgorde:

1. **Orientation X vs Y (Pi/HAT facing display).** Foundational blocker — gates alle interne mounting bosses; gates de oplossing van #2 hieronder. Pi 5 bottom-side (USB-A/RJ45) face naar display = Orientation X (standoff ~14-15 mm to clear connectors, allows HAT stacking on top); Pi 5 top-side (GPIO header) naar display = Orientation Y (standoff ~5-8 mm, but HAT cannot stack on the GPIO side anymore — needs side-mount). Pieter-keuze; alle interne feature-extrudes wachten hierop. Cross-ref [`dev-log/2026-05-14-c1-depth-stackup-arithmetic.md`](dev-log/2026-05-14-c1-depth-stackup-arithmetic.md) §Layout / orientation uncertainty.

2. **Front-depth squeeze: -5 mm tekort.** `front_depth = 60 mm` extern → 57 mm intern; honest stack-up van display (15) + Pi+HAT (27) + standoff Orientation X (~14-15) + window (3) + clearances (~3) = ~62 mm benodigd. Resolutie afhankelijk van Orientation: X = 5 mm tekort (front_depth moet groeien naar ~65 mm); Y = ~5 mm marge (HAT side-mount frees up vertical stack). Cross-ref audit-filter tabel in [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md) P1.1 verdict — Autodesk Assistant audit's "11 mm slack" was wrong (forgot the standoff).

3. **Rear-shell / front-shell X-asymmetry interpretatie.** 1.5 mm step aan parting plane: front-shell-body bbox = ±90 (= outer_w/2), rear-shell-body bbox = ±88.5 (offset inward 1.5 mm). Drie opties: (a) intentionele tongue-and-groove parting plane wrap voor IP65 sealing; (b) accidenteel sketch-source verschil (rear extruded from inner-gasket-offset profile instead of outer envelope); (c) onbekend-maar-werkt. Verifieer welke sketch + dimensie-expressie de outer envelope definieert in beide shells voordat IP65 als "done" geclaimed wordt. Cross-ref [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md) §"Front-shell / rear-shell X-asymmetry geometric audit".

4. **Rear compartment slack (30 mm X-axis).** A1689 echte lengte 119.9 mm vs originele aanname 154 mm laat ~30 mm overschot langs device-X in de rear compartment. Drie opties: (a) device-footprint krimpen (180 → ~150 mm langs X) voor sneller-print prototype; (b) slack laten staan voor cable-routing marge en design-flex; (c) hergebruiken voor stowage (commissioning magnet holster, silica gel pocket, spare antenna). Architectuur-keuze, Pieter. Cross-ref [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md) §(a).

5. **Sketch origin hygiëne op `door-profile`.** Sketch origin ligt niet op de geometric center van de deur — door spans sketch-X (-38 to -2), centered op sketch-X = -20. Boss circles op sketch-X = -31.5 en -8.5 (beide 11.5 mm van door center, dus geometrisch symmetrisch). Cosmetisch verwarrend (audit-bot las het als asymmetric vanaf sketch origin). Voorkom toekomstige verwarring door sketch+coordinates te alignen: translate sketch zodat door center op (0, 0) ligt; verifieer dependent features (gasket groove offset curves, body bbox, `battery-door-relocate` Move feature) blijven correct resolven. CLI-uitvoerbaar zodra Orientation (#1) gevallen is (door-area-X is independent van Pi+HAT orientation choice, dus kan ook parallel).

6. **Heat-spreader pocket volume delta — open hypothese (low priority).** Cut removed 11,939 mm³ uit rear-shell-body; ideale pocket math (80 × 60 × 1.5) = 7,200 mm³. Forensic op 2026-05-14: pocket geometry correct (footprint 80×60, depth 1.5 mm, no through-cut, no participating second body), maar 4 cavity-interior faces (Z=-20, Z=-18.5, Z=0, allen centroid bij origin) verdwenen tijdens de cut en 1 onverklaard kleine face verscheen op Z=-39.25 (52 mm²). Working hypothese: Fusion's BREP solver collapsed kleine sliver-topology in de shell-feature's interior cavity tijdens de cut-operation, account voor de extra ~4,739 mm³. Pocket is functioneel correct; delta is geometrische ruis, niet ontwerpgebrek. Cross-ref [`dev-log/2026-05-14-anker-dims-and-gate-propagation.md`](dev-log/2026-05-14-anker-dims-and-gate-propagation.md) §"Heat-spreader pocket cut (audit C4 fix)" en de End-of-day forensic sectie. Investigate verder alleen als pocket-related geometry issues opduiken in latere features.

## Open spikes (handheld pivot)

The 2026-05-06 pivot opened a research lane, not a decision. Each spike below is enumeration-only at open and converts to a decision-note + follow-up at close. None of them edits ADRs inline.

- [ ] [`spikes/handheld-pivot-doc-audit-spike.md`](spikes/handheld-pivot-doc-audit-spike.md) — **closed 2026-05-07** ([`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](dev-log/2026-05-07-handheld-pivot-doc-audit-close.md)). Cascading non-ADR doc edits in flight.
- [ ] [`spikes/gateway-handheld-substrate-spike.md`](spikes/gateway-handheld-substrate-spike.md) — substrate ranking; feeds ADR-015. Pi 4 retired 2026-05-07; spike is unblocked; candidates are Pi 5 / Pi 5+USB / CM5 / Zero 2W. See [`dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](dev-log/2026-05-07-pi4-retirement-substrate-decision.md).
- [ ] [`spikes/gateway-handheld-power-architecture-spike.md`](spikes/gateway-handheld-power-architecture-spike.md) — battery topology + protections + signal contract (~~POWER_GOOD, BATTERY_STATE, CHARGE_STATE~~ — all retired from firmware surface 2026-05-14 after pogo drop; THERMAL_STATE, SHUTDOWN_REQUEST remain); feeds ADR-015 + ADR-016. Pogo-drop + gate re-scope per [`dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md).
- [ ] [`spikes/gateway-handheld-enclosure-spike.md`](spikes/gateway-handheld-enclosure-spike.md) — IP target, material, sealing strategy, bulkhead inventory; feeds ADR-017.
- [ ] [`spikes/tag-handheld-enclosure-spike.md`](spikes/tag-handheld-enclosure-spike.md) — pocket form factor, SOS button physical type, buzzer port, tag identity surface; feeds ADR-017 + ADR-002 annex.
- [ ] [`spikes/ble-commissioning-scope-spike.md`](spikes/ble-commissioning-scope-spike.md) — gateway-as-central / relay+tag-as-peripheral topology, just-works + commissioning-window auth, write allow-list; firmware-side contract.
- [ ] [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md) — UI-side flow for the same: long-press marker → Relay Health modal (verdict + live state + recent activity + actions drawer); preserves ADR-007 read-only map.
- [ ] [`spikes/tak-cot-integration-spike.md`](spikes/tak-cot-integration-spike.md) — outbound LAN CoT/TAK export under the ADR-016 gate; Phase 1 multicast experiment runs in parallel with v1a firmware bring-up.
- [ ] [`spikes/gateway-runtime-task-architecture-spike.md`](spikes/gateway-runtime-task-architecture-spike.md) — task split + channel contracts + clean-shutdown semantics; allocates `cot_gate` / `cot_emitter` / `power_monitor` / `wifi_monitor` / `ble_central` / `shutdown_orchestrator` task slots.
- [ ] [`spikes/fake-position-injector-spike.md`](spikes/fake-position-injector-spike.md) — synthetic POSITION source for e2e relay/gateway testing; the `node_id` width example needs the u8 / u32 fix (dev-log A11).
- [ ] [`spikes/pmtiles-walkers-spike.md`](spikes/pmtiles-walkers-spike.md) — display-class retarget for the handheld panel + walkers' PMTiles tile-source verification on the chosen Pi GPU.
- [ ] [`spikes/duty-cycle-measurement-workflow-spike.md`](spikes/duty-cycle-measurement-workflow-spike.md) — ADR-014 enforcement workflow; unchanged by the pivot.
- [ ] [`spikes/datasheet-source-of-truth-inventory-spike.md`](spikes/datasheet-source-of-truth-inventory-spike.md) — central inventory the substrate + enclosure spikes cite; unchanged by the pivot.
- [ ] [`spikes/physical-fabrication-brief-spike.md`](spikes/physical-fabrication-brief-spike.md) — Fusion 360 brief that ADR-017 commits to.

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
- [ ] First Yocto image experiment: build a minimal image with `meta-raspberrypi` + `meta-rust`, boot it on the Pi (Pi 5 per retirement; target scarthgap or later for `meta-raspberrypi` Pi 5 support), verify the touchscreen comes up (display class open per pending ADR-015). Add a device-tree overlay for `i2c-rtc,ds3231` and wire `hwclock --hctosys` into the boot path per [ADR-011](decisions/ADR-011-gateway-time-source.md).
- [ ] Kiosk UI spike: `egui` + `walkers` hello-world rendering a **local `.pmtiles` archive** (per [ADR-005](decisions/ADR-005-map-and-ui.md)) on the dev workstation first, then on the Pi. Display-class retarget for a handheld panel is owned by [`spikes/pmtiles-walkers-spike.md`](spikes/pmtiles-walkers-spike.md) (pending ADR-015). If PMTiles does not render on the Pi's GPU stack, the fallback is writing a custom MBTiles tile provider — this must be resolved before any other kiosk work lands.
- [ ] `persistence` crate first draft: single core table `tag_reports` matching [ARCHITECTURE.md §10](ARCHITECTURE.md) per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) — `seq_nr` as u32-in-INTEGER, **no permanent UNIQUE dedup index**, recent-window dedup via `INSERT ... WHERE NOT EXISTS` keyed by `(node_id, seq_nr)`. (Tags and relays both produce rows here; presentation distinguished by `nodes.toml`.) A future reception-log table for per-hop RSSI/SNR coverage analysis is explicitly out of v1 — see [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md).

## v0 — when hardware arrives (three nodes talking on the desk)

Matches [ARCHITECTURE.md §15 v0](ARCHITECTURE.md). Goal: gateway stdout shows a parsed POSITION frame from the tag via the relay, CRC-verified.

- [ ] First-boot smoke test on a Wireless Tracker V2 over USB-C — verify `espflash board-info` sees it, flash a blink.
- [ ] Tag bring-up: NMEA from UC6580 reaches the MCU; LoRa TX fires sentinel frames on 868.1 MHz; packet visible on an SDR or second Tracker V2 in RX.
- [ ] Relay bring-up: RX from tag on 868.1 MHz, validation (MAGIC / VER / TYPE / LEN / CRC-16/CCITT-FALSE), seen_cache lookup on `(node_id, seq_nr)`, byte-identical rebroadcast on the same channel. Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) §5.
- [ ] Gateway bring-up: RX from relay on the Dragino HAT (via `lora-phy` on SX1276), parse, stdout print — **no DB yet** at v0. (Polled RX is the v1a default per the Pi 5 / RP1 dev-log finding; interrupt-driven RX is deferred. Pi class itself is open per pending ADR-015.)
- [ ] DS3231 RTC wired to the Pi I²C bus; verify the kernel reads it across a power cycle and the coin-cell path works.
- [ ] Mount the relay: adhesive PCB standoffs + 3M VHB in the Solar Kit enclosure, IPEX1.0→SMA pigtail from the Tracker V2 LoRa port to the Solar Kit bulkhead, external 868 MHz antenna fitted, solar lead into the Solar Kit charge controller, 18650 pack to Tracker V2 battery input, seal the enclosure. (Relay enclosure stays OEM Solar Kit per [ADR-003](decisions/ADR-003-relay-hardware.md); the gateway and tag get custom 3D-printed shells under pending ADR-017.)
- [ ] Straighten the Dragino HAT pins, fit the HAT to the chosen Pi (substrate + form factor open per pending ADR-015 — the prior "fit the HAT to the touchscreen back panel" wording is no longer the working assumption).

## v0.5 — gateway writes to SQLite, kiosk renders a marker

Matches [ARCHITECTURE.md §15 v0.5](ARCHITECTURE.md). Goal: a dot appears on the touchscreen for every packet the gateway receives, rendered from a hardcoded valid test coordinate (`GPS_VALID=1`) — never from sentinel values.

- [ ] Gateway writes to `tag_reports` using the `persistence` crate; recent-window dedup on `(node_id, seq_nr)` works correctly (retransmits inside 24 h suppressed; wrap-after-reset accepted).
- [ ] Kiosk module in the gateway binary reads SQLite and draws a marker. Verify that sentinel-coordinate frames go to the "no fix" side list and **never** produce a map marker.
- [ ] Clock-not-set banner renders when the RTC is absent at boot.
- [ ] `nodes.toml` is loaded at boot; `node_id` → `label` + `ui_kind` controls the marker glyph (hiker dot / pole icon / drone icon).

## v1a — single-relay garden test (multi-hop protocol, single hop physically deployed)

Matches [ARCHITECTURE.md §15 v1a](ARCHITECTURE.md). Per [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md): one wire packet type (POSITION), packet_id dedup, no role byte. Tag + 1 relay + gateway physically; protocol multi-hop-capable from day one.

- [ ] Garden deployment: relay on the designed pole, gateway carried by the operator (or placed near a window during early bring-up — handheld substrate per pending ADR-015), tag in the pocket with real GNSS.
- [ ] Relay rebroadcasts received POSITIONs by `packet_id`, dedup'd locally on `(node_id, seq_nr)` with 60 s expiry.
- [ ] Gateway dedup'd writes to `tag_reports`.
- [ ] Kiosk shows the dot moving as the user walks, with correct "last seen X ago" timestamps from the RTC-disciplined clock; node icon and label come from `nodes.toml`.
- [ ] 72-hour unattended solar soak on the relay; still rebroadcasting at the end of the window.
- [ ] Tag emits a buzzer pulse pattern on entering SOS state and falls silent on exit. Bench-verify before garden test.
- [ ] Deliberate SOS trigger (bench switch if the button isn't wired yet) flips marker to red within `DISTRESS_WINDOW` and clears correctly.
- [ ] Duty-cycle behaviour matches [ARCHITECTURE.md §13](ARCHITECTURE.md) within ±10% measured.
- [ ] Entire stack has never touched the internet during the test on the LoRa side. The base-mode CoT/TAK export path (pending ADR-016) is OFF by default — explicitly verify it does not emit unless WiFi + manual opt-in are both true. (Gate language re-scoped from 3 inputs to 2 on 2026-05-14 after pogo drop; see [`dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`](dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md).)

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

- [ ] SOS button wiring — waiting on a decision for which GPIO on the Tracker V2 + button type. **Physical button choice (sealed tactile vs. silicone membrane vs. magnetic) is owned by [`spikes/tag-handheld-enclosure-spike.md`](spikes/tag-handheld-enclosure-spike.md)** per the audit close + dev-log D8; GPIO assignment is unblocked separately and can be picked tentatively now to avoid blocking tag firmware bring-up.
- [ ] Commissioning trigger mechanism — leading candidate is magnet + reed switch (works through sealed IP67 enclosure for the relay; the gateway-side equivalent is owned by [`spikes/gateway-handheld-enclosure-spike.md`](spikes/gateway-handheld-enclosure-spike.md) — sealed tactile button vs. magnet+reed). Final choice deferred to relay bring-up.
- [ ] Field deployment in real mountains — blocked on garden v1a passing (v1b drone overlay desirable but not required for mountain field deployment).

## Deferred (v2+)

Explicitly NOT in v1 scope. Listed here so they don't rot in open tabs.

- **Reception-log / coverage-analysis layer.** Per-hop RSSI/SNR for who-heard-what coverage science. Designed when v1 forwarding is working and the analysis question is concrete. Not part of v1 protocol. See [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md).
- **Terrain-constrained empirical RSSI fingerprinting voor no-GPS-fix tag localization (v2+ research lane).** Bayesian fusion van last-known-GPS-fix + relay-sector observations (per de RF-switch + 4-sector-antenne aanpak) + multi-receiver RSSI signatures + DEM- en trail-network-priors → particle filter localization op een terrain-constrained manifold. Bootstrap via DEM + propagation simulation (Longley-Rice / ITM); refine met crowd-sourced empirical fingerprints uit normal-mode tag walks die GPS fix hebben. Particle filter draait op de Pi 5 gateway. Past architecturaal in [ADR-013 §10](decisions/ADR-013-multi-hop-flood-via-packet-id.md) deferred reception-log telemetry lane (RELAY_OBSERVATION packets met per-antenne RSSI per tag) plus een nieuwe terrain-data-asset pipeline (DEM tiles + trail-network GeoJSON + pre-computed synthetic fingerprint baseline gebundeld in de Yocto image, naast de bestaande PMTiles). Path-loss-based multilateration expliciet afgewezen ten gunste van empirisch terrein-bewuste aanpak. Eerst v1a + v1b passing; eerst reception-log layer ontworpen; eerst empirisch bewijs uit garden + terril testing dat no-GPS-fix scenarios echt voorkomen voordat investment-decision. Bron: chat-thread 2026-05-14 late-night brainstorm tussen Pieter + Claude.
- **Software-sim test track (parallel to H1 fake-tag).** Deferred — post-v1a, picked up after the H1 fake-tag RF path (per [`spikes/fake-position-injector-spike.md`](spikes/fake-position-injector-spike.md)) lands or is explicitly chosen. Source: the fake-tag spike currently flags H1 as the primary/default fake-tag RF path; software-sim is a separate, larger future investment with its own scoping needs.
  - **Prerequisites to evaluate before scoping:**
    - `crates/protocol` stable and host-runnable (encoder, CRC layer, `node_id` type exposed).
    - Clear radio boundary in tag/relay logic so a host-side `MockRadio` or virtual radio can substitute for real `lora-phy` hardware during simulation.
    - Host-runnable time/peripheral abstraction for firmware logic; do not assume a specific executor until scoped.
    - Gateway receiver path pluggable to a virtual channel (e.g. `mpsc`) instead of only SPI to SX1276.
  - **What this unlocks that H1 cannot:**
    - Multi-node mesh tests at scale: N synthetic nodes, zero hardware.
    - Deterministic packet loss / collision / relay-dropout injection.
    - Invariant testing of dedup/forwarding behaviour across the mesh; property-based testing is a candidate, not yet a dependency decision.
    - Full pipeline-style tests per commit: protocol → relay logic → gateway ingest → SQLite → kiosk-readable test DB.
    - Per-developer local runs of the system model on a laptop.
  - **Reuse from H1 design:**
    - Scenario TOML format.
    - Reserved test-only `node_id` range, subject to [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) and protocol-crate type confirmation.
    - Verification surface contracts: `tag_reports` row, gateway log, kiosk render / read-model check.
  - **What this does NOT replace:**
    - H1's coverage of the real SX1262 driver path.
    - `lora-phy` SPI calls and radio init.
    - Real IRQ/timing behaviour.
    - Real walking-tag validation of GNSS, RF propagation, antenna/enclosure effects, and power behaviour.
  - **First step when picked up:**
    - Create a separate scoping spike.
    - Confirm the radio trait/boundary shape in firmware.
    - Pick a virtual-channel model.
    - Decide whether invariant/property-style testing is worth introducing.
    - Do not start with implementation work.
  - **Cross-refs:** [`spikes/fake-position-injector-spike.md`](spikes/fake-position-injector-spike.md); [ADR-013](decisions/ADR-013-multi-hop-flood-via-packet-id.md) (single packet type, dedup by `(node_id, seq_nr)`); [ADR-014](decisions/ADR-014-duty-cycle-budget-as-gate.md) (duty-cycle gate — does not gate pure-sim airtime, but sim scenarios should mirror RF cadences for parity).
- ~~BLE maintenance CLI on the relay~~ — **moved to v1** per [ADR-006](decisions/ADR-006-relay-has-gnss.md); contradiction A2 resolved on the v1 side. Topology is gateway-as-central + relay/tag-as-peripherals (no phone in v1) per [`spikes/ble-commissioning-scope-spike.md`](spikes/ble-commissioning-scope-spike.md); UI flow per [`spikes/ble-gateway-ui-flow-spike.md`](spikes/ble-gateway-ui-flow-spike.md). The original phone/laptop-as-BLE-peer formulation stays out of v1.
- ~~Phone-friendly read-only map view (a local HTTP server on the gateway, reached from a phone on the hut's WiFi)~~ — **partially superseded** by the pending ADR-016 outbound-LAN CoT/TAK export path. Phones running ATAK / iTAK / WinTAK on the same WiFi will see SARCOM tag positions through that export; the gateway does **not** host an inbound HTTP server (ADR-008 categories (a)/(b)/(c) stay closed). A native phone-app or in-house phone-friendly HTTP UI remains v2+.
- Cloud sync of the SQLite tag_reports to a Postgres backend
- Downlink control (gateway → tag commands): cadence change, SOS escalation
- HMAC per packet, key management, tile auth
- Multi-gateway topology
- OTA firmware updates
- FreeRTOS / advanced power management
