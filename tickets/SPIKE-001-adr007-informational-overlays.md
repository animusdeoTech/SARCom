---
title: "Spike — Does ADR-007 permit informational overlays for read-only operator scenarios in v1a?"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-18
---

# Spike: ADR-007 informational overlays for v1a

> **Executor test:** another engineer who does not know the author's preferred answer should be able to run this spike and produce the same shape of evidence: ADR-007 text, ARCHITECTURE.md §11 text, the BLE-commissioning precedent, and the v1a operator-need cases — and recommend one of Variants A / B / C with the reasoning visible.

## To verify before close

- The footnote at [`README.md:36`](../README.md) cross-references ARCHITECTURE.md and the BLE-commissioning spike; verify those cross-references are still authoritative (no superseding doc has landed mid-review).
- Verify no draft of pending ADR-015 has already pre-decided the overlay question.

## Why this spike exists

[`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../decisions/ADR-007-touchscreen-primary-ui.md) lists *"No modals, no dialogs, no 'are you sure?' prompts. No settings screen, no CRUD on tags or relays. No remote dashboard, no web interface ... No data entry of any kind. No alert acknowledgement flow."*

The same invariant is preserved with a refinement at [`ARCHITECTURE.md:464`](../ARCHITECTURE.md): *"The read-only-map invariant from ADR-007 is preserved by `spikes/ble-gateway-ui-flow-spike.md`: commissioning interaction lives inside an explicit modal opened from a marker by deliberate gesture, not on the map itself."*

The BLE-commissioning spike at [`spikes/ble-gateway-ui-flow-spike.md:17`](../spikes/ble-gateway-ui-flow-spike.md) goes further: *"Surface form. Full-page screen replace ... not an overlay modal over the map ... Page is named 'Commissioning' in code and copy — not 'modal' — to avoid the read-only-map-invariant tension."* But the same spike at line 169 says *"All write actions live inside the modal. The modal is not the map — it is a maintenance overlay opened by deliberate gesture and closed when the operator is done."* The terminology is contested even within the BLE spike.

v1a operator needs surfaced in this conversation that touch this question:

- Tap-tag → see full per-node detail (currently impossible — [`tools/sarcom-kiosk-lab/src/ui/sidebar.rs:31-34`](../tools/sarcom-kiosk-lab/src/ui/sidebar.rs) only changes background tint)
- SOS alert that survives a glance-away (currently a non-interactive red bottom strip at [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../tools/sarcom-kiosk-lab/src/app.rs))
- Cursor-coordinate readout for voice-relayed coordinates (currently no path)

These needs can be satisfied via overlays (Variant A), via inline sidebar/bottom-strip extensions (Variant B), or deferred (Variant C). The choice has cascading effects on KIOSK-002, KIOSK-004, KIOSK-006, and KIOSK-007.

## Hypothesis / research question

**Q.** For each operator-facing UI surface needed in v1a (detail view, SOS alerting, coordinate readout), is a strict-ADR-007 (Variant B) design operationally adequate for the core SAR task it serves — or is a Variant A overlay-shaped surface necessary?

**Posture:** strict ADR-007 (Variant B) is the default. Variant A is the burden-of-proof position. A Variant A surface is adopted on a per-surface basis ONLY if the spike documents a specific SAR-operator task where the Variant B design fails operationally — *not* "feels nicer," *not* "more familiar," *not* "matches what other tactical UIs do."

**H1 (default — Variant B per-surface across the board).** The strict reading of [`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../decisions/ADR-007-touchscreen-primary-ui.md) holds for every v1a surface. Detail view is sidebar replacement, SOS alerting is persistent bottom strip with stronger visual hierarchy, coordinate readout is bottom-strip line. No overlays, no popovers, no banners, no acknowledgement flow.

**H0 (mixed — Variant B baseline with named exceptions).** Variant B holds for most surfaces, but one or more specific surfaces fail a named SAR operator task and earn a Variant A exception. The spike enumerates *which* surface(s), *which* task(s), and *why* the inline alternative fails. The exception list is small and each item is justified in writing.

**H2 (Variant A across the board) — rejected up front unless H1 and H0 both fail.** This is the position the spike must defeat the default with, not the default. It exists in the hypothesis space to ensure the spike actually compares alternatives, not as a credible v1a outcome.

**H3 (Variant C — defer all overlay-like surfaces) — fallback only.** If neither H1 nor H0 produces a defensible v1a design, the spike commits to deferring all overlay-like surfaces past v1a and shipping the current behaviour unchanged. Operator workflow stays at current fidelity; re-opened post-v1a operator feedback.

## Scope fence

- No code.
- No ADR edits in the spike itself (the spike's output may *commit* to an ADR-007 amendment as the follow-up).
- No new ticket writing (tickets exist; this spike updates their variant dependency).
- No UI implementation.
- No re-litigation of ADR-007 wholesale — only its applicability to read-only informational overlays in v1a.
- No re-litigation of ADR-008 (no downlink). Any acknowledgement flow considered must be UI-local only.

## What to verify

The spike applies a per-surface burden-of-proof: Variant B is the default for each surface; Variant A is adopted only where the inline design fails a named SAR operator task.

1. **Per-surface operator-task test (the main test).** For each of the three v1a surfaces below, name the SAR operator task served and judge whether the Variant B design (sidebar replacement / bottom strip / stronger visual hierarchy) is operationally adequate. Evidence: the primary artifact is [`UX/mockups/v1a-operator-map-mockup.md`](../UX/mockups/v1a-operator-map-mockup.md) (and its sibling SVG); [`CLAUDE-DESIGN-PROMPT-v1a-operator-map.md`](CLAUDE-DESIGN-PROMPT-v1a-operator-map.md) is the **task input** to the mockup work, not the evidence output of this spike. Operator-workflow walkthroughs and (where possible) operator-interview notes supplement the mockup.
   - **Detail view** — task: operator selects a node to confirm its full state (coords, age breakdown, flags). Variant B = sidebar replacement. Variant A = slide-in panel.
   - **SOS alerting** — task: operator glances at the screen mid-radio-call and needs to know an SOS is active. Variant B = persistent red bottom strip with stronger visual hierarchy (larger text, optional blink/pulse). Variant A = top-anchored red banner with acknowledge button.
   - **Coordinate readout** — task: operator dictates lat/lon of an arbitrary map point to a ground team over voice. Variant B = persistent bottom-strip line while touch is held. Variant A = popover.
   - **SOS × coordinate-readout interaction (cross-surface)** — task: while an SOS is active (persistent red bottom strip), the operator touch-and-holds the map to dictate lat/lon of an arbitrary point to a ground team. Both surfaces compete for the same bottom-strip zone. **Variant B remains default.** The coordinate popover is **not** promoted by this interaction. The preferred strict-ADR resolution: during touch-and-hold while SOS is active, the bottom strip may temporarily split or expand to carry both — zone 1 keeps the persistent SOS state, zone 2 carries the coordinate readout for the duration of the touch. On release, the strip returns to normal SOS-only state. No popover. No overlay. No acknowledgement flow. This is the **current recommended resolution, to be validated when KIOSK-002 is implemented** — not a closed ADR decision in this spike.
2. **Strict-reading test.** Read [`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../decisions/ADR-007-touchscreen-primary-ui.md) cold without surrounding context. For each surface where the spike is tempted to promote Variant A: does the natural-language reading of ADR-007 prohibit that specific surface shape? If yes, the prohibition is the priority; Variant B is preserved unless the operator-task test in §1 shows operational failure.
3. **Acknowledgement-flow test.** [`decisions/ADR-007-touchscreen-primary-ui.md:46`](../decisions/ADR-007-touchscreen-primary-ui.md) specifically prohibits an "alert acknowledgement flow." A Variant A SOS banner with an acknowledge button is the surface most directly testing this prohibition. **The default position is that this prohibition holds.** Promoting Variant A on the SOS surface requires showing both (a) the persistent bottom-strip approach is operationally inadequate for the SAR task, and (b) the UI-local-only acknowledgement (no protocol, no DB write beyond a dismissal flag) is meaningfully different from the kind of "acknowledgement flow" the ADR prohibits.
4. **Precedent test (background, not decisive).** The BLE-commissioning spike at [`spikes/ble-gateway-ui-flow-spike.md:17, 65, 169`](../spikes/ble-gateway-ui-flow-spike.md) adds an overlay surface for write-action maintenance. **This precedent is for write actions on relay markers, not for informational overlays for read-only scenarios.** The spike must explicitly note whether each Variant A surface under consideration falls under the same precedent or is a category expansion. Category expansion is harder to justify.
5. **Cascading-effect test.** For each surface where the spike retains Variant B (the default): KIOSK-002 / KIOSK-004 / KIOSK-006 / KIOSK-007 update to their Variant B scope. For each surface where the spike promotes Variant A: the affected ticket inherits the named exception and the operator-task failure rationale.

## Comparable options

Listed in the order the spike applies them: Variant B is the default; each Variant A surface must be earned per the per-surface burden-of-proof test.

| Option | default-or-burden-of-proof | downstream-ticket impact | reversibility | ADR-007 risk |
|---|---|---|---|---|
| B. Strict no-overlays (per-surface) — **default** | default for every surface | KIOSK-002 / KIOSK-004 / KIOSK-006 / KIOSK-007 ship their Variant B scope | reversible (a future amendment may loosen if operator feedback warrants) | none — literal reading is preserved |
| A. Overlays allowed (per-surface exception) — **burden of proof** | adopted only on a per-surface basis where the operator-task test in §1 shows Variant B operational failure | only the affected ticket inherits the Variant A scope and the failure rationale | reversible (amendment may tighten later) | requires written interpretation artifact citing the failing SAR task and the ADR-007 lines tested |
| C. Defer all overlay-like surfaces — **fallback only** | adopted only if both Variant B and Variant A fail to produce a defensible v1a design | KIOSK-002 / KIOSK-004 / KIOSK-006 are deferred past v1a | reversible | none — v1a ships with current behaviour, no ADR pressure |

## Pass criteria

- Evidence enough to commit one explicit follow-up: either (a) an interpretation appendix on [`decisions/ADR-007-touchscreen-primary-ui.md`](../decisions/ADR-007-touchscreen-primary-ui.md) reaffirming the strict reading (H1 — pure Variant B), or (b) an amendment file naming the small, justified set of per-surface Variant A exceptions (H0), or (c) a [`TODO.md`](../TODO.md) deferral entry (H3).
- A per-surface verdict table: for each of the three v1a surfaces (detail view, SOS alerting, coordinate readout) **plus the SOS × coordinate-readout cross-surface interaction**, one row stating *Variant B retained* or *Variant A promoted, because [named SAR task] fails operationally on Variant B because [specific failure mode]*. The interaction row also records the chosen strict-ADR resolution for the bottom-strip split (or its alternative).
- A list of cascading edits to KIOSK-002 / KIOSK-004 / KIOSK-006 / KIOSK-007 induced by the choice (what changes in each ticket).

## Fail criteria

- If any Variant A surface is proposed without naming the specific SAR task that fails under Variant B, that proposal is rejected. "Feels nicer," "more familiar," "matches what tactical UIs do," and "the BLE-commissioning spike already does this" are explicitly not acceptable justifications for category expansion to informational overlays.
- If H1 (pure Variant B) and H0 (Variant B with named exceptions) both fail to produce a defensible design, the spike closes H3 (defer all overlay-like surfaces past v1a). Promoting H2 (pure Variant A) is not a pass condition — it requires every surface to fail Variant B independently and would imply a fundamentally bad fit between ADR-007 and v1a, which the spike treats as evidence ADR-007 needs revisiting at the ADR layer, not bypassing at the ticket layer.

## Fallback / next action

- If H1 (pure Variant B): spike commits to writing a short interpretation appendix on [`decisions/ADR-007-touchscreen-primary-ui.md`](../decisions/ADR-007-touchscreen-primary-ui.md) reaffirming the strict reading in v1a context. Tickets KIOSK-002 / KIOSK-004 / KIOSK-006 / KIOSK-007 all adopt their Variant B scope.
- If H0 (Variant B with named exceptions): spike commits to writing `decisions/ADR-007-amendment-v1a-overlay-interpretation.md` (or a sibling ADR) that documents *only* the specific surface exceptions, names the operator task each exception serves, and reaffirms the default-strict reading for all other surfaces. Only the affected tickets adopt Variant A scope on the named surface; all others stay Variant B.
- If H3 (defer): spike commits to a one-line entry in [`TODO.md`](../TODO.md) recording the deferral and the triggering condition for re-open. Tickets KIOSK-002 / KIOSK-004 / KIOSK-006 are marked deferred-pending-v1b.

## Decision note template

```
Date:
Headline verdict: H1 (pure Variant B) / H0 (B + named exceptions) / H3 (defer)

Per-surface verdict table:

  Detail view (KIOSK-004):
    SAR task served: ___
    Verdict: Variant B retained / Variant A promoted
    If promoted: specific operational failure of Variant B = ___

  SOS alerting (KIOSK-006):
    SAR task served: ___
    Verdict: Variant B retained / Variant A promoted
    If promoted: specific operational failure of Variant B = ___
    Acknowledgement-flow test outcome (ADR-007:46): ___

  Coordinate readout (KIOSK-002):
    SAR task served: ___
    Verdict: Variant B retained / Variant A promoted
    If promoted: specific operational failure of Variant B = ___

  SOS × coordinate-readout interaction (cross-surface, KIOSK-002 ↔ KIOSK-006):
    SAR task served: operator dictates coordinates while SOS is active
    Verdict: Variant B retained (default) / Variant A promoted
    Strict-ADR resolution chosen (see §1 for the recommended candidate to evaluate): ___
    Status: ___

Reading of ADR-007:38-46 applied:
  ___

Reading of ARCHITECTURE.md:464 + BLE-commissioning precedent applied:
  ___ (note: precedent is for write-action maintenance, not informational
  overlays; category-expansion is a higher bar)

Cascading edits to tickets:
  KIOSK-002: variant = B / A — [scope change]
  KIOSK-004: variant = B / A — [scope change]
  KIOSK-006: variant = B / A — [scope change]
  KIOSK-007: variant = B / A — [doc artifact = appendix / amendment / TODO]

Low-confidence claims resolved:
  ___

Not implemented in this spike: code, ADR edits, ticket implementation.

Follow-up filed: <appendix path (H1) / amendment file path (H0) / TODO entry (H3)>
```

## Cross-references

- [`decisions/ADR-007-touchscreen-primary-ui.md`](../decisions/ADR-007-touchscreen-primary-ui.md) — the ADR being interpreted
- [`ARCHITECTURE.md:464`](../ARCHITECTURE.md) — the invariant note
- [`spikes/ble-gateway-ui-flow-spike.md`](../spikes/ble-gateway-ui-flow-spike.md) — precedent
- [`UX/mockups/v1a-operator-map-mockup.md`](../UX/mockups/v1a-operator-map-mockup.md) — **primary evidence artifact**: the mockup that surfaces both variants per surface (sibling SVG at [`UX/mockups/v1a-operator-map-mockup.svg`](../UX/mockups/v1a-operator-map-mockup.svg))
- [`tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md`](CLAUDE-DESIGN-PROMPT-v1a-operator-map.md) — task input to the mockup work (not evidence output of this spike)
- [`tickets/KIOSK-002-cursor-coordinate-readout.md`](KIOSK-002-cursor-coordinate-readout.md), [`tickets/KIOSK-004-selection-detail-panel.md`](KIOSK-004-selection-detail-panel.md), [`tickets/KIOSK-006-sos-alerting.md`](KIOSK-006-sos-alerting.md), [`tickets/KIOSK-007-doc-cleanup.md`](KIOSK-007-doc-cleanup.md) — downstream tickets that depend on this variant choice
