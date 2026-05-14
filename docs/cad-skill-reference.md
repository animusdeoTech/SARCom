---
title: "CAD skill design — SARCom gateway enclosure (prescriptive)"
status: living
type: skill-design
scope: cad-work-on-sarcom-mechanical-enclosures
source-sessions:
  - dev-log/2026-05-13-gateway-v1-cad-session-risks.md
  - dev-log/2026-05-14-c1-depth-stackup-arithmetic.md
  - dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md
  - dev-log/2026-05-14-anker-dims-and-gate-propagation.md
  - dev-log/2026-05-14-cad-day-retrospective.md
  - retrospectives/2026-05-14-design-decisions.md
  - retrospectives/2026-05-14-meta-retro-missing-angles.md
companion: docs/sarcom-cad-doc-map.md
---

# CAD skill design — SARCom gateway enclosure

**Prescriptive companion** to `docs/sarcom-cad-doc-map.md` (descriptive per-document analysis). This file says **what to DO** during CAD work on the SARCom gateway enclosure; the doc map says **what exists** in the source corpus. When authoring the future Claude skill, this file is the behavioural spec; the doc map is the reference compilation the skill loads on demand.

## §0 — Activation rules

The skill activates when **ALL** of:

(a) The user message OR conversation context contains any of: `fusion`, `cad`, `enclosure`, `doosje`, `IP65`, `IP67`, `gasket`, `bulkhead`, `parting plane`, `battery door`, `heat-spreader`, `pocket`, `bezel`, `mounting boss`, `shell`, `front_depth`, `rear_depth`, `pi 5`, `pi touch display`, `dragino`, `anker a1689`, `gateway-v1`, `spike-close`, `spike close`. (Single flat OR list intentional; per-category splitting was considered and rejected to avoid logic-complexity that doesn't earn its keep at activation time.)

(b) The working tree shows uncommitted changes to `docs/spike-*` OR `spikes/gateway-handheld-*` OR the Fusion 360 file `gateway-v1` is open via MCP bridge (verifiable via `fusion_execute` returning a non-error response with `app.activeDocument.name == 'gateway-v1'`).

(c) No other CAD-domain skill is currently active for tag enclosure (`spikes/tag-handheld-enclosure-spike`) or relay enclosure (`spikes/physical-fabrication-brief-spike` / `ADR-003`).

The skill does **NOT** activate for:

- Tag enclosure (separate spike, different mechanical envelope)
- Relay enclosure (Solar Kit OEM, ADR-003)
- Firmware, protocol, UI software (different problem domain)
- Doc-process files (`docs/spike-rules.md`, `docs/spike-template.md`, `docs/spike-prelude.md` — those are meta, not design)

Borderline cases (trigger judgment, not auto-activate): vendor dimension verification for components used in the gateway (Anker, RPi, Dragino) — activate if the verification is in the context of a CAD parameter or spike-close edit; otherwise let it pass.

## §1 — First-action checklist

When the skill activates, the first five actions are non-negotiable:

1. **Re-read** `CLAUDE.md` §"Tone and working style (Pieter)" and the project-values block ("physical plug-and-play, quality > speed, hates fastest-time-to-market shortcuts"). Values can shift; always reload, never cache.
2. **Query the live Fusion state** via `fusion_execute`:
   ```python
   print(f"Doc: {app.activeDocument.name}")
   print(f"Params: {design.userParameters.count}")
   print(f"Bodies + components + timeline-entries inventory")
   ```
   If Fusion is not running or the wrong document is open, ask Pieter to open `gateway-v1` before proceeding.
3. **Read top-of-file dated supersession sections** of every `spikes/gateway-handheld-*` spike-close. These OVERRIDE the original §Closed and §Decision text. Source-of-truth order documented in §2 below.
4. **Read** `TODO.md` §"Carry-over voor volgende CAD sessie" for active blockers. Decisions in that list cascade — many internal features can't proceed until #1 (Orientation X vs Y) lands.
5. **Open with a state summary** to Pieter following this template:
   ```
   CAD state snapshot:
   - Live Fusion: <N components, M bodies, P params, T timeline entries>
   - Active spike amendments: <list any post-2026-05-14 supersession sections>
   - Open blockers from TODO carry-over: <prioritized list>
   - I understood your request as: <restate>
   - Proposed first action: <single concrete step>
   - Anything I'm missing before I start?
   ```

The summary protects against silent assumption drift between sessions. It also forces the skill to look at live state before proposing actions. Format is intentionally rigid — flexibility here would re-introduce the narrative drift this skill restructure eliminated.

## §2 — Source-of-truth hierarchy + anti-patterns

When sources conflict, the ordering is:

```
Live Fusion state (via fusion_execute)
  > spike-close §Decision post-amendments (dated supersession sections + inline [CORRECTED/SUPERSEDED] markers)
    > §Closed verdict (the 2026-05-08 / original close text)
      > spike-close prose (the surrounding non-§Decision text)
        > dev-logs (chronological session journals — historical)
          > audit-bot output (Autodesk Assistant, similar — NEVER authoritative)
```

Conflict-resolution rules for adjacent pairs:

| Pair | Rule |
|---|---|
| Live Fusion vs spike-close §Decision | Live Fusion describes reality; spike-close describes intent. When they disagree, correct the spike-close to match reality (don't shrink the Fusion to fit a wrong spec). Example: 75 mm in Fusion + 45-55 mm in spike → corrected spike to 85-100 mm. |
| §Decision post-amendments vs §Closed verdict | Amendments win. The original §Closed text remains for history but is no longer current commitment. |
| §Closed verdict vs spike-close prose | Verdict wins. Prose hand-waves or summary statements that conflict with the formal §Closed are sloppy phrasing, not contradictory decisions. |
| Spike-close prose vs dev-logs | Spike-close wins. Dev-logs are AI-session records; they can be wrong (e.g., the "active-cooler-stack-equivalent" phrasing in 2026-05-13 contradicted the spike-close's passive commitment — dev-log was amended). |
| Dev-logs vs audit-bot output | Dev-logs win. Audit-bot output is input requiring filtering, never authoritative on its own. |

### Anti-patterns (do NOT)

| Anti-pattern | Why it's wrong | Source |
|---|---|---|
| Accept audit-bot findings as action items without filtering | 4 of 5 Autodesk Assistant findings on 2026-05-14 were STALE or HALLUCINATION; acting on them would have re-introduced pogo bore + shrunk depth wrong + chased a non-issue on door bosses | `cad-day-retrospective` G/F section; `anker-dims-and-gate-propagation.md` §"Post-Autodesk-Assistant-audit pass" |
| Propose materials, cooling, or topology that contradict committed spike-close §Decision | Passive cooling was flipped to active twice on 2026-05-14 before Pieter caught it | Consult user memory for cooling-related feedback rules — the passive heat-spreader commitment in the enclosure spike-close §Decision is the source of truth; any drift toward active cooling terminology is the anti-pattern. Cross-ref: `cad-day-retrospective` F12 |
| Extrude internal features before upstream carry-over decisions are resolved | Orientation X vs Y (carry-over #1) gates all Pi/HAT mounting bosses, display window cutout, gasket-groove offsets, heat-spreader pocket details | `TODO.md` carry-over; `cad-day-retrospective` O1, O2 |
| Trust WebSearch for vendor mechanical dimensions | Pi Touch Display 2: WebSearch returned 8.55 mm, direct PDF gave 15 mm; 6.45 mm error would cascade through stack-up arithmetic | `cad-day-retrospective` F1 / `anker-dims-and-gate-propagation` §(a) |
| Iterate-and-delete-inline on Fusion `comp.features` | `InternalValidationError: dmFeature || pmFeature` after cascade-delete; loop variable references already-deleted feature | `cad-day-retrospective` F10 |
| Use `defineAsFreeMove(transform)` for parametric translations | Rigid Matrix3D breaks parametric coupling; use `defineAsTranslateXYZ(x, y, z, isLocal)` with `ValueInput.createByString(...)` | `cad-day-retrospective` F9 |
| Rewrite spike-close §Decision text in place without supersession header | Loses the history trail; readers in 2027 can't see what changed when or why | `meta-retro-missing-angles` §1 Pattern 5; structural observation in old Part 2 |
| Treat hand-wave dimensions in spike-closes as input for downstream specs | One hand-wave (Anker 154×62×30) propagates exponentially; correcting it later required 8+ inline edits across 2 spike files plus CAD param comments | `cad-day-retrospective` P2 |
| Place dimensions on sketches on offset construction planes without first verifying axis-mapping | Sketch-local X on offset YZ plane mapped to world Z on 2026-05-14, not Y; required dimension-swap to correct | `cad-day-retrospective` F7; Recipe E |

## §3 — Workflow recipes

Concrete actionable patterns. Each recipe carries a **frequency tag** indicating how often it fires per CAD session, plus references to relevant Fusion API survival entries from §4.

### [EVERY SESSION] Recipe J — Sketch convention: origin at body geometric center

**Trigger:** Creating a new sketch that will define a body.

**Steps:**

1. Place the sketch's origin at the **geometric center** of the body the sketch defines (not at an arbitrary corner or offset).
2. Use coincident or symmetry constraints to enforce centring.
3. Document the convention in the sketch's name or in a comment.

**Rationale:** Inconsistent sketch-origin conventions led to audit-bot misreads ("bosses asymmetric") and human confusion on 2026-05-14. Door-profile at sketch-X=-20 vs bezel-outline at origin = inconsistent.

**Related API:** F8 (distance dim sign disambiguation — origin at center makes anchor dims symmetric, removes sign ambiguity).

### [EVERY SESSION] Recipe E — Test-extrude before dimensioning on offset construction plane

**Trigger:** A new sketch is being created on a construction plane that isn't the XY, XZ, or YZ origin plane.

**Steps:**

1. Create the construction plane parametrically (e.g., `outer_w / 2 - 1.5 mm` offset from YZ).
2. Add the sketch.
3. **Before placing any dimensions:** sketch a small **asymmetric** test profile (e.g., 10×5 mm rectangle near origin), extrude 1 mm, inspect the body's world bbox. Record the sketch-X / sketch-Y → world-axis mapping.
4. Delete the test extrude + profile.
5. Proceed with the real sketch using the verified axis-mapping.

**Rationale:** Sketch-X on offset YZ plane mapped to world Z (not Y) on 2026-05-14. 5 min test prevents 30 min wrong-orientation rebuild.

**Related API:** F7, F12.

### [EVERY SESSION] Recipe G — Iterate-collect-delete pattern for Fusion mutations

**Trigger:** Looping over a Fusion collection (features, sketches, bodies) to delete or modify entries matching a condition.

**Steps:**

1. **Collect first:** `to_change = [item for item in collection if matches(item)]` — materialise to a Python list.
2. **Mutate second:** `for item in to_change: item.deleteMe()` (or modify).
3. Never combine iteration and mutation in one loop.

**Rationale:** Cascade-deletes on Fusion features mutate the collection mid-iteration; subsequent access throws `InternalValidationError: dmFeature || pmFeature`. End-state may still be correct, but the exception is fragile and confusing.

**Related API:** F10.

### [EVERY SESSION] Recipe F — Parametric Move feature

**Trigger:** A body needs to be translated by an amount tied to a user parameter.

**Steps:**

1. `move_input = comp.features.moveFeatures.createInput2(bodies_collection)`
2. Translation via `defineAsTranslateXYZ(x_value, y_value, z_value, isLocal=False)` where each `_value` is `adsk.core.ValueInput.createByString("-rear_depth")` (or any parametric expression).
3. `comp.features.moveFeatures.add(move_input)`.
4. **Do not use** `defineAsFreeMove(transform)` with a hard-coded `Matrix3D` — that's rigid and breaks parametric coupling.

**Rationale:** Battery-door rebuild on 2026-05-14 initially used rigid -40 mm; Pieter's review required parametric refactor to `-rear_depth`. Use parametric from the start.

**Related API:** F9.

### [OCCASIONAL — every session on doc-heavy days] Recipe K — Amend a spike-close (NEW)

**Trigger:** A committed Accepted spike-close needs an inhoudelijke wijziging (decision change, dimension correction, retirement of a feature, addition of a new constraint).

**Steps:**

1. **Add a dated supersession section at the top** of the spike-close file, immediately after the frontmatter and before the `## Closed YYYY-MM-DD` heading. Format:
   ```
   ## YYYY-MM-DD partial supersession — <one-line summary>

   The <X> commitment from the YYYY-MM-DD verdict is superseded. <One-paragraph
   explanation: what changed, why, what consuming docs/spikes need to know.>

   <Bulleted list of concrete clause-level changes.>

   See also <dev-log path>.
   ```
2. **Update `amended: YYYY-MM-DD`** in the frontmatter.
3. **Inline-mark every superseded clause** in §Closed verdict and §Decision text. Use `[SUPERSEDED YYYY-MM-DD — see top-of-file]` for removed/replaced content, `[CORRECTED YYYY-MM-DD — <source URL>]` for value corrections. Use markdown strikethrough `~~old text~~` plus the marker for visually-clear replacement.
4. **Add an amendment block at the top of the §Decision code block** listing exactly which §Decision clauses changed. Preserves the formal decision-note integrity.
5. **Propagate** via Recipe B to all consuming docs (cross-spike implications, CLAUDE.md, ARCHITECTURE.md, README.md, TODO.md, bom.md as applicable).
6. **Do NOT rewrite the §Closed verdict or §Decision text in place.** History trail is part of the value.

**Rationale:** Multi-amendment spike-closes need a working pattern that preserves traceability without obscuring current state. On 2026-05-14, the enclosure spike got two top-of-file supersession sections (depth correction + pogo retirement) plus inline markers — readers can still see what 2026-05-08 committed to AND what was changed when.

**Related API:** none directly; this is a doc-discipline recipe. Cross-ref Recipe B for the propagation pass.

### [OCCASIONAL] Recipe B — Propagate a corrected value across the doc set

**Trigger:** A previously-cited value (dimension, term, signal name) has been corrected and needs to land everywhere it's consumed.

**Steps:**

1. `Grep` for the old value across the repo with multiple format variations.
2. Categorise hits: spike-closes (need supersession headers + inline markers via Recipe K); dev-logs (historical, preserve as-is unless contradicting current state); canonical project docs (CLAUDE.md, ARCHITECTURE.md, README.md, TODO.md, bom.md — update inline with footnote refs).
3. For canonical docs: update inline; add a single `[^correction-yyyy-mm-dd]` footnote at the first occurrence with full explanation + cross-refs.
4. Verify via second grep that only intentional historical/amendment references remain.
5. Update CAD user-param comments to cite the new source URL.

**Rationale:** Anker dim correction on 2026-05-14 required 8+ inline edits across 2 spike files + dev-logs + canonical docs. Forgetting one location creates drift that surfaces in audits weeks later.

**Related API:** none.

### [OCCASIONAL] Recipe C — Stack-up arithmetic for a depth/footprint/volume spec

**Trigger:** A spike-close cites depth/footprint/volume without per-row math, OR Fusion geometry disagrees with the spec.

**Steps:**

1. Create a per-row table: `Layer | mm | Source`.
2. List every mechanical contributor in stack order (outer face → inner reference).
3. For each row, `Source` is one of: `path/to/spike-close.md` text, datasheet URL, or **HAND-WAVE** (explicitly flagged).
4. Sum. Compare to the spec.
5. If sum > spec by a meaningful margin: spec is wrong, not geometry. Correct via Recipe K + B.
6. Surface HAND-WAVE rows in a "To verify before close" block.

**Rationale:** Per-row arithmetic forces every contributor to be cited or admitted as hand-wave. Writing the table is the verification. 45-55 mm depth target survived because no one wrote this table; the table killed it in 30 minutes on 2026-05-14 morning.

**Related API:** none.

### [OCCASIONAL] Recipe A — Verify a vendor dimension

**Trigger:** A spike-close, dev-log, or CAD param cites a vendor part's dimensions.

**Steps:**

1. Identify vendor + part number.
2. Fetch official vendor product page or datasheet PDF directly via `WebFetch` (NOT WebSearch snippets).
3. Cite the URL in param comment or doc text.
4. If verified differs from existing claim, run Recipe B + Recipe K.

**Rationale:** WebSearch returned 8.55 mm for Pi Touch Display 2 thickness; direct PDF fetch gave 15 mm. Anker 154×62×30 in spike-close vs 119.9×73.4×31.4 actual. Vendor pages are authoritative; WebSearch summaries are not.

**Related API:** none (uses `WebFetch` tool, not Fusion API).

### [OCCASIONAL] Recipe H — Cross-component extrude

**Trigger:** A sketch in component A is needed as the profile for an extrude producing a body in component B.

**Steps:**

- **Option 1 (preferred):** Recreate the sketch in component B with the same parametric geometry. On 2026-05-14: front-shell-outline rebuilt directly in `front-shell` component using `outer_w` / `outer_h` / `corner_r` user params.
- **Option 2 (acceptable):** Extrude in component A, then `body.moveToComponent(target_occurrence)` to relocate. Side effect: extrude feature stays in component A while body lives in component B (timeline asymmetry, cosmetic only).
- **Do NOT:** call `extrudeFeatures.createInput()` on component B with a profile from component A's sketch — Fusion throws `InternalValidationError: bSet`.

**Rationale:** Cross-component sketch consumption is restricted in the Fusion API.

**Related API:** F2, F6.

### [RARE] Recipe I — Forensic on volume-delta anomaly

**Trigger:** A feature's reported volume change disagrees with geometric ideal by more than rounding-error.

**Steps:**

1. Save `original_marker = design.timeline.markerPosition`.
2. Walk the timeline at marker positions N-1 and N around the feature; record body volume, face count, bbox at each.
3. Compute delta.
4. If discrepant: collect face signatures `(Z_round, area_round, X_centroid_round, Y_centroid_round)` pre and post; identify disappeared + appeared faces.
5. Hypothesise (e.g., shell-feature cavity-interior sliver cleanup, hidden topology fragments).
6. **Restore the marker:** `timeline.markerPosition = original_marker` — non-negotiable.

**Rationale:** Heat-spreader pocket cut removed 11,939 mm³ vs 7,200 mm³ ideal on 2026-05-14. Face-level diff isolated 4 cavity-interior sliver faces at origin centroid as the source. Volume math alone can mislead on complex shell+cut interactions.

**Related API:** F11.

## §4 — Fusion API survival guide

Frequency-tagged for skill cognitive budget: load MUST-KNOW always, MAY-KNOW on context match, RARE only when forensic mode.

| Freq | ID | Gotcha | Workaround |
|---|---|---|---|
| **MUST-KNOW** | F2 | `extrudeFeatures.createInput(profile, op)` throws `InternalValidationError: bSet` when called on a component whose sketches don't include the profile's parent | Do the extrude in the source-sketch's component, OR recreate the sketch in the target component (Recipe H) |
| **MUST-KNOW** | F5 | `sketchLines.addByTwoPoints` with four separate Point3D arguments does NOT auto-coincide corners — profile detection fails | Use `sketchLines.addTwoPointRectangle(p1, p2)` for closed rectangles |
| **MUST-KNOW** | F10 | Iterate-and-delete-inline on `comp.features` triggers `InternalValidationError: dmFeature || pmFeature` | Collect into list first, then delete (Recipe G) |
| **MUST-KNOW** | F4 | `adsk.fusion.FeatureOperations` enum: Join=0, Cut=1, Intersect=2, NewBody=3, NewComponent=4 | Query via `dir(adsk.fusion.FeatureOperations)`; don't guess from memory |
| **MUST-KNOW** | F1 | `mcp__fusion360__fusion_screenshot` broken on Fusion 2702.x ("takes 2 positional arguments but 5 were given") | Use `app.activeViewport.saveAsImageFile(path, w, h)` directly via `fusion_execute` |
| **MAY-KNOW** | F3 | `setSymmetricExtent(distance, isFullLength=True)` means `distance` IS the total extent (not half) | Want N mm total: pass `distance="N mm"` with `isFullLength=True`. Want 2*N total: `isFullLength=False`. |
| **MAY-KNOW** | F7 | Sketch local X/Y axis mapping on offset construction planes is plane-orientation-dependent | Test-extrude asymmetric profile before placing dimensions (Recipe E) |
| **MAY-KNOW** | F8 | `addDistanceDimension` returns absolute distance; solver may disambiguate sign unexpectedly | Anchor via coincident-to-construction-point or midpoint constraint when sign matters |
| **MAY-KNOW** | F9 | `MoveFeature` with `defineAsFreeMove(transform)` is rigid; `defineAsTranslateXYZ(x, y, z, isLocal)` accepts parametric ValueInputs | Use `defineAsTranslateXYZ` from the start (Recipe F) |
| **MAY-KNOW** | F6 | `sketch.project()` cross-component fails silently (returns 0 profiles) | Recreate the geometry parametrically in the target sketch (Recipe H Option 1) |
| **MAY-KNOW** | F12 | `sketch.referencePlane` shows the construction plane but doesn't expose a clean world-axis mapping | Inspect via test-extrude (Recipe E) or query the construction plane's origin + normal |
| **RARE** | F11 | Shell + cut feature interaction can remove more material than the cut geometry due to BREP solver cavity-interior sliver cleanup | Use timeline-rollback + face-level diff (Recipe I) |
| **RARE** | F13 | `extrudeFeatures.add(ext_input).bodies` exists but feature may report participantBodies access errors mid-edit | Save the body reference immediately after `add()`; avoid mid-edit feature property access |

## §5 — Audit-bot filter workflow

Promoted from Recipe D because it's the highest-leverage filter the skill has. Apply whenever an audit-bot (Autodesk Assistant in Fusion 360, similar in-app LLMs, or any review-style AI output) presents findings.

### Verdict table template

For each finding, classify:

| Verdict | Definition | Action |
|---|---|---|
| **REAL** | Finding is correct against current live state + current spec | Act on it (file as a CAD task or carry-over) |
| **STALE** | Finding identifies an issue correctly but reads pre-correction spec or out-of-date state | Note as already-resolved; do NOT act |
| **HALLUCINATION** | Finding misreads geometry or specs (wrong measurement, wrong reference frame, wrong inference) | Note as wrong; do NOT act; if pattern persists, flag the audit-bot's blindspot |
| **VERIFIED-OK** | Finding asks a question that resolves to "no issue" after live check | Document the resolution; do NOT act |

### Procedure

1. For each finding, query the live Fusion state via `fusion_execute` (don't trust the bot's reading).
2. Cross-check against the current spike-close (after-amendments state) and recent dev-logs for context.
3. Apply one verdict per finding with cited evidence in a markdown table.
4. Act only on REAL findings.

### Concrete example from 2026-05-14

Autodesk Assistant audit #3 surfaced 5 conflicts. Filter result:

| Finding | Verdict | Why |
|---|---|---|
| C1 — total depth 100 mm vs 45-55 mm spec | **STALE** | Spec was updated to 85-100 mm same morning; audit read pre-correction |
| C2 — rear-shell undersize 177×117 | **REAL** | Confirmed via live bbox; carry-over for X-asymmetry interpretation |
| C3 — battery door bosses asymmetric (-31.5, -8.5) | **HALLUCINATION** | Both bosses 11.5 mm from door center (-20); symmetric about door center, audit measured from sketch origin (0,0) |
| C4 — heat-spreader-pocket sketch has no extrude | **REAL** | Confirmed; fixed in same session via offset-start cut |
| C5 — pogo bore missing | **STALE** | Pogo retired same morning; absence is correct |

Two REAL findings out of five. Without the filter: would have re-introduced pogo + shrunk depth wrong + chased non-issue.

**Source pattern:** `dev-log/2026-05-14-anker-dims-and-gate-propagation.md` §"Post-Autodesk-Assistant-audit pass (filter then act)".

## §6 — Worked example: "add the heat-spreader-pocket cut"

Concrete walkthrough of how the skill would handle a typical CAD-touch request. Adapted from the 2026-05-14 actual session.

### User input

> "Cut the heat-spreader pocket into the rear shell."

### Skill opening response (templated per §1)

> CAD state snapshot:
> - Live Fusion: `gateway-v1`, 5 components, 4 bodies (front-shell-body, rear-shell-body, divider-body, battery-door-body, bezel-body), 23 user params, 28 timeline entries
> - Active spike amendments: enclosure spike-close has 2 supersession headers (depth correction + pogo retirement); power-arch has 1 (pogo retirement); tak-cot + runtime-task + ble-gateway-ui-flow each have 1 (gate language correction)
> - Open blockers from TODO carry-over: #1 Orientation X vs Y, #2 front-depth squeeze, #3 X-asymmetry, #4 rear-compartment slack, #5 door-profile sketch origin, #6 heat-spreader pocket volume delta
> - I understood your request as: extrude-cut the existing `heat-spreader-pocket` sketch (80×60 mm) into `rear-shell-body`, recess depth = `spreader_depth` (1.5 mm), on the inner-back-face
> - Proposed first action: verify the sketch's current state and reference plane, then design the cut feature
> - Anything I'm missing before I start?

### First-pass fusion_execute queries

```python
# Locate the sketch
rs = find_comp("rear-shell")
hsp_sketch = next(s for s in rs.sketches if s.name == "heat-spreader-pocket")

# Inspect plane + profile
print(f"Reference plane normal: {hsp_sketch.referencePlane.geometry.normal}")
print(f"Profile area: {hsp_sketch.profiles.item(0).areaProperties().area * 100:.2f} mm²")
print(f"Profile centroid: {hsp_sketch.profiles.item(0).areaProperties().centroid}")

# Confirm rear-shell-body bbox to identify inner-back-face position
for b in rs.bRepBodies:
    if b.name == "rear-shell-body":
        bb = b.boundingBox
        print(f"Body Z range: {bb.minPoint.z*10} to {bb.maxPoint.z*10} mm")
```

### Recipe path

1. **Recipe J** (sketch origin convention) — already satisfied; sketch is centered at origin.
2. **Recipe E** (test-extrude on offset plane) — NOT needed; sketch is on XY plane (Z=0), straightforward mapping.
3. **Pre-cut analysis:** Sketch is on parting plane (Z=0); needs to cut into the rear shell's back wall (inner-back-face at Z=-37, outer-back at Z=-40). The cut should not start at sketch plane (which is Z=0, inside the cavity) but at the inner-back-face level (Z=-37). Use `OffsetStartDefinition` for parametric start position.
4. Build the cut feature:
   ```python
   ext_input = rs.features.extrudeFeatures.createInput(
       hsp_sketch.profiles.item(0),
       adsk.fusion.FeatureOperations.CutFeatureOperation  # value=1 per F4
   )
   start_offset = adsk.core.ValueInput.createByString("-(rear_depth - wall)")
   ext_input.startExtent = adsk.fusion.OffsetStartDefinition.create(start_offset)
   distance = adsk.core.ValueInput.createByString("-spreader_depth")
   ext_input.setDistanceExtent(False, distance)
   ext_input.participantBodies = [rear_body]
   feat = rs.features.extrudeFeatures.add(ext_input)
   feat.name = "heat-spreader-pocket-cut"
   ```
5. **Forensic check (Recipe I)** if volume delta seems anomalous: timeline rollback at N-1 and N, face-level diff, restore marker.

### Expected confirmation back to user

> Heat-spreader-pocket cut applied. Live verification:
> - New feature: `heat-spreader-pocket-cut` at timeline position 28
> - rear-shell-body volume: 155,677 → 143,738 mm³ (Δ = -11,939 mm³)
> - Note: removed volume is ~4,700 mm³ over the geometric ideal (80×60×1.5 = 7,200 mm³). Likely cause: BREP solver cavity-interior sliver cleanup (F11). Pocket geometry verified correct: bottom-face at Z=-38.5 mm with area 4,704.97 mm² (target 4,800 minus corner fillets), 4 sidewalls at Z=-37.75, outer back face unchanged.
> - This is logged as carry-over blocker #6 (low priority — investigate further only if pocket-related issues appear in later features).

Total context loaded for this operation: §1 first-action snapshot, §3 Recipe J + Recipe I, §4 F2 + F4 + F11 + F10. The skill did NOT need to load: door-related recipes, audit-filter workflow, propagation passes.

## §7 — Skill self-maintenance

Concrete triggers, not vague "watch for staleness":

### Triggers for doc-map update (`docs/sarcom-cad-doc-map.md`)

| Event | Action | SLA |
|---|---|---|
| A `spikes/gateway-handheld-*` spike-close gets a new dated supersession section | Update the corresponding entry in doc-map §Spike-closes with the new amendment summary + what was learned | Within 24 hours of the spike-close commit |
| A new CAD-day dev-log lands in `dev-log/` | Add it to doc-map §Dev-logs with role + what was learned + how to use | Within 24 hours of dev-log commit |
| A new ADR is committed that constrains the gateway enclosure | Add to doc-map §ADRs with the mechanical constraint it enforces | Within 24 hours |
| An external datasheet is consulted for a vendor part not yet in the doc-map | Add to doc-map §External datasheets with URL + verified dimensions + learned lessons | Same session as the consultation |

### Triggers for skill-design update (this file)

| Event | Action | SLA |
|---|---|---|
| A new Fusion API gotcha is encountered with a workaround | Add to §4 with F-ID, frequency tag, gotcha description, workaround | Same session as the encounter |
| A new workflow pattern emerges across 2+ sessions | Add to §3 as a new Recipe with frequency tag + Trigger / Steps / Rationale / Related API | After the second occurrence |
| An anti-pattern is observed and corrected (e.g., a flip on a committed decision) | Add to §2 anti-patterns table with the corrective rule | Same session |
| The activation rules in §0 admit a borderline case that didn't work well | Refine §0 (a)/(b)/(c) trigger logic | Within 1 week of the misfire |

### Quarterly review

Re-read the `dev-log/2026-05-14-cad-day-retrospective.md` family + `retrospectives/2026-05-14-design-decisions.md` + `retrospectives/2026-05-14-meta-retro-missing-angles.md`. Check if any meta-pattern from the day's retros has been promoted to a recipe yet, or if new patterns have emerged that deserve promotion. Calendar-trigger: every 3 months from skill installation date.

### Sources of staleness to watch

- **Spike-close amendments** — supersession sections override §Decision text; the skill's source-of-truth hierarchy in §2 must stay consistent with this convention.
- **Fusion API changes** — F1 (screenshot wrapper bug) is tied to Fusion 2702.x; future Fusion versions may fix it or introduce other behaviours.
- **Vendor dimension changes** — Anker SKU revisions, RPi accessory revisions can change dimensions; Recipe A re-verification on SKU change is mandatory.

## Cross-refs

- `docs/sarcom-cad-doc-map.md` — descriptive per-document analysis (companion to this file)
- `dev-log/2026-05-14-cad-day-retrospective.md` — fuller F1-F13 explanations + meta-pattern reasoning
- `retrospectives/2026-05-14-design-decisions.md` — 8 design decisions of 2026-05-14 with trade-off framing
- `retrospectives/2026-05-14-meta-retro-missing-angles.md` — positive-pattern catalogus + source-of-truth hierarchy origin + decision residue map
- All four 2026-05-14 dev-logs — chronological context for the day's work
- `docs/spike-rules.md` + `docs/spike-template.md` + `docs/spike-prelude.md` — spike-writing discipline docs
- `CLAUDE.md` — project values + "Do NOT re-open" list, the design-choice filter
