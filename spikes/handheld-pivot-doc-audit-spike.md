---
title: "Spike — Handheld pivot doc/ADR consistency audit"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-06
---

# Spike: Handheld pivot doc / ADR consistency audit

## Why this spike exists

On 2026-05-06 the v1 architecture pivoted from **fixed mountain-hut kiosk** to **local-first handheld gateway with opportunistic base-mode export**. The pivot adds:

- handheld portable form factor, custom Fusion 360 waterproof enclosures (gateway and tag)
- battery + charging (rechargeable)
- 5-inch display as primary UI target (was 7" DSI wall-mount)
- BLE commissioning interface (gateway-side, central role)
- WiFi-aware base/sync mode
- conditional outbound CoT/TAK export when WiFi exists and stable/external power is present

The previously-Accepted ADR ledger (001–014) and the rest of the doc set still describe the **pre-pivot** architecture in many places: "mountain hut staff", "wall- or shelf-mounted", "snowstorm hits, WiFi is gone — system unaffected", "no internet-hosted server", "pure uplink", "7" DSI touchscreen". A coherent doc set after the pivot needs:

- a clear list of which sentences contradict the pivot
- a verdict per ADR: kept-as-is / refined-in-part / superseded
- an enumerated list of new ADRs the pivot triggers (ADR-015 etc.) — without writing them yet

This spike does **not** rewrite the architecture. It produces the audit + a supersession plan so the follow-up edits land cleanly.

## Hypothesis / research question

**H1.** The pivot is large enough that >2 ADRs need superseding-in-part and ≥2 new ADRs are required (gateway hardware/form-factor + ADR-008 amendment for outbound CoT). The other docs (ARCHITECTURE.md, README.md, TODO.md, bom.md, production-concerns.md) need section-level edits, not rewrites.

**H0.** Most contradictions are wording-level; one new ADR (ADR-015 covering form-factor + base-sync + power) plus targeted ARCHITECTURE.md sectional edits is sufficient.

## Scope fence

- **No ADR edits.** Spike output is an enumerated list of which ADRs need supersession, refinement, or new follow-up ADRs — not the supersessions themselves.
- **No ARCHITECTURE.md / TODO.md / README.md / bom.md / CLAUDE.md edits.** Spike output is a per-file edit checklist; the follow-up tickets execute the edits.
- **No re-litigation of accepted decisions** that the pivot does not actually disturb. Wire format (ADR-013), CRC (ARCHITECTURE.md §7), duty-cycle gate (ADR-014), SQLite-as-source-of-truth (ADR-009) are out of scope.
- **No new architecture text.** The doc-audit spike does not invent the new gateway architecture — that is the substrate / power / enclosure / runtime spikes' job.

## What to verify

For each of the files below, produce a list of contradicting sentences/sections introduced by the pivot. Cite line numbers.

1. **CLAUDE.md** — "Do NOT re-open" list (`let's just NTP the clock when WiFi is around`, `let's just add a small web dashboard`); "Tools this project uses"; principles file framing.
2. **ARCHITECTURE.md** — §1 mission ("at the mountain hut"), §3 ("at the mountain hut"), §4 diagram (`+ 7" DSI touchscreen`, `Yocto Linux`), §5 ("graceful degradation … WiFi is down"), §10 (gateway-as-Pi-with-Dragino-HAT-and-7"-DSI), §11 (kiosk dimensions, fullscreen process), §14 ("WiFi dies — nothing happens"), §15 (v1a "gateway at the kitchen window", v0.5 "kiosk renders a marker"), §16 (open risks not yet covering battery / enclosure / antenna routing).
3. **decisions/ADR-004-gateway-platform.md** — Pi 3B+/4 + 7" DSI + mains-power; "single-purpose image", "appliance".
4. **decisions/ADR-005-map-and-ui.md** — 7" DSI 1024×600, "wall- or shelf-mounted", "browser objection". Display size and orientation move; map UX changes for handheld.
5. **decisions/ADR-007-touchscreen-primary-ui.md** — "wall- or shelf-mounted"; "auto-recovers from power cycles" (now also from low-battery states).
6. **decisions/ADR-008-no-cloud-no-downlink.md** — "No REST API, no web dashboard, no mobile app"; "No outbound network calls". Conditional outbound CoT/TAK is a new category that needs explicit boundary language; the ADR's vocabulary does not currently distinguish *outbound LAN-multicast under explicit gate* from *cloud-bound REST*.
7. **decisions/ADR-011-gateway-time-source.md** — DS3231 + GPS HAT both still work conceptually but physical mounting in a handheld enclosure changes; antenna routing for the L80 GPS through a sealed shell is new.
8. **decisions/ADR-006-relay-has-gnss.md** — BLE maintenance scope already in conflict with ARCHITECTURE.md §9 / TODO.md (per dev-log audit A2). The pivot adds gateway-side BLE central role; ADR-006 is silent on which BLE peer takes which role.
9. **README.md** — "Hardware in hand" (also stale per dev-log audit A7); status framing.
10. **TODO.md** — "Right now" section, v0/v0.5/v1a gates referencing 7" DSI / Dragino HAT / Yocto / wall-mount / kitchen window; "Deferred (v2+)" entries that the pivot promotes (BLE maintenance, phone-friendly access, cloud-or-LAN sync).
11. **bom.md** — entire gateway hardware section (Pi 5 + 7" DSI cart was never placed; pivot changes it again); production-concerns.md §4 SD-card power-loss rewords against battery + clean shutdown.
12. **production-concerns.md** — §1 (relay VHB, unaffected), §2 (18650 cold-charge — also applies to gateway pack, not just relay), §3 (IPEX strain relief — applies to gateway too if external SMA antenna routes through 3D-printed shell), §4 (SD-card power-loss reframes against battery + power-good signal).

### Additional explicit checklist rows (added 2026-05-06 on Cowork review)

These are pre-existing contradictions that the audit must explicitly carry into its per-file checklist, not absorb implicitly:

a. **ADR-003 / bom.md / TODO.md pole-hardware contradiction.** ADR-003 "Decision" + Order checklist say *"u-bolts or stainless hose clamps"*. bom.md "Explicitly NOT ordering" strikes hose clamps + wooden pole and replaces with a Fusion 360 three-legged base + ground-stake. TODO.md "Right now" still orders the wooden pole + hose clamps from the local hardware store. Three docs, three positions. The audit must list this as a per-file edit (ADR-003 + bom.md + TODO.md) and pick which doc carries the truth. (Pre-existing, dev-log audit A3.)

b. **bom.md NTP-on-WiFi preface.** bom.md "Deferred — v1a prep" preface contains the wording *"v0 runs behind WiFi with NTP at mom's place — no RTC needed yet"*. ADR-008 + ADR-011 + CLAUDE.md "Do NOT re-open" all explicitly forbid NTP. The audit must verify no remaining bom.md wording re-opens NTP or WiFi-time-sync as a v0 development convenience; rewrite the preface so it does not re-open the closed door. (Pre-existing, dev-log audit A4.)

c. **bom.md "Aligned to" ADR-012 reference.** bom.md line 12 cites *"ADR-002, ADR-003, ADR-004, ADR-011, ADR-012, ADR-013, …"* as alignment. ADR-012 is *Superseded in part by ADR-013, ADR-014*. The audit must decide whether bom.md should drop the ADR-012 line, replace with *"ADR-012 (buzzer + non-goals only; rest superseded by ADR-013/014)"*, or replace it with citations to ADR-013/014 directly. (Pre-existing, dev-log audit A12.)

d. **Forward references to non-existent docs.** Across ARCHITECTURE.md, ADRs, and README.md, multiple paths are referenced as if they exist: `software/repo-layout.md`, `hardware/relay-assembly.md`, `hardware/desk-inventory.md`, `hardware/gateway-assembly.md`, the planned `architecture/{system-overview, sighting-model, protocol, operational-modes, non-goals}.md` split. None exist. Each citation is hedged with "(planned; for now see ...)" but the volume is high enough to confuse contributors. The audit must enumerate every forward reference and decide per-file: remove the reference, inline the content, or convert to a future TODO. (Pre-existing, dev-log audit C1.)

e. **ARCHITECTURE.md §9 validation list.** §9 lists 7 numbered validation rules (length window, MAGIC, VER, TYPE, LEN, CRC, accept). `crates/protocol/src/position.rs` performs two additional checks the doc list does not name: (i) **reserved-flag-bits check** (`flags & 0xF8 != 0` → reject) and (ii) **GPS-valid / sentinel-consistency check**. Code is the careful version; doc is incomplete. The audit must enumerate the missing checks and decide whether to fold them into §9's list or move them to a gateway-only validation step with explicit annotation. (Pre-existing, dev-log audit A5.)

f. **ETSI / EN 300 220 citation spread.** *"ETSI EN 300 220-2 V3.2.1"* is hard-coded across multiple docs (ARCHITECTURE.md §12, ADR-010, ADR-014, others). When ETSI revises the standard, all sites must move together. The audit must decide whether to centralize the citation in one place (ARCHITECTURE.md §12 as the single source-of-truth, with the ADRs cross-linking) or accept the duplication and document a manual-update rule. (Pre-existing, dev-log audit D9.)

g. **production-concerns.md review-trigger reframe.** §1 (relay VHB cold-cycle creep) stays a post-v1a concern — the relay still uses the OEM Solar Kit shell per ADR-003, unchanged by the pivot. §2 (18650 cold-charge), §3 (IPEX strain relief), and §4 (SD-card power-loss / clean-shutdown) **promote into active v1 concern surface** under the handheld pivot — §2 because the gateway now has a battery pack with the same physics, §3 because the gateway's external LoRa SMA pigtail through a 3D-printed shell shares IPEX strain-relief problems with the relay, §4 because battery operation makes clean-shutdown a daily concern not a once-in-a-blue-moon mains-loss concern. The audit must add review-trigger lines to §2 / §3 / §4 ("review at gateway power-architecture spike close" / "review at gateway enclosure spike close" / "review at gateway runtime-task spike close") and leave §1's review trigger as "before the relay first leaves the garden". (New under pivot.)

These rows are audit scope only. The audit does not fix the underlying docs — it enumerates and routes the fixes.

## Pass criteria

Spike is done when the decision note contains:

- **Contradiction list**, citing file + line per item, grouped by severity (A: outright contradiction; B: stale framing; C: cosmetic).
- **ADR ledger verdict**, one row per existing ADR:
  - Kept (no change)
  - Refined-in-part (specific section needs editing in a follow-up ADR amendment, or in ARCHITECTURE.md when the ADR is downstream of architecture text)
  - Superseded-in-part (a new ADR-NNN must be written; existing ADR's status moves to `Superseded in part by ADR-NNN`)
- **New ADR enumeration**, one row per planned new ADR, with: working title, scope, which existing ADR(s) it supersedes-in-part, expected status `Proposed` until written. Minimum expected list:
  - ADR-015 (working title) "Gateway form factor v1: handheld portable"
  - ADR-016 (working title) "Base-mode export: WiFi + power-good gated outbound LAN CoT/TAK"
  - ADR-017 (working title) "Custom 3D-printed waterproof enclosures for gateway and tag"
  - The audit may add more or merge — not predetermined.
- **Per-file edit checklist** for each of the 12 files in §What to verify, naming the sections that need edits without writing the edits.
- **Cross-link map** to the new spikes covering the substantive questions (substrate, power, enclosure, runtime, BLE-commissioning, base-sync, fake-tag verification, duty-cycle measurement, datasheet inventory). The doc audit cites those spikes; it does not duplicate their content.

## Fail criteria

- Audit produces a contradiction list but no per-file edit checklist or ADR verdict — re-scope, the audit needs to land actionable artifacts.
- Audit attempts to write any of the new ADRs inline — stop and re-scope; the spike is enumeration, not authoring.
- Audit gets pulled into resolving substantive technical questions (battery topology, enclosure material, etc.) — those belong in their own spikes; this spike only points at them.

## Fallback / next action

If the contradiction surface is materially bigger than expected (>40 specific sentences across the doc set), split the audit into two: ADR-only audit + ARCHITECTURE.md+TODO.md+README.md+bom.md+production-concerns.md audit. Do not push the timebox past 1 day.

## Decision note template

```
Date:
H1 / H0 verdict: H1 / H0 / mixed (explain)

Contradiction count by severity:
  A (outright contradiction):   __
  B (stale framing):            __
  C (cosmetic / one-word swap): __

ADR ledger verdict (one row per existing ADR-001..014):
  ADR-001: Kept / Refined / Superseded-in-part by ADR-NNN — note:
  ADR-002: ...
  (etc.)

New ADRs to write (enumerated, not authored):
  ADR-015 — title: ___ — supersedes-in-part: ___ — status: Proposed
  ADR-016 — title: ___ — supersedes-in-part: ___ — status: Proposed
  ADR-017 — title: ___ — supersedes-in-part: ___ — status: Proposed
  (etc.)

Per-file edit checklist (file → sections, no edit text):
  CLAUDE.md:        [...]
  ARCHITECTURE.md:  [...]
  ADR-004:          [...]
  ADR-005:          [...]
  ADR-007:          [...]
  ADR-008:          [...]
  ADR-011:          [...]
  ADR-006:          [...]
  README.md:        [...]
  TODO.md:          [...]
  bom.md:           [...]
  production-concerns.md: [...]

Cross-link map (this audit ↔ pivot spikes):
  substrate spike:        ___
  power spike:            ___
  gateway enclosure:      ___
  tag enclosure:          ___
  ble commissioning:      ___
  runtime tasks:          ___
  base-sync (tak-cot):    ___
  pmtiles retarget:       ___
  fake-position update:   ___
  duty-cycle workflow:    ___
  datasheet inventory:    ___

Not implemented in this spike: confirmed (no ADR/doc edits made).

Next action:
```

## Cross-references

- This spike is the registrar for the pivot. Every new spike below should be referenced from the audit's cross-link map and should reference back to this audit.
- `dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md` — pre-pivot detective audit; many of its findings (A1, A2, A4, A6, A7, A8, A11) survive the pivot and should be folded into the contradiction list.
- ADR-013, ADR-014 — out of scope; the pivot does not move these.
