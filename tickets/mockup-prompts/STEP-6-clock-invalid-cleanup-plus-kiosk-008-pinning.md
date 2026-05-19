# Clock-invalid cleanup + KIOSK-008 open-question pinning

> **STATUS: completed 2026-05-19.** Historical work-instruction artifact. References to `TagData` / `clock_valid` / `self_ann_age_secs` are pre-collapse names; the post-collapse data model lives in `NodeData` + `inventory: HashMap<u8, NodeKind>` per [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md). The `ClockInvalid` scenario and `clock_valid` field are removed from `data.rs`; the three KIOSK-008 open questions are pinned in [`tickets/KIOSK-008-marker-track-rendering.md`](../KIOSK-008-marker-track-rendering.md). Do not re-run this STEP prompt.

Pieter has decided two things that together require one coherent cleanup pass:

1. **Clock-invalid framing is dropped from v1a.** Tags and relays don't carry their own clock — gateway rx timestamps are the source of truth. The gateway has a DS3231 RTC + CR2032 backup per ADR-011, so "RTC unset at boot" is a hardware-failure mode, not an operational state worth designing UI around. The `ClockInvalid` scenario, the `clock_valid: bool` field, and every ticket/mockup acceptance criterion that exercises clock-invalid behaviour are noise. They go.
2. **Three KIOSK-008 open questions are pinned**, so the KIOSK-008 mockup can be re-rendered without those mockup-render-only caveats:
   - **#4 SOS-state tail: NO.** SOS tag renders current dot + pulse ring only. No three-fix tail. The pulse ring is the attention-grabbing surface; an extra red trail competes with it.
   - **#7 Selection polyline colour: tag colour at opacity 0.5.** Identity wins; neutral colour confuses "whose path is this" with multiple tags on the map.
   - **#8 Tail vs current-dot colour parity: (a) fix-in-passing.** KIOSK-008 implementation also fixes the PMTiles inline tag-draw at `pmtiles_map.rs:173-179` to use `freshness_color` from `markers.rs:40-48` instead of BLUE-only. Tail and current dot both use per-state colour (BLUE/RED/GREY). Scope-creep but small, and the BLUE-only inline draw is a pre-existing honesty-discipline bug (colour lies about freshness state).

Open questions #1, #2, #3, #5, #6 remain implementer-choice within Pieter's intent — surface in the ticket Risks unchanged.

## Halt conditions (re-check)

HALT if any are true. Same guards as the orchestrator:

1. `sarcom-ux` skill still carries Variant A/B framing or floating buttons → STEP-2 not run.
2. `sarcom-svg-wireframe` skill still requires both `Variant A` and `Variant B` labels → STEP-1 not run.
3. Any per-prompt file under `tickets/mockup-prompts/` still carries Variant A/B framing or crosshair references → STEP-3 not run.
4. The five active mockup-prompts don't yet include the three-layer basemap composition → STEP-4 not run.
5. `tickets/KIOSK-008-marker-track-rendering.md` does not exist → STEP-5 not run.

If any halt condition trips, report which and stop.

## Hard constraints on your edits

You MAY edit / delete:

- `tools/sarcom-kiosk-lab/src/data.rs` — remove `clock_valid` field on `SimState`, remove `ScenarioKind::ClockInvalid` variant + label + constructor, remove any helper that exists only to construct the clock-invalid scenario.
- `tools/sarcom-kiosk-lab/src/ui/mod.rs` — simplify `format_age_or_unavailable` (drop the clock_valid guard; the function may keep handling `Option<f32>` no-data cases for `self_ann_age_secs`-style fields), remove the `unavailable_when_clock_invalid` test at lines 48-53, rename to `format_age_or_unset` if cleaner — implementer call.
- Any other file under `tools/sarcom-kiosk-lab/src/` that references `clock_valid`, `ClockInvalid`, or `format_age_or_unavailable` — update the call site to match the simplified API. Use `git grep -n clock_valid`, `git grep -n ClockInvalid`, `git grep -n format_age_or_unavailable` to find every site. Update systematically.
- `tickets/KIOSK-001-map-scale-north.md` — remove acceptance criterion about clock-invalid (currently #6), remove manual validation step (currently #5).
- `tickets/KIOSK-003-sidebar-row-redesign.md` — remove acceptance criterion #6 (clock-invalid), remove manual step #6.
- `tickets/KIOSK-004-selection-detail-panel.md` — remove acceptance criterion #5 (clock-invalid), remove manual step #9.
- `tickets/KIOSK-005-gateway-status-surface.md` — remove acceptance criterion #7 (clock-invalid), remove manual step #6, remove any `GatewayLowBattery` scenario mentions that pair with clock-invalid (the scenario itself stays — it's an independent battery scenario).
- `tickets/KIOSK-006-sos-alerting.md` — remove manual step #5 (Clock Invalid scenario), check acceptance criteria for any clock-invalid leftover and remove.
- `tickets/KIOSK-008-marker-track-rendering.md`:
  - Remove the clock-invalid row from the per-state table.
  - Remove acceptance criterion #9 (clock-invalid suppression).
  - Remove manual validation step #5.
  - Pin open question #4 (SOS tail = NO).
  - Pin open question #7 (polyline colour = tag colour @ 0.5).
  - Pin open question #8 (resolution = (a) fix-in-passing). Add explicit scope note: "KIOSK-008 also updates `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:173-179` to call `freshness_color` from `tools/sarcom-kiosk-lab/src/map/markers.rs:40-48` so the current dot's colour matches the tail's colour per state. This is a pre-existing honesty-discipline bug fix bundled into KIOSK-008."
  - Keep questions #1, #2, #3, #5, #6 in the Risks section.
- `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md`:
  - Remove Panel C entirely (the clock-invalid panel).
  - Update the per-state table SOS row to `tail = no`.
  - Update Panel A scenario spec: tag-2 SOS shows current dot + pulse ring only (no tail, no fix-dots).
  - Pin polyline colour as tag-colour @ 0.5 in the Panel B spec (remove from the mockup-render-only choices list).
  - Add a new hard constraint stating that the mockup also reflects the fixed-in-passing PMTiles inline tag-draw: tails and current dots both use per-state colour (BLUE/RED/GREY). The mockup must show this consistency.
  - Update the open-questions list at the bottom to remove #4, #7, #8 (now pinned) and keep #1, #2, #3, #5, #6.
  - Update the "three stacked panels" to "two stacked panels" (A + B only).
- `tickets/mockup-prompts/00-RUN-ALL.md` — update the KIOSK-008 per-ticket reminder to reflect the pinned decisions and remove any clock-invalid references.
- `UX/mockups/KIOSK-008-marker-track.svg` — **DELETE.** Stale render; orchestrator regenerates after STEP-6.
- `UX/mockups/KIOSK-008-marker-track.md` — **DELETE.** Same.

You MUST NOT edit:

- Any ADR under `decisions/`. (ADR-011 already says DS3231 + CR2032 backup; the implications are accepted, not re-litigated.)
- `ARCHITECTURE.md`, `README.md` (root), `CLAUDE.md`.
- Anything under `.claude/`.
- `UX/mockups/v1a-operator-map-mockup.{svg,md}` or `UX/mockups/mockup_1.{svg,png}`.
- `UX/mockups/KIOSK-001-map-scale-north.{svg,md}`, `UX/mockups/KIOSK-003-sidebar-row-redesign.{svg,md}`, `UX/mockups/KIOSK-004-selection-detail-sidebar.{svg,md}`, `UX/mockups/KIOSK-005-gateway-status.{svg,md}`, `UX/mockups/KIOSK-006-sos-strip.{svg,md}` — these renders don't depict clock-invalid, so they stay. If their rationale `.md` files mention clock-invalid in a "what a reviewer verifies" section, surface that under Open Questions for Pieter rather than edit (the change is small enough to do in a separate STEP-7 if needed).
- Any other ticket file or spike file not listed above.

Do NOT create a worktree. Do NOT commit. Leave the working tree dirty for Pieter to review.

## Order of execution

Recommended order to keep the diff coherent:

1. **Code first.** `data.rs` → `ui/mod.rs` → call-site sweep. Verify `cargo check --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` passes after the code edits.
2. **Tickets second.** KIOSK-001, -003, -004, -005, -006 acceptance/manual-step removals (alphabetical).
3. **KIOSK-008 third.** Ticket edits (clock-invalid removal + three open-question pinnings) before the mockup-prompt edits.
4. **Mockup-prompt fourth.** `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md` reflects the pinned decisions.
5. **Orchestrator fifth.** `tickets/mockup-prompts/00-RUN-ALL.md` per-ticket reminder updated.
6. **Delete the stale KIOSK-008 mockup last.** `Remove-Item UX/mockups/KIOSK-008-marker-track.svg` and `Remove-Item UX/mockups/KIOSK-008-marker-track.md` so the orchestrator's idempotency check finds no existing pair and re-renders KIOSK-008 cleanly on the next run.

## Verification

After your edits:

1. `cargo check --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` — must compile.
2. `cargo test --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` — must pass (the `unavailable_when_clock_invalid` test is removed; other tests should still pass).
3. `git grep -n clock_valid tools/sarcom-kiosk-lab/src/` — must return zero matches.
4. `git grep -n ClockInvalid tools/sarcom-kiosk-lab/src/` — must return zero matches.
5. `git grep -n "Clock Invalid" tickets/` — must return zero matches in the five active KIOSK ticket files (the KIOSK-002 deferred stub may legitimately retain its historical mentions; check and don't touch).
6. `git grep -n "ClockInvalid" tickets/` — same.
7. KIOSK-008 ticket no longer lists open questions #4, #7, #8 in the Risks section (they should appear in a new "Decisions pinned" subsection above Risks, citing this STEP-6 prompt as the source).
8. `UX/mockups/KIOSK-008-marker-track.svg` and `.md` do NOT exist on disk.
9. The orchestrator's per-ticket reminder for KIOSK-008 no longer mentions clock-invalid.

## Final output

When done, print:

1. `git status --short` (full status, since edits span both `tools/` and `tickets/`)
2. `git diff --stat` (size summary across all edits)
3. The list of files edited, grouped:
   - Kiosk-lab source files (with line-count delta)
   - Ticket files (one line each, naming the section changed)
   - Mockup-prompt files
   - Orchestrator file
   - Deleted UX/mockups files
4. `cargo check` result (must be PASS)
5. `cargo test` result (must be PASS; one test was removed)
6. `git grep -n clock_valid tools/sarcom-kiosk-lab/src/` output — must be empty
7. `git grep -n ClockInvalid tools/sarcom-kiosk-lab/src/` output — must be empty
8. `git grep -n "Clock Invalid" tickets/` output — must show only KIOSK-002 deferred-stub historical content, if any
9. Confirmation that no files outside the allowed-edit list were touched, and that the four "MUST NOT edit" categories are intact:
   - `decisions/`, `ARCHITECTURE.md`, `README.md`, `CLAUDE.md`, `.claude/` — untouched
   - `UX/mockups/v1a-operator-map-mockup.{svg,md}` + `mockup_1.{svg,png}` — untouched
   - The five existing KIOSK-NNN mockup pairs under `UX/mockups/` (except KIOSK-008 which is deleted) — untouched
   - SPIKE files, KIOSK-002 stub, KIOSK-007 ticket — untouched
10. "Open questions surfaced for Pieter" section listing:
    - Whether the existing KIOSK-001/003/004/005/006 mockup rationale `.md` files contain any "clock invalid" references in their "what a reviewer verifies" sections that survived this cleanup. List them path:line for Pieter to decide whether a follow-up STEP-7 patches them, or whether they're acceptable as historical context (the mockup renders don't depict clock-invalid, so the references would be vestigial text-only).
    - Whether `format_age_or_unavailable` should be renamed to `format_age_or_unset` (or similar) to reflect its narrowed responsibility — minor naming call, surface it.
    - Any other clock-invalid leftover encountered that you couldn't cleanly resolve within the scope fence.

## Next step (after this cleanup)

Re-run `tickets/mockup-prompts/00-RUN-ALL.md` in a fresh CLI session. The orchestrator's idempotency check will find:

- Five existing KIOSK-NNN mockup pairs (001, 003, 004, 005, 006) — SKIP all.
- KIOSK-002 deferred stub — SKIP.
- KIOSK-008 mockup pair — does NOT exist (you deleted them). RE-RENDER with the pinned decisions + two panels (A + B only) + freshness-color consistency.

The KIOSK-008 re-render will produce the only diff in `UX/mockups/`, easy to review against the just-edited mockup-prompt.

Do NOT commit. Leave the working tree dirty for Pieter to review.
