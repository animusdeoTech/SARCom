---
title: "Spike — Synthetic POSITION source for e2e relay/gateway testing"
status: open
type: spike
timebox: 0.5 day
updated: 2026-05-06 (handheld pivot + node_id width fix per dev-log audit A11)
---

# Spike: Synthetic POSITION source for e2e relay/gateway testing

## UPDATES 2026-05-06

Two changes, neither rewrites the spike's core scope:

- **`node_id` is `u8` (0–254, 255 reserved), not u32.** The original spike's reserved test-only range proposal (`0xFFFF_FF00..=0xFFFF_FFFF` and TOML examples like `node_id = 0xFFFFFF01`) does not fit the wire type per `crates/protocol/src/frame.rs` and ARCHITECTURE.md §7. Per dev-log audit A11. Corrected reserved range below: **`node_id = 240..=254` for synthetic test sources** (15 slots — enough for any plausible bench scenario; `255` stays protocol-reserved per ADR-013).
- **Verification surface adds two rows for the handheld pivot:** (a) **handheld kiosk render** — the synthetic frame renders on the actual handheld unit, not just on a desk Pi; (b) **base-sync export status** — when the WiFi+power-good gate is open, the synthetic frame appears as a CoT message on a connected ATAK client (per the updated `tak-cot-integration-spike.md`).

The rest of the spike stands.

---

## Why this spike exists

We need to exercise the deployed gateway + relay firmware end-to-end without depending on a real Heltec tag walking the garden with GPS lock. The current test loop is: flash a tag with hardcoded coordinates → battery in → wait for GNSS lock → walk → check the kiosk. That is a 30-minute round trip per test, gated on weather and battery state, and it does not let us drive deterministic edge cases (dedup, SOS bit, malformed payloads, `seq_nr` rollover).

A **synthetic POSITION source** lets us push deterministic POSITION frames into the system and verify the relay forwards them, the gateway dedups + stores them, and the kiosk renders them — without real tags or GNSS. The preferred artifact is a **fake tag transmitter**: a spare Heltec Wireless Tracker V2 running test-only firmware that emits valid SARCOM POSITION frames over the same SX1262 path as production. This is what unblocks reproducible bring-up testing for v1a.

Terminology, fixed up front so the spike does not drift:

- The wire-level packet type remains exactly `POSITION` (per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md)). There is no new packet type.
- "Synthetic POSITION source" / "fake tag transmitter" is a **test/source concept**, not a production service.
- No new production API, downlink, REST endpoint, WebSocket, or control plane is introduced. The deployed gateway gains nothing permanent from this work.

This spike is **scoping**: pick the hardware path, the host control surface, and the test-scenario catalogue. **Do not implement the source as part of this spike.** Implementation is a separate ticket once we know what we are building.

## Scope fence

- Output is a decision note + a one-page implementation outline. **No code.**
- The encoder is `crates/protocol`. The synthetic source must reuse it, not re-implement the wire format. If `protocol` does not yet expose what is needed, the spike records the gap as a follow-up; it does not work around it with a hand-rolled byte layout.
- Verification surface (gateway log inspection, SQLite queries against `tag_reports`, kiosk visual check) is a separate concern. This spike notes which surface the test loop will read; it does not deliver that surface.
- [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) (single packet type: POSITION; multi-hop dedup by `(node_id, seq_nr)`), [ADR-010](../decisions/ADR-010-sos-encoding.md) (SOS as a flag bit in POSITION), and [ADR-014](../decisions/ADR-014-duty-cycle-budget-as-gate.md) (every protocol change updates the duty-cycle budget table) frame the rules. Do not invent new packet types.
- Production tag and relay firmware are **not** modified to support injection. The synthetic source lives alongside production firmware, never inside it.
- **Service boundary, hard.** A host-side CLI / scenario runner that drives the fake tag is fine. The deployed gateway binary must **not** gain a permanent fake-ingest service, REST endpoint, socket listener, or any API a non-test build can reach. Any direct gateway-internal injection (see Fallback A) lives behind an explicit **test binary**, **test feature flag**, or internal test harness — never in the production startup path.
- **`lora-phy` preflight dependency.** Any follow-up implementation touching radio code (H1, H0, or any other RF path) must run `/rust_lora_phy_preflight` and cite `resources/docs/lora-phy-preflight.md` in its preflight statement. **This spike itself does not invent `lora-phy` API calls** — it scopes which calls will be needed, not how they look.

## Hypotheses

**H1 (primary — fake-tag transmitter over real RF).** A spare Heltec Wireless Tracker V2 flashed with a dedicated `fake-tag` test firmware that transmits valid SARCOM POSITION frames over its SX1262, optionally driven by the host over USB-serial for scenario selection. The actual injection into the system is **RF**, same as a real tag. This exercises:

- protocol encoding from `crates/protocol`
- SX1262 TX path (same chip family + `lora-phy` code path as production tags)
- relay RX, relay dedup, relay forward
- gateway RX (SX1276 on Dragino HAT)
- gateway SQLite ingest
- kiosk render

H1 is the **default answer** unless an inventory blocker (no spare board, protocol crate not stable enough, duty-cycle math fails) forces otherwise.

**H0 (fallback — host-driven SX1276 via second Pi + Dragino HAT).** A host-side Rust binary on a second Pi pushes POSITION frames over SPI through `lora-phy` on an SX1276. Skips the firmware target entirely. Useful for gateway RX and some e2e tests, **but it is not equivalent to H1**: it does not exercise the SX1262 tag-side TX path, and the relay only ever sees frames from a radio that does not match production tag hardware. Choosing H0 is choosing **less coverage**. Acceptable only if the decision note explicitly accepts that coverage loss and the reasons.

**H2 (rejected up front — software-defined radio).** SDR + gr-lora. Outside this project's toolchain. Considered only if both H1 and H0 fail.

## Timebox

**0.5 day hard stop.** Output is a decision and a follow-up implementation ticket, not a working source.

```
0.5 h — protocol crate inventory: what is exported today (POSITION fields, encoder, CRC layer, node_id type), what the source needs that is missing
0.5 h — H1 path inventory: spare Heltec V2 availability, USB-serial control-protocol shape, fake-tag firmware build path
0.5 h — H0 path inventory: spare Pi + Dragino HAT availability, SX1276-vs-SX1262 TX parameter parity, coverage-loss accounting
0.5 h — test scenario catalogue v0 (TOML sketch, not prose)
0.5 h — duty-cycle accounting + reserved test-only node_id range + CRC layer ownership
0.5 h — write decision note + follow-up implementation ticket
```

Deliberately well under the half-day limit — leaves slack for inventory questions to take longer than expected without blowing the timebox.

## What to verify

1. **Wire-format reuse.** Does `crates/protocol` expose a `Position` encode path the source can call? If not, what is missing — a `serialize` method, a `WireFrame::from(Position)`, the CRC, the sentinels, the byte layout for the SOS flag bit per [ADR-010](../decisions/ADR-010-sos-encoding.md)? Document the gap.

2. **CRC layer ownership.** Two distinct CRCs may be in play:
   - **SARCOM payload/frame CRC** owned by `crates/protocol`. If this exists, the gateway application can observe a parse failure and a "wrong app-level CRC" scenario is meaningful.
   - **LoRa PHY CRC** owned by the SX126x / SX127x radio. Generated and checked below the application layer; corrupted PHY frames are typically dropped by the radio before the gateway ever sees them. Cannot necessarily be exercised from a normal `lora-phy` TX.

   The spike must identify which CRC(s) exist and whether the gateway application can observe the failure. **Do not claim a "wrong CRC" scenario tests gateway drop behaviour until that question is answered.**

3. **Hardware availability.** Once the v1 hardware order arrives (10× Heltec Tracker V2 per project memory), how many boards are realistically reservable for a fake-tag role? Is the spare Dragino HAT (one ordered with the gateway hardware) earmarked for H0, or is it expected to stay with the production gateway?

4. **Host control surface.** Bias toward the simplest reproducible mechanism, in this order:
   1. **TOML scenario replay** — host reads a scenario file and feeds the fake-tag firmware (or H0 host binary) a sequence of frames at startup. Reproducible by file diff; no interactive state to manage. **Default choice.**
   2. **One-shot USB-serial command** — single command from host CLI → fake-tag firmware emits one packet. Useful for ad-hoc probing on top of (1).
   3. **TCP / Unix-socket protocol** — only if the decision note records a concrete need that (1) and (2) cannot meet. Do not design an interactive orchestration service by default.

   The control surface lives on the host, never on the production gateway. Trade-off accepted: reproducibility over interactivity.

5. **Reserved test-only `node_id` range.** Per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md), node presentation is `nodes.toml`-driven on the gateway. **Confirmed 2026-05-06:** `node_id` is **`u8` (0–254, 255 reserved)** per `crates/protocol/src/frame.rs` and ARCHITECTURE.md §7. Reserved range:
   - **`node_id = 240..=254` for synthetic test sources** (15 slots; `255` is protocol-reserved per ADR-013).
   - Production tags / relays SHOULD use `node_id < 240` to keep the synthetic range unambiguous.
   - Synthetic node entries land in `nodes.toml` **only for test sessions** — never in the deployed config. The decision note must include a "do not deploy this config" warning.
   - Synthetic nodes must be visually distinguishable in the kiosk (icon / colour / label prefix).
   - **Unknown-node behaviour is a separate scenario**, using a `node_id` *outside* the configured synthetic range (e.g. `node_id = 100` with no `nodes.toml` entry). Frames from inside the range with no entry are *also* unknown — but the range itself is configured.

6. **Duty-cycle accounting.** Per [ADR-014](../decisions/ADR-014-duty-cycle-budget-as-gate.md), the duty-cycle budget table is the gate. The synthetic source is not a protocol change but it does add airtime. If used during real garden tests it shares the 1% sub-band M budget with production traffic. Document the maximum sustained cadence inside the budget, and whether a separate test channel is justified.

7. **Verification surface required.** Pick what the test loop will read to confirm a packet landed. Choose from:
   - `tag_reports` SQLite table (row-level proof of ingest)
   - gateway serial log (RX/validate path proof)
   - **handheld kiosk render** (the actual production UI; required after the handheld pivot — desk-Pi dev kiosk is no longer "the" kiosk)
   - **base-sync export status** when the WiFi + power-good gate is open: ATAK marker observation OR FreeTAKServer log entry (proves the synthetic frame transits the full pipeline including the gated outbound emitter — relevant once `tak-cot-integration-spike.md` Phase 2 lands)
   Combination is fine; justify each chosen surface in one sentence.

8. **Scenario catalogue v0** — see Pass criteria; deliverable is a TOML sketch, not a prose list.

## Pass criteria (spike is done)

- **Hardware path chosen** (H1 / H0 / H2) with a one-paragraph rationale.
- **Coverage matrix** filled in for the chosen path. Tick which layers are exercised:

  | Layer | H1 (fake-tag SX1262) | H0 (host SX1276) | Fallback A (gateway/UI fixture) |
  |---|---|---|---|
  | protocol encoding |   |   |   |
  | RF TX |   |   |   |
  | relay RX |   |   |   |
  | relay forward |   |   |   |
  | gateway RX |   |   |   |
  | SQLite ingest |   |   |   |
  | kiosk render |   |   |   |

  Filling this in makes the coverage loss of H0 or fallback A explicit, not implicit.

- **Protocol crate gap list.** Either confirmed sufficient or each missing item named (encoder method, CRC layer, sentinels, `node_id` type, etc.) with a follow-up ticket per gap.

- **Scenario catalogue v0 as a TOML sketch**, not prose. Example shape (the spike refines this — exact field names depend on the protocol crate inventory):

  ```toml
  [[scenario]]
  name = "single_known_node"
  node_id = 240          # u8, in reserved synthetic range 240..=254
  lat = 50.9
  lon = 5.4
  seq_nr = 0
  sos = false
  count = 1
  verification = "tag_reports row appears within 5 s; handheld kiosk renders marker"

  [[scenario]]
  name = "dedup_same_seq"
  node_id = 240
  seq_nr = 0
  count = 2
  spacing_ms = 200
  verification = "tag_reports has exactly 1 row for (node_id, seq_nr)"

  # Further scenarios — each tied to a verification surface:
  #   sos_set                — SOS flag bit on; handheld kiosk SOS indicator + tag_reports flag;
  #                            base-sync gate open → ATAK distress event observed
  #   heartbeat_stream       — single synthetic node at production cadence; relay forward holds
  #   app_crc_invalid        — only included if the protocol crate exposes an observable
  #                            app-level CRC; otherwise dropped from v0 (see What to verify §2)
  #   unknown_node           — node_id OUTSIDE reserved synthetic range (e.g. 100 with no
  #                            nodes.toml entry); gateway unknown-node path
  #   seq_nr_rollover        — seq_nr near u32::MAX then 0; dedup window edge
  ```

  Each scenario names its verification surface (per What to verify §7).

- **Reserved test-only `node_id` range** confirmed against the protocol crate's `node_id` type, with the test-session `nodes.toml` entries proposed and an explicit "do not deploy this config" warning attached.

- **Implementation ticket filed and staged** (path or title). Work is split into phases, and the **first ticket targets the smallest slice that proves RF TX with a valid POSITION frame** — not the full scenario catalogue. Phases:

  1. **Host-only scenario generator** using `crates/protocol`. Produces encoded POSITION bytes (or hex). **No radio.** Validates protocol-crate integration in isolation.
  2. **Fake-tag firmware: one fixed valid POSITION over SX1262.** No scenario engine yet — a single hard-coded frame is enough to prove RF TX → relay RX → gateway ingest end-to-end. Smallest RF proof.
  3. **Fake-tag accepts one minimal host command or TOML-selected scenario** and transmits it. Adds the control surface chosen per What to verify §4.
  4. **Full scenario catalogue against relay + gateway + kiosk.** All v0 scenarios from the TOML sketch, each tied to its verification surface.

  Phase 1 may be folded into Phase 2 if the implementer prefers, but **Phase 4 must not be bundled with earlier phases**. Each radio-touching phase requires the implementer to run `/rust_lora_phy_preflight` and cite `resources/docs/lora-phy-preflight.md` in its preflight statement.

- **Implementation guardrails** (capture in the ticket itself, not just here):
  - v0 host control surface should prefer TOML scenario replay or one-shot USB-serial commands.
  - Do **not** design an interactive orchestration service unless the decision note explicitly justifies why TOML / one-shot commands are insufficient.
  - The first implementation ticket targets the smallest phase that proves RF TX with a valid POSITION frame — not the whole scenario catalogue.

- **Explicit "not implemented in this spike" note** in the decision note. The spike does not produce code.

## Fail criteria (re-scope)

- The `protocol` crate's wire format is not yet stable enough for an external consumer to lock onto. The spike's deliverable then becomes "block on protocol-crate stabilisation," with the gap documented as a follow-up.
- Neither a spare Heltec V2 nor a spare Dragino HAT is realistically available before bring-up starts — the spike returns "blocked on hardware" and the user picks between waiting and falling back to Option A below.
- Duty-cycle accounting shows the source cannot run at any useful cadence without breaching the 1% sub-band M budget — fall back to a narrower scenario set, or commit to a separate test channel and record that as a v1a deviation.
- The CRC-layer-ownership question (What to verify §2) cannot be answered without first stabilising the protocol crate — drop the wrong-CRC scenario from v0 and record it as a follow-up.

## Fallback options

**Option A — gateway/UI fixture only (not e2e).** The synthetic source bypasses RF and the relay entirely and feeds frames into the gateway via an internal channel (the same `mpsc` the real receiver feeds into) behind a test binary or test feature flag.

- **Direct SQL `INSERT` into `tag_reports` is not packet injection and not e2e — do not use it as the fixture path.** It skips the protocol decode and the gateway ingest pipeline.
- **Internal-channel injection** exercises gateway ingest, SQLite write, and kiosk render, but skips RF and the entire relay path. Useful only for gateway/UI smoke testing while real RF hardware is not yet available.
- Per the service-boundary rule in Scope fence, this fixture lives behind a test feature flag and is **never compiled into the deployed gateway binary**.

**Option B — replay from a captured `.bin` of recorded frames.** Once a real tag exists, capture its TX over the air with the gateway in promiscuous-log mode, save as a binary file, replay later via H1 or H0. Pros: no encoder dependency, exact production wire shape. Cons: chicken-and-egg — needs real tag firmware to exist first.

**Option C — defer.** Accept that v1a bring-up uses real tag hardware walking the garden, and the synthetic source becomes a v1b or post-v1 concern. Cheapest. Slowest feedback loop.

## Decision note template

```
Date:
Result: H1 / H0 / H2 / re-scope (per Fail criteria) / fallback A / fallback B / fallback C

Hardware path:
  - chosen:
  - rationale (one paragraph):
  - hardware on hand to execute:

Coverage matrix (chosen path):
  - protocol encoding:
  - RF TX:
  - relay RX:
  - relay forward:
  - gateway RX:
  - SQLite ingest:
  - kiosk render:
  - coverage gaps accepted (if H0 or fallback chosen):

Protocol crate state:
  - exposes today (POSITION fields, encoder, CRC layer, node_id type):
  - missing for source:
  - follow-up ticket for protocol gaps (if any):

Host control surface:
  - chosen:
  - rationale:
  - confirmation: control surface lives on host, not in deployed gateway:

Reserved test-only node_id range:
  - node_id type/width (from protocol crate): u8 (0–254, 255 reserved) — confirmed 2026-05-06
  - proposed range: 240..=254
  - applied to nodes.toml? (test session only / N/A — gateway not built):
  - "do not deploy this config" warning attached: yes
  - synthetic node visual distinction in kiosk:

CRC layer ownership:
  - app-level CRC owner:
  - PHY-level CRC owner:
  - wrong-CRC scenario observable by gateway? yes / no / N/A
  - if no: scenario dropped from v0:

Duty-cycle accounting:
  - source's max sustained cadence inside 1% budget:
  - shared with production traffic? yes / no / test-only channel
  - budget table updated? yes / no / N/A

Test scenario catalogue v0:
  - TOML sketch (paste or file path):

Verification surface required:
  - chosen:
  - reason:

Implementation tickets (staged):
  - Phase 1 (host-only scenario generator, no radio) — link / path:
  - Phase 2 (fake-tag fw: one fixed POSITION over SX1262) — link / path:
  - Phase 3 (fake-tag accepts one host command or TOML scenario) — link / path:
  - Phase 4 (full scenario catalogue: relay + gateway + kiosk) — link / path:
  - First ticket targets smallest RF-TX-proving phase (not Phase 4)? yes / no
  - Phase 1 folded into Phase 2? yes / no
  - Each radio-touching phase carries /rust_lora_phy_preflight requirement: yes / no / N/A
  - Control-surface choice (per What to verify §4): TOML / one-shot serial / TCP-socket
  - If TCP-socket: justification why TOML and one-shot are insufficient:

Not implemented in this spike: confirmed (no code produced).

Next action:
```
