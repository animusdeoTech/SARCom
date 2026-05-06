---
title: "Retroactive proposed-list artifact for the 2026-05-06 spike audit pass"
date: 2026-05-06
type: dev-log
session-trigger: "Cowork review noted the spike-audit prompt asked for a printed proposed list before writing — that artifact was missing. Reconstruct retroactively."
---

# Retroactive proposed-list — 2026-05-06 spike audit pass

## What this is

The 2026-05-06 prompt that drove the spike audit said:

> Before writing:
> 1. Print a proposed spike list with: create/update, filename, reason, affected docs/ADRs.
> 2. Then proceed to write/update the tickets unless there is genuine ambiguity about where they belong.

The proposed list was generated in-session but only as part of the chat reply — it was not committed as a standalone artifact. Cowork review flagged this. Reconstructing here from the final spike set so the audit pass leaves a paper trail in `dev-log/`.

This is an **audit artifact only**. It does not change any decisions. It is the list that *should* have been committed before the writes happened.

## Final shape of the audit pass

| Action | File | Reason | Affected docs / ADRs | Duplicate-risk notes |
|---|---|---|---|---|
| create | `spikes/handheld-pivot-doc-audit-spike.md` | Survey every contradiction the handheld pivot creates; produce supersession plan + ADR-015/016/017 enumeration. Owns area A. | CLAUDE.md, ARCHITECTURE.md §1/§3/§4/§5/§10/§11/§14/§15/§16, ADR-004, ADR-005, ADR-007, ADR-008, ADR-011, README.md, bom.md, TODO.md, production-concerns.md | Registrar role: every other new spike cross-links *back* to this one. No duplicate risk if the others stay focused on substantive technical questions. |
| create | `spikes/gateway-handheld-substrate-spike.md` | Pi 5 vs alternatives, HAT physical fit, antenna paths through enclosure, RP1/GPIO/SPI/UART, USB/CM5 fallback. Owns area B. | ADR-004, ARCHITECTURE.md §10/§16/§17, gateway-rx-bringup-spike | Adjacent to existing `gateway-rx-bringup-spike.md`. Distinguished: substrate spike picks the SBC + HAT + antenna shape; rx-bringup spike proves byte-correct RX *given* the chosen substrate. Cross-linked. |
| create | `spikes/gateway-handheld-power-architecture-spike.md` | Battery topology, charger/PD, BMS, NTC, power-good, runtime budget. Owns area C. | ADR-004, ARCHITECTURE.md §10, production-concerns.md §4 | No prior spike on power. New ground. |
| create | `spikes/gateway-handheld-enclosure-spike.md` | Fusion 360 waterproofing, IP65/67, materials, gasket/seals, display window, bulkheads, thermal, condensation. Owns area D (gateway). | ADR-004, ADR-005, ADR-007, production-concerns.md | Sibling to tag enclosure spike below. Different problem (size, has-display, battery service door). |
| create | `spikes/tag-handheld-enclosure-spike.md` | Fusion 360 tag enclosure (pocket form, button, buzzer port, USB-C). Pivot stated both gateway and tag get custom 3D-printed casings. | ADR-002, ARCHITECTURE.md §8, production-concerns.md §3 | Sibling to gateway enclosure. Reuses sealing-techniques research; not a duplicate. |
| create | `spikes/ble-commissioning-scope-spike.md` | Gateway-as-BLE-central commissioning surface, advertising format, auth, no-mesh-control fence. Owns area E. | ADR-006, ARCHITECTURE.md §10, field-deployment-test-fleet-spike.md §4 | `field-deployment-test-fleet-spike.md` §4 already drafted the relay-side BLE health surface. New spike adds gateway-as-central + tag peripheral; **cites §4 verbatim** for the relay surface, does not redo. |
| create | `spikes/gateway-runtime-task-architecture-spike.md` | Task split (lora_rx, validate, db_writer, kiosk, BLE, WiFi monitor, power monitor, cot_gate, cot_emitter, nmea/time), channel boundaries, SQLite WR/RO pattern, low-battery handling. Owns area G. | ADR-004, ADR-009, ARCHITECTURE.md §10/§11/§17 | No prior runtime-task spike. New ground. |
| create | `spikes/duty-cycle-measurement-workflow-spike.md` | ADR-014 gate enforcement, airtime calculator, firmware TX log fields, desk-measurement scenarios, fake-tag cadence budget. Owns area H. | ADR-014, ARCHITECTURE.md §13, fake-position-injector-spike | Adjacent to fake-position spike. Distinguished: this spike commits the airtime accounting + measurement workflow; the fake-position spike consumes the budget rules. Cross-linked. |
| create | `spikes/datasheet-source-of-truth-inventory-spike.md` | Inventory list of every datasheet/schematic that must exist locally before bring-up. Owns area L. | claude-rust-docs-spike (resolved), ADR-002, ADR-003, ADR-004, ADR-011 | `claude-rust-docs-spike.md` (resolved) explicitly deferred this; new spike picks up the deferred work. No duplicate. |
| update | `spikes/tak-cot-integration-spike.md` | Reframe from "optional, ADR-008 conflict" → "architectural commitment under pivot"; commit power-good + WiFi-stable + manual-enable gate; ADR-008 amendment thread routed to doc-audit. Owns area F. | ADR-008 (amendment), ARCHITECTURE.md §10/§14, ADR-007 | Existing spike, was exploratory. Update preserves Phase 1 multicast experiment as the cheap-truth-test. |
| update | `spikes/pmtiles-walkers-spike.md` | Retarget 800×480 7" → 5" / 1280×720 landscape (Pi Touch Display 2 candidate); update Pi GPU references for RP1/Pi 5 stack; add touch-target sizing + portrait-vs-landscape question. Owns area I. | ADR-005, ARCHITECTURE.md §11 | Existing spike; in-place update keeps the H1/H0 + Windows-offline + Pi-evidence framework. |
| update | `spikes/fake-position-injector-spike.md` | Fix `node_id` u32 → u8 (per dev-log audit A11); add handheld kiosk render and base-sync export status to verification surface. Owns area J. | ADR-013, fake-position-injector-spike (existing) | Existing spike; targeted edits, no rewrite. |
| update | `spikes/gateway-rx-bringup-spike.md` | Pivot context note: substrate now load-bearing on Pi 5 not "Pi 3B+/4 whichever has healthy ports"; antenna-through-enclosure new constraint; B11 (WiFi+cloud pivot) marked resolved; B10 (Kiwi cart) marked moot; downstream of new substrate spike. | ADR-004, gateway-handheld-substrate-spike (new) | Existing spike; substantive content (hardware-rev questions, lora-phy on Linux, polled-RX-on-RP1 fallback) preserved. |
| pivot-note | `spikes/field-deployment-test-fleet-spike.md` | Top-of-file partial-staleness banner: pool model + fleet sizing + BLE relay §4 + tag identity §7 still valid; wall-mount/kitchen-window/appliance-on-a-shelf framing reframes; cost envelope changes under handheld. | None edited substantively. | Spike is huge; full rewrite would lose load-bearing argumentation. Banner-only edit. |
| untouched | `spikes/commercial-viability-spike.md` | Domain-fit analysis is orthogonal to form factor / base-sync export. The spike's framing ("local-first, no SIM per tag, sparse periodic sightings") is *strengthened* by the pivot, not changed. | — | — |
| untouched | `spikes/claude-rust-docs-spike.md` | `status: resolved`. Doc-source mechanics unaffected by pivot. Deferred datasheet inventory is now owned by `datasheet-source-of-truth-inventory-spike.md` per its explicit cross-reference. | — | — |

## Cowork review patches applied 2026-05-06

After the initial audit pass landed, Cowork reviewed and flagged seven minor issues. All applied as targeted patches:

1. `datasheet-source-of-truth-inventory-spike.md` — battery cell row no longer hard-codes INR18650-25R; status is `pending-part-selection`; upstream is the power-architecture spike. (No SKU lock-in.)
2. `gateway-runtime-task-architecture-spike.md` — task table prefaced with "working hypotheses, not decisions" disclaimer; `cot_gate` row no longer restates the predicate (consumes the contract from `tak-cot-integration-spike.md`).
3. `ble-commissioning-scope-spike.md` — H1 auth model explicitly named as a candidate, not the only option; "What this spike does not compare in the timebox" note added (passkey, LESC, stronger flows out of scope; escalate if threat model demands).
4. `gateway-rx-bringup-spike.md` — hard pass criterion added: first gateway RX commit must set SX1276 syncword `0x12` explicitly with a unit test asserting the constant.
5. `tag-handheld-enclosure-spike.md` — buzzer audibility promoted from qualitative to measurable: ≥80 dBA at 1 m on bench after sealing, phone SPL meter; field 5 m note remains observational.
6. `handheld-pivot-doc-audit-spike.md` — explicit checklist rows added for: ADR-003/bom.md/TODO.md pole-hardware contradiction; bom.md NTP-on-WiFi preface; bom.md ADR-012 alignment citation; forward references to non-existent docs; ARCHITECTURE.md §9 validation-list completeness; ETSI citation spread; production-concerns.md §2/§3/§4 review-trigger reframe under pivot (§1 stays post-v1a).
7. This dev-log artifact (the retroactive proposed list itself).

## Notes / non-actions

- **No ADRs created or edited** in either pass.
- **No code, no dependencies, no BOM commitments.**
- **TODO.md not edited** — pre-existing unrelated modifications in working tree, plus item 9 spirit (don't bundle unrelated edits). Backlog items (`DropReason` granularity, SeenCache capacity doc) listed in the audit-pass summary as deferred.
- **`decisions/ADR-004-gateway-platform.md` modified in working tree** — unrelated to this audit pass; left for separate commit/revert by Pieter.
