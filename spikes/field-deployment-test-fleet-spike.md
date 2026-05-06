---
title: "Spike — Field deployment, test fleet, and BLE maintenance scope"
status: open
type: spike
timebox: 1 day
updated: 2026-05-06 (handheld pivot — partial staleness)
---

# Spike: Field deployment, test fleet sizing, and BLE maintenance scope

## PIVOT NOTE 2026-05-06 — partial staleness

The 2026-05-06 pivot to a **handheld portable battery-powered gateway with a 5" display + custom 3D-printed waterproof shell** affects this spike's framing in three places. The substantive content (pool model, fleet sizing, BLE relay maintenance §4, tag identity §7) survives the pivot.

- **§1 Hardware lifecycle pools — "Gateway deployment pool / Gateway development pool":** the deployment Pi at maman's framing reframes. The deployed gateway is now a *handheld* unit, not a wall-mounted kiosk; "off-limits during the experiment" still applies, but the form factor and the operator's interaction pattern (carry it vs. glance at a wall) are different. The 7" DSI / Yocto wording in this spike is now downstream of the pivot. Re-read with that in mind.
- **§3 Two-Pi gateway model:** still a useful framing, but the dev-Pi-vs-deployment-Pi distinction is now "the handheld build at maman's vs a desk mock-up of the same handheld" rather than "appliance-on-a-shelf vs desk Pi". The argument for two Pi units (uninterrupted live data run vs. dev iteration) holds; the cost/envelope changes because the Pi 5 / 5" / battery / Fusion 360 enclosure stack is more expensive than the 7" DSI mains-powered desk-Pi assumption baked in here.
- **§4 BLE maintenance scope for the sealed relay:** unchanged for the **relay** side. The 2026-05-06 pivot adds a new BLE peer (gateway-as-central). That live in the new sibling `spikes/ble-commissioning-scope-spike.md` — which **cites this §4 verbatim** for the relay surface and adds a smaller tag-side surface and the gateway-as-central role.
- **§9 Cost-control framing:** the costs in the table are pre-pivot. Substrate / power / enclosure spending is now bounded by the handheld envelope, not the kiosk-on-a-shelf envelope.

**Recommended way to read this spike post-pivot:** treat it as the planning frame for **what the deployed unit needs to be trustworthy unattended**, abstracted from the 2026-04-26 kiosk-on-a-shelf form factor. The §1 pool model, the deployment-vs-dev-Pi separation in §3, the BLE-relay surface in §4, the gateway-data-continuity workflow in §5, the off-wire debug story in §6, the tag-identity model in §7, the staged-acceptance criteria in §8 are all valid. Where the spike still says "wall-mount" / "kitchen window" / "appliance-on-a-shelf", mentally substitute the handheld + base-mode framing.

A full rewrite is not warranted; the bones are correct.

---

## Why this spike exists

Up to now the docs treat v1a as a single line — *"tag + 1 relay + gateway, garden test, dot moves on the screen"*. That framing skips the part of the project that is actually expensive and irreversible: **the relay goes on a pole, sealed inside a Solar Kit enclosure, in maman's garden, for weeks to a month or more, and from that moment on it is no longer a piece of bench hardware.** USB-C is not casually accessible. Serial logs are not casually accessible. `cargo run` against it is not a thing.

That changes:

- how many Heltec boards we actually need to own
- how many Raspberry Pis we actually need to own
- what BLE has to do for the v1a relay to be trustworthy on a pole
- what the gateway-side data continuity story looks like
- what counts as a v1a pass condition

This spike does not write a BOM. It does not amend an ADR. It frames the physical-deployment reality sharply enough that a follow-up BOM revision and a BLE-scope ADR can be written without inventing the hardware lifecycle on the fly.

The single principle that drives everything below:

> **A deployed board is not development hardware.** Once a board enters the deployed pool — sealed, on a pole, on solar — it must not be assumed available for daily flashing, serial debug, cargo iteration, or destructive testing. Anything you need to know about it has to come back over the radio or over BLE.

## Out of scope

- **BLE firmware OTA.** Explicitly out for v1a. Firmware update path is **physical retrieve → USB-C flash → redeploy**.
- Cloud backend. (Per [ADR-008](../decisions/ADR-008-no-cloud-no-downlink.md).)
- LoRa downlink. (Per [ADR-008](../decisions/ADR-008-no-cloud-no-downlink.md).)
- Routed mesh, hop counts, path arrays. (Per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md).)
- Mobile app polish.
- Production fleet management software.
- Drone / v1b scope. (Per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md), v1b is gated on v1a passing.)
- Final BOM, final ADRs, code changes.

## Hypotheses

**H1 (the working hypothesis):** v1a needs (a) more Heltecs than the 3-board count in the current draft BOM, (b) two Raspberry Pis rather than one, (c) a v1a-scoped BLE maintenance surface that is read-mostly and update-free. The 3× Pi 5 desk-radio-simulator idea is suspect compared to spending the same money on Heltec inventory.

**H0 (cheaper alternative):** The current draft BOM's 3-Heltec / 1-Pi setup is enough for v1a and the BLE surface can be deferred. We accept that the deployed relay will need physical retrieval whenever anything looks weird, and that gateway iteration interrupts the live data run.

This spike does not pick H1 vs H0 outright. It produces the analysis that lets the BOM revision and the BLE ADR pick.

## 1. Hardware lifecycle pools

The project today implicitly treats every Heltec the same way — bench hardware that happens to be running firmware. Once we deploy, that fiction breaks. Define explicit pools so a board's job is unambiguous.

| Pool | Reflashable mid-experiment? | Physically retrievable? | OK to lose local state? | Holds experiment data? | Failure mode | Environment |
|---|---|---|---|---|---|---|
| **Deployed relay pool** | No (without breaking the experiment) | Yes, but each retrieval = climbing a pole, breaking the seal, ending that data window | Yes — relays are stateless forwarders + GNSS + counters | No (counters are observability, not data of record) | Coverage hole until reflash; the run continues elsewhere | Outdoor, sealed, solar + 18650, weather-exposed |
| **Field tag pool** | Yes (tags walk back to the desk in someone's pocket) | Yes — tags are with the user | Yes — tags hold no DB rows | No | Walker carries a dead tag, swap on return | Outdoor, IP-something, battery-only, on a person |
| **Bench firmware pool** | Yes — this is the iteration loop | Always | Yes | No | Misflash, brick, smoke — replace from spare | Indoor, USB-C tethered, mains-powered |
| **Gateway deployment pool** | **No, not casually.** This is the canonical SQLite. Reboots OK. Reflashes OK only if planned | Yes (it's at maman's, a car ride away) | **No — losing the SQLite file is losing the experiment** | **Yes — `tag_reports` is the data of record** | Power loss = OK (RTC keeps time); SD corruption = run ends | Indoor, mains, 7" DSI, Dragino HAT, RTC |
| **Gateway development pool** | Yes — wipe and reimage at will | Always | Yes — synthetic data only | No (uses copies of deployed DB) | Iterate freely | Indoor, mains, on the desk |
| **Spare / recovery pool** | N/A until promoted | N/A | N/A | No | Promotes into bench / field / deployment as needed | Drawer |

**Why this matters:** as soon as you label the deployed relay as *not reflashable mid-experiment*, two follow-on requirements appear: (a) the deployed firmware has to be good enough to last the run, and (b) you need a way to *look* at it without reflashing. The first pulls in bench-twin testing; the second pulls in the BLE maintenance surface in §4.

**Existing docs to revise later:** [TODO.md](../TODO.md) treats hardware as one undifferentiated bag. The "Right now" section orders 3 Heltecs without naming pools. [ARCHITECTURE.md §15](../ARCHITECTURE.md) describes acceptance gates without naming which pool a board belongs to.

## 2. Minimum Heltec count for real v1a

The current draft BOM count is **3 Wireless Tracker V2** (per [bom.md](../bom.md) cart sanity-check, called out in [TODO.md](../TODO.md)): 2 tag + 1 relay. **No order has been placed yet — this is the previous cart shape, not a fleet that exists.** That allocation comes from a desk-demo mental model, not a deployment one.

Physical requirement before any sizing argument:

- 1 relay deployed on the pole for weeks/month
- 1 or more tags in active walking use
- at least 1 bench board still available for firmware bring-up *while the deployment is running*
- ideally a bench twin for both tag and relay so you can reproduce a problem without retrieving the deployed unit
- spares because hardware will be misflashed, dropped, water-damaged, or tied up in a destructive test

### Option A — 3 Heltecs *(previous cart shape)*

- 1 deployed relay
- 2 field tags
- 0 bench Heltec left

**Tradeoffs.** Cheapest. Works for the *first* relay deployment. After deployment, every firmware iteration on relay code requires retrieving the pole-mounted unit, which ends that data window. Tag firmware iteration cannibalises a walking tag. There is no bench twin to reproduce a problem you observe over BLE. Effectively, this option says "build firmware until it's perfect, deploy once, and don't touch it." The history of this project (three duty-cycle iterations on ADR-010, the FORWARD-envelope rollback in ADR-013) suggests "perfect on first deploy" is not realistic.

### Option B — 5 Heltecs

- 1 deployed relay
- 2 field tags
- 1 bench tag/relay dev board (reflashable into either role)
- 1 spare

**Tradeoffs.** First option that survives a misflash without ending the run. One bench board can play tag-twin or relay-twin but not both at once; reproducing a relay problem temporarily blocks tag bring-up and vice versa. Plausible **minimum** for a serious v1a.

### Option C — 7 Heltecs

- 1 deployed relay
- 1 bench relay twin (always-relay, mirrors the deployed firmware)
- 2–3 field tags (one always carried, others available)
- 1 bench tag twin (always-tag, for tag firmware iteration)
- 1 spare

**Tradeoffs.** First option where bench iteration on one role does not block the other. The bench relay twin is the unit you BLE-poke and serial-log to interpret what you saw on the deployed unit. This is the count where "reproduce-on-bench, fix-on-bench, then plan a single physical retrieve to redeploy" becomes a clean workflow.

### Option D — 10 Heltecs

- Everything in C, plus:
- 1 v1b drone-pod candidate (can be flashed and integrated without disturbing v1a)
- 1 destructive-test board (sun-bake, drop test, antenna mismatch test)
- 1–2 deeper spares

**Tradeoffs.** This is the count where v1b can begin physical preparation in parallel with v1a running, where you can intentionally break a board to learn (e.g. measure what happens if the antenna pigtail isn't seated correctly), and where misflashes during nightly iteration are noise rather than events. The right cost comparison is **not** "10 Heltecs vs. one bare Pi" — it is **"10 Heltecs vs. the 3× Pi 5 desk-radio-simulator idea"** (3 Pi 5s + 3 Dragino HATs + 3 SDs + 3 PSUs + cabling). Going from 7 to 10 Heltecs is the marginal step that buys **destructive-test budget, v1b prep without cannibalising v1a, and deeper spare depth**. The 3-Pi simulator at the same price point reduces *protocol-iteration risk*, which is already low (one frozen packet type, frozen test vectors per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md)). The marginal Heltecs reduce *physical-deployment* and *iteration-cost-during-deployment* risk, which is the actual v1a exposure. So if there is room in the runway for one of those two purchases, this one wins on risk reduced per euro — but only after the 7-Heltec / 2-Pi shape is in place.

### Why more Heltecs may beat more Pis

The desk LoRa simulator idea (3 Pi 5s + 3 Dragino HATs all running gateway code to simulate a network) optimises for *protocol-level* iteration. The actual v1a risk is not protocol-level — the protocol is small (one packet type, dedup-only, frozen test vectors per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md)). The actual v1a risk is **physical**: solar power budget, sealed enclosure thermal/moisture, antenna SWR after pigtail bend, GNSS cold-start in a real garden, BLE link through a sealed plastic box, 18650 behaviour in cold weather. None of those are reproduced by a Pi-with-HAT on a desk. They are reproduced by *more Heltecs in the relay/tag form factor*.

So the Heltec-vs-Pi question is not "either/or about budget" — it's about which class of risk each spend reduces. Heltecs reduce **physical-deployment risk** and **firmware iteration cost**. Pi-with-HAT desk simulators reduce **protocol-iteration risk**, which is already low.

**Existing docs to revise later:** [TODO.md](../TODO.md) "Right now" Heltec line item (3× Wireless Tracker V2) and the [bom.md](../bom.md) cart sanity-check. ARCHITECTURE.md does not currently sort hardware by pool, so the BOM revision is the right place for this.

## 3. Two-Pi gateway model

Same shape of argument applies to Pis. **There is no usable Pi inventory** — the project starts from zero on the Pi side. Every Pi, PSU, SD card, Ethernet cable, and accessory is a fresh purchase. The 3 Dragino LoRa HATs on the desk are still usable (some have bent pins; straightenable per [TODO.md](../TODO.md)) and can be reused; everything else has to be ordered. The question is therefore not "how do we allocate the existing Pis" — it is "how many Pis do we buy from scratch, and what do they each do?"

### Option A — 1 Pi only

- One Pi does everything: live data collection AND gateway/kiosk dev iteration.
- **Tradeoffs.** Cheapest in money. Maximally expensive in *experiment integrity*. Every time the dev cycle touches the Pi — kiosk UI tweak, schema migration, Yocto rebuild — the live deployment stops. SQLite gets touched by a build script, Yocto gets reflashed, the experiment window shrinks. The "weeks of unattended runtime" claim becomes "weeks unless I needed to test something."

### Option B — 2 Pis (deployment + dev)

- **Deployment Pi:** at maman's. Dragino HAT. 7" DSI. RTC. SQLite is canonical. Runs continuously. **Off-limits during the experiment** except for planned, scripted SD-image swap or read-only DB pull.
- **Development Pi:** on the desk. Same image, same hardware fittings as far as practical. Used for kiosk UI iteration, gateway-receiver iteration, SQLite schema work, PMTiles experiments, Yocto rebuilds, Dragino HAT pin-numbering work, anything where "wipe the SD and reflash" is a sentence anyone wants to say.
- **Tradeoffs.** Clean boundary. Deployment Pi becomes the canonical data of record; dev Pi can be reimaged at will. Likely the **minimum** serious setup. From a zero-Pi start, this is **2× Pi 5 + 2× PSU + 2× SD + 1× extra Dragino HAT (the 3 HATs on the desk cover both Pis with one held back as spare)** — material money up front, but bought against the value of an uninterrupted month of real data.

### Option C — 3 Pis (incl. desk-radio simulators)

- Add a third Pi as a desk LoRa node — gateway-style code on a board behaving like a relay or a second receiver, on the same desk as the dev Pi.
- **Tradeoffs.** Only justified if you have a specific *protocol-iteration* question that needs more than one receiver listening at once (e.g. comparing what each gateway hears under controlled distance/attenuation). Per §2 above, that risk is small in v1a — the protocol is intentionally minimal and frozen. **Money probably better spent on Heltecs**, where the risk is concrete and physical. Keep this option as "we can grow into it" rather than "we should buy this now."

**The shape of the recommendation (not the recommendation itself):** **B is the credible v1a floor.** A is too fragile for a month of live data. C is reasonable later but has a worse cost/risk ratio than buying more Heltecs first.

**Existing docs to revise later:** [README.md](../README.md) "Hardware in hand" row currently claims 3 Raspberry Pis on the desk; that row is **stale** — those Pis are dead and only the 3 Dragino HATs remain usable. The row should be rewritten to reflect "3× Dragino LoRa HAT (some bent pins, straightenable); 1× 7" DSI touchscreen; **no usable Pi, no PSU, no SD, no Ethernet — Pi side starts from scratch.**" [TODO.md](../TODO.md) "Right now" desk-inventory note: same correction. [ARCHITECTURE.md §15](../ARCHITECTURE.md) v1a hard gates do not currently distinguish deployment Pi from dev Pi.

## 4. BLE maintenance scope for the sealed relay

[ADR-006](../decisions/ADR-006-relay-has-gnss.md) already pulled BLE maintenance into v1 (not v0, not v2+) with the explicit rationale: *"you cannot deploy a sealed solar relay in a field without a way to verify it is alive without opening the enclosure."* That rationale is the right one. What ADR-006 does not yet do is enumerate the v1a-required surface in enough detail to write firmware against.

**Hard non-goal in v1a: BLE firmware OTA.** The relay does not accept new firmware over BLE in v1a. Firmware update is *physical retrieve → USB-C flash → redeploy*. This is a deliberate constraint, not a regression. Reasons:

- OTA is an attack surface and a brick risk. The deployed relay is sealed and not casually reflashable; an OTA bug is the worst failure mode possible.
- OTA needs a partition scheme, a signed-image story, a rollback story, and a watchdog story. None of that is in scope for v1a.
- Pieter is two car rides away from maman's house, not two continents. Physical retrieve is annoying but not infeasible.

### What BLE *must* provide in v1a (read-mostly health/debug surface)

Mandatory — without these, the relay is opaque on a pole:

- **Identity:** `node_id`, role (string label, e.g. `"relay"`), firmware version + git short hash + build timestamp
- **Uptime:** seconds since boot
- **Power:** battery voltage in mV (if measurable on the Tracker V2 / Solar Kit), charge state if exposed by the Solar Kit, last reset reason
- **GNSS:** last-fix age, last-fix lat/lon (so you can confirm commissioning landed), satellite count if available, GNSS-on-or-off state
- **LoRa counters:**
  - RX packets seen (total since boot)
  - RX packets that passed CRC + MAGIC/VER/TYPE/LEN validation
  - RX packets dropped — *split by reason*: bad CRC, bad header, unknown TYPE, self-echo, dedup hit
  - TX packets enqueued
  - TX packets actually transmitted
  - TX packets dropped on duty-cycle budget
- **Duty-cycle budget counter:** rolling 1-hour airtime in ms, current % of 1% sub-band M cap (per [ADR-014](../decisions/ADR-014-duty-cycle-budget-as-gate.md))
- **Queue depth:** pending rebroadcasts, peak depth seen
- **seen_cache state:** entries currently held, evictions since boot (sanity check on [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) §7 dedup)
- **Recent events ringbuffer:** last N (16–32) decoded events as short fixed-format strings: `"DUP"`, `"SELF_ECHO"`, `"UNKNOWN_TYPE"`, `"BUDGET_DROP"`, `"BOOT"`, `"COMMISSION_OK"`, `"COMMISSION_TIMEOUT"`. Enough to reconstruct what the relay was doing, not a syslog stream.

Strongly recommended — if free in implementation:

- Last RX RSSI / SNR (for the most recent successfully decoded packet — this is **debug-only**, not protocol-state, per [ADR-013 §10](../decisions/ADR-013-multi-hop-flood-via-packet-id.md))
- Ambient temperature inside the enclosure if any onboard sensor gives it for free

### What BLE *may* allow as a config write in v1a

Allow only if there is a clear v1a use case AND a safe default that survives a bad value. Each of these is a *justify-or-omit*:

- **Trigger fresh commissioning broadcast.** Already in [ADR-006](../decisions/ADR-006-relay-has-gnss.md). Useful: lets the engineer push a new self-POSITION without a magnet-on-reed-switch dance. Safe: bounded, idempotent, falls back to forwarding if GNSS times out.
- **Reboot.** Useful for clearing a wedged state. Safe if the firmware boots cleanly. Worth including.
- **Clear counters.** Allowed. Useful for "I just plugged this in, I want a clean slate before I start measuring." Safe — counters are RAM-only observability state and clearing them does not change forwarding behaviour. The BLE response should echo the prior counter values back so the engineer can record the pre-clear snapshot.
- **Clear seen_cache.** **Debug-only, and treated as an experiment-reset event.** Different from clearing counters: the seen_cache is *load-bearing protocol state* per [ADR-013 §7](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) — clearing it can cause the relay to rebroadcast packets it has already forwarded, briefly inflating airtime and potentially producing duplicate `tag_reports` rows on the gateway. If used during an active measurement window, it must be logged as a reset event and the data window must be considered restarted from that point. Should not be exposed in any "operator-friendly" UI; require a confirm-style write or a dedicated debug-build BLE characteristic so it cannot be tapped accidentally.

### What BLE *must NOT* allow as a config write in v1a

These are bricking risks or experiment-invalidation risks. Hold them out of v1a even if they're "easy":

- Frequency / sub-band / SF / BW / CR — wrong value here makes the relay invisible to the gateway, indistinguishable from a dead relay.
- `node_id` change — would invalidate `nodes.toml` mapping ([ADR-013 §9](../decisions/ADR-013-multi-hop-flood-via-packet-id.md)) silently. Set at flash time, not over BLE.
- Firmware feature flags that change protocol semantics.
- Anything that disables duty-cycle enforcement.
- Any factory-reset that wipes commissioned coordinates.
- Firmware OTA. (Already out of scope — restated for completeness.)

### The minimum mental test

Walk up to a sealed relay on a pole. You have a phone. You should be able to answer, in 60 seconds without opening the enclosure: *Is it alive? Is it on time? Is it hearing the network? Is it forwarding? Is it staying within budget? When was the last GNSS fix? What firmware is on it?* If the BLE surface above is implemented, the answer is yes. If anything in the read-mostly list above is missing, the answer is no.

**Existing docs to revise later:** [ADR-006](../decisions/ADR-006-relay-has-gnss.md) lists BLE as v1 with a thin minimum-viable line. A follow-up ADR (or an ADR-006 update) should adopt the §4 surface above as the v1a contract. [TODO.md](../TODO.md) currently has "BLE maintenance CLI on the relay" in the **Deferred (v2+)** list — **this contradicts ADR-006** and needs reconciliation either way (see contradictions section at the end). [ARCHITECTURE.md](../ARCHITECTURE.md) does not currently have a BLE section.

## 5. Gateway data continuity

The deployed gateway is going to write canonical SQLite for weeks. The deployed gateway is also the only node in the system with hardware that anyone is tempted to develop against. Those two facts fight each other unless we say so explicitly.

### Option A — Single SQLite DB on deployed Pi, manually copied when needed

- **Tradeoffs.** Simplest. Implicit assumption: nothing developmental happens on the deployment Pi. Manual copy = `scp` from maman's Pi back to the desk on visits. Fragile because there is no policy for what happens if the deployment Pi misbehaves mid-experiment.

### Option B — WAL mode + periodic snapshot

- WAL is already required by [ADR-009](../decisions/ADR-009-database-sqlite.md) for concurrent reader/writer.
- Snapshot = periodic `VACUUM INTO` or hot copy (WAL-safe) to a parallel `tag_reports.snapshot.$DATE.sqlite` on the same SD or, better, on a USB stick.
- **Tradeoffs.** Cheap insurance against SD corruption mid-run. Snapshots can be pulled to the dev Pi on visits without touching the live DB. **Almost certainly worth doing** — the cost of losing a month of data to an SD failure is large; the cost of cron'ing a snapshot is small.

### Option C — Gateway receiver writes canonical DB, kiosk reads read-only

- Already structurally implied: the kiosk module is part of the same gateway binary ([CLAUDE.md](../CLAUDE.md), [ARCHITECTURE.md](../ARCHITECTURE.md)). But "same binary" doesn't automatically mean the kiosk side opens the DB read-only.
- Make this explicit: kiosk side opens with `?mode=ro` (or rusqlite read-only flags). The receiver side is the only writer. Eliminates an entire class of "the kiosk wrote something during a render" bugs.
- **Tradeoffs.** Pure win. Not a hardware question — a code-shape question. Belongs in the persistence-crate spec.

### Option D — Lab Pi replays copied DBs for UI/dev work

- Dev cycle: copy a deployed `tag_reports.sqlite` to the dev Pi, point the dev kiosk at it read-only, iterate. The kiosk-lab already has the synthetic-scenario story for UI work; this adds a "real data, frozen" path.
- **Tradeoffs.** Cheap, works without any new infrastructure. Worth setting up so kiosk iteration *never* runs against live data.

### Migration policy during active deployment

A schema migration on the deployed gateway during an active run is a reset event — the run effectively ends, because the data shape changes mid-stream. Therefore: **no schema migrations on the deployment Pi during an experiment.** If a migration is needed, it is treated like a planned outage: announce, snapshot, retrieve, migrate on the dev Pi, ship a new image, redeploy.

### "Do not touch" rule

The deployment Pi runs a known-good image for the duration of the experiment. The only touch operations allowed are:

- planned snapshot pull (read-only)
- planned reboot (RTC + chrony hold the time)
- emergency end-of-run pull (which ends the experiment)

Anything else is dev work and goes on the dev Pi.

**Existing docs to revise later:** [ADR-009](../decisions/ADR-009-database-sqlite.md) covers WAL but not the snapshot/replay workflow. [ARCHITECTURE.md §15 v1a](../ARCHITECTURE.md) acceptance criteria do not currently mention "no migration during experiment."

## 6. Radio/debug observability without lying in the protocol

Constraint from [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md): the wire format does not carry per-hop RSSI/SNR, hop count, or path arrays. The kiosk-lab already enforces this on the UI side (no "via relay-X" annotations, no path lines — see [tools/sarcom-kiosk-lab/README.md §SARCOM v1 truth](../tools/sarcom-kiosk-lab/README.md)).

Development still needs to know *whether the relay is forwarding*. The honest separation:

| Lane | Where it lives | What it can show | What it must NOT show |
|---|---|---|---|
| **Production protocol** | POSITION packet on the wire | Only what's in the [ADR-013 §3](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) frame | Hop count, RSSI, path |
| **Operator kiosk UI** | `gateway/src/ui/` | Tag positions, freshness, SOS, relay self-announce, RTC validity | Anything the protocol can't prove from a single packet |
| **Development / debug** | Relay BLE counters, gateway logs, bench serial logs, an *optional* lab-only debug packet type behind a build flag, LED blink codes, field notebook timestamp correlation | Whatever helps reproduce a problem | Anything that ships in a deployed firmware build |

Concrete dev-time observability options, ranked by how invasive they are:

1. **Relay BLE counters** (§4 surface). RX-passed-CRC, TX-emitted, dedup hits. Tells you the relay is forwarding without putting anything on the wire.
2. **Relay event ringbuffer** (§4 surface). Reconstruct the last N decisions.
3. **Gateway-side packet counters.** What did the gateway hear. Compare against the relay's TX-emitted count from BLE → that's how you measure end-to-end deliverability without per-hop fields in the protocol.
4. **Bench-twin serial logs.** Same firmware, USB-C tethered, full UART logs. Reproduces the scenario you saw on the deployed unit's BLE counters.
5. **LED blink codes on the relay.** Cheap, but only useful when the engineer is standing at the pole.
6. **Field notebook timestamp correlation.** Walk past the relay at a noted time, then check both the BLE recent-events ringbuffer and the gateway's `tag_reports` for that timestamp. This is the human-readable end-to-end check.
7. **Lab-only debug packet type.** Last resort, behind a `--features lab-debug` build flag, *never* on a deployed firmware. Adds a row to [ARCHITECTURE.md §13](../ARCHITECTURE.md) duty-cycle budget per [ADR-014](../decisions/ADR-014-duty-cycle-budget-as-gate.md). Probably not needed in v1a if 1–6 are present.

The principle: **protocol semantics are minimal and frozen; debug instrumentation lives off-wire (BLE, serial, logs); the kiosk UI only shows what the protocol can prove.**

**Existing docs to revise later:** none directly — the ADRs are already coherent. A BLE-scope ADR (per §4) would absorb most of items 1–3 above.

## 7. Tag commissioning and daily use

Tags are retrievable; relays are not casually retrievable. That asymmetry is the whole reason §4 exists for relays. For tags, the asymmetry runs the other way: it's fine if a tag needs a USB-C cable now and then, because the tag is in a pocket.

The question is: *how does each physical tag get its identity, and how do humans know which tag is which?*

### Options for tag identity assignment

- **Build-time `node_id`** — the value is baked in at compile time, but sourced from a per-device input rather than a hardcoded constant. Plausible mechanisms: a generated `config.rs` / `node_id.rs` written by `build.rs` from an env var (`SARCOM_NODE_ID=7 cargo build --release`), a small per-device TOML/JSON config the build script reads, or a `cargo` profile that pins the value. Same one-build-per-device shape, but the per-device input is data, not a feature flag. Cheapest mechanism that scales to a 5–10 tag fleet without a feature-flag-per-id explosion. Mistakes are visible (wrong build = wrong `node_id` shows up in the kiosk).
- **NVS-stored config** set once via a dedicated commissioning firmware, then normal firmware reads NVS at boot. Good story for production: same firmware binary on every tag, identity programmed once. More machinery than v1a strictly needs but a credible upgrade path.
- **Boot-button mode** to enter a tag-id-set state. Adds a UX surface on the tag. Probably more machinery than v1a needs.
- **BLE commissioning of `node_id`.** Adds attack surface and config-mistake risk. **Per §4, this is on the MUST NOT list for v1a.** `node_id` is not BLE-writable in v1a, full stop.
- **Sticker / QR / manual mapping** — the *physical-world* layer that sits next to whatever mechanism above. Required regardless: humans need to be able to look at a tag and know its `node_id`.
- **Hardcoded test-fleet table** in the gateway's `nodes.toml` ([ADR-013 §9](../decisions/ADR-013-multi-hop-flood-via-packet-id.md)). This is what makes a `node_id` show up as `"tag-walker-1"` in the kiosk.

### Recommended shape for v1a (analysis, not decision)

For a 5–7-tag fleet, **build-time `node_id` + sticker on the case + `nodes.toml` entry** is the minimum that works without building infrastructure. The mechanism on the build side is *not* a feature flag per id — it is a per-device input read at build time, e.g. `SARCOM_NODE_ID=7 cargo build --release` (read by `build.rs` and codegen'd into a constant), or a tiny per-device config file the build script ingests. The sticker says `tag-7 / Pieter`; `nodes.toml` says `[nodes.7] label = "Pieter walker" ui_kind = "hiker"`. Three places, all human-readable, all version-controllable. **The principle that survives any mechanism choice: `node_id` is not BLE-writable in v1a.**

Open questions worth answering when this becomes real:

- Which build-time mechanism is least error-prone in practice — env-var-via-`build.rs`, a per-device config file, or a cargo profile? Pick during firmware bring-up, not now.
- Does the build-time `node_id` survive a misflash gracefully? (Probably yes — the worst case is a tag that reports as the wrong `node_id`, which is visible in the kiosk because the label/sticker mismatch will be obvious.)
- Do we need a `commissioning` firmware vs `running` firmware distinction for tags? (Probably no for v1a — tags don't need GNSS-disable like relays do, GNSS is *the* feature.)
- Does the gateway need to flag unknown `node_id`s as "tag-?" rather than dropping them? (Yes — `nodes.toml` should be additive, unknown ids should appear with a sentinel label.)

**Existing docs to revise later:** [ARCHITECTURE.md §11](../ARCHITECTURE.md) `nodes.toml` schema definition (already on the [TODO.md](../TODO.md) list under "While hardware is in transit"). The TODO entry should also specify an "unknown node_id" UI behaviour.

## 8. Month-long garden experiment acceptance criteria

[ARCHITECTURE.md §15 v1a](../ARCHITECTURE.md) currently lists *"Relay 72 h unattended on solar"* as a hard gate. 72 hours is a sensible **first** gate but it is not the deployment story. A real v1a should be staged.

### Suggested staging (analysis, not decision)

**Pass condition (v1a "passes" at this line):**

- 72 h relay uptime on solar, no manual intervention
- Daily tag walks during the 72 h window producing real `tag_reports` rows in canonical SQLite
- Relay BLE health check at end of window: identity, uptime, RX-passed-CRC > 0, TX-emitted > 0, duty-cycle counter < 1%, last-fix age plausible
- Kiosk renders correctly throughout: dot moves on walks, "no fix" routes to side list (not a fake marker), `RTC NOT SET` if RTC fails
- Entire stack untouched by the internet
- Duty-cycle measured ±10% of [ARCHITECTURE.md §13](../ARCHITECTURE.md)

**Stretch condition (v1a "is real" at this line):**

- 1 week relay uptime on solar including at least one cloudy day
- Successful BLE health check after 7 days without retrieving the relay
- SQLite snapshot at day 7 retrieved without ending the run (per §5)
- Kiosk has handled at least one SOS trigger correctly (button or bench)

**Hardening condition (v1a "ships to a mountain hut" at this line):**

- 1 month relay uptime
- No manual relay retrieve during the month
- SQLite preservation verified at the end (no corruption, complete row range)
- Kiosk has handled stale/no-fix/SOS states correctly with real data, not just synthetic kiosk-lab scenarios
- Cold-night behaviour observed (Li-Ion below 0°C is [open risk #5 in ARCHITECTURE.md §16](../ARCHITECTURE.md))

**Out of v1a (v1b / v2+):**

- Drone-pod overlay (gated on v1a passing per [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md))
- Multi-tag scale (per [ADR-014](../decisions/ADR-014-duty-cycle-budget-as-gate.md), two simultaneous SOS tags is over budget)
- Multi-relay duty-cycle interaction
- Real mountain deployment (per [TODO.md](../TODO.md) Blocked: gated on garden v1a passing)

**Existing docs to revise later:** [ARCHITECTURE.md §15 v1a](../ARCHITECTURE.md) currently has a single uptime number. A staged version that names a pass / stretch / hardening line lets the project decide consciously when v1a is "done enough" for v1b to start without inventing a new gate mid-run. [TODO.md](../TODO.md) v1a section currently lists the 72 h soak only.

## 9. Cost-control framing

Pieter is unemployed. Runway matters. The answer is not "buy nothing" — that locks the project at desk-demo scope and wastes the architecture work already done. The answer is to spend where the spend reduces a concrete project risk, and not where it doesn't.

| Spend | Risk reduced | Approx. cost | Priority |
|---|---|---|---|
| 4 extra Heltec Wireless Tracker V2s (3 → 7) | Firmware iteration cost during deployment; bench-twin debugging; physical-deployment realism | €120–€160 | **High** |
| 2× Pi 5 + 2× PSU + 2× SD (zero existing Pi inventory; 3 desk HATs reusable) | Experiment integrity (live data run isn't interrupted by dev work); also: there is no Pi to use at all without this spend | €240–€260 | **High** |
| 1× Pi 5 + 1× PSU + 1× SD only (single-Pi fallback if runway is tight) | Boots a gateway at all; deployment and dev fight each other | €120–€140 | **Medium — fallback** |
| Snapshot USB stick + small UPS for deployment Pi | SD corruption / power blip protection during a month-long run | €30–€60 | **Medium** |
| Multimeter / USB current meter (already on TODO desk-hygiene order) | Power-budget verification, solar-charge debugging | €15–€30 | **Medium** |
| Spare 18650 cells beyond the 2+2+2 already on order | Replace cells that fail the cold-night test without ending the run | €15–€30 | **Medium** |
| Better 868 MHz antenna + spare pigtail | Antenna/pigtail issues showed up as a real risk mode in [bom.md](../bom.md) sanity-check | €20–€40 | **Medium** |
| 3rd Pi for desk-radio simulator | Protocol-iteration scenarios (already low risk in v1a) | €100+ | **Low — defer** |
| 6+ extra Heltecs (7 → 10+) | v1b prep, destructive testing, deeper spares | €120+ | **Low — buy after v1a is real** |
| Logic analyser / SDR | Deep radio debugging (likely v1b territory) | €30–€400 | **Low — buy on demand** |

**All prices in the table above are placeholders for relative ranking only and must be vendor-checked against current Heltec DE / Tinytronics / Amazon listings during the BOM revision before any order is placed.**

**The shape:** the Pi side cannot start at €0 — there is no usable Pi on the desk, so a working v1a needs **at least 1× Pi 5 + PSU + SD** (~€125) just to boot a gateway. Going from the 1-Pi fallback to the 2-Pi (deployment + dev) shape costs roughly the same again on top, and is what buys experiment integrity per §3. On the Heltec side, going from the 3-board draft BOM to the 5-board minimum is ~€50 marginal; 5 → 7 is another ~€50; 7 → 10 is ~€75 and is v1b/destructive-test budget. Marginal returns drop fast above the 5-Heltec / 2-Pi shape until v1b is on the table.

This is a risk-reduction frame, not a permission slip. The decision still has to live with runway.

**Existing docs to revise later:** [bom.md](../bom.md) — the next BOM revision should sort line items by which risk they reduce, not by vendor.

## 10. Closing — which docs/ADRs would need revision later

If the analysis above is accepted, the downstream edits are roughly:

- **[bom.md](../bom.md)** — Heltec count goes from 3 to 5/7/10 depending on the chosen pool model; **add Pi 5 + PSU + SD line items from scratch** (zero existing Pi inventory; the 3 desk Dragino HATs are reusable); add snapshot USB; sort by risk-reduced.
- **[TODO.md](../TODO.md)** —
  - "Right now" Heltec line item: revised count.
  - "Right now" parallel order: add **2× Pi 5 + 2× PSU + 2× SD + Ethernet cabling** (the desk has no usable Pi or accessories — only the 3 Dragino HATs survive).
  - **Reconcile contradiction**: BLE maintenance CLI is currently in **Deferred (v2+)** while [ADR-006](../decisions/ADR-006-relay-has-gnss.md) puts it in v1. Pick one.
  - v1a section: rewrite uptime gate as pass / stretch / hardening per §8.
  - Add a "Deployment vs. development" subsection clarifying which Pi and which Heltec is which pool (per §1).
- **[README.md](../README.md)** — "Hardware in hand" row is currently stale (claims 3 Pis); rewrite to reflect that the desk Pis are dead and only the 3 Dragino HATs + 7" DSI touchscreen remain usable. After the 2-Pi model is adopted, the row should also distinguish deployment Pi from dev Pi.
- **[ADR-006](../decisions/ADR-006-relay-has-gnss.md)** or a new ADR — adopt §4 BLE surface as the v1a contract; explicitly mark BLE OTA out of scope; enumerate read-mostly fields and the small allow-list of writes.
- **New ADR (or [ADR-013](../decisions/ADR-013-multi-hop-flood-via-packet-id.md) extension note)** — make the protocol/kiosk/debug separation in §6 explicit.
- **[ADR-009](../decisions/ADR-009-database-sqlite.md)** or [ARCHITECTURE.md §10](../ARCHITECTURE.md) — write down the snapshot/read-only-kiosk/dev-replay workflow from §5.
- **[ARCHITECTURE.md §15 v1a](../ARCHITECTURE.md)** — staged acceptance criteria per §8; "no schema migrations on deployment Pi during run" rule.
- **[ARCHITECTURE.md §11](../ARCHITECTURE.md)** — `nodes.toml` schema (already on TODO) plus an unknown-`node_id` UI fallback.

None of these edits happen in this spike. This spike is the framing that makes them safe to write.

## Decision note template

```
Date:
Heltec count chosen: 3 / 5 / 7 / 10 / other:
Pi count chosen: 1 / 2 / 3
Deployment Pi physical location: maman's garden / other:
Dev Pi: yes / no
BLE v1a surface: §4 read-mostly / smaller / larger
BLE writes allowed in v1a: [list — must be subset of §4 allow-list]
BLE OTA in v1a: explicitly out
v1a pass condition: 72 h / 1 week / 1 month / other:
Snapshot strategy: §5 option chosen
Tag identity strategy: build-time (env-var / per-device config / cargo profile) / NVS / boot-button / other:
node_id BLE-writable in v1a? must be NO
Cost envelope chosen: €
Next action:
```
