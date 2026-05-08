---
title: "Decision note — handheld-pivot doc / ADR consistency audit (close)"
date: 2026-05-07
type: dev-log
session-trigger: "Closing spikes/handheld-pivot-doc-audit-spike.md per its decision-note template"
spike: spikes/handheld-pivot-doc-audit-spike.md
---

# Decision note — handheld-pivot doc / ADR consistency audit

Closes [`spikes/handheld-pivot-doc-audit-spike.md`](../spikes/handheld-pivot-doc-audit-spike.md). The spike is enumeration only — no ADR or doc edits land in this note.

## Date

2026-05-07.

## H1 / H0 verdict

**H1 — large pivot.** The 2026-05-06 reframe to "local-first handheld gateway with opportunistic base-mode export" disturbs >2 ADRs and triggers ≥2 new ADRs. Three new ADRs proposed (ADR-015 / 016 / 017). Five existing ADRs flip to *Refined-in-part* or *Superseded-in-part*. The remaining six ADRs are kept as-is. Section-level (not whole-document) edits are sufficient for `ARCHITECTURE.md`, `README.md`, `TODO.md`, `bom.md`, `production-concerns.md`, `CLAUDE.md`.

H0 (one new ADR plus targeted edits) is rejected: form factor, base-mode export gate, and custom enclosures are three separable concerns and merging them into one ADR-015 would re-litigate any one of the three whenever the others move.

## Contradiction count by severity

- **A (outright contradiction):** 12 — A1, A2, A3, A4, A6, A7, A8, A11, A12 inherited from `dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md`; plus three pivot-induced flips: A15 (gateway form factor: ADRs say wall-/shelf-mounted, pivot says handheld), A16 (ADR-008 outbound-network ban vs. base-mode CoT/TAK export), A17 (no enclosure decision currently exists for the gateway; pivot demands one). A9 retracted, A10 reframed per dev-log.
- **B (stale framing):** 9 — `ARCHITECTURE.md:32` "snowstorm scenario" framing; `ARCHITECTURE.md:355` "Pi 3B+ or 4 + 7" DSI"; `ARCHITECTURE.md:435` fullscreen kiosk module; `ARCHITECTURE.md:580` §14 "WiFi dies"; `ARCHITECTURE.md:617` v0.5 "kiosk renders a marker"; `ARCHITECTURE.md:632` v1a "gateway at the kitchen window"; `decisions/ADR-005-map-and-ui.md:18` "kiosk at the mountain hut"; `decisions/ADR-005-map-and-ui.md:22` "7" DSI 1024×600"; `decisions/ADR-007-touchscreen-primary-ui.md` "wall- or shelf-mounted".
- **C (cosmetic / one-word swap):** 6 — `CLAUDE.md:77` `rppal` → `rpi-pal` (still pending from 2026-05-05); `bom.md:99` NTP-on-WiFi preface; `bom.md:12` ADR-012 alignment line; `TODO.md:20` wooden-pole / hose-clamps line; forward-references to non-existent docs (`software/repo-layout.md`, `hardware/relay-assembly.md`, etc., per dev-log C1); ETSI citation spread (per dev-log D9).

Cited evidence locations are repo-local: `rg -n` patterns include `rppal`, `mountain hut`, `kitchen window`, `WiFi dies`, `7\"`, `wall.mount`, `wooden pole`, `hose clamp`, `v0 runs behind WiFi with NTP`, `ADR-012`.

## ADR ledger verdict

| ADR | Title | Verdict |
|----|---|---|
| ADR-001 | Rust everywhere (firmware + gateway + UI) | **Kept.** Pivot does not move language stack. |
| ADR-002 | Tag hardware: Wireless Tracker V2 | **Refined-in-part by ADR-017** (custom tag enclosure annex; the OEM board pick is unchanged). |
| ADR-003 | Relay hardware: Wireless Tracker V2 + Solar Kit | **Kept.** Solar Kit shell stays for the relay. Pole-hardware contradiction (A3) is a doc-edit problem, not an ADR-level move. |
| ADR-004 | Gateway: RPi + Dragino HAT + Yocto | **Superseded-in-part by ADR-015** (handheld substrate + form factor); Yocto + Dragino + single-binary stay. |
| ADR-005 | Map & UI — native Rust kiosk | **Refined-in-part by ADR-015** (display moves from 7" DSI 1024×600 wall-mount to a 5"-class handheld panel; handheld-orientation language replaces the "wall- or shelf-mounted" framing). Native-Rust + PMTiles stance unchanged. |
| ADR-006 | Relay has GNSS + BLE maintenance | **Refined-in-part by ADR-015** (gateway gains BLE *central* role; relay stays peripheral; BLE topology is the new addition). The relay-side contract is unchanged. The A2 v1-vs-v2+ contradiction is resolved by ARCHITECTURE.md / TODO.md edits, not by ADR-006 amendment. |
| ADR-007 | Touchscreen is the only UI | **Refined-in-part by ADR-015** ("wall- or shelf-mounted" wording; "auto-recovers from power cycles" extends to "and from low-battery shutdowns"). Read-only-map invariant is preserved by the ble-gateway-ui-flow spike's modal-as-maintenance-overlay reconciliation. |
| ADR-008 | No cloud, no downlink, pure uplink | **Superseded-in-part by ADR-016.** ADR-008's "no internet-hosted server / no REST / no WebSocket / no phone app" stance is preserved. The new vocabulary distinguishes (a) no inbound network surface, (b) no cloud-bound REST, (c) no internet-routed destinations, (d) outbound LAN multicast/unicast under explicit gate. Only category (d) is added. (a)-(c) stay closed. |
| ADR-009 | Database: SQLite | **Kept.** WAL + recent-window dedup + single-file unchanged. Clean-shutdown semantics for SQLite move from "ARCHITECTURE plus production-concerns §4 mains-loss" to "v1-active concern via runtime-task spike + power spike", but that is a runtime-task issue, not an ADR-009 move. |
| ADR-010 | SOS encoding: single band, flag bit, jittered cadence | **Kept.** Wire format unchanged. |
| ADR-011 | Gateway time source: DS3231 RTC + opportunistic GPS | **Kept (with mounting note).** No NTP, RTC primary, GPS opportunistic — all preserved. Antenna routing for the L80 GPS through a 3D-printed shell is a mounting concern that the gateway-handheld-enclosure spike picks up; that is not an ADR-011 move. |
| ADR-012 | Node roles, sighting semantics, v1a/v1b scope | **Kept (already Superseded-in-part by ADR-013/014).** No further movement from this pivot. v1a/v1b split + tag buzzer + non-goals survive untouched. |
| ADR-013 | Multi-hop flood forwarding via packet_id dedup | **Kept.** Wire format and dedup unchanged. |
| ADR-014 | Duty-cycle budget table as mandatory protocol gate | **Kept.** Budget table is unchanged by the pivot. |

## New ADRs to write (enumerated, not authored)

Each is a separate follow-up ticket. **None drafted in this spike.**

- **ADR-015 — "Gateway form factor v1: handheld portable"** *(Status: Proposed)* — supersedes-in-part ADR-004 (substrate + form factor + display + 7" DSI assumption); refines-in-part ADR-005 (display size class), ADR-006 (BLE central role on the gateway is a new peer), ADR-007 (handheld vs. wall-mounted; low-battery recovery). Substantive content lives in [`spikes/gateway-handheld-substrate-spike.md`](../spikes/gateway-handheld-substrate-spike.md), [`spikes/gateway-handheld-power-architecture-spike.md`](../spikes/gateway-handheld-power-architecture-spike.md), and [`spikes/gateway-runtime-task-architecture-spike.md`](../spikes/gateway-runtime-task-architecture-spike.md). The ADR pulls their close-of-spike outcomes into a single decision; it does not author them. (Pi 4 retirement 2026-05-07 means the substrate pre-condition the audit close flagged is now resolved; Pi 5 is the working candidate.)
- **ADR-016 — "Base-mode export gate: WiFi + power-good gated outbound LAN CoT/TAK"** *(Status: Proposed)* — supersedes-in-part ADR-008 (introduces category (d) "outbound LAN multicast/unicast under explicit gate"). Substantive content lives in [`spikes/tak-cot-integration-spike.md`](../spikes/tak-cot-integration-spike.md) and [`spikes/gateway-runtime-task-architecture-spike.md`](../spikes/gateway-runtime-task-architecture-spike.md). The ADR fixes the wording boundary; it does not invent the gate predicate.
- **ADR-017 — "Custom 3D-printed waterproof enclosures for gateway and tag"** *(Status: Proposed)* — refines-in-part ADR-002 (tag enclosure annex) and ADR-003 (relay enclosure stays OEM Solar Kit; the gateway and tag get custom shells). Substantive content lives in [`spikes/gateway-handheld-enclosure-spike.md`](../spikes/gateway-handheld-enclosure-spike.md), [`spikes/tag-handheld-enclosure-spike.md`](../spikes/tag-handheld-enclosure-spike.md), and [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md). The ADR commits IP target + material shape; it does not draft Fusion 360 geometry.

No fourth ADR needed in this audit. Buzzer-survival, v1a/v1b split, and pole-hardware all stay inside existing ADR ledger entries.

## Per-file edit checklist

Naming the sections that need editing. **No edit text drafted here.** Phase B / separate tickets execute the edits.

### `CLAUDE.md`

- line 77: `rppal` → `rpi-pal` (one-word fix carried over from `dev-log/2026-05-05`).
- "Do NOT re-open" list: leave `let's just NTP the clock when WiFi is around` and `let's just add a small web dashboard` intact in spirit — but ADR-008's text is now scoped to *internet-bound* by ADR-016. Add a short "Note: outbound LAN CoT/TAK under the ADR-016 gate is not the same as a web dashboard" parenthetical, OR cite ADR-016 directly so the door does not look like it has been re-opened. Pick one in Phase B.
- "Tools this project uses": no edit until ADR-015 lands; the "Pi + Dragino + Yocto" line is downstream of ADR-004/ADR-015.

### `ARCHITECTURE.md`

- §1 (`:16`–`:34`): mission/operational context wording — "at the mountain hut" → handheld-portable framing; mountain-hut survives as *one possible* deployment site, not THE site.
- §3 (`:60`): system-concept paragraph — same reframe as §1.
- §4 (`:68`–`:96`): architecture-overview ASCII diagram — replace `+ 7" DSI touchscreen` line with the handheld panel framing; add base-mode export edge as a dashed conditional arrow.
- §5 (`:96`–`:115`): "Why this architecture fits the mission" — graceful-degradation framing keeps "system unaffected by WiFi loss" but adds "base-mode export *requires* WiFi and is the gated path".
- §9 Modes (per dev-log A2 / A5): BLE maintenance v1-vs-v2+ flip already known; per ADR-015 the gateway-as-BLE-central is added; reserved-flag-bits + sentinel-consistency checks (dev-log A5) folded into the validation rule list.
- §10 (`:353`–`:425`): "Gateway responsibilities" — Pi 3B+/4 + Dragino HAT + 7" DSI sentence replaced with handheld-substrate framing (cite pending ADR-015).
- §11 (`:431`–`:469`): kiosk dimensions + fullscreen process — display class language updated; fullscreen still valid but "single-purpose appliance" framing softened to "single-purpose handheld".
- §13 (`:530`–`:565`): duty-cycle budget — no movement; keep as-is.
- §14 (`:566`–`:601`): graceful-degradation cases — "WiFi dies (snowstorm scenario)" is preserved with a footnote that base-mode export turns OFF when WiFi is gone (the export path is the *only* layer that depends on WiFi; nothing else does).
- §15 (`:602`–`:670`): roadmap — v1a "gateway at the kitchen window" / v0.5 "kiosk renders a marker" reworded for handheld; mountain-hut wording softened; cite the new spikes.
- §16 (`:672`–`:714`): open technical risks — add three rows for the pivot: (a) battery cold-charge / runtime envelope, (b) 3D-printed-enclosure IP rating + IPEX strain relief on gateway, (c) Pi-class onboard BLE/WiFi through a plastic shell.
- §17 (`:715`–`:746`): repo layout — no edit; the kiosk-as-gateway-binary-module statement is preserved by ADR-015.

### `decisions/ADR-004-gateway-platform.md`

- **Out of scope for this Phase B.** Edits land when ADR-015 lands. Status row will move to *Superseded in part by ADR-015* at that point.

### `decisions/ADR-005-map-and-ui.md`

- **Out of scope for this Phase B.** Display-size and orientation move when ADR-015 lands.

### `decisions/ADR-007-touchscreen-primary-ui.md`

- **Out of scope for this Phase B.** "Wall- or shelf-mounted" + "auto-recovers from power cycles" wording moves when ADR-015 lands.

### `decisions/ADR-008-no-cloud-no-downlink.md`

- **Out of scope for this Phase B.** Categorical wording move (a/b/c/d) lands when ADR-016 lands.

### `decisions/ADR-011-gateway-time-source.md`

- **Out of scope for this Phase B.** No NTP / RTC primary / GPS opportunistic stance unchanged. Antenna-routing language is enclosure-spike scope.

### `decisions/ADR-006-relay-has-gnss.md`

- **Out of scope for this Phase B.** BLE topology refinements land via ADR-015 (gateway-as-central is the new addition; relay-as-peripheral is unchanged). The A2 v1-vs-v2+ contradiction is resolved in `ARCHITECTURE.md` and `TODO.md`, not here.

### `README.md`

- "Status at a glance" → "Hardware in hand" row (per dev-log A7): Heltec order shipped 2026-05-05, no longer "not yet placed".
- "Status at a glance" → "Code" row (per dev-log A8): `crates/protocol` exists, ships 22 unit tests, frozen canonical vectors. "Production firmware/gateway code not yet written" is wrong; replace with the per-crate status.
- Project-summary opening paragraph: mountain-hut framing softens to "handheld gateway, optionally deployed at a mountain hut or carried by hut staff". No banishment, just reframing.

### `TODO.md`

- "Right now" section: fold dev-log A6/A7 stale-inventory rows into a one-pass status update *before* applying handheld-pivot edits.
  - replace "3× Wireless Tracker V2 ... 1× Solar Kit" with past-tense "Heltec order #110639 shipped 2026-05-05: 10× Tracker V2 + 2× Solar Kit". This resolves A6.
  - line 20 "Order the wooden pole + stainless hose clamps for the garden relay (local hardware store)." resolves to match `bom.md`'s designed-pole position (A3 / dev-log row a). Drop the hose-clamps line.
- "Right now": add cross-references to the open pivot spikes (substrate, power, enclosure, runtime, BLE-commissioning, ble-gateway-ui-flow, tak-cot, fake-position-injector, pmtiles-walkers).
- "Deferred (v2+)" section: BLE maintenance moves out of v2+ (A2 resolved on the v1 side per ADR-006; gateway-as-central is added).
- v0/v0.5/v1a gates: 7" DSI / Dragino HAT / Yocto / wall-mount / kitchen-window references are softened where the pivot demands; no item is *deleted*, but each gets a parenthetical "(handheld substrate per pending ADR-015)" or "(base-mode export per pending ADR-016)".
- "Blocked" section: SOS button GPIO + button type (dev-log D8) — add a note that physical button choice is owned by `tag-handheld-enclosure-spike.md`.

### `bom.md`

- line 12 (alignment list): drop ADR-012 from "Aligned to", or replace with "ADR-012 (buzzer + non-goals only; rest superseded by ADR-013/014)" (dev-log A12 / audit row c).
- line 99 ("Deferred — v1a prep" preface): rewrite the "v0 runs behind WiFi with NTP at mom's place — no RTC needed yet" sentence so it does not re-open the closed NTP door (dev-log A4 / audit row b). Recommended replacement language: "v0 desk bring-up runs against the dev workstation's manually-set system clock; the DS3231 + CR2032 only become load-bearing at v1a when the gateway moves into field deployment. Per ADR-011, no NTP at any deployment stage."
- Gateway hardware section: substrate (Pi 5 + Dragino HAT + 5" panel, or pending ADR-015 outcome) replaces the prior 7" DSI cart. Footnote-cite this audit close + pending ADR-015.
- "Explicitly NOT ordering": already lists "stainless hose clamps" / "wooden pole from hardware store" / "Fusion 360 designed three-legged base + ground-stake". This stays as the source of truth for the pole-hardware contradiction; ADR-003 + TODO.md align to it.
- **Update 2026-05-08:** "Relay pole — local build" §section replaced with "Relay tripod + Solar Kit adapter (selection per spike)"; "Explicitly NOT ordering" §section now retires the Fusion-360 designed-pole approach alongside the existing hose-clamps + wooden-pole-from-hardware-store entries. CLAUDE.md / TODO.md / ARCHITECTURE.md / ADR-003 status banner updated in the same commit. See [`2026-05-08-relay-mount-tripod-decision.md`](2026-05-08-relay-mount-tripod-decision.md).

### `production-concerns.md`

- §1 (relay VHB cold-cycle creep): add "review trigger: before the relay first leaves the garden" line. Stays a post-v1a concern; pivot does not promote it.
- §2 (18650 cold-charge): add "review trigger: at gateway power-architecture spike close" line. Pivot promotes from "post-v1 risk register" to v1-active scope per audit row g.
- §3 (IPEX strain relief): add "review trigger: at gateway enclosure spike close" line. Pivot promotes; same physics applies to the gateway's external SMA pigtail through the 3D-printed shell.
- §4 (SD-card power-loss / clean-shutdown): add "review trigger: at gateway runtime-task spike close" line. Pivot promotes; battery operation makes it a daily concern.

## Cross-link map (this audit ↔ pivot spikes)

- substrate spike: [`spikes/gateway-handheld-substrate-spike.md`](../spikes/gateway-handheld-substrate-spike.md) — feeds ADR-015 substrate row.
- power spike: [`spikes/gateway-handheld-power-architecture-spike.md`](../spikes/gateway-handheld-power-architecture-spike.md) — feeds ADR-015 + ADR-016 (power-good signal in the gate).
- gateway enclosure: [`spikes/gateway-handheld-enclosure-spike.md`](../spikes/gateway-handheld-enclosure-spike.md) — feeds ADR-017.
- tag enclosure: [`spikes/tag-handheld-enclosure-spike.md`](../spikes/tag-handheld-enclosure-spike.md) — feeds ADR-017 + ADR-002 annex.
- ble commissioning (firmware contract): [`spikes/ble-commissioning-scope-spike.md`](../spikes/ble-commissioning-scope-spike.md) — feeds ADR-015 (BLE central role) + ADR-006 refinement.
- ble gateway UI flow: [`spikes/ble-gateway-ui-flow-spike.md`](../spikes/ble-gateway-ui-flow-spike.md) — feeds ADR-007 reconciliation (modal-as-maintenance-overlay preserves read-only map).
- runtime tasks: [`spikes/gateway-runtime-task-architecture-spike.md`](../spikes/gateway-runtime-task-architecture-spike.md) — feeds ADR-015 (task split + clean-shutdown semantics).
- base-sync (TAK / CoT): [`spikes/tak-cot-integration-spike.md`](../spikes/tak-cot-integration-spike.md) — feeds ADR-016.
- pmtiles retarget: [`spikes/pmtiles-walkers-spike.md`](../spikes/pmtiles-walkers-spike.md) — feeds ADR-015 display retarget (kiosk on a 5"-class panel).
- fake-position update: [`spikes/fake-position-injector-spike.md`](../spikes/fake-position-injector-spike.md) — u8 vs u32 example fix (dev-log A11) is owned there.
- duty-cycle measurement: [`spikes/duty-cycle-measurement-workflow-spike.md`](../spikes/duty-cycle-measurement-workflow-spike.md) — unchanged by pivot; cited for ADR-014 reaffirmation.
- datasheet inventory: [`spikes/datasheet-source-of-truth-inventory-spike.md`](../spikes/datasheet-source-of-truth-inventory-spike.md) — unchanged by pivot; cited for the substrate + enclosure spikes' datasheet citations.
- physical fabrication brief: [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md) — feeds ADR-017.

## Not implemented in this spike

Confirmed (no ADR or doc edits made). The audit is enumeration only. Phase B (cascading non-ADR doc edits) and the three new-ADR tickets (015 / 016 / 017) are separate.

## Next action

1. Print this decision-note path. Wait for explicit "go phase B" before touching any other file.
2. Phase B (separate go-ahead): cascade non-ADR doc edits across `CLAUDE.md`, `README.md`, `TODO.md`, `bom.md`, `ARCHITECTURE.md`, `production-concerns.md`. One commit per file. No `decisions/` edits in Phase B.
3. ADR-015 / ADR-016 / ADR-017 authoring: separate tickets, each gated on its respective spike close.
