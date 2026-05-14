---
title: "SARCom CAD-relevant document map (reference compilation)"
status: living
type: reference
source-sessions:
  - dev-log/2026-05-13-gateway-v1-cad-session-risks.md
  - dev-log/2026-05-14-c1-depth-stackup-arithmetic.md
  - dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md
  - dev-log/2026-05-14-anker-dims-and-gate-propagation.md
  - dev-log/2026-05-14-cad-day-retrospective.md
  - retrospectives/2026-05-14-design-decisions.md
  - retrospectives/2026-05-14-meta-retro-missing-angles.md
---

# SARCom CAD-relevant document map

Descriptive reference compilation: every artefact that shapes SARCom enclosure CAD work, with each document's role + what was learned about its strengths and gaps during the 2026-05-13/14 sessions. This is the reference companion to `docs/cad-skill-reference.md` (which is prescriptive — what to DO; this file is descriptive — what exists).

The glossary Pieter wrote on 2026-05-14 organises the SARCom CAD-relevant documents into six categories. Below, each document gets a per-doc analysis: role, what was learned today, gaps to know about, how to use it.

## Spike-closes — primary sources-of-truth

### `spikes/gateway-handheld-enclosure-spike.md`

**Role:** THE primary mechanical spec for the gateway enclosure. IP65 verdict, ASA material commitment, Buna-N 2 mm cord-stock gasket, 3 mm polycarbonate display window, threaded SMA + IP67 power button bulkheads, Gore PolyVent rear membrane, internal layout, passive heat-spreader path, drop tolerance. Every CAD feature in Fusion traces back here directly or transitively.

**Learned 2026-05-14:**

- The §Form factor depth field gave "~45–55 mm" — a hand-wave estimate, not stack-up arithmetic. Corrected to "~85–100 mm" via amendment on 2026-05-14 after the C1 depth dev-log produced cited per-row math.
- The Internal layout text consumed "~155 × 60 × 30 mm" as the bank envelope; this was wrong (real Anker A1689 is 119.9 × 73.4 × 31.4 mm). Corrected to the verified spec on 2026-05-14.
- Bulkhead inventory included a magnetic-pogo charging connector. **Retired** in the 2026-05-14 amendment after Pieter dropped the in-shell charging concept.
- Battery service door was "OPTIONAL" in the 2026-05-08 verdict. **Promoted to mandatory** in the 2026-05-14 amendment when it became the only regular access path.
- Bank orientation in the rear compartment was **implicit** (long axis assumed along device X) — made explicit in the 2026-05-14 amendment §Internal layout.
- Two file-level amendments landed on 2026-05-14 with supersession headers (depth correction + pogo retirement); §Closed and §Decision text amended inline with [SUPERSEDED 2026-05-14] / [CORRECTED 2026-05-14] markers.

**Known gaps that survived 2026-05-14:**

- Heat-spreader thermal path crosses the divider plate but the 3D topology question ("how does the SoC heat-spreader block reach the AlMg3 plate in the rear compartment?") is not resolved geometrically. Documented in the C1 depth dev-log §C1.4.
- Pi-on-Touch-Display-2 mounting standoff height is not specified (Raspberry Pi docs don't publish it). Stack-up math has it as a hand-wave (6-15 mm range depending on Pi orientation).
- Front-shell / rear-shell outer-envelope sketch-source convention is ambiguous: front extruded from full 180×120 envelope (X=±90), rear from gasket-offset inner profile (X=±88.5), creating an unintended 1.5 mm step at the parting plane.

**How to use:**

Read first when starting any enclosure CAD work. The §Decision note code block is the authoritative spec; the prose above it is summary. Always check for the **top-of-file partial supersession** sections (these override §Decision text). When in conflict between prose and §Decision, §Decision wins; when in conflict between §Decision and a dated top-of-file supersession, the supersession wins.

### `spikes/gateway-handheld-substrate-spike.md`

**Role:** Defines WHAT the enclosure has to house. Pi 5 + Dragino LoRa GPS HAT + Pi Touch Display 2 (7", 22-pin DSI) + USB-C-PD power path. Sets the inwendige bounding box for the enclosure. Active cooler **rejected** here — passive heat-spreader is the mechanical consequence.

**Learned 2026-05-14:**

- The "HAT Z-stack ~25-30 mm above Pi PCB" claim was correct but was **not propagated** to the enclosure spike's depth-estimate. Two adjacent spikes opened together but didn't cross-consume their numbers properly. This is the proximate cause of the 45-55 mm depth target being unachievable.
- Pi 5 was selected as substrate, but the choice of Pi 5 RAM variant + the broader substrate-decision was being empirically tested separately (4 GB / 8 GB / 16 GB variants).
- Active cooler rejection means **no fan, no cooling vent** — the Gore PolyVent (rear shell, pressure equalisation) serves a different purpose than convective cooling. Easy to misread.

**Known gaps:**

- Component-level depth contributions are listed but not summed into a verified front-stack arithmetic. Substrate spike says "HAT Z-stack ~25-30 mm" but enclosure spike says "depth 45-55 mm" — the cross-spike consistency check was never run.

**How to use:**

Consult when adding internal mounting features (Pi mounting bosses, HAT mounting standoffs, display attachment). The substrate spike defines the components; the enclosure spike defines how they fit. When in conflict on a component-level dimension, the substrate spike's component-spec wins (it's closer to the source).

### `spikes/gateway-handheld-power-architecture-spike.md`

**Role:** Battery, charging, signal contract, runtime envelope. Defines the Anker A1689 envelope (originally wrong, corrected 2026-05-14), the service door contract, the firmware signal surface (POWER_GOOD retired 2026-05-14), and the charging path (magnetic-pogo retired 2026-05-14).

**Learned 2026-05-14:**

- The Anker A1689 dimensions in this spike were **never fact-checked** against Anker's product page before the 2026-05-08 verdict. The actual spec (119.9 × 73.4 × 31.4 mm) differs from the spike's "154 × 62 × 30 mm" in length by 34 mm and width by 11 mm — large enough to invalidate the battery-door aperture sizing and to surface 30 mm of rear-compartment slack.
- The signal contract for POWER_GOOD / BATTERY_STATE / CHARGE_STATE assumed an in-shell charging input. Pogo drop retired all three from the firmware surface. SHUTDOWN_REQUEST survives (Pi 5 reads VBUS-droop on its own USB-C input).
- The §Service block was rewritten: the optional battery-service door is now mandatory, and the main-clamshell-open path for battery swap is explicitly retired for routine service.

**Known gaps:**

- Capacity / wattage / Wh figures (25600 mAh / 87 W / ~95 Wh) inherited from the 2026-05-08 hand-wave; not yet verified against Anker's product spec at the time of writing this reference doc.
- Cold-charge cutoff is an **operational caveat** ("do not charge below 0°C") because the commercial bank exposes no NTC; this caveat is operator-facing, not firmware-enforced.

**How to use:**

Read when designing battery service features, bulkhead inventory items, or anything touching the power signal contract. Specifically, when adding the battery service door, refer to this spike's §Service block + the 2026-05-14 amendment for mandatory-door promotion.

## ADRs — formal constraints on the enclosure

### `decisions/ADR-007-touchscreen-primary-ui.md`

**Role:** Touchscreen is the only UI. Forces the polycarbonate display window to be capacitive-touch-passable. The 3 mm PC choice in the enclosure spike-close needs verification during the print-1-test-1 prototype.

**Learned 2026-05-14:**

- ADR-007 is preserved literally — the polycarbonate window is an input-passthrough surface; the touchscreen below remains the only UI. No physical input affordances on the shell that compete with the touchscreen (the IP67 power button is power-only, not a UI element).
- The capacitive-touch-through-PC claim is **not yet validated**. It's an open verification item for the first physical print.

**How to use:**

Cite when designing the display window + bezel assembly. When considering future v2 features (Steam Deck side buttons, etc.), this ADR is the gate that decides whether the addition complies with the read-only kiosk constraint.

### `decisions/ADR-011-gateway-time-source.md`

**Role:** DS3231 RTC primary, GPS opportunistic. Negates the need for an external GPS antenna bulkhead — the L80-M39 patch antenna fires through the ASA shell back/top.

**Learned 2026-05-14:**

- One bulkhead less in the inventory (no GPS SMA) is a direct consequence of this ADR. If field-test shows the patch antenna cannot fix through 3 mm ASA, an external GPS SMA becomes a v2 amendment — but it's not a v1 blocker.

**How to use:**

Cite when bulkhead inventory is being discussed; specifically when someone asks "why no GPS antenna bulkhead?" the answer is ADR-011 (RTC primary, GPS opportunistic).

## Dev-logs — chronological session journals

### `dev-log/2026-05-13-gateway-v1-cad-session-risks.md`

**Role:** Yesterday's CAD session risk register after the first Fusion sessie (22 user params, sketches, no extrudes). Source of the C1 depth problem, the gasket groove offset risk (G2), the Option B boss layout asymmetry, and the open question on substrate empirical testing.

**Learned 2026-05-14:**

- The "active-cooler-stack-equivalent" phrase in §C1 was misleading — it does NOT mean an active cooler was used in the design (the spike-close commits to passive). The phrase was a height-envelope comparison, not a design statement. Amended 2026-05-14 to explicitly name the passive heat-spreader stack.
- The 2026-05-08 spike-closes were flagged here as "closed on a small chat-Q&A round, not on the slow research process documented in `docs/spike-rules.md`." Today's findings confirmed this and traced multiple cascading errors back to those closes.

**How to use:**

The risk register format is a model for end-of-CAD-session capture. Use it as template when wrapping up a Fusion session: list each open risk + its location + how it was discovered + how to resolve.

### `dev-log/2026-05-14-c1-depth-stackup-arithmetic.md`

**Role:** First-principles stack-up arithmetic with cited per-row datasheet dimensions. Corrected the enclosure depth spec from "~45-55 mm" to "~85-100 mm". Established the morning's discipline of "every dimension cited or flagged as hand-wave."

**Learned 2026-05-14:**

- The per-row table format with explicit `Source` column ("spike-close text", "datasheet URL", or **HAND-WAVE**) is highly effective for surfacing where the spec hand-waves. Replicate this format when correcting any future numeric spec.
- The C1.4 finding (heat path crosses divider) was surfaced here as a meta-observation, not the primary subject — but it became one of the most-important carry-over blockers for the next CAD session.

**How to use:**

When a depth/footprint/volume number in any spike-close looks suspicious, write a stack-up table in the same format. Cite each row. The act of writing the table is the verification.

### `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`

**Role:** Records Pieter's mid-day decision to drop magnetic-pogo charging, the ADR-016 gate question and its (b) answer, and the first 3D shell extrudes. Bulkhead inventory shortened, depth params bumped, supersession sections landed in two spike-closes.

**Learned 2026-05-14:**

- The HALT-and-ask pattern for an ADR-relevant decision worked well: present the question with three options, wait for written answer, capture the answer + reasoning in the dev-log, then propagate. Replicate.
- The body-organisation cleanup (front-shell-body in reference-sketches needed to move to front-shell) was a Fusion API consequence that the spike-close didn't anticipate. Future CAD work should put each body's source sketch in the same component as the target body lives.

**How to use:**

When a v1 feature is being retired or fundamentally re-scoped, this dev-log is the model: state the decision, list alternatives considered, capture the trade-off, propagate to all consuming docs in one pass.

### `dev-log/2026-05-14-anker-dims-and-gate-propagation.md`

**Role:** Anker A1689 dimension verification against the vendor page, gate-language propagation to 5 more files, battery door rebuild with gasket groove restored, heat-spreader pocket cut (audit C4 fix), end-of-day state snapshot, plus the Autodesk Assistant audit-filter table.

**Learned 2026-05-14:**

- The audit-filter table format is the right way to handle bot-generated audit reports: each finding gets a verdict (REAL, STALE, HALLUCINATION, VERIFIED-OK). Audit reports are not authority — they're noisy input that needs filtering against current spec + live geometry. Save this filter pattern for future audit-bot interactions.
- The grep-then-edit propagation pass (find all occurrences of an old spec value via `Grep`, then `Edit` each in turn with marker comments) is reliable when the change is well-scoped. Use this for any cross-doc dimension or terminology correction.

**How to use:**

When a vendor spec, dimension, or canonical term needs to be corrected across the doc set, use this dev-log as a procedural model:
1. Verify the new value at an authoritative source (datasheet URL, vendor product page).
2. Grep for the old value across the entire repo.
3. Filter out historical dev-logs (preserve as-is) and explanatory amendment headers (intentionally quote the old value).
4. Edit each remaining occurrence with an inline `[CORRECTED yyyy-mm-dd — source URL]` marker.
5. Cite the verification source in the spike-close param-comment if it's a CAD parameter.

### `dev-log/2026-05-14-cad-day-retrospective.md`

**Role:** Retrospective covering everything that went unexpectedly wrong during the 2026-05-14 CAD day. 9 documentation gaps (G1-G9), 7 theoretical design blind spots (O1-O7), 13 execution / Fusion API surprises (F1-F13), three meta-patterns (P1-P3), and a 10-point checklist for next CAD session.

**Learned 2026-05-14:**

- The "what went unexpectedly wrong" format is high-leverage. It surfaces lessons that get buried in the chronological dev-logs. **Repeat this retrospective format at the end of any multi-session CAD day.**
- The Fusion API gotchas section (F1-F13) is the most reusable part — it's the closest thing to a Fusion API gotchas catalogue this project has.

**How to use:**

Read this BEFORE starting a new CAD session — the checklist at the bottom prevents repeating today's mistakes. The F-section is a debug reference when Fusion behaves unexpectedly.

## Constraining context

### `production-concerns.md` §3 (IPEX strain relief)

**Role:** Strain relief is mandatory at the IPEX end of the LoRa pigtail that feeds the SMA bulkhead. Direct consequence in enclosure spike: "printed clamp or hot-glue spot" — a feature that still needs to be extruded in the internal-feature pass.

**Learned 2026-05-14:**

- Not yet implemented in Fusion. Belongs to the internal-feature pass (gated on Orientation X vs Y decision).

**How to use:**

Reference when adding the LoRa SMA bulkhead's pigtail-routing internal features. Strain relief is functionally a clamp body or a glued attachment point; spec-language flexibility is intentional but the function (no IPEX-end stress under cable strain) is mandatory.

### `CLAUDE.md`

**Role:** Project values + the "Do NOT re-open" list. Not a mechanical spec; the **filter** through which every design choice passes.

**Learned 2026-05-14:**

- Today, CLAUDE.md was explicitly invoked to block the custom-PCB pivot (retrospective design decision #1). "physical plug-and-play, quality > speed, hates fastest-time-to-market shortcuts" is the precise text that justified accepting the 100 mm depth instead of redesigning.
- The "Do NOT re-open" list is enforceable — past Pieter explicitly closed those doors, and any future Claude trying to re-litigate is contradicting committed decisions. The 2026-05-14 amendments (pogo drop, gate re-scope) were NOT re-opening any closed door; they were within the open decision surface of the pivot.

**How to use:**

Read at the start of any session as a context primer. When a design alternative seems attractive but contradicts CLAUDE.md's values, name the contradiction explicitly and let Pieter override consciously — don't silently re-frame.

## External datasheets — vendor specs, fetched + cited

### Raspberry Pi Touch Display 2

**Sources:**
- Product brief PDF: `datasheets.raspberrypi.com/display/touch-display-2-product-brief.pdf`
- Docs page: `raspberrypi.com/documentation/accessories/touch-display-2.html`

**Verified dimensions:**
- Depth: **15 mm** (NOT 8.55 mm as initial WebSearch reported — direct PDF fetch corrected this)
- Outline: 189.32 × 120.24 mm
- M2.5 mounting boss instruction: "Align the four corner stand-offs of your Raspberry Pi with the four mounting points on the back of the Touch Display 2"

**Learned 2026-05-14:**

- WebSearch initial answer for Pi Touch Display 2 thickness gave 8.55 mm — wrong. Direct fetch of the official PDF gave 15 mm. **Lesson: WebSearch answers for component dimensions are not authoritative; always cross-check against the vendor PDF / docs page directly.**
- The mounting boss height is the missing variable in stack-up math — Pi docs say "align" but give no number. This is a real-world measurement that has to wait for hardware.

**How to use:**

Cite the docs page URL in user-param comments when this display is part of any dimension chain. Verify against the official source, not WebSearch snippets.

### Dragino LoRa GPS HAT

**Source:** `dragino.com/downloads/downloads/LoRa-GPS-HAT/LoRa_GPS_HAT_UserManual_v1.0.pdf` §1.8

**Verified dimensions:** **60 × 53 × 25 mm**

**Learned 2026-05-14:**

- The 25 mm HAT envelope was THE missing variable in the original 45-55 mm depth hand-wave. Adding it back to the stack-up arithmetic produces the corrected 85-100 mm range. **Single-source omission cascades into entire-spec error.**

**How to use:**

When designing the Pi-on-display + HAT stack, count the 25 mm Dragino HAT envelope as a contributor to front-compartment depth. Always cite the v1.0 manual §1.8 URL in the user-param comment.

### Anker A1689 product page

**Source:** `anker.com/eu-en/products/a1689`

**Verified dimensions:** **119.9 × 73.4 × 31.4 mm**

**Learned 2026-05-14:**

- The pre-correction spec ("154 × 62 × 30 mm") was raw-memory placeholder from the 2026-05-08 spike-close, never fact-checked. The 34 mm error in length and 11 mm in width drove the rear-compartment slack discovery (30 mm X-axis slack) and the battery-door aperture resize (72×32 → 78×36 mm).
- **Lesson: vendor-cited dimensions in any spike-close must be verified against the product page at close time.** "Working candidate Anker A1689" without dimension verification is candidate-vermelding, not a verified input for downstream geometry.

**How to use:**

Cite the Anker EU product page URL in any param-comment that consumes this bank's dimensions. Re-verify if the bank SKU is changed (Anker product line revisions can change dimensions).

## Explicitly out of scope

The glossary's "Bewust niet in deze glossary" list is a scope fence that should be preserved when authoring the skill:

- Tag-side docs (`spikes/tag-handheld-enclosure-spike.md`, ADR-002) — different enclosure
- Relay-side docs (`spikes/physical-fabrication-brief-spike.md`, ADR-003) — different enclosure
- Firmware / protocol / UI-software ADRs (001, 005, 008, 009, 010, 012, 013, 014) — no mechanical influence
- `TODO.md`, `README.md`, `ARCHITECTURE.md`, `bom.md` — work-tracking, downstream of design
- Doc-process files (`docs/spike-rules.md`, `docs/spike-template.md`) — process, not design
- Pi 5 mechanical drawing PDF — fetch was empty; only well-known values used (85×56 board, 1.6 mm PCB)

A future skill should NOT pull these into its corpus.
