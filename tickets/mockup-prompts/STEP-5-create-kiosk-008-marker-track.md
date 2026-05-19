# Create KIOSK-008 — Map marker + track rendering ticket + mockup prompt

> **STATUS: completed 2026-05-19.** Historical work-instruction artifact. References to `TagData` / `Selection::Tag(_)` / `data.rs:142` are pre-collapse names; the post-collapse types live in `NodeData` (`data.rs:134` for `track`) and `Selection::Node(_)`. See [`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md). Current sources of truth: [`tickets/KIOSK-008-marker-track-rendering.md`](../KIOSK-008-marker-track-rendering.md) (post-rename) and [`tickets/mockup-prompts/KIOSK-008-mockup-prompt.md`](KIOSK-008-mockup-prompt.md) (post-rename). Do not re-run this STEP prompt.

You are creating a new implementation ticket and its mockup prompt for
**map marker + track rendering** in the SARCOM kiosk. This is split out
of KIOSK-001 (which owns map chrome only — scale bar + compass rose).

Pieter has decided the track-history pattern: **B + D**.

- **B (baseline):** every tag with `gps_valid=true` shows its last three fixes on the map as small dots with fade-out (newest brightest, oldest dimmest), so the operator sees direction and relative speed at glance without selecting.
- **D (selection):** tapping a tag → the full track polyline for that tag becomes visible on the map until selection clears.

No-fix tags keep their existing ghost-marker treatment (point at `last_valid_fix_pos` + dashed ring per `markers.rs:265-302`); they do NOT carry a fix-tail. SPIKE-002 closed reject — no uncertainty disc.

## Read first

- `.claude/skills/sarcom-ux/SKILL.md` and `.claude/skills/sarcom-svg-wireframe/SKILL.md` — post-cleanup posture
- `tickets/README.md` lines 14-33 (v1a UX posture), 45-46 (spike closures), 52-58 (per-ticket scopes), 77-81 (per-ticket mockup canonical paths)
- `tools/sarcom-kiosk-lab/src/data.rs:124-150` — `TagData` including `track: Vec<[f32; 2]>` (line 142), `last_valid_fix_pos` (line 146), `last_valid_fix_age_secs` (line 149)
- `tools/sarcom-kiosk-lab/src/map/markers.rs` — current marker rendering, especially:
  - `tag_visible_pos` helper at lines 32-38
  - Tag dot rendering at lines 194-262
  - SOS pulse ring at lines 216-227
  - Selection-outline at lines 231-237
  - No-fix ghost at lines 265-302
- `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-180` — full three-layer render stack + marker draw loop (read in full, not just :82-125)
- `tools/sarcom-kiosk-lab/src/map/fake_grid.rs` — legacy track-line rendering, if any (read for context to confirm what's actually shipped today vs what the umbrella mockup invented)
- `tools/sarcom-kiosk-lab/src/ui/palette.rs` — palette constants
- `tickets/KIOSK-001-map-scale-north.md` — sibling ticket; KIOSK-008 splits marker/track work out
- `tickets/KIOSK-003-sidebar-row-redesign.md` — provides the `Selection` enum that KIOSK-008's selection-track consumes
- `tickets/KIOSK-004-selection-detail-panel.md` — sibling ticket; KIOSK-004 owns the sidebar-replacement detail surface, KIOSK-008 owns the map-render side of the same selection
- `decisions/ADR-007-touchscreen-primary-ui.md:38-46` — strict UI invariant (the on-map track is not an overlay; ADR-007 is not in tension)
- `UX/mockups/v1a-operator-map-mockup.svg` — historical reference; the polyline track shown for tag-1 there was mockup-only invention, not shipped behaviour

## Halt conditions

HALT if any are true:

1. The `sarcom-ux` skill still carries Variant A/B framing or lists floating map buttons as primary chrome → STEP-2 cleanup not run.
2. The `sarcom-svg-wireframe` skill still requires `Variant A` as a mandatory `<text>` label, OR the validator unconditionally fails on missing `Variant A` → STEP-1 cleanup not run.
3. Any per-prompt file under `tickets/mockup-prompts/` still carries Variant A/B framing or crosshair/coord-readout references → STEP-3 cleanup not run.
4. The five active mockup-prompts under `tickets/mockup-prompts/` do not yet include the three-layer basemap composition citing `pmtiles_map.rs:82-125` → STEP-4 cleanup not run.

If any halt condition trips, report which and stop.

## Hard constraints on your edits

You MAY:

- CREATE `tickets/KIOSK-008-marker-track-rendering.md` (new ticket)
- CREATE `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md` (new mockup prompt)
- EDIT `tickets/README.md` — add KIOSK-008 to the per-ticket scope list and the per-ticket mockup canonical paths list
- EDIT `tickets/mockup-prompts/README.md` — add KIOSK-008 to the prompt-files list
- EDIT `tickets/mockup-prompts/00-RUN-ALL.md` — add KIOSK-008 to the execution plan table AND to the allowed-writes whitelist

You MUST NOT:

- Edit any other `tickets/KIOSK-NNN-*.md` or `tickets/SPIKE-NNN-*.md`. KIOSK-008 stands on its own; do not retro-amend the others.
- Edit anything under `decisions/`, `ARCHITECTURE.md`, `README.md` (root), `CLAUDE.md`, `.claude/`, `tools/`, `spikes/`, `dev-log/`, `UX/mockups/`.
- Generate the mockup SVG. That is the orchestrator's job after Pieter approves the prompt.
- Create a git worktree.
- Commit anything.
- Invent design decisions Pieter hasn't approved. Where the ticket needs a concrete value Pieter hasn't decided yet, **surface it as an open question** in the ticket's "Risks / open questions" section instead of locking a number.

## Required content — `tickets/KIOSK-008-marker-track-rendering.md`

Frontmatter:

```yaml
---
id: KIOSK-008
title: "Map marker + track rendering"
status: ready-for-review
type: implementation-ticket
opened: 2026-05-19
adr007-variant-dependency: none
---
```

Sections, in this order:

### Problem statement

The kiosk's PMTiles render path currently draws tag dots (filled circles per state colour) and the no-fix ghost marker, but **does not render any track history** beyond what the legacy `MapMode::FakeGrid` may have shown. Tag data carries `track: Vec<[f32; 2]>` per `tools/sarcom-kiosk-lab/src/data.rs:142`, but nothing in `pmtiles_map.rs` consumes it.

For SAR operator-need ("hoe gaat het met de mensen waar ik verantwoordelijk voor ben? wie is er allemaal en waar zijn ze?" per `tickets/README.md` v1a posture), a single dot per tag answers "where are they right now" but not "in welke richting bewegen ze en hoe snel." A small fix-tail per tag answers both at glance. A full track on demand (via selection) lets the operator confirm route history when investigating a specific person.

This ticket is **ADR-007-independent**. Track rendering is on-map content, not an overlay surface; no modal/popover/banner tension.

### User story

*As a SAR operator at the gateway, I want to see at glance where each tag is, in which direction it's moving, and roughly how fast — and I want the option to see the full route history for a specific person without leaving the map.*

### Scope

**Baseline track rendering (always on, every tag with `gps_valid=true`):**

- Each tag renders its current dot **plus its three most recent fixes as small dots with fade-out**.
- Fade-out: newest fix at full tag colour, older fixes dimmer (concrete opacity values are an open question — see Risks).
- Three-fix tail size is smaller than the current-position dot.
- A thin connector line between the fixes is acceptable but optional (implementer chooses based on at-2m-glance readability — see open question).
- When the tag has fewer than three fixes in `track`, render whatever exists; no padding, no placeholder.
- Tails are rendered **inside the same `Map::show` closure** as the current marker draw at `pmtiles_map.rs:127+`, after the three basemap layers and before the current-position dots, so the current dot always paints on top of its own tail.

**Selection-driven full track (consumes `Selection::Tag(usize)` from KIOSK-003):**

- When a tag is selected, render the **full track polyline** for that tag connecting every fix in `TagData.track`.
- Style: matched to tag colour, opacity 0.5, slightly thicker stroke than the baseline tail.
- The selected tag's baseline three-fix tail is **replaced** by the full polyline for the duration of the selection (no double-render).
- Other tags continue to render their three-fix tail unchanged.
- On selection clear (`Selection::None`), the full polyline disappears; the three-fix tail returns.

**Per-state behaviour:**

| Tag state | Current dot | Three-fix tail | Full track on selection |
|---|---|---|---|
| Normal (`gps_valid=true`, fresh) | yes, BLUE | yes, BLUE fade | yes |
| SOS (`sos=true`, `gps_valid=true`) | yes, RED + SOS pulse ring per `markers.rs:216-227` | yes, RED fade — **OPEN QUESTION** (see Risks) | yes |
| Stale (`gps_valid=true`, last_seen > 660s) | yes, GREY/dim per current behaviour | yes, GREY fade | yes (the existing track is what it is) |
| No-fix (`gps_valid=false`) | ghost at `last_valid_fix_pos` per `markers.rs:265-302` | **no tail** — ghost is the entire indicator; track to before last-valid-fix would mislead | yes for the historical fixes that DO exist (open question: does a no-fix tag's selection show the polyline up to the last valid fix?) |
| Clock invalid (`clock_valid=false`) | suppress entire track rendering (consistent with `format_age_or_unavailable` discipline at `tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26`) | n/a | n/a |

**Markers:** the current marker rendering (relay orange cross, gateway green square outline, tag colour-coded circles per state, no-fix ghost dashed ring) is **preserved as-is**. This ticket only adds track-tail + selection-polyline; it does not redesign existing markers.

### Non-goals

- No track for relay / gateway. Only tags.
- No track-history pruning logic in this ticket. Whatever ends up in `TagData.track` is what gets rendered. (Pruning, retention policy, etc. are out of scope.)
- No animation. Static render. The newest-to-oldest fade is positional, not temporal.
- No "follow the latest track-point" camera mode. Selection still does the one-shot pan per `KIOSK-004:36`.
- No track-aware annotations on the map (no labels like "moving south at 1.2 m/s"). Direction + speed are visual-inference only.
- No protocol or data-model changes. Uses existing `TagData.track` field at `data.rs:142`.
- No uncertainty disc on no-fix tags. SPIKE-002 closed reject per `tickets/README.md:46`.
- No tail or polyline coloured by age relative to the wall clock — the fade is positional ordering only, not time-derived.

### Acceptance criteria

1. PMTiles render path draws three-fix baseline tail for every tag with `gps_valid=true` and ≥1 fix in `track`.
2. Tail fades from current-fix colour (full opacity) to oldest visible fix (open-question opacity values).
3. Current-position dot always paints on top of its own tail.
4. Selecting a tag (`Selection::Tag(_)` from KIOSK-003) replaces that tag's three-fix tail with the full polyline through `TagData.track`.
5. Other tags retain their three-fix tail when one tag is selected.
6. Deselecting returns the selected tag to baseline three-fix tail behaviour.
7. No-fix tags render the ghost marker only (no baseline tail). Selection of a no-fix tag's behaviour resolved per Risks open question.
8. SOS tag tail behaviour resolved per Risks open question.
9. Clock-invalid scenario suppresses all track rendering (current dots still render per existing behaviour).
10. No regression in current marker rendering at `markers.rs:194-302`.
11. Existing tests pass; new tests cover the baseline-tail + selection-polyline branches.

### Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Default `MultiTag` scenario: tag-1 (normal) shows a current dot plus three smaller dots trailing in its movement direction.
3. Tap tag-1 in the sidebar: sidebar replaces with detail (KIOSK-004 behaviour), and on the map the three-fix tail is replaced by a full polyline connecting every fix in `track`.
4. Tap another tag (or back-to-list): the polyline for tag-1 disappears; the three-fix tail returns.
5. Switch to `Clock Invalid`: all tails and polylines suppressed; current dots remain.
6. Switch to `SosNoFix`: no-fix tag renders ghost only (no tail), per Risks resolution.
7. Switch to `Stale`: stale tag shows GREY-dim tail.
8. `cargo test` passes.

### Likely files / modules touched

- `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs` — primary site; new track-tail + selection-polyline draw inside the `Map::show` closure after the basemap layers and before / interleaved with the current marker draw at lines 127-262.
- `tools/sarcom-kiosk-lab/src/map/markers.rs` — possibly extract a `draw_tag_tail` and `draw_tag_full_track` helper for reuse; current marker logic at lines 194-262 stays intact.
- `tools/sarcom-kiosk-lab/src/ui/palette.rs` — possibly add a `TAIL_FADE_*` helper if interpolating opacities cleanly is awkward inline; otherwise reuse existing tag colours with multiplied opacity.
- New test file likely required for the baseline-tail and selection-polyline rendering paths.

### Risks / open questions

(Surface these in the ticket explicitly. Do NOT pre-fill — Pieter decides.)

- **Fade opacity values for the three-fix tail.** Concrete numbers — e.g. newest 1.0, middle 0.6, oldest 0.3, or some other curve. Implementer-choice within Pieter's intent? Or pin a specific triple now?
- **Tail-dot size.** Current dot is ~7-8px. Tail dots should be smaller — 3px? 4px? Visually distinct from current dot at 2m glance.
- **Connector line between tail dots.** Faint line connecting the three-fix tail dots, or pure dots only? Trade-off: line makes direction more obvious; dots-only is cleaner at high tag density.
- **SOS-state tail.** Does a tag in SOS state still render its three-fix tail? Arguments for: shows recent movement context which may be operationally useful. Arguments against: the SOS pulse ring is already attention-grabbing; an extra red trail may visually compete.
- **No-fix tag selection.** When a no-fix tag is selected, does the full polyline render for the historical fixes that DO exist (everything up to `last_valid_fix_pos`)? Or stays ghost-only?
- **Selection polyline thickness.** "Slightly thicker than baseline tail" — concrete `stroke-width` value? Currently undefined.
- **Selection polyline colour.** Match tag colour (preserves identity) or use a neutral colour (e.g. TEXT_DIM) to indicate "this is historical, not state"?

### Confidence

**Medium.** The baseline tail is mechanically straightforward (loop over last three fixes, render with decreasing opacity). The selection-polyline read of the `Selection` enum is a small extension to the current marker draw. The load-bearing risks are the open-question design values; once Pieter pins them, implementation is small.

### Dependencies

- **Depends on** `tickets/KIOSK-003-sidebar-row-redesign.md` — provides the `Selection` enum that selection-polyline consumes.
- **Soft sibling of** `tickets/KIOSK-004-selection-detail-panel.md` — KIOSK-004 owns the sidebar-replacement detail surface; KIOSK-008 owns the map-render side of the same selection. Both consume `Selection`.
- **No dependency on** `tickets/KIOSK-001-map-scale-north.md` — can land in either order; chrome and tracks are independent.
- **SPIKE-001 closed strict** — track-on-map is not an overlay; no ADR-007 tension.
- **SPIKE-002 closed reject** — no uncertainty disc; ghost marker stays as point + dashed ring.

## Required content — `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md`

Standard SARCOM mockup-prompt structure (same shape as the other five). Specifically:

- `## Read first` block citing CLAUDE.md, ADR-007:38-46, `pmtiles_map.rs:82-180`, `markers.rs` (full), `data.rs:131-150`, `palette.rs`, the KIOSK-008 ticket, the three-layer-basemap dev-log, and KIOSK-001 sibling.
- `## Hard constraints` mirroring the ticket's scope: single design, strict ADR-007, three-layer basemap composition (the standard block + SVG fidelity guidance per STEP-4), no floating buttons, lab fixture 800×480 annotated ADR-015-pending, palette-only from `palette.rs`.
- `## The design to render`:
  - **Three stacked 800×480 panels** so the baseline + selection contrast is visible.
  - **Panel A** — `MultiTag` scenario, no selection. Show tag-1 with current-dot + three-fix tail (newest brightest), tag-2 SOS with pulse-ring + tail-per-Pieter's-decision, tag-3 no-fix ghost (no tail), tag-4 stale with grey-dim tail. Relay-1, gw-0 unchanged.
  - **Panel B** — same `MultiTag` scenario, tag-1 selected. Tag-1's three-fix tail is replaced by a full polyline through all its `track` fixes; other tags retain their three-fix tails.
  - **Panel C** — `ClockInvalid` scenario. All tails and polylines suppressed; current dots only. Sub-annotation noting the suppression.
- Open questions section: include the same list from the ticket Risks (fade values, dot size, connector line, SOS tail, no-fix selection, polyline thickness, polyline colour) so the mockup-maker surfaces them too.
- Standard `## Annotation requirements`, `## Rationale markdown content`, `## Non-goals`.
- Canonical output paths: `UX/mockups/KIOSK-008-marker-track.svg` + `.md`.

## Required edits — `tickets/README.md`

Add KIOSK-008 to:

1. The implementation-tickets bullet list (after KIOSK-007 or in a logical position). Suggested line: `` - [`KIOSK-008-marker-track-rendering.md`](KIOSK-008-marker-track-rendering.md) — map marker rendering + three-fix tail per tag (baseline) + full track polyline on selection ``
2. The per-ticket mockup paths list (`tickets/README.md:77-81`). Add: `` - `UX/mockups/KIOSK-008-marker-track.svg` + `.md` ``
3. The "Recommended review order" list if it makes sense in context (KIOSK-008 logically reviews after KIOSK-003 and alongside KIOSK-004).

## Required edits — `tickets/mockup-prompts/README.md`

Add KIOSK-008 to the prompt-files list. Order with the other KIOSK-NNN prompts.

## Required edits — `tickets/mockup-prompts/00-RUN-ALL.md`

Add KIOSK-008 to:

1. The **Execution plan** table — new row after KIOSK-006.
   - Prompt: `tickets/mockup-prompts/KIOSK-008-mockup-prompt.md`
   - Canonical SVG: `UX/mockups/KIOSK-008-marker-track.svg`
   - Canonical md: `UX/mockups/KIOSK-008-marker-track.md`
2. The **Allowed writes whitelist** — add the two new canonical paths.
3. The **Per-ticket reminders** — short paragraph for KIOSK-008 covering: baseline three-fix tail per tag, selection-driven full polyline, no tail for no-fix tags, suppress all tracks under clock-invalid, no relay/gateway tracks, palette-from-palette.rs only.

## Final output

When done, print:

1. `git status --short tickets/`
2. List of created files (the two new files)
3. List of edited files (the three READMEs / orchestrator)
4. Confirmation no files outside the allowed-edit list were touched
5. Any low-confidence assumption (do NOT silently resolve; surface here)
6. A short "what to do next" line: `Pieter reviews KIOSK-008 ticket + mockup-prompt. If approved, run 00-RUN-ALL.md in a fresh CLI session — idempotency check will skip the five existing mockups and only render KIOSK-008.`

Do NOT commit. Leave working tree dirty for Pieter to review.
