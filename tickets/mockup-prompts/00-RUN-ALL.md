# Orchestrator task — execute every v1a kiosk mockup prompt

You are running every active mockup prompt under
`tickets/mockup-prompts/`. Each per-prompt file is self-contained — its
own context, constraints, citations, and outputs are inside it. Your job
is to execute them in order, validate each SVG with the project's
wireframe validator, and leave a single reviewable diff under
`UX/mockups/`.

## Skills to activate first

Activate these two skills before reading any per-prompt file:

- **`sarcom-ux`** — loads the v1a UX strict-ADR posture, bias rules, non-goals, and honesty discipline.
- **`sarcom-svg-wireframe`** — loads the SVG structure discipline (viewBox, grouped layers, annotation conventions, single-design vs dual-variant rules).

Both skills live under `.claude/skills/`. If either skill carries pre-cleanup wording (Variant A/B framing as live, floating map buttons as primary chrome, mandatory `Variant A` text label in every SVG), HALT and tell Pieter the skill-cleanup passes (`STEP-1-cleanup-svg-wireframe-skill.md` and `STEP-2-cleanup-sarcom-ux-skill.md`) need to run first.

## Read first (orientation only — per-prompt files own their own detailed reading)

- `tickets/README.md` — v1a UX posture (lines 14-33), spike closures (lines 45-46), per-ticket scopes (lines 52-58), per-ticket mockup canonical paths (lines 77-81), non-mockup tickets (lines 85-86).
- `tickets/mockup-prompts/README.md` — index of active mockup prompts and the KIOSK-002 deferred stub.
- `decisions/ADR-007-touchscreen-primary-ui.md:38-46` — the read-only-UI invariant the mockups honour.
- `UX/mockups/v1a-operator-map-mockup.md` — umbrella reference; per-ticket mockups zoom into specific surfaces.

## Halt conditions

HALT and report to Pieter without producing any mockup if any of these conditions are true:

1. The `sarcom-ux` skill still presents Variant A/B as a live tension, OR lists `fit-all / home / zoom buttons` as primary first-class chrome. → STEP-2 cleanup not run.
2. The `sarcom-svg-wireframe` skill still requires `Variant A` as a mandatory `<text>` label in every SVG, OR the validator at `.claude/skills/sarcom-svg-wireframe/scripts/validate_svg.py` unconditionally fails on missing `Variant A`. → STEP-1 cleanup not run.
3. Any per-prompt file under `tickets/mockup-prompts/` still carries Variant A / Variant B framing, `IGNORE Variant A scope` wording, or crosshair / coord-readout references that contradict `tickets/README.md:14-33`. → mockup-prompts cleanup not run.
4. `tickets/mockup-prompts/KIOSK-002-mockup-prompt.md` is not a deferred stub. → mockup-prompts cleanup not run.

If any halt condition trips, do nothing further; report which condition tripped and which cleanup prompt needs to run first.

## Cross-cutting posture (the skills should establish this; restated here for safety)

- Strict ADR-007. No modals, popovers, banners, overlays, acknowledgement flows, touch-hold UI morphing.
- Map chrome budget: **scale bar + compass rose only.** No zoom +/−, fit-all, home, clear-UI, or any other floating button.
- No coord readout outside per-node detail surfaces. KIOSK-002 is deferred per `tickets/README.md:30, 53, 71, 85`.
- No no-fix uncertainty disc (SPIKE-002 closure: reject for v1a).
- No invented RSSI / SNR / hop fields. Data model is `tools/sarcom-kiosk-lab/src/data.rs:131-167`.
- No fake radio / process health signals. (Gateway-self status — battery / RTC / render-tick liveness — is deferred from v1a per `tickets/KIOSK-005-gateway-status-surface.md` deferred stub. The mockups do not surface gateway-self state at all.)
- No acknowledgement button anywhere in the kiosk.
- Palette: only constants from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.
- Lab fixture: 800×480 per panel per `tools/sarcom-kiosk-lab/README.md:53`; annotate every mockup as ADR-015-pending.
- Cite `path:line` for every concrete claim in each mockup's rationale markdown.
- Per-ticket v1a mockups are **single-design SVGs** (post-SPIKE-001-closure). No Variant A panel. No `Variant A` label.

## Execution plan (sequential, in this order)

Process these six prompts one at a time. The canonical output paths come from `tickets/README.md:77-82`. If a per-prompt file specifies a different output path (e.g. `UX/mockups/KIOSK-NNN-mockup.svg`), USE THE TICKETS-INDEX CANONICAL PATH below. Write each pair to ONE location only; do not duplicate.

| Order | Prompt file | Canonical SVG output | Canonical md output |
|---|---|---|---|
| 1 | `tickets/mockup-prompts/KIOSK-001-mockup-prompt.md` | `UX/mockups/KIOSK-001-map-scale-north.svg` | `UX/mockups/KIOSK-001-map-scale-north.md` |
| 2 | `tickets/mockup-prompts/KIOSK-003-mockup-prompt.md` | `UX/mockups/KIOSK-003-sidebar-row-redesign.svg` | `UX/mockups/KIOSK-003-sidebar-row-redesign.md` |
| 3 | `tickets/mockup-prompts/KIOSK-004-mockup-prompt.md` | `UX/mockups/KIOSK-004-selection-detail-sidebar.svg` | `UX/mockups/KIOSK-004-selection-detail-sidebar.md` |
| 4 | `tickets/mockup-prompts/KIOSK-006-mockup-prompt.md` | `UX/mockups/KIOSK-006-sos-strip.svg` | `UX/mockups/KIOSK-006-sos-strip.md` |
| 5 | `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md` | `UX/mockups/KIOSK-008-marker-track.svg` | `UX/mockups/KIOSK-008-marker-track.md` |

`tickets/mockup-prompts/KIOSK-002-mockup-prompt.md` is a deferred stub — **SKIP**. No mockup produced.

`tickets/mockup-prompts/KIOSK-005-mockup-prompt.md` is a deferred stub (deferred from v1a along with `tickets/KIOSK-005-gateway-status-surface.md` per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`) — **SKIP**. No mockup produced. The deferred-stub mockup-md at `UX/mockups/KIOSK-005-gateway-status.md` is the canonical landing — do not regenerate.

KIOSK-007 has no operator UI mockup per `tickets/README.md:86` — **SKIP**.

## Allowed writes (whitelist)

The ONLY filesystem writes you may perform during this run are:

- `UX/mockups/KIOSK-001-map-scale-north.svg`
- `UX/mockups/KIOSK-001-map-scale-north.md`
- `UX/mockups/KIOSK-003-sidebar-row-redesign.svg`
- `UX/mockups/KIOSK-003-sidebar-row-redesign.md`
- `UX/mockups/KIOSK-004-selection-detail-sidebar.svg`
- `UX/mockups/KIOSK-004-selection-detail-sidebar.md`
- `UX/mockups/KIOSK-006-sos-strip.svg`
- `UX/mockups/KIOSK-006-sos-strip.md`
- `UX/mockups/KIOSK-008-marker-track.svg`
- `UX/mockups/KIOSK-008-marker-track.md`

Any write outside this whitelist is a process failure. If you find yourself about to edit anywhere else, STOP and report the conflict.

## Per-ticket reminders (refresher; the per-prompt files are the source of truth)

- **KIOSK-001:** Map chrome only — compass rose top-left, fixed 80 px scale bar bottom-left, no zoom/fit/home/clear buttons.
- **KIOSK-003:** Sidebar rows only; sticky `DISTRESS` section; rows ≥48 px; full-row selected tint; battery-low as inline `🔋 BATT` on primary line; mission-first sort preserved; no counter footer card. Kind-specific glyphs (tag `●` / relay `✚` / gateway `■`) from inventory map. Timestamps render bare (no `last` / `POSITION` prefix); the `last fix` framing on no-fix rows stays as a scoping label for lat/lon.
- **KIOSK-004:** Sidebar replacement detail; no slide-in; no overlay; no tap-outside dismiss; 150 ms smooth recenter; **one uniform detail layout for any selected node** (post-collapse per `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`); rows that don't apply for the selected node are simply absent (not `N/A` placeholders); tag-3 ghost shown with `LAST FIX · {age}` framing; NOT SHOWN block is reserved for protocol-level closures (ADR-013 §10 RSSI/SNR/hop), NOT for sim-fixture gaps.
- **KIOSK-006:** Persistent SOS bottom strip only; no banner; no acknowledge; no dismiss; strip height unchanged (24 px); DISTRESS text size unchanged (13 pt); subtle background pulse synchronised with marker pulse-ring; strip text format `DISTRESS · {label} · last frame {age} · ack at tag`; multi-SOS shows most-recent only.
- **KIOSK-008:** Map marker + track rendering. Baseline three-fix tail per tag with `gps_valid=true` (newest brightest, oldest dimmest, per-state colour from `freshness_color`); on `Selection::Node(_)` from KIOSK-003 (post-collapse selection enum — one variant, no per-kind split), the selected node's three-fix tail is replaced by the full polyline through `NodeData.track` rendered in **per-state colour at opacity 0.5** (Decisions pinned #7); other nodes retain their baseline tail. **No tail for SOS state** (Decisions pinned #4 — current dot + pulse ring only). **No tail for no-fix state** (ghost marker only). Tail and current dot both use per-state colour; KIOSK-008 also fixes the PMTiles inline draw to call `freshness_color` from `markers.rs:40-48` (Decisions pinned #8). Fade-opacity triple, tail-dot size, connector line, no-fix selection behaviour, and polyline thickness remain open questions — the mockup picks concrete render-only values and surfaces them.

## For each prompt — execution steps

1. Read the prompt file in full.
2. Read each source file it cites — targeted line ranges only, not whole files. The per-prompt files include path:line hints; honour them.
3. **Idempotency check:** before generating, check whether the canonical SVG + md pair already exists AND is complete (SVG opens with `<svg` and closes with `</svg>`; md has the closing "what a reviewer verifies" section). If yes → SKIP, log `KIOSK-NNN ⊘ already present — skipped`, move on.
4. Generate the SVG at the canonical path. Generate the markdown rationale at the canonical path.
5. The rationale markdown must close with a "what a reviewer verifies" section per the per-prompt file's spec; do not skip it.
6. Verify both files exist and are non-empty.
7. **Run the validator:**
   ```
   python .claude/skills/sarcom-svg-wireframe/scripts/validate_svg.py UX/mockups/<canonical-svg-path>
   ```
   Capture exit code. Expected: `[PASS]` (exit 0). If `[FAIL]`, diagnose: the SVG is malformed (missing viewBox, wrong root element, mixed variant labels) or contains a forbidden term without an exception marker. Fix and re-validate.
8. Append one line to the running progress log: `KIOSK-NNN ✓ produced <svg path> + <md path> · validator [PASS]`.
9. Move to the next prompt.

## Hard constraints on your execution

- Do NOT edit any file outside the whitelist above. This means no edits to `tickets/`, `decisions/`, `ARCHITECTURE.md`, `README.md` (root), `CLAUDE.md`, `.claude/`, `tools/`, `spikes/`, `dev-log/`, or any other `UX/mockups/*` file.
- Do NOT edit any `tickets/mockup-prompts/*.md` — those are the spec for this run.
- Do NOT modify `UX/mockups/v1a-operator-map-mockup.svg` or `UX/mockups/v1a-operator-map-mockup.md` (the legacy umbrella artifact).
- Do NOT create a git worktree.
- Do NOT commit anything. Leave the working tree dirty for Pieter to review one diff covering all six mockup pairs.
- Do NOT introduce any new prompt files.
- Do NOT spawn sub-agents. One linear session, one reviewable diff.

## Quality floor

Wireframe-fidelity decision artifacts, not marketing polish. Every mockup MUST:

- Pass the validator (`[PASS]`, exit 0).
- Use only palette constants cited from `tools/sarcom-kiosk-lab/src/ui/palette.rs`.
- Annotate every chrome element with a small marginal callout naming the element and its source-of-truth (`path:line`).
- Render the lab fixture frame at the per-prompt-specified canvas size (KIOSK-001, -003 are 800×480 single panel; KIOSK-004 / -005 / -006 / -008 use stacked panels per their per-prompt specs) with an `ADR-015-pending` margin annotation.
- Include the `<svg xmlns="http://www.w3.org/2000/svg" viewBox="...">` namespace declaration at the root.
- Be diffable text — no embedded raster images, no base64 blobs.
- Be single-design (no `Variant A` or `Variant B` text labels). The post-cleanup validator allows zero variant labels; mixed (only one of two) is the failure mode it catches.

If you cannot find a citation for a concrete claim, the claim does not go in the mockup; surface the absence under "Open questions for Pieter" instead.

## If context gets tight

If processing all six in one session looks like it will exhaust your context budget, STOP after the current mockup. Write the progress log so far. Tell Pieter:

```
Context budget near limit; restart this prompt in a fresh session.
The idempotency check will skip what's already produced and complete
the remaining mockups in order.
```

Do NOT degrade quality to fit. Do NOT skip the rationale md. Do NOT skip path:line citations. Do NOT skip the validator run.

## Final output

When all six are processed (or you've stopped early), print, in this order:

1. `git status --short UX/mockups/` (shows untracked / modified file list)
2. `git diff --stat UX/mockups/` (size summary per mockup file)
3. The progress log (one line per mockup: `produced [PASS]` / `skipped` / `failed`)
4. Per-SVG validator result (one line per SVG: `KIOSK-NNN: [PASS]` or `KIOSK-NNN: [FAIL] <reason>`)
5. Confirmation that **no files outside the allowed-writes whitelist were edited**
6. Any prompt conflicts or low-confidence assumptions encountered (do NOT silently resolve them; surface as "Open questions for Pieter")
7. Reviewer next-step:
   ```
   Review each of the six SVG + md pairs under UX/mockups/. For each:
   - Open the SVG and confirm the visual matches the prompt's intent
   - Read the md rationale and confirm the citations resolve
   - Cross-check against the source ticket (tickets/KIOSK-NNN-*.md)
   - Approve, or send back with specific notes per file
   ```

Do NOT print full SVG bodies in your final output — the diff stat is enough.
