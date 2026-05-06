---
title: "Spike — BLE commissioning interface scope (gateway central + relay/tag peripherals)"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-06
---

# Spike: BLE commissioning interface scope

## Why this spike exists

The pivot context names a **BLE commissioning interface** as a v1 architectural element. ADR-006 already pulled "BLE maintenance" into v1 scope for **relays** with relay-as-peripheral framing (a service engineer with a phone walks up to the pole). The pivot adds a new BLE peer: the **handheld gateway**, which now must act as a **BLE central** in some flows. `field-deployment-test-fleet-spike.md` §4 has a detailed read-mostly maintenance surface for the relay side; this spike must reconcile with that without duplicating it.

Open questions the pivot exposes:

- BLE topology — gateway as BLE central; relay/tag as peripherals; phone/laptop is **out** (re-derives the v1 simplification, no third-party BLE peer)
- relay BLE advertising format — what relays advertise so the gateway can find them
- relay health fields exposed (already drafted in `field-deployment-test-fleet-spike.md` §4 — confirm reuse)
- "trigger fresh commissioning broadcast" semantics
- pairing / authentication / bonding — what stops a stranger's phone from connecting to the relay
- accidental access — read-mostly enforcement; allow-list of writes
- explicit non-goal: BLE is for commissioning + maintenance only, **not** mesh command/control, **not** alternative data path for SARCOM packets

This spike scopes the contract; it does not implement BLE firmware.

## Hypothesis / research question

**H1.** v1 BLE topology is **gateway as central + relay/tag as peripherals only**. Phones / laptops are explicitly excluded from v1 BLE peers — keeps auth simple (relay/tag accept connection only from the gateway's bonded MAC, plus a one-time discovery window during initial pairing) and removes ADR-007 read-only-UI tension. Relay surface = `field-deployment-test-fleet-spike.md` §4 (already accepted). Tag surface = identity readback + battery/buzzer self-test only. Gateway surface = none (gateway is central; it does not advertise as a peripheral in v1).

**H0.** Phones with a maintenance app are an unavoidable v1 use case (Pieter wants to walk to a pole with a phone, not the gateway), so the relay accepts a wider peer set; auth gets harder; ADR-006 needs amending.

## Scope fence

- **No firmware coding.** BLE stack choice (ESP32-S3 NimBLE on relay/tag; BlueZ on Pi 5 on gateway) is a follow-up.
- **No GATT service UUIDs picked.** This spike scopes **what** is exposed; UUID assignment + endianness conventions are the implementation ticket.
- **No SARCOM-protocol-over-BLE.** The wire packets stay on LoRa (ADR-013); BLE is commissioning/maintenance only.
- **No firmware OTA.** Per `field-deployment-test-fleet-spike.md` §4: BLE OTA is explicitly out of v1.
- **No mesh control.** BLE does not become a downlink path; ADR-008's no-downlink stance is preserved end-to-end on the LoRa side.
- **No phone-app development.** Even if H0 lands, the spike doesn't write a phone app.

## What to verify

### Topology

1. Gateway as BLE central (default). Relay/tag as peripherals. Phone/laptop excluded.
2. Pi 5 onboard BLE through 3D-printed shell — relies on the substrate spike's measurement (`gateway-handheld-substrate-spike.md`). If onboard BLE fails through-shell, this spike inherits a USB BLE dongle constraint.
3. ESP32-S3 onboard BLE on relay/tag inside their respective enclosures — bench-measure RSSI through the OEM Solar Kit shell (relay) and the planned 3D-printed tag shell (tag). Relay shell may attenuate noticeably; field-deployment §4 already accepted this risk.

### Advertising format

- Relay advertises: SARCOM service UUID + `node_id` (1 byte) + maybe firmware version short hash + maybe a "commissioning-mode" flag bit. Keep ad payload small (≤ 31 bytes legacy advertising).
- Tag advertises: SARCOM service UUID + `node_id` only (commissioning is rarer for tags than for relays).
- Gateway as central scans for SARCOM service UUID; gateway's `nodes.toml` tells it which `node_id`s are expected.

### Pairing / auth / bonding

- **v1 auth model (candidate, not the only option):** Just-Works pairing (no user-displayable PIN on a relay/tag) **plus** a *commissioning window*. A relay/tag advertises a 60-second pairing window only when manually triggered (magnet+reed for relay, button-hold for tag). Outside that window it only accepts connections from a bonded MAC list (1–2 entries: the gateway). This keeps the threat model honest: an attacker physically present at the pole during the commissioning window can pair, but a passive eavesdropper across the field cannot.
- **No HMAC over BLE in v1** — same trust model as the LoRa side per `ARCHITECTURE.md §14 Trust model`. The CRC-16 is integrity, not auth; BLE is integrity-via-link-layer + bonding, not strong crypto.
- Bonding stored in NVS on relay/tag.

**What this spike does not compare in the timebox:** passkey pairing, LE Secure Connections / LESC, or stronger authenticated pairing flows. If Pieter's actual threat model needs more than just-works-plus-commissioning-window, escalate to a separate BLE security spike before BLE firmware commits.

### Read-mostly health/debug surface

For **relay**: reuse `field-deployment-test-fleet-spike.md` §4 verbatim (identity, uptime, power, GNSS, LoRa counters, duty-cycle counter, queue depth, seen_cache state, recent events ringbuffer). This spike does not redraft that list — it cites it.

For **tag** (new in this spike, smaller surface):
- identity (`node_id`, firmware version + git short hash + build timestamp)
- uptime / last reset reason
- battery voltage in mV, charge state if exposed by Tracker V2 PMIC
- last GPS fix age + sat count
- LoRa TX counters (heartbeat TX, SOS TX), TX-dropped-on-budget
- duty-cycle counter
- buzzer self-test (write-only, see writes below)
- last SOS state transition timestamp

For **gateway** (BLE central): no peripheral surface in v1.

### Writes — small allow-list

For **relay**: reuse `field-deployment-test-fleet-spike.md` §4 allow-list (trigger commissioning, reboot, clear counters, clear seen_cache as debug-only).

For **tag**:
- trigger buzzer self-test (3-second pulse)
- reboot
- clear counters
- nothing else — explicit MUST NOT list:
  - `node_id` write (set at flash time per `field-deployment-test-fleet-spike.md` §7)
  - frequency / SF / BW / sub-band
  - SOS state forced on/off (SOS is button-driven; BLE-forced SOS would invalidate the immediate-first-frame test contract from ADR-010)
  - SOS clear during active distress (button-driven only)

### Service-boundary fence (mirrors fake-position-injector spike's principle)

- BLE control surface lives on the **device firmware** as a maintenance build feature.
- The gateway binary's BLE-central role is a **maintenance task** that the operator triggers; not a service that runs continuously and silently.
- BLE never becomes a SARCOM-data-plane peer.

## Pass criteria

- Topology committed: H1 vs H0.
- Bonding / auth model named: just-works + 60-second commissioning window + bonded MAC allow-list.
- Tag read-mostly surface enumerated.
- Tag write allow-list enumerated; tag MUST NOT list enumerated.
- Relay surface confirmed as cite-and-reuse from `field-deployment-test-fleet-spike.md` §4 (no duplication).
- Gateway-as-central role confirmed; gateway-as-peripheral explicitly out for v1.
- Cross-spike implications recorded.

## Fail criteria

- Through-shell BLE attenuation (gateway shell or tag shell) is too high for arm's-length commissioning — this spike inherits a USB BLE dongle constraint from the substrate spike or pivots to magnetic-Pogo paired commissioning (no BLE for commissioning at all). Document and re-plan.
- Just-works + commissioning-window auth is judged insufficient for the actual threat model (e.g. Pieter wants to deploy in a publicly-accessible alpine area where the attack surface includes "hiker with a phone messing with a pole") — escalate to a stronger pairing scheme and amend ADR-006.
- The tag write allow-list cannot be implemented without forking a separate "maintenance firmware" build vs "operational firmware" build — accept the fork explicitly and add it to the build-system follow-up; do not silently allow writes that fail the MUST NOT list.

## Fallback / next action

- If H1 holds: cite as the v1 BLE contract; write follow-up implementation ticket(s) for relay BLE firmware, tag BLE firmware, gateway BLE central role.
- If H0 (phone in the loop): amend ADR-006 to add the phone to the v1 BLE peer set; redraft auth model with at least a write-key shared secret; do not implement before ADR-006 amendment lands.

## Decision note template

```
Date:
Topology: H1 (gateway-central, relay/tag-peripheral, no phone) / H0 (phone in v1)
  reason:

BLE through-shell verdict (from substrate spike):
  gateway:  acceptable / needs USB dongle:
  relay:    acceptable / marginal — see field-deployment §4:
  tag:      acceptable / needs internal antenna change:

Auth model:
  pairing: just-works + 60-second window + bonded MAC allow-list / other:
  bonding storage: NVS / other:
  no HMAC at BLE in v1 — confirmed:

Relay surface:    cited from field-deployment-test-fleet-spike.md §4 verbatim (no redraft):

Tag read-mostly surface (enumerate):
  identity (node_id, fw version, git hash, build ts):
  uptime / last reset reason:
  battery mV / charge state:
  last GPS fix age / sat count:
  LoRa TX counters / TX-dropped-on-budget:
  duty-cycle counter:
  last SOS transition timestamp:

Tag writes — allow-list (enumerate):
  trigger buzzer self-test (3 s pulse):
  reboot:
  clear counters:

Tag writes — MUST NOT (enumerate):
  node_id write:
  RF parameter writes (freq/SF/BW/sub-band):
  SOS state forced on/off:
  SOS clear during active distress:
  others:

Gateway role:
  central only in v1 — confirmed:
  no peripheral surface in v1 — confirmed:

Service-boundary fence:
  BLE never carries SARCOM data plane — confirmed:
  Gateway BLE central is a maintenance task, not always-on — confirmed:

Cross-spike implications:
  substrate (BLE through shell):                 ___
  gateway enclosure (BLE antenna):               ___
  tag enclosure (BLE antenna):                   ___
  field-deployment §4 (relay surface reuse):     ___
  ADR-006 amendment needed? yes / no:            ___

Not implemented in this spike: GATT UUIDs, BLE firmware, gateway BLE central code, phone app.

Next action:
```

## Cross-references

- `decisions/ADR-006-relay-has-gnss.md` — relay BLE in v1; this spike adds gateway-as-central and tag-as-peripheral but does not redo §4 of field-deployment.
- `decisions/ADR-007-touchscreen-primary-ui.md` — read-only UI; BLE is *not* a UI surface, separate maintenance interface.
- `decisions/ADR-008-no-cloud-no-downlink.md` — preserved; BLE never becomes a SARCOM downlink.
- `decisions/ADR-010-sos-encoding.md` — SOS button immediate-first-frame; BLE MUST NOT force SOS state.
- `decisions/ADR-013-multi-hop-flood-via-packet-id.md` — wire packet types; BLE never carries them.
- `spikes/field-deployment-test-fleet-spike.md` §4 — relay BLE surface (reused, not redone).
- `spikes/gateway-handheld-substrate-spike.md` — BLE-through-shell constraint.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
