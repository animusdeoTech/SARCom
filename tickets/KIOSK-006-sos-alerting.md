---
id: KIOSK-006
title: "SOS alerting surface — persistent bottom strip"
status: ready-for-review
type: implementation-ticket
opened: 2026-05-18
adr007-variant-dependency: closed — SPIKE-001 closed strict; persistent bottom strip only
---

# KIOSK-006 — SOS alerting surface

## Problem statement

The current SOS treatment is implemented at [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../tools/sarcom-kiosk-lab/src/app.rs): when any tag has `tag.sos == true`, the bottom strip turns wide and red with the text `DISTRESS · {label} · since {wall} · flags.SOS=1 · {age} · read-only · ack at the tag`. A pulsing red ring is also drawn around the SOS tag's marker at [`tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs).

This already honours [`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../decisions/ADR-007-touchscreen-primary-ui.md) — no acknowledgement flow, no dialog, no settings, no CRUD. The line `ack at the tag` correctly tells the operator that acknowledgement happens at the tag, not at the gateway.

Two open items for v1a:

1. The visual hierarchy can be sharpened without adding any surface (pulse / blink on the existing strip, synchronized with the existing marker pulse ring).
2. The operator-facing copy is too verbose and leaks implementation detail (`flags.SOS=1` is operator-irrelevant debug copy).

## User story

*As a SAR operator, I want SOS state to be impossible to miss when the screen is glanced at, with operator-facing copy that reads cleanly without implementation noise.*

## Scope

Per [`SPIKE-001-adr007-informational-overlays.md`](SPIKE-001-adr007-informational-overlays.md)'s closure (strict ADR-007 retained for v1a) and Pieter's v1a UX posture, the SOS surface is **the existing bottom strip only**. No banner, no acknowledge, no dismiss, no modal, no popover.

### Existing bottom strip — strengthened in place

- **Keep the existing bottom red strip** at [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../tools/sarcom-kiosk-lab/src/app.rs) as the SOS surface. **Strip height remains current.** **DISTRESS text size remains current.** Structural changes are not in scope; aesthetic tuning is.
- **Add a subtle pulse / blink on the strip's background**, synchronized — or visually coherent — with the existing marker pulse ring at [`tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs). The strip pulse and the marker pulse should read as the same beat at glance.
- The pulse is *subtle*: the strip remains legible at every phase of the pulse; the operator should not have to wait for a peak to read the text.

### Operator-facing copy (shortened)

Replace the current verbose strip text with:

```
DISTRESS · tag-2 · 42s · ack at tag
```

- Specifically: **remove `flags.SOS=1`** (debug copy, operator-irrelevant).
- **Remove `since {wall}`** (the persistent red strip and the live age already carry the temporal information).
- **Drop `last frame` prefix on the age** — the strip context makes the meaning unambiguous.
- **Keep `ack at tag`** (slightly shortened from `ack at the tag`) — it correctly directs the operator that acknowledgement happens at the tag's physical button per [ADR-010](../decisions/ADR-010-sos-encoding.md).
- Age via `format_age_or_unavailable` at [`tools/sarcom-kiosk-lab/src/ui/mod.rs:20-26`](../tools/sarcom-kiosk-lab/src/ui/mod.rs).
- The strip's existing `read-only` token may be retained or dropped — implementer chooses based on what reads cleanly. The strict-ADR posture is operator-evident from the kiosk's overall shape; the token is not load-bearing.

### Multi-SOS handling

- **v1a shows the most recent SOS only** if multiple tags are simultaneously in SOS state. Do not design stacked rows or a queued-SOS strip for v1a. This matches ADR-014's duty-cycle observation that two simultaneous SOS tags do not fit the 1% sub-band M cap ([`ARCHITECTURE.md:559-595`](../ARCHITECTURE.md) §13) — multi-tag SOS is a v2 concern.
- If a more recent SOS arrives, the strip switches to the new tag. No stacking, no carousel.

### What is explicitly NOT in scope

- **No banner.**
- **No acknowledge button.**
- **No dismiss button.**
- **No modal.**
- **No popover.**
- **No top-anchored surface.**
- **No tap targets on the strip.**

## Non-goals

- **No protocol downlink.** ADR-008 stays closed. No frame sent from gateway to tag.
- **No DB write of acknowledgement state.** No `tag_reports` entry, no future CoT/TAK export of a kiosk-side ack (there isn't one).
- **No "are you sure" confirmation** anywhere in the surface (ADR-007:42).
- **No SMS / no email / no external notification.**
- **No audible alert.** That is [`SPIKE-003-audible-sos-hardware-support.md`](SPIKE-003-audible-sos-hardware-support.md) (blocked).
- **No no-fix uncertainty disc.** That is [`SPIKE-002-nofix-uncertainty-disc-semantics.md`](SPIKE-002-nofix-uncertainty-disc-semantics.md) (closed: not in v1a).
- **No changes to the SOS+NoFix ghost-marker rendering** at [`tools/sarcom-kiosk-lab/src/map/markers.rs:265-302`](../tools/sarcom-kiosk-lab/src/map/markers.rs).
- **No use of the word "modal"** in code, comments, or docs. Use *bottom strip*.
- **No multi-SOS UI** (stacked rows, carousel, queue). Most-recent-only for v1a.

## Acceptance criteria

1. SOS scenario renders the existing bottom red strip at [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../tools/sarcom-kiosk-lab/src/app.rs); **strip height and DISTRESS text size match the current implementation** (no structural change).
2. Strip background carries a **subtle pulse / blink** synchronized — or visually coherent — with the marker pulse ring at [`tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs). The strip remains legible at every phase of the pulse.
3. Strip text is `DISTRESS · {label} · {age} · ack at tag` (age via `format_age` in `tools/sarcom-kiosk-lab/src/ui/mod.rs`). `flags.SOS=1`, `since {wall}`, and the `last frame` prefix on the age are all removed.
4. No banner, no overlay, no acknowledge button, no dismiss button rendered anywhere in the kiosk under any condition.
5. Pulsing red ring around the SOS tag's marker continues to render unchanged; its pulse beat coordinates with the strip's pulse.
6. SOS+NoFix scenario continues to render the ghost marker with red SOS pulse at [`tools/sarcom-kiosk-lab/src/map/markers.rs:279-289`](../tools/sarcom-kiosk-lab/src/map/markers.rs); the strip carries the same shortened copy.
7. If a `MultiTag`-style scenario ever surfaces two simultaneous SOS tags, the strip shows the **most recent** SOS only. No stacking, no carousel.
8. Code comment cites SPIKE-001's closure (strict ADR-007 retained, no overlay / banner / acknowledge surface).

## Manual validation steps

1. `cargo run --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`
2. Switch to `SOS` scenario. Only the existing bottom red strip is present; no other SOS surface. Text matches the shortened format.
3. Observe the strip's subtle pulse and the marker's pulse ring; they read as one coordinated beat.
4. Switch to `SosNoFix`. Ghost marker with red SOS pulse present; strip carries `DISTRESS · {label} · {age} · ack at tag`.
5. Switch to `MultiTag` (one SOS tag among four). Strip shows the SOS tag.
6. (Simulated multi-SOS, if scenarios permit injecting a second SOS) — strip shows the most recent SOS only; no stacking.
7. Confirm no tap targets exist on the strip (no clickable surface).

## Likely files / modules touched

- [`tools/sarcom-kiosk-lab/src/app.rs`](../tools/sarcom-kiosk-lab/src/app.rs):
  - lines 230-282: shorten the strip text, add the subtle pulse / blink animation, add the most-recent-SOS selection logic
  - comment cite of SPIKE-001 closure
- [`tools/sarcom-kiosk-lab/src/map/markers.rs:216-227`](../tools/sarcom-kiosk-lab/src/map/markers.rs) — confirm the marker pulse beat is shareable so the strip pulse can coordinate with it (extract a shared phase if needed)
- [`tools/sarcom-kiosk-lab/src/ui/palette.rs:13`](../tools/sarcom-kiosk-lab/src/ui/palette.rs) — `RED` reused for the strip; no new palette entries required

## Risks / open questions

- **Pulse coordination.** The marker pulse animation timing lives in the markers renderer; sharing the same phase with the strip animation requires either a shared time source (egui's frame time) or an explicit phase parameter. Implementer's choice; both work.
- **Pulse subtlety.** Too strong a pulse drifts toward animated-banner territory and may feel like a modal-substitute. Too weak loses the glance signal. Tune against the v1a operator-map mockup at [`UX/mockups/v1a-operator-map-mockup.md`](../UX/mockups/v1a-operator-map-mockup.md), and document the chosen duty-cycle (e.g. 70%–100% opacity, 1.2 s period) in the per-ticket mockup `UX/mockups/KIOSK-006-sos-strip.{svg,md}`.
- **Token retention (`read-only`).** The strict-ADR posture is evident from the kiosk's overall shape (no buttons, no modals). The `read-only` token in the strip text is not load-bearing; implementer drops if the line is too dense, retains if it reads cleanly.
- **`ack at tag` wording.** The shortened form drops "the" for compactness. If operator-feedback objects, expand back to `ack at the tag` — both are honest and ADR-aligned.

## Confidence

**High.** Implementation is small: shorten copy in one branch, add a subtle pulse, ensure most-recent selection. No new surfaces, no variant decisions pending.

## Dependencies

- **SPIKE-001 closed strict** — no variant decision is pending; persistent bottom strip is the only design.
- **Soft cross-reference** to [`SPIKE-003-audible-sos-hardware-support.md`](SPIKE-003-audible-sos-hardware-support.md) — audible SOS is a separate concern (blocked on substrate).
- **Soft cross-reference** to [`spikes/ble-gateway-ui-flow-spike.md`](../spikes/ble-gateway-ui-flow-spike.md) — commissioning flow precedence during SOS (commissioning cannot open during active SOS per [`spikes/ble-gateway-ui-flow-spike.md:171`](../spikes/ble-gateway-ui-flow-spike.md)).
