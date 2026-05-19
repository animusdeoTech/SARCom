# v1a operator-map mockup — rationale

- **Date:** 2026-05-19 (rev 2 — post-`tickets/` read)
- **Author:** Claude (sarcom-ux-mockup worker)
- **Artifact:** `UX/mockups/v1a-operator-map-mockup.svg` (composite, two 800×480 panels side-by-side)

## Revision note

Rev 1 was produced without access to `tickets/` and reconstructed the brief from
context. It got the scenario wrong (tag-3 and tag-4 semantics were swapped,
relay-2 was missing, Variant A unfairly dropped the persistent strip). Rev 2 is
the version aligned with [`tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md`](../../tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md)
and the downstream tickets KIOSK-002, KIOSK-004, KIOSK-006 plus the gating
spike SPIKE-001.

## Scenario rendered in both panels

Per the design prompt [`tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md:60-69`](../../tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md):

| Node | Type | State | Notes |
|---|---|---|---|
| tag-1 | hiker | Normal | last seen ~12 s, `gps_valid=true`, batt ok |
| tag-2 | hiker | SOS | last seen 42 s, `gps_valid=true`, batt ok, flags.SOS=1 |
| tag-3 | hiker | No-fix + last-valid-fix | `gps_valid=false`, `last_valid_fix_age_secs ≈ 480` (= 8 m) — ghost drawn at `last_valid_fix_pos`, NOT at the sentinel current `pos` per [`tools/sarcom-kiosk-lab/src/map/markers.rs:265-302`](../../tools/sarcom-kiosk-lab/src/map/markers.rs) |
| tag-4 | hiker | Stale | last seen ~700 s (crosses 660 s aging→stale boundary per `data.rs:33`) — dim grey dot |
| relay-1 | relay | Healthy | `last_seen_secs ≈ 840` (14 m), within 1800 s POSITION cadence per `data.rs:42-48` — blue ✚ |
| relay-2 | relay | Overdue | `last_seen_secs ≈ 3900` (65 m), past 3600 s threshold — amber ✚ |
| gw-0 | gateway | Healthy | clock valid — square outline per [`tools/sarcom-kiosk-lab/src/map/markers.rs:168-184`](../../tools/sarcom-kiosk-lab/src/map/markers.rs) |

Detail view is rendered for **tag-3** in both panels so the no-fix case is
fully visible.

## What the two panels show

- **Variant B (left, the design to ship).** Strict ADR-007 per
  [`decisions/ADR-007-touchscreen-primary-ui.md:38-46`](../../decisions/ADR-007-touchscreen-primary-ui.md):
  no overlays, no popovers, no banners, no acknowledgement flow.
  - Sidebar detail view **replaces the list in place** when tag-3 is selected;
    a `← Back to list` row sits at the top per
    [`tickets/KIOSK-004-selection-detail-panel.md:28-31`](../../tickets/KIOSK-004-selection-detail-panel.md).
  - SOS lives only in the persistent bottom strip with strengthened visual
    hierarchy per [`tickets/KIOSK-006-sos-alerting.md:31-39`](../../tickets/KIOSK-006-sos-alerting.md).
  - Cursor coordinate readout uses the bottom strip in non-SOS state per
    [`tickets/KIOSK-002-cursor-coordinate-readout.md:26-31`](../../tickets/KIOSK-002-cursor-coordinate-readout.md);
    shown as an annotated INSET below the live strip because the live strip is
    currently red. The interaction between SOS precedence and coordinate
    readout is called out as an unresolved design question (see below).

- **Variant A (right, pressure-test).** Three Variant A surfaces overlaid on
  the same scenario:
  - **Top-anchored SOS banner** with an `ACKNOWLEDGE — pressure-test against
    ADR-007:46` button per [`tickets/KIOSK-006-sos-alerting.md:46-52`](../../tickets/KIOSK-006-sos-alerting.md).
    The **persistent bottom strip is preserved underneath** per KIOSK-006:50
    — this is what makes A a fair pressure-test (Rev 1 mistakenly dropped the
    strip, which made A fail by removing a required state surface rather than
    by adding a contested one).
  - **Slide-in detail panel** (~280 px wide) overlaying the right portion of
    the map, showing the same tag-3 detail per
    [`tickets/KIOSK-004-selection-detail-panel.md:33-38`](../../tickets/KIOSK-004-selection-detail-panel.md).
  - **Coordinate popover** near a simulated touch point on the still-visible
    left strip of the map per
    [`tickets/KIOSK-002-cursor-coordinate-readout.md:33-38`](../../tickets/KIOSK-002-cursor-coordinate-readout.md).

## Per-surface judgement (the SPIKE-001 substrate)

This section is what SPIKE-001 [`tickets/SPIKE-001-adr007-informational-overlays.md:60-65`](../../tickets/SPIKE-001-adr007-informational-overlays.md)
asks for: per surface, name the SAR operator task, judge each variant against
that task, and say who wins.

### Surface 1 — Detail view (KIOSK-004)

| | Variant B (sidebar replacement) | Variant A (slide-in panel) |
|---|---|---|
| Surface | Sidebar list collapsed to `← Back to list`; full 320 px of sidebar real estate carries tag-3 detail (state, last-frame age, last-fix age, last-fix lat/lon, battery, flags) | 280 px slide-in panel overlays the right portion of the map; sidebar list remains visible with sticky alerts (tag-2 SOS pinned) |
| Map occlusion | None — the sidebar is its own panel, not on the map | The slide-in covers ~280 px × 416 px of map area. In the rendered scene this hides relay-1, tag-2's right edge, and tag-3's ghost. |
| Sticky-alert visibility | Sticky alert section is hidden while the detail view is active; the SOS strip + map pulse ring are still visible. The Back-to-list row reminds the operator the list is one tap away. | Sticky alerts remain in the sidebar — operator can still see "tag-2 SOS" while reading tag-3 detail. |
| ADR-007 risk | None — strict reading preserved | Slide-in is the category the BLE-commissioning precedent permits for **write actions on relay markers** ([`spikes/ble-gateway-ui-flow-spike.md:17, 65-67, 169`](../../spikes/ble-gateway-ui-flow-spike.md)); using the same shape for a **read-only informational** surface is category expansion. ADR-007 risk is real but small (no new write affordance, no acknowledgement). |
| **SAR task served** | Operator selects tag-3 to confirm its full state: how recently it sent a frame, how old its last valid fix is, what those coordinates are, whether the battery is low | Same task. |
| **Verdict** | **Wins.** The information density and 2 m glance legibility on a full 320 px column are better than on a 280 px partial-overlay panel. The temporary loss of the list is recovered with one tap, and the sticky map pulse ring already conveys the most important alert (tag-2 SOS) regardless of which panel is open. | Loses unless a written operator-task failure is documented. The plausible failure ("operator needs the sidebar list visible while reading detail to cross-reference state") is not currently demonstrated. SPIKE-001 should default to Variant B for this surface. |

### Surface 2 — SOS alerting (KIOSK-006)

| | Variant B (persistent strip + stronger hierarchy) | Variant A (top banner + ACK + strip preserved) |
|---|---|---|
| Surface | Bottom 24 px y-band painted red, full panel width; carries `DISTRESS · tag-2 · since 14:08:23 · flags.SOS=1 · last frame 42 s · read-only · ack at the tag`. Strengthened: 13 pt bold `DISTRESS`, white leading dot, saturated red fill `Color32::from_rgb(160, 28, 28)` per [`tools/sarcom-kiosk-lab/src/app.rs:230-282`](../../tools/sarcom-kiosk-lab/src/app.rs). | 40 px top-anchored red band ABOVE the map, with an `ACKNOWLEDGE` button. Bottom strip is **also red** — preserved underneath per KIOSK-006:50. |
| Operator surface ratio | One SOS surface (strip) plus the on-map pulse ring | Two SOS surfaces (banner + strip) plus the pulse ring |
| ADR-007 risk | Low. The strip is the existing surface; aesthetic strengthening (size, contrast, optional pulse) is structurally identical to today. No acknowledgement affordance. `read-only · ack at the tag` is preserved — it accurately tells the operator the gateway is not the acknowledgement surface. | High. The `ACKNOWLEDGE` button is the exact shape of an "alert acknowledgement flow" prohibited by [`decisions/ADR-007-touchscreen-primary-ui.md:46`](../../decisions/ADR-007-touchscreen-primary-ui.md). Even if the acknowledgement is UI-local-only (no protocol, no DB write per KIOSK-006:49), the strip's `read-only · ack at the tag` text now disagrees with the banner's button — they cannot both be true at once. |
| **SAR task served** | Operator glances at the kiosk mid-radio-call and needs to know an SOS is active | Same task, plus the banner attempts to demand attention more loudly. |
| **Verdict** | **Wins.** A red 24 px strip across the full panel width, persistent, in the same y-band the operator already reads for `read-only`/map-mode, is impossible to miss at 2 m. The map pulse ring is the strongest single visual signal anyway and is unchanged. | Loses unless SPIKE-001 documents both (a) a SAR task that fails on Variant B's strip and (b) a written resolution of the ADR-007:46 acknowledgement-flow tension that the banner's button creates. KIOSK-006:43-45 explicitly says this surface is the most expensive to promote and the default position is that the prohibition holds. |

### Surface 3 — Coordinate readout (KIOSK-002)

| | Variant B (bottom-strip readout while touch held) | Variant A (popover near touch point) |
|---|---|---|
| Surface | Bottom strip carries `coord · 50.92341° N · 5.41872° E` while the operator touches the map; reverts on release per KIOSK-002:28-29 | Small popover near the touch point shows lat/lon |
| Operator's hand | Operator's hand is on the map; the bottom strip is at the bottom of the screen — hand does **not** occlude the readout | Popover is near the finger; in a finger-on-glass posture the hand naturally occludes the popover unless the operator lifts to read |
| Map occlusion | None | Popover covers a small patch of map around the touch point — the patch the operator is trying to locate |
| Conflict with SOS | **Yes.** KIOSK-002 acceptance criterion 6 [`tickets/KIOSK-002-cursor-coordinate-readout.md:80`](../../tickets/KIOSK-002-cursor-coordinate-readout.md) and KIOSK-002:29-30 give the SOS strip precedence over the coordinate readout. In an SOS-active scenario the coordinate readout has **no surface** in Variant B. | No conflict — the popover lives on the map, the strip is independent. |
| **SAR task served** | Operator reads a lat/lon aloud to a ground team over voice radio (e.g. "the col is at 50.92341 north, 5.41872 east") | Same task. |
| **Verdict** | **Wins in non-SOS state.** Bottom strip is the lowest-friction surface, no occlusion by hand or by popover, integrates with the existing strip the operator already glances at. **Unresolved during SOS.** | Wins **only** in SOS-active state, **and only** if the SOS-conflict in Variant B is judged operationally fatal. Otherwise loses on hand-occlusion alone. |

## Did the bias rules survive contact?

The bias rules in [`tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md:24-29`](../../tickets/CLAUDE-DESIGN-PROMPT-v1a-operator-map.md):

| Rule | Survived? | Notes |
|---|---|---|
| Always-visible map chrome > popups | Yes | North arrow, scale bar, zoom +/−, fit-all, home, clear-UI all render as first-class chrome in B (and in A — chrome is identical, only the alert/detail/readout surfaces differ). |
| Sidebar replacement > slide-in | Yes for detail view | Variant B's full-width sidebar replacement is operationally adequate for the detail-view task; the slide-in costs map area without buying anything not already in the sidebar. |
| Bottom-strip readout > coord popover | Yes in non-SOS state. **Cracks in SOS state.** | KIOSK-002:80 makes SOS precedence explicit. The mockup surfaces this as an open question — not pretended-solved. See "what I'd ask Pieter next." |
| Persistent SOS strip > acknowledgement banner | Yes | Variant B's strip is structurally identical to today; Variant A's banner adds an `ACKNOWLEDGE` button that ADR-007:46 closes. The persistent strip is preserved in BOTH variants — Variant A only ADDS a banner, it does not REMOVE the strip. |
| Stronger visual hierarchy > new UI surfaces | Yes | The strengthened strip (bold DISTRESS, leading dot, saturated red) is an aesthetic change inside an existing surface, not a new surface. |
| No ack flow rendered as accepted | Yes | The button is rendered ONLY as `ACKNOWLEDGE — pressure-test against ADR-007:46`, with the literal exception text per the sarcom-svg-wireframe rule. |

## Honesty discipline checklist ([`ARCHITECTURE.md:493-496`](../../ARCHITECTURE.md))

- [x] No stale position rendered as if current. tag-4 (stale, 700 s) is drawn
      as a dim grey dot, clearly distinct from a fresh marker; the sidebar age
      string in the Variant A list reads `stale · 12 m`.
- [x] No sentinel coordinates as a map marker. tag-3 is drawn as a ghost at
      its `last_valid_fix_pos`, **not** at the sentinel current `pos`, per
      [`tools/sarcom-kiosk-lab/src/map/markers.rs:32-38`](../../tools/sarcom-kiosk-lab/src/map/markers.rs)
      `tag_visible_pos`. The dashed outer ring + sub-label `NO FIX · 8 m`
      makes the no-fix state visible.
- [x] No no-fix uncertainty disc. SPIKE-002 owns that question; the mockup
      shows only the ghost dashed ring + faded fill, no `last_valid_fix_age ×
      walking_speed` disc.
- [x] No implied real-time tracking. tag-1's track is shown as discrete
      faded points joined by a thin line at 0.5 alpha — not a heavy
      interpolated path.
- [x] No invented RSSI/SNR/hop fields. The detail view explicitly carries a
      `NOT SHOWN` block listing them with the reason (data model does not
      carry them). The map and sidebar surfaces do not mention RSSI/SNR/hop.
- [x] No acknowledgement affordance rendered as if accepted. The Variant A
      button is labelled `ACKNOWLEDGE — pressure-test against ADR-007:46` so
      it cannot be read as an endorsement of the flow.

## What I'd ask Pieter next

1. **Coordinate readout precedence during SOS.** KIOSK-002 acceptance
   criterion 6 says the SOS strip takes precedence over the coordinate
   readout. In an SOS-active scenario the operator has no Variant B surface
   for reading lat/lon — but reading coordinates aloud to a ground team is
   plausibly **more** important during an SOS than in calm state. Three
   candidate resolutions: (a) accept it — operator uses the marker's known
   lat/lon for the SOS tag instead of an arbitrary map point during SOS;
   (b) split the strip into two stacked lines during SOS so both can coexist
   (costs ~24 px of vertical map area while a touch is held); (c) promote
   coordinate readout to Variant A popover **only** when SOS is active. Each
   has a different ADR-007 surface count. Worth a SPIKE-001 line.

2. **Sidebar-list visibility while detail is open (Variant B).** In Variant B
   the list is hidden while tag-3 detail is shown; the operator must tap
   `Back to list` to see the other rows. If the operator routinely needs to
   cross-reference (e.g. compare tag-3's last-fix age against tag-4's
   stale age), Variant B's sidebar replacement starts to lose. The map
   markers + sidebar sticky-alert pulse ring carry the most urgent state
   (tag-2 SOS) regardless. Is there a real SAR task where the rest of the
   list needs to remain visible while reading detail? If yes, that is the
   single strongest argument for Variant A on this surface.

3. **Relay-1 / relay-2 selection content.** KIOSK-004 makes relay rows
   selectable. Tapping relay-2 would surface `last frame = POSITION 65 m`. What
   does the operator do with that? If the answer is "go check the relay
   physically," the detail view's job is done; if the answer is "trigger
   BLE commissioning," that crosses the precedent at
   [`spikes/ble-gateway-ui-flow-spike.md`](../../spikes/ble-gateway-ui-flow-spike.md)
   and is out of scope here. The mockup shows relay-1/relay-2 as map
   markers and sidebar rows but does not render a selected-state detail
   view for them — that's a different detail-view layout than the one for
   tag-3 (no SOS/GPS_VALID/battery_low flags, no last-fix coords).

4. **Banner-vs-RTC stacking (Variant A).** KIOSK-006:116 notes the
   banner-during-commissioning conflict; the analogous RTC-strip-during-SOS
   stacking is also a real case the lab code already handles (RTC `Panel::top`,
   SOS `Panel::bottom`). Variant A's top banner stacks where the RTC strip
   would also live — and they would compete for the same top slot. Not
   rendered in this mockup (scenario is clock-valid); worth a follow-up
   sketch.

5. **800×480 fixture vs handheld substrate.** The mockup is drawn against
   the kiosk-lab fixture per [`tools/sarcom-kiosk-lab/README.md:53`](../../tools/sarcom-kiosk-lab/README.md).
   Display class is open per pending ADR-015 ([`README.md:36`](../../README.md)).
   Layout proportions (320 px sidebar, 24 px strip, 40 px banner, 44 px touch
   targets) should re-flow at 720×480 or 1024×600 without redesign — but
   the banner-pushes-map-down cost in Variant A becomes proportionally
   different at shorter screen heights, which is worth re-checking when
   ADR-015 lands.
