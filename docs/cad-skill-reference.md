---
title: "CAD skill reference — SARCom gateway enclosure design (precursor to a future Claude skill)"
status: living
type: meta
scope: cad-work-on-sarcom-mechanical-enclosures
source-sessions:
  - dev-log/2026-05-13-gateway-v1-cad-session-risks.md
  - dev-log/2026-05-14-c1-depth-stackup-arithmetic.md
  - dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md
  - dev-log/2026-05-14-anker-dims-and-gate-propagation.md
  - dev-log/2026-05-14-cad-day-retrospective.md
  - retrospectives/2026-05-14-design-decisions.md
---

# CAD skill reference — SARCom gateway enclosure design

## What this document is

Reference compilation for **any future CAD work on SARCom mechanical enclosures** (gateway primarily; tag + relay enclosures by analogy when spike-closes exist for them). Precursor to a future Claude skill — not the skill itself. When Pieter authors the skill, this is the corpus.

The document does three things:

1. **Per-document map** of every artefact that shapes SARCom enclosure CAD work, with each document's role + what was learned about its strengths and gaps during the 2026-05-13/14 sessions.
2. **Cross-cutting lessons** synthesised from the retrospective — the patterns that explain why the day produced four dev-logs instead of one clean session.
3. **Workflow recipes** — concrete, actionable patterns for the most common CAD operations, derived from the gotchas encountered.

Scope is **mechanical enclosure design + Fusion 360 drawing process**. Firmware, protocol, tag-side, relay-side, and UI-software documents are explicitly excluded (mirror of the glossary scope fence Pieter set on 2026-05-14).

## When future Claude should consult this

Trigger this skill (or read this document) when:

- A CAD session starts on the SARCom gateway enclosure (Fusion 360 file: `gateway-v1`).
- A spike-close in `spikes/gateway-handheld-*` is being amended or read.
- A geometric question touches: depth, bulkhead inventory, gasket sealing, internal layout, mounting, heat path.
- A vendor dimension needs to be cited or verified.
- An audit-bot report (Autodesk Assistant or similar) is being filtered for stale-vs-real findings.
- Pieter mentions: enclosure, doosje, sealing, IP65, gasket, bulkhead, depth, footprint, battery door, mounting boss, heat-spreader, pocket, parting plane.

Do NOT trigger for: tag enclosure (separate spike), relay enclosure (Solar Kit OEM), firmware, protocol, UI software.

---

## Part 1 — Per-document map

Per-document analysis lives in [`docs/sarcom-cad-doc-map.md`](sarcom-cad-doc-map.md). This skill reference is **prescriptive** (what to DO); the doc map is **descriptive** (what exists). When working in the CAD domain, consult the doc map for per-source role + gaps + how-to-use entries; this file for activation rules, workflow recipes, and Fusion API survival patterns.

---

## Part 2 — Cross-cutting lessons

Synthesised from the retrospective (`dev-log/2026-05-14-cad-day-retrospective.md`) and the design-decisions doc (`retrospectives/2026-05-14-design-decisions.md`). Three meta-patterns and one structural observation.

### P1 — 2026-05-08 spike-closes are shallow

The three spike-closes from that day (substrate, power-architecture, enclosure) were closed in a "short chat-Q&A round" — not via the canonical research process documented in `docs/spike-rules.md`. Today's findings traced multiple cascading errors back to those three closes:

- Anker A1689 wrong dimensions (G1)
- Depth target hand-wave (G2)
- Heat path topology over the divider unaddressed (G3)
- Battery door no location/orientation (G5)
- Magnetic-pogo no vendor-class with current rating (O3)
- Bank orientation implicit (G7)

**Skill implication:** When reading a spike-close that was closed quickly (look for the "closed on the same day it was opened" pattern, or for terse §Decision blocks with placeholder vendor SKUs), treat its concrete claims as **candidate-pending-verification**, not authoritative. Verify dimensions, citations, and cross-references before consuming.

### P2 — Hand-wave specs propagate exponentially downstream

One hand-wave dimension in a spike-close (e.g., Anker `154 × 62 × 30`) gets consumed by **all** downstream artefacts without verification. When the value is later corrected, all consumers need to be updated. Today's Anker propagation pass required 8 inline edits across 2 spike files plus separate edits in dev-logs, TODO.md, and CAD parameter comments.

**Skill implication:** Spike-closes MUST cite each concrete claim with a source URL OR label it as "candidate-pending-comparison". Non-cited values must NOT be used as input for other spikes' decisions. The first-pass cost of citing sources is far smaller than the downstream-correction cost.

### P3 — Sketch-time intuition fails on offset planes + sign disambiguation

A class of today's bugs are "I thought X, was Y" around axis-mapping and sign conventions on offset sketch planes:

- Battery-door sketch X-axis mapped to world Z (not Y as intuited) — F7
- Distance dimensions don't preserve sign; solver disambiguates with surprising choices — F8
- `setSymmetricExtent(distance, isFullLength=True)` semantics — F3
- Sketch origin convention inconsistent across files (door at -20, bezel at 0) — G8

**Skill implication:** Before placing dimensions on any sketch on an offset construction plane, do a **test extrude of an asymmetric profile** (e.g., 10×5 mm rectangle drawn near origin, extruded 1 mm) to verify the axis-mapping. 5 minutes of verification prevents 30 minutes of wrong-orientation rebuild.

### Structural observation — multi-amendment spike-closes need a top-of-file supersession layer

By end-of-day 2026-05-14, the enclosure spike-close had **two top-of-file supersession sections** (depth correction + pogo retirement), and the §Closed verdict and §Decision text had inline `[CORRECTED]` / `[SUPERSEDED]` markers. This is a working pattern — it preserves the historical trail while making the current state legible.

**Skill implication:** When amending an existing spike-close, prefer adding a dated supersession section at the top + inline markers in the §Closed/§Decision text. Don't rewrite the original verdict in place. The history trail is part of the value.

---

## Part 3 — Workflow recipes

Concrete actionable patterns derived from the day's gotchas. Each recipe: when to use, the steps, the rationale.

### Recipe A — Verify a vendor dimension before using it in CAD

**Trigger:** A spike-close, dev-log, or CAD parameter cites a vendor part's dimensions.

**Steps:**

1. Identify the vendor + part number.
2. Fetch the official vendor product page or datasheet PDF directly (not WebSearch snippets — see Pi Touch Display 2 thickness incident).
3. Cite the URL in the param comment or doc text.
4. If the verified value differs from the existing claim, run a propagation pass (see Recipe B).

**Rationale:** WebSearch and AI summaries are unreliable for precise mechanical dimensions. Pi Touch Display 2 returned 8.55 mm via search; direct PDF fetch gave 15 mm.

### Recipe B — Propagate a corrected value across the doc set

**Trigger:** A previously-cited value (dimension, term, signal name) has been corrected and needs to land everywhere it's consumed.

**Steps:**

1. `Grep` for the old value across the repo (cast wide — different formatting variations).
2. Categorise hits: spike-closes (need supersession headers + inline markers), dev-logs (historical, preserve as-is unless contradicting current state), canonical project docs (CLAUDE.md, ARCHITECTURE.md, README.md, TODO.md, bom.md — update inline with footnote refs).
3. For each spike-close hit, add a dated supersession section at top + inline `[CORRECTED yyyy-mm-dd — source URL]` markers in the original text.
4. For each canonical project doc hit, update inline; add a single footnote `[^correction-yyyy-mm-dd]` at the first occurrence with full explanation + cross-refs.
5. Verify via a second grep that only intentional historical/amendment references to the old value remain.
6. Update CAD user-param comments to cite the new source URL.

**Rationale:** A correction is only complete when the value is consistent everywhere. Forgetting one location creates drift that surfaces weeks later in audits.

### Recipe C — Stack-up arithmetic for a depth/footprint/volume spec

**Trigger:** A spike-close's depth, footprint, or volume claim is being established OR a CAD session is producing geometry that disagrees with the spec.

**Steps:**

1. Create a per-row table: `Layer | mm | Source`.
2. List every mechanical contributor in stack order (outer face → inner reference).
3. For each row, fill `Source` with one of: spike-close-text-path:line / datasheet-URL / **HAND-WAVE** (explicitly flagged).
4. Sum the rows. Compare to the spec.
5. If sum > spec by a meaningful margin: the spec is wrong, not the geometry. Correct the spec via Recipe B.
6. If a row is HAND-WAVE: surface it as an "open verification" item in a top-of-file "To verify before close" block, low-confidence flag.

**Rationale:** Per-row arithmetic forces every contribution to be either cited or admitted as hand-wave. The act of writing the table is the verification.

### Recipe D — Filter an audit-bot report

**Trigger:** An Autodesk Assistant audit or similar bot-generated review presents N "conflicts" or "findings."

**Steps:**

1. For each finding, query the live Fusion state (don't trust the bot's reading).
2. Apply one of four verdicts:
   - **REAL** — finding is correct and actionable.
   - **STALE** — finding correctly identifies an issue but reads pre-correction spec or out-of-date state.
   - **HALLUCINATION** — finding misreads geometry or specs.
   - **VERIFIED-OK** — finding asked a question that resolves to "no issue."
3. Document the verdict + reasoning in a table.
4. Act only on REAL findings.

**Rationale:** Audit bots have no awareness of recent amendments, dev-log context, or design-intent nuance. Filtering protects against acting on stale or hallucinated findings that would create unnecessary work or regressions.

### Recipe E — Sketch on offset construction plane

**Trigger:** A new sketch is being created on a construction plane that isn't the XY, XZ, or YZ origin plane.

**Steps:**

1. Create the construction plane parametrically (e.g., offset from a base plane by a user-param expression like `outer_w / 2 - 1.5 mm`).
2. Add the sketch on the construction plane.
3. **Before placing any dimensions:** sketch a small asymmetric test profile (e.g., 10×5 mm rectangle drawn near origin), extrude it 1 mm, and inspect the body's world-coordinate bounding box. Note the sketch-X / sketch-Y → world-axis mapping.
4. Delete the test extrude + profile.
5. Proceed with the real sketch, using the verified axis-mapping for all dimension orientations.

**Rationale:** Sketch-local X/Y axis mapping on offset planes is plane-orientation-dependent and not always intuitive. Battery door rebuild today required a dimension-swap because sketch-X mapped to world Z (not Y as assumed). 5-minute test prevents 30-minute rebuild.

### Recipe F — Move feature, parametric translation

**Trigger:** A body needs to be translated by an amount tied to a user parameter.

**Steps:**

1. Use `moveFeatures.createInput2(bodies_collection)`.
2. Use `defineAsTranslateXYZ(x_value, y_value, z_value, isLocal=False)` where each `_value` is a `ValueInput.createByString("-rear_depth")` (or any parametric expression).
3. **Do not use** `defineAsFreeMove(transform)` with a hard-coded Matrix3D — that's a rigid offset that breaks parametric coupling.

**Rationale:** Move features support parametric expressions natively via `defineAsTranslateXYZ`. Discovered late on 2026-05-14 — earlier Move was rigid -40 mm before refactor. Use parametric from the start.

### Recipe G — Iterate-and-delete on Fusion features

**Trigger:** Looping over a component's features to delete ones matching a condition.

**Steps:**

1. **First pass: collect.** `to_delete = [f for f in comp.features if matches(f)]` — materialise the list.
2. **Second pass: delete.** `for f in to_delete: f.deleteMe()`.

**Do NOT** combine the iteration and deletion in one loop. The cascade-delete behaviour of Fusion features mutates the iterator's underlying collection, triggering `InternalValidationError: dmFeature || pmFeature` on the next access.

**Rationale:** Pattern observed during front-shell-solid feature relocation cleanup on 2026-05-14. End-state was correct but the loop threw a transient exception.

### Recipe H — Extrude profile from cross-component sketch

**Trigger:** A sketch in component A is needed as the profile for an extrude that should produce a body in component B.

**Steps:**

Option 1 (preferred): Recreate the sketch in component B with the same parametric geometry. This is what the front-shell-outline rebuild did today (rounded rectangle via outer_w/outer_h/corner_r in front-shell, replacing the cross-component projection attempt).

Option 2 (acceptable): Do the extrude in component A, then `body.moveToComponent(target_occurrence)` to relocate. Side effect: the extrude feature stays in component A while the body lives in component B (timeline asymmetry, cosmetic).

**Do NOT** call `extrudeFeatures.createInput()` on component B's features with a profile from component A's sketch — Fusion throws `InternalValidationError: bSet`.

**Rationale:** Cross-component sketch consumption is restricted. Discovered on 2026-05-14 with the first shell-extrude attempt.

### Recipe I — Forensic on a volume-delta anomaly

**Trigger:** A feature's reported volume change disagrees with the geometric ideal.

**Steps:**

1. Identify the timeline markers around the feature: position N-1 (before), position N (after).
2. Save `original_marker = design.timeline.markerPosition`.
3. Walk the timeline: `timeline.markerPosition = N-1`, record body volume + face count + bbox.
4. Step forward to `timeline.markerPosition = N`, record same.
5. Compute delta.
6. If discrepant, query face-level diff: collect face signatures (Z, area, centroid) pre and post; identify disappeared + appeared faces.
7. Hypothesise based on the diff (e.g., BREP solver sliver-face cleanup, hidden interior topology).
8. **Restore the marker:** `timeline.markerPosition = original_marker`.

**Rationale:** Volume math can be misleading on complex feature interactions (shell + cut interaction produced ~4,700 mm³ excess removal today). Face-level diff surfaces what's actually changing.

### Recipe J — Sketch convention: origin at body geometric center

**Trigger:** Creating a new sketch that will define a body.

**Steps:**

1. Place the sketch's origin at the **geometric center** of the body the sketch defines (not at an arbitrary corner or offset).
2. Use coincident or symmetry constraints to enforce centring.
3. Document the convention in the sketch's name or in a comment.

**Rationale:** Inconsistent sketch-origin conventions (door-profile centred at sketch-X=-20, bezel-outline centred at origin) led to audit-bot misreads ("bosses asymmetric") and human confusion. Standardising prevents both.

---

## Part 4 — Fusion API gotchas catalogue

Quick-reference for Fusion API behaviours that bit us on 2026-05-14. Cross-ref the cad-day-retrospective F1-F13 entries for full context. This is the section most likely to be useful as raw skill content.

| ID | Gotcha | Workaround |
|---|---|---|
| F1 | `mcp__fusion360__fusion_screenshot` broken on Fusion 2702.x ("takes 2 positional arguments but 5 were given") | Use `app.activeViewport.saveAsImageFile(path, w, h)` directly via `fusion_execute` |
| F2 | `extrudeFeatures.createInput(profile, op)` throws `InternalValidationError: bSet` when called on a component whose sketches don't include the profile's parent | Do the extrude in the source-sketch's component, OR recreate the sketch in the target component |
| F3 | `setSymmetricExtent(distance, isFullLength=True)` means `distance` IS the total extent (not half) | If you want N mm total, pass `distance="N mm"` with `isFullLength=True`; if you want 2*N total, pass `distance="N mm"` with `isFullLength=False` |
| F4 | `adsk.fusion.FeatureOperations` enum: Join=0, Cut=1, Intersect=2, NewBody=3, NewComponent=4 | Query via `dir(adsk.fusion.FeatureOperations)` and direct enum access, don't guess |
| F5 | `sketchLines.addByTwoPoints` with four separate Point3D arguments does NOT auto-coincide corners — profile detection fails | Use `sketchLines.addTwoPointRectangle(p1, p2)` for closed rectangles |
| F6 | `sketch.project()` cross-component fails silently (returns 0 profiles) | Recreate the geometry parametrically in the target sketch instead |
| F7 | Sketch local X/Y axis mapping on offset construction planes is plane-orientation-dependent (offset YZ plane → local X often maps to world Z, not Y) | Test-extrude an asymmetric profile before placing dimensions (Recipe E) |
| F8 | `addDistanceDimension` returns absolute distance; solver may disambiguate sign in unexpected ways | Anchor via coincident-to-construction-point or midpoint constraint when sign matters |
| F9 | `MoveFeature` with `defineAsFreeMove(transform)` is rigid (Matrix3D); `defineAsTranslateXYZ(x, y, z, isLocal)` accepts parametric ValueInputs | Use `defineAsTranslateXYZ` from the start for parametric coupling |
| F10 | Iterate-and-delete-inline on `comp.features` triggers `InternalValidationError: dmFeature || pmFeature` after a cascade-delete | Collect first into a list, then delete (Recipe G) |
| F11 | Shell feature + cut feature interaction can remove more material than the ideal cut geometry due to BREP solver cavity-interior sliver cleanup | Use timeline-rollback + face-level diff to forensically verify (Recipe I) |
| F12 | `sketch.referencePlane` shows the construction plane but doesn't expose a clean "this maps to world axes X→? Y→? Z→?" lookup | Inspect via test-extrude or by querying the construction plane's geometry origin + normal |
| F13 | `extrudeFeatures.add(ext_input).bodies` exists but the feature might also report participantBodies access errors mid-edit | Save the body reference immediately after `add()`; avoid accessing properties on features whose timeline editing isn't complete |

---

## Part 5 — Skill activation triggers (for the future skill author)

When Pieter authors the actual Claude skill from this reference, suggested activation triggers (matched against user input or context):

**High-confidence triggers** (almost always relevant):
- "fusion 360" / "fusion" + "sarcom" / "gateway" / "enclosure"
- Editing any file under `spikes/gateway-handheld-*`
- Editing the `gateway-v1` Fusion file (via fusion_execute or fusion_screenshot)
- Words: doosje, enclosure, IP65, gasket, bulkhead, battery door, heat-spreader, parting plane, shell, bezel

**Medium-confidence triggers** (relevant in context):
- Vendor dimension verification work involving Anker, Raspberry Pi, Dragino
- CAD parameter changes in user-params
- Cross-doc propagation of a corrected term/value

**Out-of-scope triggers** (skill should NOT activate):
- Tag enclosure (different spike)
- Relay enclosure (Solar Kit OEM)
- Firmware, protocol, UI software
- Doc-process / spike-rules / templates

---

## Part 6 — When this document is wrong

This is a living reference. Sources of staleness to watch for:

- **Spike-close amendments** — when `spikes/gateway-handheld-*` get new supersession sections, the relevant Part 1 entry needs updating.
- **New dev-logs** — when a CAD session produces a new dev-log, the Part 1 dev-log section needs the new entry.
- **Fusion API changes** — F1 (screenshot bug) is tied to Fusion 2702.x; future Fusion versions may fix it or change other behaviours.
- **New gotchas** — Part 4 catalogue grows monotonically; add new entries with F-IDs.
- **Skill triggers** — if Pieter ever extends scope to tag/relay enclosures, Part 5 needs updating.

The structure (Parts 1-6) should remain stable; the content of each Part will accumulate.

## Cross-refs

- `dev-log/2026-05-14-cad-day-retrospective.md` — fuller F1-F13 explanations and meta-pattern reasoning
- `retrospectives/2026-05-14-design-decisions.md` — 8 design decisions of the day with trade-off framing
- All four 2026-05-14 dev-logs — chronological context for the day's work
- `docs/spike-rules.md` + `docs/spike-template.md` + `docs/spike-prelude.md` — process docs for spike writing (the discipline these spike-closes failed to follow consistently)
- `CLAUDE.md` — project values and the "Do NOT re-open" list, applied as a design filter
