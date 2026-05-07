---
title: "Detective audit — contradictions and future blockers across the SARCom doc set"
date: 2026-05-06
type: dev-log
session-trigger: "Pieter asked Claude to play detective for contradictions and future blockers"
---

# Detective audit — 2026-05-06

A second-pass read across the doc set, the ADRs, the spikes, the dev log, the BOM, and the only committed Rust crate (`crates/protocol`). Goal: surface direct conflicts, decisions in tension, stale references, dangling TODOs, ambiguous wording, unstated assumptions, and physical/regulatory/supply-chain blockers that aren't captured anywhere yet.

Findings are sorted by severity. Each entry cites the specific file and line / section so you can re-check rather than trust this file.

---

## A. Direct contradictions (two docs say opposite things)

### A1. CLAUDE.md still says `rppal`; the dev log claims it was swapped

`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md` line 87–93 lists the four files where `rppal` was swapped to `rpi-pal`, with line numbers, and says "**Decision taken this session:** swap all `rppal` references to `rpi-pal` across: `CLAUDE.md` (line 65) ...". Lines 158–161 list those edits as "**Doc updates applied this session**".

Reality (verified by grep on 2026-05-06):
- `ARCHITECTURE.md:693` → `rpi-pal` ✓
- `decisions/ADR-004-gateway-platform.md:39` → `rpi-pal` ✓
- `docs/claude-code-setup.md:61` → `rpi-pal` ✓
- `CLAUDE.md:65` → still **`rppal`** ✗

The CLAUDE.md edit was claimed but never applied. CLAUDE.md is loaded into every session, so the wrong crate name is the one Claude sees most. Fix: swap `rppal` → `rpi-pal` on CLAUDE.md line 65.

### A2. BLE relay maintenance scope: ADR-006 + CLAUDE.md say v1; ARCHITECTURE.md + TODO.md say v2+

- `decisions/ADR-006-relay-has-gnss.md` "Decision" §3: "**v1, not v0.** ... Implemented in v1 (after v0 desk prototype is working) — you cannot deploy a sealed solar relay in a field without a way to verify it is alive without opening the enclosure."
- `CLAUDE.md` line 19: "BLE maintenance CLI is **v1 (not v0, not v2+)**".
- `ARCHITECTURE.md:264` (§9 Modes, item 4): "**BLE maintenance.** v2+. Explicitly not v1."
- `TODO.md` "Deferred (v2+)" section: "BLE maintenance CLI on the relay (service engineer stands next to the pole with a phone/laptop, reads battery mV / RX count / last RSSI, triggers a fresh commissioning broadcast)".
- `bom.md` line 20: "*BLE commissioning interface planned for v1 — see ADR-006 update pending.*"
- `spikes/field-deployment-test-fleet-spike.md` whole §4 argues BLE is critical for v1a.

ADR-006 is Accepted. Per CLAUDE.md's own rules, ARCHITECTURE.md and TODO.md drifted from the ADR rather than the other way around. Either re-affirm BLE-in-v1 by editing ARCHITECTURE.md §9 and moving the TODO.md item out of "Deferred (v2+)", or write a new ADR that supersedes ADR-006 §3. Pick one — right now the doc set says both at once.

### A3. Pole hardware: ADR-003 says "hose clamps", BOM rejects them, TODO still orders them

- `decisions/ADR-003-relay-hardware.md` "Decision": "Wooden pole (local hardware store, ~2.5 m, pressure-treated), u-bolts or stainless hose clamps". Order checklist: "Wooden pole (~2.5 m, pressure-treated), u-bolts or stainless hose clamps — local hardware store".
- `bom.md` "Explicitly NOT ordering": "~~Stainless hose clamps~~ — replaced by pole design approach" and "~~Wooden pole from hardware store~~ — replaced by pole design approach". The "Relay pole — local build" section then describes a Fusion360-designed three-legged base + ground-stake built at a woodworking shop, with the explicit note "*No hose clamps. No improvised strapping. Do it once, do it right.*"
- `TODO.md` "Right now": "Order the wooden pole + stainless hose clamps for the garden relay (local hardware store)."

Three positions in three docs. The BOM is the most recent and most considered, so it is probably the true intent. Fix: update ADR-003's checklist with a "Superseded in part" note pointing at the BOM pole-design approach, and remove the hose-clamps line from TODO.md "Right now". Or write ADR-015 if you want the woodworking-shop pole as a real decision.

### A4. v0 time source: BOM allows NTP-on-WiFi, ADR-008 + ADR-011 + CLAUDE.md forbid it

- `bom.md` "Deferred — v1a prep": "These are NOT in the immediate cart. **v0 runs behind WiFi with NTP at mom's place — no RTC needed yet.** v1a is the first field deployment where the gateway has no internet."
- `decisions/ADR-008-no-cloud-no-downlink.md` Decision: "**'Offline' is the normal state**, not a degraded mode. The system assumes zero internet and happens to work with it if present."
- `decisions/ADR-011-gateway-time-source.md`: "**No NTP. No `systemd-timesyncd`. No phone-home to pool.ntp.org.**" — and later, "No NTP over the hut's WiFi, even if WiFi exists. ADR-008 forbids the gateway from reaching out. **That stance does not bend for time sync.**"
- `CLAUDE.md` "Do NOT re-open" list: "let's just NTP the clock when WiFi is around — stop. That door is closed."

The BOM line is internally reasonable as a development convenience ("we don't need an RTC on the desk yet"), but the wording it uses re-opens an explicitly closed door. If the intent is just "RTC is buyable later, the desk Pi can use whatever clock", say that without invoking NTP. Recommended fix: rewrite the BOM "Deferred — v1a prep" preface as "v0 desk bring-up runs against the laptop's manually-set system clock; the DS3231 + CR2032 only become load-bearing for v1a when the gateway moves off mains-with-keyboard and into field deployment. Per ADR-011, no NTP at any deployment stage."

### A5. ARCHITECTURE.md §9 validation rule list misses two checks the code performs

`ARCHITECTURE.md:268-279` lists "Validation rules (drop or accept)" as 7 numbered checks (length window, MAGIC, VER, TYPE, LEN, CRC, accept). Missing from the list:

- Reserved-flag-bits check (`flags & 0xF8 != 0` → reject) — exists in `crates/protocol/src/position.rs` line 56–58 and is documented in `explainers/how-the-network-works.md` and `operations/troubleshooting-guide.md`.
- GPS-valid / sentinel consistency check — exists in `crates/protocol/src/position.rs` line 65–80 and is documented in the same explainer + troubleshooting docs.

Code is the more careful version; the architecture text is incomplete. Either add steps 4a (reserved bits) and 7a (sentinel consistency) to §9's numbered list, or move those checks out of the relay path with an explicit note that they are gateway-only. Right now ARCHITECTURE.md describes a looser relay than the relay actually is.

A subtler point inside the same section: ARCHITECTURE.md says "Received bytes ≥ 6 (minimum frame size)" and "≤ MAX_FRAME (e.g. 64)", but `decode_position` rejects anything where `raw.len() != FRAME_LEN` (exactly 22). Practically equivalent for v1 because LEN is fixed, but the text suggests a flexible parser the code doesn't implement. If a future packet type lands you'll need both the loose pre-filter *and* the type-specific exact-length check; today the code is conservative-strict and the doc is generous-lazy.

### A6. Heltec quantity: TODO.md says 3, BOM says 10 + 1, dev log records actual order as 10 + 2

- `TODO.md` "Right now": "3× Wireless Tracker V2 (EU 863–928 MHz variant; 2 tag + 1 relay), 1× Solar Kit for Dev-board, ..."
- `bom.md` "Cart sanity-check" Heltec section: "10× Wireless Tracker V2 (EU 863–870 MHz variant)", "1× Solar Kit for Dev-board (LoRa + 2.4G variant)".
- `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`: "Heltec — order #110639 ($403.40, Bancontact, ordinary delivery from CN) ... 10× Wireless Tracker V2 ... **2× Solar Kit for Dev-board**".
- `bom.md` line 22 even says "1× Solar Kit" but the dev log records 2 ordered.

Three different shapes for the Heltec footprint. The dev log is the only one that records what actually happened on the credit card. Fix: re-cut TODO.md "Right now" to past tense for what was ordered ("Placed Heltec order #110639: 10× Tracker V2 + 2× Solar Kit") and update the BOM Solar Kit count to 2 (or write a one-liner explaining why one of the two will go back).

### A7. README.md still says "Heltec order not yet placed"

- `README.md` "Status at a glance" → "Hardware in hand": "**Heltec order not yet placed** — BOM refreshed 2026-04-24 for RTC, pigtails, antennas, corrected battery count."
- (Verified 2026-05-06: CLAUDE.md does *not* duplicate this status row — it carries decision principles only, no inventory table. So this is a single-file staleness, not a multi-file one.)

The order shipped on 2026-05-05 (dev log). The README row is 11 days stale. Fix: change to "Heltec order placed 2026-05-05 (#110639); ETA 2026-05-19 → 2026-06-02. Kiwi gateway-stack cart prepared, not yet ordered."

### A8. README.md says "Production firmware/gateway code not yet written"; `crates/protocol` is committed and tested

- `README.md` "Status at a glance" → "Code": "Production firmware/gateway code not yet written. UX and tooling prototypes exist under `tools/`."
- `crates/protocol/` exists, ships 22 unit tests, frozen canonical vectors per TODO.md, and is the encoder/decoder + relay decision logic + SeenCache shared between tag, relay, and gateway. That is production code by every reasonable definition.

`TODO.md` already flags this in the "Doc review findings" list ("README 'Code: Not yet written' is now inaccurate") with a proposed replacement. The fix is one line; it just hasn't been applied. README has the same staleness as A7, dated to a state of the world from before 2026-05-04.

### A9. ~~Display size: 7" vs 5"~~ **RETRACTED 2026-05-06**

> **Audit error.** This finding originally claimed the 5" Touch Display 2 was "ordered" and that the docs needed to either accept the deviation or revert. That was wrong. The 5" displays were in a Kiwi gateway-stack cart that Pieter never checked out. There is no 5" display in hand and no decision to acquire one. The docs (ADR-005, ADR-007, README, CLAUDE.md, ARCHITECTURE.md) correctly describe the on-hand 7" DSI on `pi3kiosk` and need no change. The kiosk-lab continues to size for 800×480.
>
> See the corrected `dev-log/2026-05-05-...md` Finding 3 for the cleanup pass on the dev log itself.
>
> **Decision #3 from the audit-question batch ("Accept 5" now and cascade docs") is retracted.** No ADR-015 is needed. No cascade is needed.

### A10. Gateway hardware: docs say Pi 3B+/4; dev log raised Pi 5 as a *possibility* (not as substrate) — **REFRAMED 2026-05-06; RESOLVED 2026-05-07**

> **RESOLVED 2026-05-07:** Pi 4s tested out of order; Pi 5 path is live. See [`2026-05-07-pi4-retirement-substrate-decision.md`](2026-05-07-pi4-retirement-substrate-decision.md). The Kiwi-cart procurement decision is no longer speculative.


- `decisions/ADR-004-gateway-platform.md`: "Raspberry Pi (3B+ or 4, whichever has healthy ports and a working SD slot)".
- `ARCHITECTURE.md` §10 / §16: "Raspberry Pi (3B+ or 4)".
- `hardware/pi{1,2,3kiosk}/specs.md`: all three units are Pi 4 Model B.
- `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`: Pi 5 was *in a draft Kiwi cart that was not placed*. The same dev log carries a same-session claim that "the Pi 4s on Pieter's desk are bricked", but that claim is unverified beyond the one sentence — no `hardware/pi*/specs.md` carries a working/non-working flag, and no power-on test has been run.

Original audit recommendation ("write ADR-015 'Gateway host = Pi 5'") was based on misreading the Kiwi cart as a placed order. **Retracted.** The actual gateway-substrate question is upstream of any ADR change:

1. Power up each of the 3× Pi 4s against a known-good PSU + micro-HDMI display + SD card. Record the result in `hardware/pi{1,2,3kiosk}/specs.md` as a working / non-working flag.
2. If at least one Pi 4 powers up cleanly: ADR-004 stays as-is, no Pi 5 acquisition needed for v0/v0.5/v1, and the Kiwi cart can be dropped (or kept as a "spare gateway substrate" cart, not placed yet).
3. If all three Pi 4s are confirmed dead: *then* the Kiwi cart (or a smaller variant — e.g. Pi 5 + reuse the on-hand 7" DSI instead of new 5" displays) becomes a real procurement decision, and *that* decision triggers the ADR-004 amendment. Not before.

The Pi-5-RP1 research in `dev-log/2026-05-05` Finding 1 is preserved as preempted gotchas if/when Pi 5 gets ordered. It is not a hard gate today.

### A11. Spike `fake-position-injector-spike.md` proposes a `node_id` range that does not fit the wire type

The spike proposes "Reserve a high-end range for synthetic nodes (proposal, contingent on `node_id` width: `0xFFFF_FF00..=0xFFFF_FFFF` if `u32`)" and gives TOML examples like `node_id = 0xFFFFFF01`.

`crates/protocol/src/frame.rs:13-16` and `ARCHITECTURE.md:151` define `node_id` as **`u8` (0–254, 255 reserved)**. The spike does flag this as "the node_id type is the question to answer here, not assume" — so the contradiction is internal-and-flagged, not a missed bug. But the example values throughout the spike are still u32-shaped, which will mislead anyone implementing from the doc without re-reading the caveat. Fix: replace the u32 example range with a u8-shaped one (e.g. 250–254 reserved for synthetic, with `0xFF=255` left as the protocol-reserved sentinel) and make the same change in the TOML scenario sketch.

### A12. ADR-013 / ADR-014 partial-supersede of ADR-012 — wire schema vs ADR-012 SIGHTING table is silently inconsistent in two BOM/Cart references

- ADR-012's role enum, RELAY_INFO, SIGHTING, three-table schema are all rolled back per ADR-013.
- `bom.md` line 12: "Aligned to: ADR-002, ADR-003, ADR-004, ADR-011, **ADR-012**, ADR-013, ..." — ADR-012 is listed as a current alignment doc, but it is superseded-in-part. Reading the BOM cold, you'd think ADR-012 was still a live decision.
- `bom.md` "Tag SOS audible cue": "tag SOS audible cue per [ADR-012]" — true (the buzzer survives the rollback) but the citation is to the superseded doc rather than to the part of ADR-012 that survived the supersession.

This is mostly cosmetic but it makes the alignment trail confusing. Fix: cite ADR-013 for SIGHTING/role-enum-related rationales and either drop the ADR-012 line from the BOM alignment list or replace it with "ADR-012 (buzzer + non-goals only; rest superseded by ADR-013/014)".

---

## B. Decisions in real tension (not direct conflict, but rubbing)

### B1. ADR-008 vs the TAK/CoT spike

`spikes/tak-cot-integration-spike.md` is explicit that an outbound CoT emitter potentially conflicts with ADR-008 ("No cloud, no downlink. No internet-hosted server. No REST API. No WebSocket."). The spike proposes a Phase-1 multicast experiment with config-flag-default-off, and defers the ADR-008 question explicitly. This is fine *as a spike*. The blocker is what happens if Phase 1 passes: there's no ADR yet that distinguishes "outbound LAN multicast to ATAK on the same WiFi" from "cloud-bound REST". ADR-008's wording bans both, and you don't have a vocabulary for the difference.

If you ever plan to act on the TAK spike, write ADR-015 (or whichever number is next) before, not after, the experiment passes. The ADR-008 list of forbidden things would benefit from an explicit category — e.g. "no inbound network surface" vs "no internet-bound network calls" vs "no cloud-hosted dependency" — so a future LAN-only emitter can fit cleanly without re-litigating ADR-008.

### B2. ADR-008 vs the gateway-rx-bringup spike's "B11. Architectural pivot to WiFi+cloud floated by Pieter on 2026-05-05"

`spikes/gateway-rx-bringup-spike.md:111-112`: "**B11. Architectural pivot to WiFi+cloud floated by Pieter on 2026-05-05.** *Likelihood: not a blocker per se, but a possible reason to never open this ticket.*"

This is the second spike in two days that contemplates re-opening ADR-008. The mountain-hut-snowstorm framing in ADR-008 is the load-bearing rationale for half the system shape (no auth, no downlink, single-channel uplink, kiosk as terminal node, sentinel-only no-fix). If the WiFi+cloud question is genuinely live — and the dev log doesn't capture what triggered it — it deserves either a fresh ADR-015 explicitly re-litigating ADR-008, or a CLAUDE.md update saying "ADR-008 is under review pending decision X". Right now it floats as half-mentioned in spike footnotes.

### B3. CLAUDE.md "Do NOT re-open" tone vs the same pivot

CLAUDE.md says "If a suggestion starts with 'let's just add a small web dashboard' or 'let's use Python for the gateway' or 'let's use React for the map' or 'let's put SOS on a separate frequency for more range' or 'let's just NTP the clock when WiFi is around' ... — stop. That door is closed." But two spikes opened on 2026-05-05 (TAK and gateway-rx) explicitly flag the architectural pivot question. The doc set is in tension with itself: the principle file says "stop", the working spikes say "this is a real open question." Either resolve the pivot question and update CLAUDE.md, or accept that "do not re-open" has been quietly suspended and update CLAUDE.md to match.

### B4. Gateway dedup window vs. recent SOS-flag staleness window

`ARCHITECTURE.md` §10: dedup recent-window is **24 h**. `ARCHITECTURE.md` §7 distress classification: `DISTRESS_WINDOW = 10 × SOS_MIN_INTERVAL = 450 s`. These are independent and consistent today, but they are tied implicitly to `received_at`, which itself is tied to the RTC. If the RTC is missing or wrong:

- Dedup over a 24 h window can falsely accept duplicates after a backward jump (the row's `received_at` is suddenly "outside" the window) or falsely reject fresh frames after a forward jump.
- DISTRESS_WINDOW classification can latch SOS state for hours after the tag has cleared, or fail to ever classify a tag as distressed.

ADR-011 partially addresses this with the clock-not-set banner and the "freshness strings suppressed" rule, but the *dedup* and *distress classification* logic both still happily run against a poisoned `received_at`. The kiosk text is honest; the underlying state machine is not. This is a future blocker the moment a real DS3231 is missing or fails. Fix: have the gateway's dedup and distress-classification paths short-circuit (or use a monotonic process clock, not `received_at`) when `clock_valid=false`, and document in §10 / §11 what dedup behaves like during a clock-invalid window.

### B5. Two Solar Kits ordered, only one relay in v1a

Dev log says **2× Solar Kit** ordered. v1a is **one relay**. v1b is gated on v1a passing. The second Solar Kit is either v1b inventory ahead-of-time (which contradicts the "v1b is gated on v1a passing — no v1b firmware work begins before that" anti-creep rule when extended to procurement, though hardware ordering vs firmware work is a soft distinction) or it is a spare. Neither story is documented anywhere. If the spare is intended for the field-deployment fleet sizing (`spikes/field-deployment-test-fleet-spike.md` Option C / D), say so in the dev log addendum.

### B6. Cargo workspace declaration vs. the 2-directory plan

`CLAUDE.md` and `ARCHITECTURE.md` §17 both describe a workspace with `crates/{protocol,persistence,heltec-wireless-tracker-v2-bsp}` + `firmware/{tag,relay}` + `gateway/`. The actual `Cargo.toml` is `members = ["crates/protocol"]`. The kiosk-lab is deliberately a *separate* workspace per its README. This is consistent with TODO.md's "In progress" status — but a fresh contributor reading just CLAUDE.md and ARCHITECTURE.md will believe the workspace exists. Either add a "current state" line to ARCHITECTURE.md §17 (single-line: "as of 2026-05-06 only `crates/protocol` exists; the rest is the planned shape") or stop describing the unbuilt skeleton in present tense.

---

## C. Stale references, dangling TODOs, ambiguous wording

### C1. Multiple references to docs that don't exist yet

ARCHITECTURE.md and several ADRs reference these planned files as if they were authoritative:

- `software/repo-layout.md` (ARCHITECTURE.md §17, ADR-004)
- `hardware/relay-assembly.md` (ADR-003, twice)
- `hardware/desk-inventory.md` (ADR-004)
- `hardware/gateway-assembly.md` (ADR-004)
- `architecture/{system-overview, sighting-model, protocol, operational-modes, non-goals}.md` (ARCHITECTURE.md preamble + README.md)

None of those files exist. Each citation is hedged with "(planned; for now see ...)" so this isn't broken-link territory, but the volume of forward references means contributors regularly bounce off non-existent docs. Either inline the relevant content where it lives today (most of the planned docs are simply sections of ARCHITECTURE.md / ADRs) or convert the references to "see issue/spike X to track this split" so they don't read as broken links.

### C2. CLAUDE.md "Tools this project uses" describes things `crates/protocol` doesn't actually use yet

CLAUDE.md lists `nmea0183`, `lora-phy`, `esp-hal`, `embassy-executor` etc. as the firmware stack. None of those are in `Cargo.lock` (the lora-phy preflight doc explicitly notes `Cargo.lock` only contains `protocol`). This is fine while you're still pre-firmware, but it means "Pin and check the doc lookup protocol" can't actually be exercised yet — there's nothing to pin. The lora-phy preflight doc and command both anticipate this, but the smoke test described in CLAUDE.md "Rust doc lookup smoke test" can't run as written until at least one of those crates is added to a workspace member.

### C3. `bom.md` "BLE commissioning interface planned for v1 — see ADR-006 update pending"

`bom.md` line 20 references an ADR-006 update that hasn't happened. The note has been there long enough that it has produced its own contradiction (A2). Fix: actually do the ADR-006 update, or remove the "update pending" sentence.

### C4. Kiosk-lab Cargo.toml warns it's a separate workspace; the protocol crate is `no_std` only and has zero dependencies

`crates/protocol/Cargo.toml` declares `[features] std = []` but no code path conditionalises behaviour on the `std` feature. The feature exists for forward-compatibility with host-side test tooling per ADR-001, but right now toggling it does nothing — which is a footgun if anyone tries to use it. Either gate something on `std` (e.g. `Display` impls, `std::error::Error` for `FrameError`, `Vec`-backed test helpers) or note in the crate's `lib.rs` that the feature exists for future use only.

### C5. `FrameError::BadVersion` vs `DropReason::UnknownType` mapping

`crates/protocol/src/relay.rs:19-21` maps both `FrameError::UnknownType` and `FrameError::BadVersion` to `DropReason::UnknownType`. Operationally fine (both are "this isn't a v1 POSITION I recognise"), but the troubleshooting guide and the ARCHITECTURE.md log examples distinguish `BadVersion` and `UnknownType`. The drop-reason enum loses information at the relay's logging boundary; if a future protocol has a v2 frame on the wire alongside v1, you'll see it as "UnknownType" in logs even though the actual cause is `BadVersion`. Fix: split `DropReason::UnknownType` into `UnknownType` and `BadVersion`, or accept the loss of fidelity but document it explicitly.

### C6. `DropReason::Malformed` is a catch-all that hides reserved-bit and sentinel-mismatch failures

Same shape as C5: `relay.rs:22` maps every `FrameError` that isn't `CrcMismatch` / `UnknownType` / `BadVersion` to `DropReason::Malformed`. That collapses `BadMagic`, `BadLength`, `ReservedFlagBits`, and `GpsValidSentinelMismatch` into one log line. Operations docs surface those as distinct decode-level errors, but the relay's serial log can't distinguish them. For v1 garden testing this is fine; for any field debugging it will hurt. Fix: surface the FrameError discriminant directly in the drop-reason or carry the FrameError variant as a payload on `Drop`.

### C7. `dev-log/2026-05-05-...` lists "TODOs generated from this session" that haven't been folded into TODO.md

The dev log says "These should be folded into TODO.md once Pieter has glanced at them." None of them have been. Most importantly:
- The `rppal` → `rpi-pal` swap on CLAUDE.md (item A1).
- The "actual order was 10× Tracker V2 + 2× Solar Kit" TODO.md correction (item A6).
- The Pi 5 active cooler upgrade.
- The Pi 5 first-boot validation checklist (intended to live in `spikes/pi5-hat-bringup-spike.md`, which doesn't exist yet).

Either fold them now, or write a one-line "Generated 2026-05-05, applied YYYY-MM-DD" footer to make the lag visible.

### C8. `Blueprint.md` is generic Claude-Code-project boilerplate; nothing in SARCOM uses it

The whole 871-line `Blueprint.md` is a CLAUDE.md scaffolding template ("Read @Blueprint.md, then walk me through the setup"). It is not SARCOM-specific. It clutters the repo root next to README, ARCHITECTURE, CLAUDE, TODO, and adds line count + reading load to anyone who scans the directory. Either move it under `archive/` or delete — the fact that CLAUDE.md exists means Blueprint.md has already been used. Keeping it at the root pretends the project is still in setup phase.

---

## D. Future blockers / unstated assumptions

### D1. Pi 5 vs. Yocto support timing

`meta-raspberrypi`'s Pi 5 support landed in scarthgap (2024). The dev log notes this and flags "confirm the active branch supports Pi 5 GPIO/SPI peripherals via device tree before committing." If you target an older LTS branch (kirkstone), the Pi 5 is unsupported and you'll discover that mid-image-build rather than at planning. Add a one-line "minimum Yocto branch: scarthgap" to ADR-004 before the first `bitbake` runs.

### D2. PMTiles-on-Pi-5-GPU has no public reference

`spikes/pmtiles-walkers-spike.md` is explicit: walkers' PMTiles support is verified on desktops, never on a Pi GPU. ADR-005 explicitly names this as a fallback gate ("if that spike fails, fall back to a custom `TilesManager` that reads from MBTiles"). The spike has a 1-day timebox. If it fails, Yocto image content (PMTiles file) and most of the kiosk module's plumbing change in shape. Do this spike *before* the Yocto recipe spike — TODO.md "While hardware is in transit" lists them in roughly that order today, which is correct, but the dependency isn't called out. Add it as a hard gate: "v0.5 acceptance is blocked until either pmtiles-walkers-spike returns H1 or fallback is implemented."

### D3. SPI CS routing defect on the in-hand Dragino HATs

`gateway-rx-bringup-spike.md` flags B1 ("All 3 HATs have the GPIO 25 CS defect") at "medium" likelihood. The mitigation is a single extra OutputPin parameter to `lora-phy`, not a code rewrite — but if it's missed during bring-up, the symptom is "SPI is dead" and you'll burn hours debugging. Suggested action before any gateway code is written: physically inspect the silkscreen rev on all 3 HATs and write the result into `hardware/pi{1,2,3kiosk}/specs.md`. Right now those files document antennas, RAM, and SD presence, but not HAT silkscreen rev.

### D4. Pi 5 RP1 polled-RX-only is not yet a TODO item

`dev-log/2026-05-05`: GPIO interrupts are reportedly flaky on Pi 5 (RadioLib #1200). Polled RX is the v1a fallback. This means the gateway's RX loop will run a busy-poll on the SX1276 status register, which has implications for CPU usage and SPI bus contention with the L80 GNSS UART. Nothing in ARCHITECTURE.md §10 acknowledges this. If it stays unwritten, the first time the gateway draws too much CPU under load you'll spend a day blaming the wrong layer. Fix: §10 "What the gateway does" item 2 should read "Listen continuously for LoRa packets via SX1276 over SPI **(polled RX in v1a per dev log 2026-05-05; interrupt-driven RX is v2 per RadioLib #1200)**".

### D5. Production-concerns risk register has no owner / no review cadence

`production-concerns.md` documents four real failure modes (VHB cold-cycle creep, 18650 cold charging, IPEX strain relief, SD-card power-loss). It explicitly says "This is a risk register, not a design backlog." Fine. But there is no "review at v1a passing", "review before any winter deployment", "review on date X" trigger. The risk of a risk register is that it accumulates entries that everyone stops reading because nothing is ever crossed off. Suggest: add a "Review trigger" line to each entry — e.g. VHB cold-cycle review triggered when the relay first leaves the garden, IPEX strain relief review triggered before any sealed enclosure ships, etc.

### D6. v1a single-tag scope vs ETSI relay duty cycle at multi-tag

ARCHITECTURE.md §13 / ADR-014 are very explicit that v1 is single-tag, and that two simultaneous SOS tags push the relay over 1% (~1.64%). Currently v1a has one tag, so this is a documented v2 concern. But the dev log has 10 Tracker V2s ordered and `field-deployment-test-fleet-spike.md` lays out 7-Heltec / 10-Heltec scenarios. The moment two tags walk the garden during the same SOS rehearsal, the budget breaks — and there is no firmware enforcement of the per-relay duty cycle today (ADR-014 §Consequences mentions "runtime enforcement is a firmware concern" but that work is unscheduled). Fix: either gate field tests to one-tag-at-a-time explicitly in `operations/troubleshooting-guide.md` and TODO.md v1a, or write a spike for runtime relay duty-cycle enforcement before the second tag is ever in the air.

### D7. SX1276 syncword on the gateway is asserted but never set in code

ARCHITECTURE.md §10 ("SX1276 ↔ SX1262 syncword"): "`lora-phy` handles the SX1262 side; the Rust gateway driver must program 0x12 on the SX1276 side. **Verify empirically during v0 bring-up.**" Trouble­shooting guide §"Radio bring-up checks" lists "Syncword compatibility between SX1262 (tag/relay) and SX1276 (gateway) if private syncword is used in config" as something to check. The gateway crate doesn't exist yet, so there's nothing to set the wrong value, but the moment it's written this is a footgun: the default `lora-phy` syncword for SX1276 may be 0x34 (LoRaWAN public) or 0x12 (LoRa private) depending on chip / version. Add a TODO.md gate: "First gateway commit must set syncword 0x12 explicitly and have a unit test verifying the constant" — otherwise this becomes the first hour of v0 bring-up debugging.

### D8. Tag SOS button + buzzer GPIO assignments are still in the "Blocked" list

`TODO.md` "Blocked" line: "SOS button wiring — waiting on a decision for which GPIO on the Tracker V2 + button type". This blocker has been there since the buzzer survived the ADR-013 rollback. If it isn't picked before tag firmware bring-up, the SOS-entry test (the load-bearing test that exercises ADR-010's immediate-first-frame rule) can't run. Suggest: pick a GPIO from the Tracker V2 unused-pin list now, even tentatively, so firmware bring-up doesn't block on it. Re-bind later if needed.

### D9. ETSI band-plan citation hard-coded across docs

Every doc that mentions duty cycle cites "ETSI EN 300 220-2 V3.2.1". That standard has revisions; the V3.2.1 number is in five places. If V3.3.x ships and changes any sub-band M / sub-band P parameters, you have five places to update with no easy way to see they all moved together. Fix: extract a single source-of-truth citation in ARCHITECTURE.md §12 and have the ADRs link to it rather than citing the version directly. Bonus: catch it in CI later via a one-liner grep.

### D10. `crates/protocol/src/seen_cache.rs` has fixed 32-entry capacity; ADR-013 says "if too small, tune"

The seen_cache is `[Option<(PacketId, u32)>; 32]` — fixed at compile time. ADR-013 §7 says "If field testing shows the seen_cache is too small or expires too quickly under realistic load (multiple tags, dense relay deployments), expiry and capacity are the parameters to tune." Capacity is a `const`. Tuning it after a relay is deployed in a sealed Solar Kit on a pole means flashing the relay — which TODO.md, the field-deployment spike, and ADR-006 all flag as expensive. Either accept the cost and document it ("capacity is fixed in firmware; revisit before v1b"), or store the cache in a heap-backed `heapless::Deque<_, N>` parametrised by `N` so future-you can change it via a const generic at compile time without rewriting the type. Today the constant is buried in `frame.rs:6` and not surfaced in any operational doc.

### D11. Battery cold-charging risk for tags too, not just relays

`production-concerns.md` §2 is "18650 cold-charging" and discusses the **relay** Solar Kit. The tag also runs an INR18650-25R per ADR-002, charged via the Tracker V2's own onboard PMIC over USB-C. The same physics applies (charging below 0°C plates lithium, dendrites, runaway risk). The PoC use case ("tag in a pocket on a hike") will charge from USB at home, which is fine, but the spec gap is real for any future "leave the tag on the dash of a parked car overnight" use case. Fix: extend §2 to cover the tag explicitly, or add a one-line "tag charges indoors from USB-C; do not charge a sub-zero tag" operational note.

### D12. Yocto recipe + RTC + GPS time discipline is described in three places; none of them is a single source

ADR-004 names `meta-rust`. ADR-011 names `i2c-rtc,ds3231` device-tree overlay + `gpsd` + `chrony`. The dev log adds Pi 5 UART config (`dtparam=uart0`) and notes that `meta-raspberrypi` Pi 5 support is scarthgap-or-later. There is no single Yocto recipe / config file / overlay list anywhere. The first Yocto image build will need all of this assembled from three docs by hand. Suggested fix: when the Yocto image experiment lands (TODO.md "While hardware is in transit"), commit a `gateway/yocto/` directory with the recipe and have ADR-004 / ADR-011 / dev-log entries link to it instead of inlining instructions.

### D13. No retention policy on `tag_reports`

`ARCHITECTURE.md` §10 / ADR-009: SQLite as single source of truth, single file at `/var/lib/lora-sar/sightings.db`, "Hundreds to a few thousand rows per day". No `VACUUM`, no row pruning, no file size cap. After a year of garden testing at heartbeat cadence + occasional SOS rehearsals + relay self-announces, the file will be ~1–3 M rows. SQLite handles that fine, but the SD card on a Pi gateway will not survive a single full-erase cycle every time WAL checkpoints. Add: "v2+ retention policy" to TODO.md, or define a v1 policy now (e.g. weekly `VACUUM`, prune rows older than 30 days). The decision can be "no pruning in v1" but it should be a decision, not a default.

### D14. No `UNKNOWN_TYPE` log entry in `crates/protocol`

ARCHITECTURE.md §9 "Debug logging (serial only)" promises a `DROP len=? UNKNOWN_TYPE ver=2 type=99` log line. The protocol crate maps to `DropReason::UnknownType` but has no logging — it can't (no_std, no formatter, no serial driver yet). The relay binary will need to translate the enum into the expected log format. Fine for now, but the doc shape implies the protocol crate produces that line. Add: "logging is the relay binary's responsibility; protocol crate produces typed enums" line to ADR-013 §5 or to the protocol crate's lib.rs.

---

## E. What I checked but found no contradiction in

For your records — these were inspected and are consistent across docs and code:

- The 22-byte POSITION frame layout — `ARCHITECTURE.md` §7, `ADR-013` §3, `explainers/how-the-network-works.md`, `crates/protocol/src/frame.rs`, `crates/protocol/src/position.rs`. All four sources agree on field order, widths, big-endian, payload length 16, frame length 22.
- CRC-16/CCITT-FALSE parameters — `ARCHITECTURE.md` §7, `ADR-013` §3, `crates/protocol/src/crc.rs` (poly 0x1021, init 0xFFFF, no reflect, xorout 0x0000), test vector `0x29B1` for `"123456789"` matches the implementation.
- SOS cadence math — `ADR-010` §4 and `ADR-014` §"Tag SOS POSITION" both land on 80 TX/h × 371 ms ≈ 29.7 s/h ≈ 0.82%. Consistent.
- Heartbeat cadence — 300–330 s positive-only jitter shows up in `ADR-013` (implicit), `ADR-014`, `ARCHITECTURE.md` §12, `tools/sarcom-kiosk-lab/src/data.rs` `freshness_for_tag`. Consistent.
- v1a/v1b scope split + tag buzzer + non-goals — preserved consistently from ADR-012 through ADR-013/014 references.
- Sentinel coordinates — `ARCHITECTURE.md` §6, `crates/protocol/src/frame.rs` constants, `tools/sarcom-kiosk-lab/src/data.rs` no-fix scenarios, all line up on `0x7FFFFFFF` for lat/lon and `0x7FFF` for alt.
- The single source of truth on flag-bit layout — `Flags(0x01) = GPS_VALID`, `Flags(0x02) = SOS`, `Flags(0x04) = BATT_LOW`, reserved `0xF8` — consistent across `flags.rs`, `ADR-010`, `ARCHITECTURE.md` §7, `explainers/how-the-network-works.md`, `operations/troubleshooting-guide.md`.
- Recent-window dedup wording — 24 h, no permanent UNIQUE index — consistent across `ADR-009`, `ARCHITECTURE.md` §10, `TODO.md` v0.5 acceptance, `ADR-013` §8.

---

## Recommended order of operations

**Updated 2026-05-06** after the dev-log Kiwi-cart correction.

If you only fix five things this week, in priority order:

1. **A1** — apply the missed `rppal` → `rpi-pal` swap on `CLAUDE.md:65`. One-line edit. Ten seconds.
2. **A4** — rewrite the `bom.md` "Deferred — v1a prep" preface so it doesn't re-open the NTP door. Two-line edit.
3. **A2 + B6** — pick: BLE-in-v1 or BLE-in-v2+, then update *one* of (ARCHITECTURE.md §9 + TODO.md "Deferred") or (ADR-006 §3 + CLAUDE.md + bom.md). Same-day work. *Decision captured 2026-05-06: BLE-in-v1; ARCHITECTURE.md + TODO.md edit.*
4. **A6 + A7 + A8** — three stale status rows that all flow from "the Heltec order shipped on 2026-05-05". Fix as one editing pass on README, TODO.md, and BOM. Same-day work.
5. **A10 (reframed)** — power-on test on the 3× Pi 4s. The "Pi 4s are bricked" claim is the load-bearing premise behind the whole Pi 5 / Kiwi-cart conversation; it has never been verified. Record the result in `hardware/pi{1,2,3kiosk}/specs.md`. This single check decides whether the Kiwi cart is even relevant.
   **RESOLVED 2026-05-07:** Pi 4s tested out of order; Pi 5 path is live. See [`2026-05-07-pi4-retirement-substrate-decision.md`](2026-05-07-pi4-retirement-substrate-decision.md). The Kiwi-cart procurement decision is no longer speculative.

~~5. A9 — close the 5-vs-7 inch question.~~ **Retracted 2026-05-06.** No 5" display exists; not a live deviation. Re-opens only if (a) Pi 4s confirmed dead, (b) Kiwi cart placed, (c) Pieter chooses 5" Touch Display 2 over the on-hand 7" DSI.

Everything else (D1–D14) is real but not blocking the path "verify Pi 4s → bring up gateway on existing hardware → walk the tag in the garden".
