---
title: "Spike — Physical fabrication brief: tripod, tag/gateway shells, transport cases"
status: blocked
type: spike
timebox: 1 day
opened: 2026-05-07
---

# Spike: Physical fabrication brief

## Decision 2026-05-08 — relay mount path committed

**Tripod chosen for v0/v1/v2.** The relay mount is an off-the-shelf plastic tripod with a standard mount thread + a printed/machined Solar-Kit-to-tripod adapter. Both the prior **fully-custom design** hypothesis and the **wooden-pole-and-clamps fallback** hypothesis are dropped — see [`../dev-log/2026-05-08-relay-mount-tripod-decision.md`](../dev-log/2026-05-08-relay-mount-tripod-decision.md).

The §Hypothesis paragraph below is preserved as history. Only these threads remain open within this spike's scope:

1. **Tripod model selection** — load rating, folded length, extended height, operating-temp envelope, stance/footprint, cost envelope.
2. **Solar-Kit-to-tripod adapter spec** — print material (prototype + production), screw thread (1/4"-20 vs 3/8"), anti-rotation feature, mounting-plate dimensions matching the Solar Kit's underside.
3. **Transport-case sizing dependency** — case A internal dimensions must accept the folded tripod alongside the Solar Kit + assembled relay (folded tripod length feeds case dimensions).

The tag + gateway production-tier artifacts (Artifacts 2 + 3 below) remain consumers of the tag + gateway design spikes' close, but they are decoupled from the tripod track and tracked here only as cross-spike dependencies — they do not block the tripod commit.

ADR-003 hardware (Tracker V2 + Solar Kit + adhesive PCB standoffs + 3M VHB) is unchanged. ADR-003 §Decision body wording is retired-but-preserved per the dev-log; a separate ADR-003 amendment ticket flips the §Decision wording when this spike returns the tripod + adapter selection.

## Why this spike exists

Three printable / machinable artifacts now sit in the v1 critical path. Two have **design** spikes; one does not; and none have the **fabrication / supply** layer scoped.

1. **Relay tripod.** No design brief. Project still carries a three-way contradiction between [ADR-003](../decisions/ADR-003-relay-hardware.md) (hose clamps + wooden pole), `bom.md` (three-legged base + ground-stake), and `TODO.md` (wooden pole + hose clamps). Logged as audit row A3 in [`handheld-pivot-doc-audit-spike.md`](handheld-pivot-doc-audit-spike.md).
2. **Tag enclosure.** Design brief at [`tag-handheld-enclosure-spike.md`](tag-handheld-enclosure-spike.md), open. Covers IP target, sealing surfaces, internal layout. Does NOT cover production-tier material grade, vendor, tolerances, CAD deliverable contract, or BOM for non-printed components.
3. **Gateway enclosure.** Design brief at [`gateway-handheld-enclosure-spike.md`](gateway-handheld-enclosure-spike.md), open. Same gap.
4. **Transport / storage cases.** Not yet scoped anywhere — needed to move tripods + Solar Kits + tag fleet + tools between sites.

This spike scopes the **fabrication and supply layer**: which questions need answering before a Fusion 360 modelling ticket can be filed and a vendor PO placed.

## Hypothesis / research question

**Tripod.** Hybrid (off-the-shelf tripod + custom adapter) vs fully-custom design vs the existing wooden-pole-and-clamps fallback. Pick one during the spike.

**Tag + gateway production tier.** At what measurable point do prototypes stop being FDM-iterated and become production-grade parts (which production process is open)? Identify the transition criterion, then name the process.

**Vendor selection.** Not a name to pick from memory — a *method* for picking. Working hypothesis: a test-quote round on the smallest non-sealing part across 2–3 candidates before committing the production-tier vendor relationship.

**Transport cases.** Rugged-case-class vs cheap-case-class vs no-case. Pick during the spike.

## Scope fence

- **No CAD geometry.** Modelling lives in follow-up tickets when the design spikes close.
- **No closing the design spikes.** Tag and gateway design briefs own IP target / sealing strategy / internal layout. This spike consumes them on their close, doesn't pre-empt them.
- **No vendor selection in the body of this spike.** Candidate list and selection happen during spike work, recorded in the decision note. No rankings or "default candidates" in the body.
- **No material recommendations in the body.** Material tier and process get chosen during spike work; the body lists the *question*, not the *answer*.
- **No cost figures in the body.** Real numbers come from real quotes. Order-of-magnitude expectations are recorded in the decision note, not pre-stated here.
- **No orders placed.** Off-the-shelf tripods, cases, vendor parts — purchase decisions are downstream of this spike and the design spikes.
- **No ADR amendments written.** If the tripod hypothesis displaces ADR-003's hose-clamps + pole, a small amendment is a follow-up ticket.
- **No firmware / electrical / RF.**
- **No assembly procedure document.**

## What to verify

### Tripod (Artifact 1)

- Which mounting hypothesis wins (hybrid / fully-custom / fallback) and on what measurable reason?
- Does the Heltec Solar Kit have a usable mating feature for whichever adapter strategy wins? Verify against the OEM kit when it arrives.
- What load rating, folded length, extended height, and operating-temp envelope are required? Numbers fall out of v1 deployment reality, not preference.
- Anti-rotation: how does the Solar Kit not spin around the mount axis once tightened?
- Soft-ground / spike adapter — in v1 or deferred?
- Does the tripod choice supersede ADR-003's hose-clamps + pole? If yes, ADR-003 amendment ticket gets filed (out of scope for *this* spike).

### Tag + gateway production tier (Artifacts 2 + 3)

- What is the measurable criterion for switching each artifact from FDM filament prototyping to a production-grade process? (Examples of *kinds* of criteria: bench-seal pass rate ≥ N across M prints; sealing-flange flatness measurement; mechanical drop test; cosmetic finish judgment; fleet quantity threshold. Spike picks which kind, then which value.)
- Which production process satisfies the design spikes' IP target + tolerance numbers once those land? (Process options to evaluate during spike work, not pre-listed here.)
- Are prototype-tier and production-tier the **same vendor** or **separate vendors**? Working hypothesis: separate; spike confirms.

### Tolerances and CAD deliverable contract

- What tolerance class for non-callout dimensions does each artifact need? (ISO 2768 grade or equivalent.)
- What per-surface flatness numbers do the sealing flanges need? Numbers fall out of the design spikes' close, not this spike — but this spike **enumerates the surfaces requiring callouts**, so the modelling tickets know what to dimension.
- What file deliverables does any candidate vendor need from us? Working list (confirm and possibly add to during spike work):
  - STEP (production primary)
  - STL / 3MF (visualization, hobby printing)
  - 2D drawing PDF (critical-tolerance callouts)
  - Material specification string (verbatim, vendor-recognized)
  - Surface finish requirement
  - Quantity per part
  - Tolerance class for non-callout dims
  - Per-artifact BOM for non-printed components
- Confirm this list is the **deliverable contract** for every modelling ticket downstream.

### Vendor selection method

- Pick a single test artifact (likely the smallest non-sealing part) and run a **quote round across 2–3 candidates**. Compare on cost, lead time, surface finish on the returned part, and tolerance hit rate against the 2D drawing. Record the comparison in the decision note.
- Do prototype-tier (cheap iteration), production-tier AM (sealed shells), and sheet-metal CNC (if a heat-spreader is needed — open question, see below) need separate vendor relationships? Working hypothesis: yes, three tiers, three vendors. Spike confirms.
- Do not commit a production-tier vendor before the test-quote round.

### Heat-spreader for the gateway (cross-spike open question)

- The gateway design spike lists active cooling and a passive aluminum back-plate as candidates. **Whichever wins drives whether this fabrication brief needs a sheet-metal vendor at all** (Artifact 3b, anodized aluminum back-plate). Spike does not pick the heat path — it records the conditional vendor relationship.

### Transport cases (Artifact 4)

- Two cases sized to: (A) folded tripod + Solar Kit + assembled relay; (B) tag fleet + spare 18650s in a battery box + screwdrivers + cables + SD card box.
- Which case class wins (rugged / cheap / none) on what measurable criterion?
- Foam approach: pick-and-pluck vs laser-cut. Defer the foam-supplier choice until case dimensions land.
- Do internal dimensions need to constrain the tripod folded-length spec, or vice versa? Resolve circularity.

### Cost envelope

- Order-of-magnitude expectations are recorded in the decision note **after** quotes return, not before. Spike body does not commit numbers.

## Pass criteria

- Tripod hypothesis chosen with reason recorded.
- Solar Kit mate question recorded as resolved or pending (acceptable to close the spike with this pending if a follow-up check ticket is filed).
- Material tier transition rule named per artifact ("switch from FDM to [process] when [measurable criterion]").
- Per-artifact list of sealing surfaces requiring tolerance callouts enumerated (numbers themselves come from the design spikes' close).
- CAD deliverable contract confirmed.
- Vendor test-quote method committed; smallest test artifact named.
- Heat-spreader conditional vendor relationship recorded (sheet-metal vendor needed only if the design spike picks the passive-back-plate heat path).
- Transport-case hypothesis chosen with reason.
- Audit row A3 status recorded; ADR-003 amendment routed as a follow-up ticket (not done here).
- Cross-spike implications recorded.

## Fail criteria

- Heltec Solar Kit has no usable mating feature for any adapter strategy — escalate to a fully-custom mount and accept the modelling cost; record the escalation.
- FDM filament fails design-spike sealing-flatness target consistently — production tier becomes the only path; record the cost rise and the loss of free prototype iteration.
- Vendor test-quote round shows no candidate meets sealing-flatness spec at any acceptable price — pivot to a hybrid printed-shell + machined-flange artifact, or drop IP target in the design spike (decision lands in the design spike's close, not here).
- Rugged-case stock unavailable at v1 timing — accept cheap-case substitute and record the IP rating drop.

## Decision note template

```
Date:

Tripod (Artifact 1):
  hypothesis chosen:                        hybrid / fully-custom / fallback:
  reason:
  Solar Kit mate verified?                  yes (date) / pending physical check:
  load rating target:                       __ kg
  folded-length target:                     __ mm
  extended-height target:                   __ mm
  operating-temp envelope:                  __ to __ °C
  off-the-shelf tripod evaluated (if hybrid):
  printed adapter material (prototype):
  printed adapter material (production):
  anti-rotation feature:
  ground-spike adapter:                     in v1 / v2:
  ADR-003 amendment ticket filed?           ticket id:

Tag enclosure fabrication tier (Artifact 2):
  prototype process:
  production process:
  transition rule:                          switch when [criterion]
  sealing surfaces requiring callouts:      [list]
  flatness number per surface:              ±__ mm — source: design spike close on (date)
  CAD deliverable contract confirmed:       STEP / STL / 3MF / PDF / material spec / finish / tolerance class / BOM
  off-the-shelf component list confirmed?   yes / open items:

Gateway enclosure fabrication tier (Artifact 3):
  prototype process:
  production process:
  transition rule:                          switch when [criterion]
  sealing surfaces requiring callouts:      [list]
  flatness number per surface:              ±__ mm — source: design spike close on (date)
  heat path (from design spike):            active / passive back-plate / other:
  Artifact 3b (sheet-metal back-plate) needed? yes / no:
  CAD deliverable contract confirmed:       STEP / STL / 3MF / PDF / material spec / finish / tolerance class / BOM
  off-the-shelf component list confirmed?   yes / open items:

Vendor (production tier, AM):
  test-quote round done?                    yes (date) / pending:
  test artifact used:
  candidates quoted:
  selected primary:
  reasons:
  fallback identified:

Vendor (prototype tier):

Vendor (sheet-metal, conditional):

Transport cases (Artifact 4):
  hypothesis chosen:                        rugged-class / cheap-class / no-case:
  reason:
  case A model + price:
  case B model + price:
  foam: pick-and-pluck / laser-cut — vendor:
  internal-dimension circularity resolved?  yes:

Cost envelope (recorded after quotes return):
  tripods (incl adapters):                  €
  tag enclosures × N:                       €
  gateway enclosures × N:                   €
  sheet-metal back-plates × N (if any):     €
  cases:                                    €
  foam:                                     €
  total v1 fabrication:                     €

Operating-procedure caveats accepted:
  prototype iteration count before production order:  __
  bench seal-test required before deployment:         yes
  service intervals:                                  ___

Cross-spike implications recorded:
  tag-handheld-enclosure (production-tier material):                 ___
  gateway-handheld-enclosure (production-tier + heat-spreader):      ___
  substrate (envelope):                                              ___
  power architecture (service door):                                 ___
  production-concerns §3 (strain relief):                            ___
  ADR-003 amendment ticket:                                          ___
  handheld-pivot-doc-audit row A3 closed:                            ___

Not implemented in this spike: CAD geometry, vendor purchase orders, prototype fabrication, sealing bench tests, ADR-003 amendment.

Next action:
```

## Cross-references

- [`spikes/tag-handheld-enclosure-spike.md`](tag-handheld-enclosure-spike.md) — design brief; consumed on close.
- [`spikes/gateway-handheld-enclosure-spike.md`](gateway-handheld-enclosure-spike.md) — design brief; consumed on close.
- [`spikes/gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md) — internal envelope source.
- [`spikes/gateway-handheld-power-architecture-spike.md`](gateway-handheld-power-architecture-spike.md) — battery service door / latch detail.
- [`spikes/handheld-pivot-doc-audit-spike.md`](handheld-pivot-doc-audit-spike.md) — registrar; tripod decision (when made) closes audit row A3.
- [`production-concerns.md`](../production-concerns.md) §3 (IPEX strain relief).
- [ADR-003](../decisions/ADR-003-relay-hardware.md) — relay mounting; small amendment follow-up after this spike + design spikes close.
- [ADR-007](../decisions/ADR-007-touchscreen-primary-ui.md) — preserved by gateway enclosure design.
