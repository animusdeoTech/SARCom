---
title: "Spike — BLE commissioning gateway-UI flow (post-install relay verification from the handheld)"
status: closed
type: spike
timebox: 0.5 day
opened: 2026-05-07
closed: 2026-05-08
---

# Spike: BLE commissioning gateway-UI flow

## Closed 2026-05-08

H1 verdict — **evolved shape**. The single-explicit-interaction-surface spirit of H1 is preserved (one deliberate path from kiosk to commissioning, no top-level BLE menu, no inline-edit on the map), but three load-bearing details changed during the 2026-05-08 review:

- **Trigger.** Sidebar-entry tap, not long-press on a map marker. Operator taps the node row in the kiosk sidebar; the bottom-of-sidebar button reflects per-peer BLE state and drives the flow.
- **Surface form.** Full-page screen replace (`Screen::Map` → `Screen::Commissioning(NodeId)`), not an overlay modal over the map. Map widget state is retained in the App struct so back-navigation is instant; only the render call differs. Page is named "Commissioning" in code and copy — not "modal" — to avoid the read-only-map-invariant tension that "modal over map" would imply.
- **Process architecture.** BLE central role lives in a separate Yocto service (`sarcom-ble-commissioning.service`, working name); the kiosk talks to it via IPC (mechanism TBD). **This breaks [ADR-004](../decisions/ADR-004-gateway-platform.md)'s "Single binary. One Tokio async runtime, one process. No IPC, no dbus gymnastics." stance.** The break is intentional — the BLE state machine must not run at kiosk-frame-rate latency. Captured here; the ADR-004 amendment ticket is owned by the named follow-up below, not this spike.

Named follow-up: [`spikes/gateway-runtime-task-architecture-spike.md`](gateway-runtime-task-architecture-spike.md) close — formalises the IPC mechanism, the task split across the gateway's processes, the ADR-004 amendment ticket, and the integration with the power-architecture-spike's signal contract.

Detailed decisions captured below in the §Decision note. Page content (which fields render, what the actions drawer holds, verdict thresholds, SOS-vs-page rule) is **out of scope for this UI-flow spike** and lives in a separate downstream implementation ticket. The body sections below (modal layout, verdict roll-up rule set, actions drawer) describe the design intent at the time the spike was opened; they remain useful as input to that downstream ticket but are no longer authoritative for the surface form (page, not modal) or the trigger gesture (sidebar tap, not long-press).

## Why this spike exists

The existing [`ble-commissioning-scope-spike.md`](ble-commissioning-scope-spike.md) settles the firmware contract: gateway-as-central, relay/tag-as-peripherals, the read-mostly health surface, the write allow-list, and the auth model. What it does **not** answer is how an operator *uses* that surface from the handheld gateway in the field.

The motivating workflow is concrete and tight:

> *I just bolted a relay to a wooden pole. I sealed the enclosure. I cannot open it again without breaking the seal. I'm standing 5–20 m from the pole with the handheld gateway in my hand. I need to know — right now, before I walk back to the hut — whether this relay is alive, transmitting, hearing anything, and not about to flatline its battery overnight. If something is wrong I need to know which something.*

Two questions, asked in this order:

1. **"Is mijn relay OK?"** — a single at-a-glance verdict (green / yellow / red + one-line reason). Operator does not want to read counters to find out.
2. **"Wat doet het allemaal momenteel?"** — live counters and recent activity, for when the verdict is yellow/red and the operator needs to diagnose, or when they just want to confirm with their eyes that the thing is doing real work.

Both questions are answered from the handheld kiosk — no phone, no laptop, no SSH. The kiosk is the **only** UI per [ADR-007](../decisions/ADR-007-touchscreen-primary-ui.md), and that constraint is preserved here: the read-only *map* stays read-only. Commissioning interaction lives inside an explicit modal opened from a marker, not on the map itself.

This spike scopes the UI flow + modal contract. It does not redraw the firmware-side BLE contract.

## The two-phase verification frame

A useful frame the existing scope spike does not name explicitly:

- **Phase A — passive LoRa observation.** The relay's commissioning broadcast (per [ADR-006](../decisions/ADR-006-relay-has-gnss.md)) emits a few self-POSITION packets. If those reach the gateway, the relay marker appears on the map automatically — same path as a tag report. **First-contact is verified without BLE.** That single fact already proves: GNSS got a fix, the LoRa TX path works, the antenna isn't dead, the packet validates end-to-end.
- **Phase B — active BLE inspection.** What Phase A cannot tell you: battery voltage, what the relay is *currently* hearing, internal queue depth, recent error events, dropped-on-budget counters, GNSS sat count at last fix, reboot history. These are the **interior** of the relay. BLE is the only way to read them in the field on a sealed enclosure.

Together: **LoRa observes the relay's output; BLE observes its interior.** The modal designed in this spike exposes Phase B in a form that respects this division — it does not duplicate Phase A info, it fills in what Phase A cannot tell.

## Scope fence

- **No firmware coding.** GATT layout, UUIDs, NimBLE wiring on the relay, BlueZ-on-Pi config on the gateway — all out. Implementation tickets are downstream of both this spike and the existing scope spike.
- **No re-do of the firmware-side contract.** Field list, write allow-list, auth model, advertising format — cited from [`ble-commissioning-scope-spike.md`](ble-commissioning-scope-spike.md) and [`field-deployment-test-fleet-spike.md`](field-deployment-test-fleet-spike.md) §4. Don't redraft.
- **No tag-side UI flow.** Tag commissioning is a separate, smaller flow (button-hold + buzzer self-test). This spike is about *relay* commissioning verification.
- **No phone-app fallback.** Even if H0 of the existing scope spike eventually wins, the gateway-side flow in this spike is independently useful and does not depend on a phone.
- **No new wire packets, no LoRa downlink, no SARCOM-over-BLE.** [ADR-008](../decisions/ADR-008-no-cloud-no-downlink.md) and [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) preserved.

## Hypothesis

**H1 (default).** Single explicit interaction surface in the kiosk:

1. Operator long-presses (or two-finger taps — to be picked) a relay marker on the map.
2. A small action sheet appears with one entry: **"Connect via BLE"**. The map itself remains read-only — no inline edit, no tap-to-send-command.
3. Tapping "Connect via BLE" triggers a BLE scan filtered to the relay's expected `node_id` advertising payload. Found in ≤ 5 s → connect (using the bonded MAC from prior commissioning, or the active 60-second commissioning window per the scope spike's auth model).
4. A full-screen **Relay Health modal** opens with three sections, in order: **Verdict**, **Live state**, **Recent activity**.
5. An explicit **Actions** drawer (collapsed by default, requires a deliberate "Show actions" tap) holds the small write allow-list (trigger fresh commissioning, reboot, clear counters).
6. Closing the modal disconnects BLE. The gateway's BLE central role is per-modal, not always-on.

**H0 (fallback).** A persistent BLE pane in the kiosk that lists all in-range relays as a side list, like a Bluetooth-settings screen. Rejected unless H1 fails through-shell BLE constraints — H0 dilutes the at-a-glance answer with noise from neighbouring relays the operator does not currently care about.

## What to verify

### Discovery and connection affordance

- The relay marker on the map is the **only** entry point. No top-level "BLE" menu, no separate pairing screen. If you're not looking at a relay marker you cannot get into the BLE flow.
- Map marker rendering for relays already exists (per [ADR-006](../decisions/ADR-006-relay-has-gnss.md), pole icon). This spike adds **a small BLE indicator badge** on the marker when the relay is currently in BLE range (advertising heard within last 10 s by the gateway's passive scanner). No badge → tapping the marker offers "Connect via BLE" but warns: *"Out of BLE range — move closer."*
- The action sheet (long-press / two-finger tap → "Connect via BLE") is the **only** write affordance reachable from the map. The map's tap behaviour for non-relay nodes (hiker dot, drone) stays read-only; tapping a hiker dot remains an info-only action.
- The "Connect via BLE" trigger is a **deliberate gesture**, not a single tap, to prevent accidental connects when panning the map. Long-press or two-finger tap is the candidate; bench-test which is more reliable through gloves.

### Verdict roll-up logic (gateway-side)

The verdict (green / yellow / red + one-line reason) is **computed on the gateway from the raw counters read over BLE**. The relay does not know its own verdict; it just exposes the underlying state. Verdict logic lives in the gateway binary so it can evolve without firmware reflashes.

Candidate rule set (analysis, not committed):

- **Green** if **all** of:
  - battery_mV ≥ low_threshold (default 3 600 mV, configurable per relay in `nodes.toml` if cell pack varies)
  - last RX from a tag within last 10 min (or relay has been up < 10 min)
  - rolling 1-h duty cycle ≤ 90% of the 1% sub-band cap
  - TX-dropped-on-budget counter not increasing in the last 5 min
  - GNSS commissioning succeeded within the last hour (sat count ≥ 4 at last fix)
  - no boot recorded in the last 5 min that wasn't a manual operator-triggered reboot
- **Yellow** if any of: battery_mV between low_threshold and (low_threshold + 200), or duty cycle 70–90%, or last RX > 10 min in a deployment that should have traffic, or recent TX-dropped events present but not currently increasing.
- **Red** if any of: battery_mV < low_threshold, duty cycle > 95%, TX-dropped count increasing, GNSS commissioning never succeeded, repeated boots in last 5 min (reboot loop), BLE disconnect failures within the modal session.

Each rule contributes a one-line reason string. The verdict surface shows the **most severe** active reason. Tapping the verdict row expands the full list of contributing reasons (so a single "Battery low" line can also reveal "duty cycle 75%" sitting at yellow underneath).

**No verdict logic is implemented in this spike.** The list above is the design intent; the actual rule set + thresholds become a small `gateway/src/health/verdict.rs` module + a config row per relay in `nodes.toml`. Spike output names the rules; firmware ticket implements them.

### Modal layout (top-down)

```
┌─ Relay Health: <node_id> "<label from nodes.toml>" ──────────── [×] ─┐
│                                                                       │
│  ● GREEN — All checks pass                              [▾ details]   │
│                                                                       │
│  Live state                                                           │
│    Battery       3.92 V        (charging from solar)                  │
│    Uptime        4 d 17 h                                             │
│    Last tag RX   42 s ago      from node_id 0x12 @ −94 dBm           │
│    Last TX       18 s ago      (rebroadcast)                          │
│    Duty cycle    0.34 % / 1.00 % cap                                  │
│    TX dropped    0   (since boot)                                     │
│    Seen-cache    11 / 32 entries                                      │
│    GNSS          last fix 3 h ago, 9 sats, HDOP 0.9                  │
│    Firmware      v0.4.1+abc1234, built 2026-05-12                     │
│                                                                       │
│  Recent activity (most recent first)                                  │
│    18s   TX REBROADCAST seq=0x0A12 from 0x12                          │
│    42s   RX OK          seq=0x0A12 from 0x12 −94 dBm SNR 5.2          │
│    1m    RX DUP         seq=0x0A11 from 0x12 (cache hit)              │
│    2m    TX SELF_ANNOUNCE seq=0x0007                                  │
│    …                  (last 32 events; ringbuffer in firmware)        │
│                                                                       │
│  ▸ Show actions                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

Notes on the layout:

- **Verdict row at top.** Big, single colour, single sentence reason. This is the answer to question 1. Operator who only wants "is it OK" reads this row and closes the modal.
- **Live state.** Static enough that it can be polled at ~2 Hz over BLE without saturating the link or chewing relay battery. Each value carries a freshness indicator (greys out if stale > 10 s, suggesting BLE link is degrading).
- **Recent activity.** The ringbuffer of last N events from `field-deployment-test-fleet-spike.md` §4. Renders as a scrollable list, reverse chronological. Rough event types: RX OK, RX DUP, RX REJ (CRC/MAGIC/etc.), TX REBROADCAST, TX SELF_ANNOUNCE, TX DROP_BUDGET, BOOT, COMMISSION_START, COMMISSION_DONE.
- **Actions drawer collapsed by default.** Reveal requires explicit tap. Reduces accidental "reboot" presses while panning.
- **One column.** No tabs. Tabs hide information; the post-install verification flow is short enough to scroll in a single view on the 5" handheld display (per [`gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md)).

### Actions drawer

When expanded, three buttons matching the write allow-list from the existing scope spike:

- **Trigger fresh commissioning** — relay re-runs the GNSS-fix + self-POSITION-broadcast sequence. Confirmation dialog: *"Relay will pause forwarding for ~90 s. Continue?"*
- **Reboot** — confirmation dialog. Disconnects BLE; modal reverts to a "Reconnecting..." spinner with timeout. If reconnect fails after 30 s, modal closes with a toast: *"Reboot triggered, BLE not yet reachable. Try again from the map."*
- **Clear counters** — confirmation dialog: *"Clears RX/TX/dropped counters and seen_cache. Use for fresh measurement runs only."*

No other writes. Specifically NOT in the drawer (mirroring the existing scope spike's MUST NOT list, plus map-level ones):
- frequency / SF / BW / sub-band changes
- node_id rewrite
- forced SOS state injection
- direct LoRa packet emission

### Multi-relay disambiguation

If two or more relays advertise within scan range, the gateway's BLE scan returns multiple candidates. The map already disambiguates by `node_id` (each marker is a node), so by the time the operator taps a marker, the target is unambiguous — gateway connects to that specific `node_id`'s advertised MAC.

What about *neighbouring relays* in BLE range that the operator did NOT tap? They are visible only in the per-marker BLE-range badge (the small indicator). They do not appear as a list. The action sheet is single-target. **No "scan for nearby relays" view in v1.**

### Disconnection and error states

- Modal close (X tap, hardware back, or app navigation) → BLE disconnect, gateway returns to passive scanner.
- BLE connection failure on initial connect → toast *"Relay <node_id> not reachable. Move closer or trigger commissioning window."* Modal does not open. No partial-state modal.
- Mid-session BLE drop → modal greys out, banner *"Reconnecting..."* with 10 s timeout. On reconnect, resume polling. On timeout, modal closes with a toast.
- Commissioning window expired (60 s window from existing scope spike) and gateway is not on the bonded MAC list → toast *"Pairing window expired. Trigger commissioning at the relay (magnet) and try again within 60 s."*

### ADR-007 reconciliation

[ADR-007](../decisions/ADR-007-touchscreen-primary-ui.md) sets the kiosk as a read-only map. Adding a write surface is the kind of change that historically gets re-litigated; pre-empt that:

- The **map itself** stays read-only. Tapping markers shows info; long-press is the *only* gesture that produces write affordances, and only on relay markers.
- All write actions live **inside the modal**. The modal is not the map — it is a maintenance overlay opened by deliberate gesture and closed when the operator is done. Closing the modal returns to read-only-map state.
- Write actions are **never** silently performed; every write requires confirmation dialog.
- The modal cannot be opened *during* an active SOS rendering on the map — open-modal logic checks for any node in SOS state in the last 60 s and displays *"SOS active — defer commissioning"*. Operator can override with a hold-to-confirm gesture, but the default is "don't get distracted by relay maintenance during a distress event."

This gives ADR-007 a clean compatibility story: "the map is read-only" is preserved literally; the modal is a separate maintenance surface with explicit entry, explicit exit, and explicit consent on every write.

## Pass criteria

- Two-phase frame committed (LoRa for output, BLE for interior).
- Discovery + connection affordance committed (long-press / two-finger from relay marker only).
- Verdict roll-up rule set drafted with thresholds; gateway-side computation confirmed (no firmware reflash to change rules).
- Modal layout committed (verdict row + live state + recent activity + collapsed actions drawer).
- Actions drawer write list = exactly the existing scope spike's relay write allow-list. No additions.
- Multi-relay disambiguation rule committed (per-marker only; no nearby-relay list view).
- Disconnection / error states enumerated.
- ADR-007 reconciliation captured (map stays read-only; modal is the maintenance surface).
- Cross-references to existing scope spike, field-deployment §4, ADR-006, ADR-007 recorded.

## Fail criteria

- BLE-through-3D-printed-shell range is too short for arm's-length-from-the-pole verification (per [`gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md) eventual measurement). If operator must hug the pole to maintain BLE link, the workflow is broken — escalate to a USB BLE dongle decision or a wired-pogo alternative, and amend [ADR-006](../decisions/ADR-006-relay-has-gnss.md).
- Verdict thresholds end up requiring per-deployment tuning that can't be expressed in `nodes.toml` — escalate to a separate health-config schema. Don't silently hardcode in firmware.
- Long-press as the gesture proves unreliable through winter gloves on the resistive/capacitive touchscreen chosen in the substrate spike — pivot to a hardware-button modifier (hold a side button + tap) or revisit the gesture choice; do not silently fall back to single-tap and erode the read-only-map invariant.
- Phase B BLE poll rate (default ~2 Hz) saturates either the BLE link or the relay's CPU during commissioning — drop poll rate, batch reads into single GATT round-trips, or accept slower live updates. Never mask the saturation behind UI smoothing.

## Decision note

```
Date: 2026-05-08
H1 / H0 / H2 verdict: H1 (single explicit interaction surface preserved)
                      with shape evolution — trigger and surface form changed,
                      process architecture broke ADR-004's single-binary stance.

Topology preserved from ble-commissioning-scope-spike.md H1: yes
  (gateway-as-central, relay/tag-as-peripherals, in-range-only, no LoRa-wake,
   no periodic relay-side BLE wake)

Two-phase frame:
  Phase A (LoRa first-contact verification) renders on map automatically: confirmed
  Phase B (BLE interior inspection) is the surface scoped here: confirmed,
    reframed — Phase B lives in a full-page Commissioning screen, not a modal
    over the map.

Architecture (new, the load-bearing change):
  BLE central role process: separate Yocto service (working name
    sarcom-ble-commissioning.service), NOT in the kiosk binary.
  IPC mechanism kiosk ↔ service: TBD; owned by runtime-task-architecture-spike
    close (UDS + JSON-Lines or D-Bus are likely; not decided here).
  Per-peer state machine in the service: Idle → Searching → {Found, NotInRange,
    Failed}; from Found: Connecting → {Connected, ConnectFailed}; from
    Connected: Disconnected (clean) or ConnectionLost (unclean). Recovery edges
    bring all error states back to Idle after a short cooldown.
  ADR-004 single-binary / no-IPC stance: BROKEN intentionally. Amendment ticket
    is a follow-up via runtime-task-architecture-spike close. NOT amended here.

Discovery scope:
  in-range BLE only: confirmed.
  no LoRa-wake of relay's BLE radio: confirmed (would require downlink byte;
    conflicts with ADR-008 + ADR-013).
  no periodic relay-side BLE-wake: confirmed (battery cost rejected).
  operator must be physically in BLE range of the peer: confirmed (matches
    ADR-006 §3 "service engineer stands next to the pole").

Discovery / connection:
  entry: SIDEBAR ENTRY TAP (not long-press marker, not two-finger tap on map).
    User taps a node row (relay or tag) in the kiosk sidebar.
  per-marker BLE-range badge: NO — replaced with a per-entry sidebar button
    fed by the BLE service's per-peer state.
  no top-level BLE menu: confirmed.

Sidebar button states (drives the flow):
  Idle / never polled  → "Search"            (enabled; tap starts BLE scan)
  Searching            → "Searching..."      (disabled)
  Found                → "Connect"           (enabled; tap initiates GATT
                                              connection)
  NotInRange           → "Not in range"      (disabled, with small "retry"
                                              affordance)
  Connecting           → "Connecting..."     (disabled)
  Connected            → button hidden; Commissioning page is open
  ConnectionLost
  / ConnectFailed      → "Retry"             (enabled)

Surface form:
  full-page Commissioning screen, NOT a modal over the map.
  render path: Screen::Map → Screen::Commissioning(NodeId).
  map widget state retained in the App struct; back-navigation is instant
    (no PMTiles re-init, no SQLite re-query).
  page name in code and copy: "Commissioning" (NOT "modal") — chosen to avoid
    the read-only-map-invariant tension that "modal over map" would imply.
  ADR-007 read-only map preserved: confirmed (the map widget itself is not
    edited; commissioning is a separate Screen variant).
  ADR-007 amendment for write-surface page: owned by ADR-015 / a future
    ADR-007 amendment ticket. NOT edited here.

Tag commissioning:
  same trigger pattern (sidebar entry tap) and same per-peer state machine.
  page content differs from relay (tag = identity readback + battery + buzzer
    self-test + SOS-button calibration; relay = battery mV + RX count + last
    RSSI + GNSS fix age + force-recommission).
  page content spec: OUT OF SCOPE for this spike — separate impl ticket.

gw-0 sidebar entry (gateway's own node row):
  tap behaviour in v1: undefined. Mockup-only. Not blocked by this spike.

Verdict roll-up:
  computed on gateway (not relay): confirmed in principle.
  rule set committed: deferred — page content spec is out of scope here.
  thresholds in nodes.toml per relay: deferred to page-content ticket.

Modal layout (renamed: page layout):
  verdict row first / live state / recent activity / actions drawer:
    deferred — page content spec is out of scope here. The layout sketch
    earlier in this spike body remains useful input to the downstream
    page-content ticket but is no longer authoritative on the surface form.
  ADR-007 read-only map preserved: confirmed (see Surface form above).

Actions drawer:
  trigger fresh commissioning / reboot / clear counters: deferred to
    page-content ticket. Write allow-list inherits from
    ble-commissioning-scope-spike.md unchanged; no expansion.
  no other writes: confirmed in principle.

Multi-relay rule:
  single-target via SIDEBAR ENTRY TAP only (one node_id per row): confirmed.
  no nearby-relay list view in v1: confirmed.

Disconnection / error states:
  covered by the per-peer state machine above.
  page exit (back to Map) → service issues clean disconnect: confirmed.
  failed connect → button shows "Retry"; no toast, no half-open page:
    confirmed.
  mid-session drop → ConnectionLost surfaces; recovery via Retry: confirmed.
  pairing-window expiry handling: deferred to page-content ticket.

SOS-vs-page rule:
  page blocked / warned during active SOS: deferred to page-content ticket.

Performance optimisation (explicitly out-of-scope here):
  map render quiescence while Commissioning is foreground: v2+.
  BLE scan duty-cycle tuning: v2+.
  IPC backpressure handling: v2+.
  v1 ships when "iets moois en werkend" works, not when optimal.

Cross-spike implications:
  scope spike (firmware contract):
    consumed unchanged from spikes/ble-commissioning-scope-spike.md.
  field-deployment §4 (relay surface):
    consumed; specific field rendering deferred to page-content ticket.
  substrate (BLE through shell):
    consumed unchanged from spikes/gateway-handheld-substrate-spike.md.
  ADR-006 (BLE peer roles):
    preserved.
  ADR-007 (read-only map):
    preserved literally. Amendment for write-surface page is a separate
    downstream ticket (owned by ADR-015 / future ADR-007 amendment).
  ADR-004 (gateway platform):
    "Single binary. No IPC." stance BROKEN by the separate-Yocto-service
    decision. Amendment ticket is owned by runtime-task-architecture-spike
    close — NOT edited here.

Not implemented in this spike: BLE central code in the service, IPC mechanism
                                choice, kiosk page widget code, verdict
                                computation module, page content spec, ADR
                                edits.

Follow-up filed: spikes/gateway-runtime-task-architecture-spike.md close —
  formalises (a) the IPC mechanism between kiosk and BLE service, (b) the
  task split across the gateway's processes (kiosk binary + BLE service +
  LoRa RX + SQLite writer + optional CoT/TAK emitter), (c) the ADR-004
  amendment ticket recording the single-binary stance break, (d) integration
  with the power-architecture-spike's signal contract (BLE service must
  respect SHUTDOWN_REQUEST, POWER_GOOD, etc.).

Next action: open / progress runtime-task-architecture-spike close.
```

## Cross-references

- [`spikes/ble-commissioning-scope-spike.md`](ble-commissioning-scope-spike.md) — firmware-side contract; this spike consumes that contract from the UI side.
- [`spikes/field-deployment-test-fleet-spike.md`](field-deployment-test-fleet-spike.md) §4 — relay BLE surface (read-mostly health/debug fields, write allow-list, ringbuffer event types). Modal renders these without redrafting them.
- [`spikes/gateway-handheld-substrate-spike.md`](gateway-handheld-substrate-spike.md) — BLE-through-shell measurement; gates the H1 workflow.
- [`spikes/duty-cycle-measurement-workflow-spike.md`](duty-cycle-measurement-workflow-spike.md) — ADR-014 enforcement; the duty-cycle counter the modal renders is the same field this spike defines TX-log fields for.
- [ADR-006](../decisions/ADR-006-relay-has-gnss.md) — relay BLE in v1; phone/laptop wording is being routed through the gateway by the existing scope spike. This UI spike is downstream of that routing.
- [ADR-007](../decisions/ADR-007-touchscreen-primary-ui.md) — read-only map; modal-as-maintenance-overlay reconciliation captured above.
- [ADR-008](../decisions/ADR-008-no-cloud-no-downlink.md) — preserved; BLE never carries SARCOM data plane, modal write actions never become a downlink to remote nodes.
- [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) — wire packet types; modal does not introduce new packets, only reads firmware state.
- [`spikes/handheld-pivot-doc-audit-spike.md`](handheld-pivot-doc-audit-spike.md) — registrar; this spike is downstream of the pivot's gateway-as-handheld + BLE-central decisions.
