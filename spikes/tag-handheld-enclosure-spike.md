---
title: "Spike — Tag custom 3D-printed enclosure (Fusion 360)"
status: open
type: spike
timebox: 1 day
opened: 2026-05-06
---

# Spike: Tag custom 3D-printed enclosure

## Why this spike exists

The 2026-05-06 pivot states custom Fusion 360-designed waterproof casings for **both gateway and tag**. Until now ADR-002 specified the bare Heltec Wireless Tracker V2 board with a 18650 cell — no enclosure spec. Tag operating reality is now:

- carried in a pocket or strapped to a person's chest/shoulder
- exposed to sweat, rain, snow, occasional drops
- requires a working SOS button reachable through the shell
- requires a working buzzer audible through the shell (per ADR-012 surviving the ADR-013 rollback — buzzer is the last-meter audible cue)
- USB-C charging port reachable without breaking the seal
- visual identity (label / sticker / colour) so a hut-staff member can tell tag-1 from tag-7 at arm's length (per `field-deployment-test-fleet-spike.md` §7 tag identity discussion)
- IPEX → SMA antenna path, OR an internal antenna behind the shell wall

The tag enclosure is materially different from the gateway enclosure: smaller, no display, has a buzzer (must transmit sound out), much higher count (5–10 units in the test fleet), and worn against a body.

## Hypothesis / research question

**H1.** A pocket-form-factor 3D-printed shell at **IP54** (splash, dust-protected) with a sealed tactile SOS button, a vented buzzer port (acoustic mesh + drain hole, not an open vent), a USB-C panel-mount-with-cap, and an internal IPEX→SMA pigtail to a side-mounted SMA stub antenna meets v1a needs. IP67 is over-engineered for a body-worn tag and complicates the buzzer.

**H0.** A printed shell + commercial silicone phone-case-style sleeve gives "good enough" splash protection without re-solving the seal problem; v1a accepts that tag IP rating is bag-or-sleeve dependent.

## Scope fence

- **No CAD modelling.** Spike output is a design brief.
- **No firmware coding.** Button + buzzer GPIO assignments live in the tag firmware bring-up (currently blocked in TODO.md "Blocked"); this spike notes those choices but does not pick GPIOs.
- **No SOS button mechanism re-litigation.** ADR-010 / ARCHITECTURE.md §8 already say SOS is a button press; the question here is *physical* button, not the immediate-first-frame logic.
- **No PCB redesign of the Tracker V2.** The tag is the OEM board; the enclosure works around it.
- **No gateway enclosure overlap.** That is `gateway-handheld-enclosure-spike.md`.
- **No relay enclosure changes** (ADR-003).

## What to verify

### Form-factor target

- pocket-friendly: long axis ≤ 110 mm, thickness ≤ 25 mm with cell installed
- weight goal: ≤ 150 g all-up
- attachment options: lanyard hole, optional belt clip channel, optional MOLLE-strap slot — pick one v1 default, leave others as v2

### IP rating target

- **IP54** (splash, dust): working v1a target, body-worn realism
- **IP67** stretch: harder because of the buzzer; would force a vented gore membrane port for sound and a more careful USB-C seal

### Materials

- PETG default (cheaper, easier to print, decent toughness) or ASA (better UV / thermal). Same call as gateway spike but at smaller volume / lower thermal load.
- Optional silicone over-mould for grip — defer to v2.

### Sealing strategy

- **Main seam:** clamshell with printed gasket groove + foam rope OR ultrasonic-weld (out of envelope for hobby print) OR adhesive seal + heat-set screw service points.
- **SOS button:** sealed tactile switch (e.g. APEM IP67 PB series equivalent) OR silicone membrane bonded over a domed printed bezel covering an ordinary tactile switch. Membrane is cheaper, sealed switch is more durable.
- **Buzzer port:** acoustic mesh (Gore acoustic vent or generic PTFE membrane) bonded to the shell — passes sound, blocks water and dust. Not an open hole.
- **USB-C charging port:** panel-mount USB-C with rubber cap (tethered) OR magnetic Pogo. Pogo loses the ability to charge with any USB-C cable but solves the seal cleanly.
- **Antenna path:** IPEX → SMA pigtail to a side-mounted SMA bulkhead with built-in O-ring + stub antenna. Alternative: internal antenna with the shell wall thinned to a documented thickness; verify acceptable RF loss in PETG/ASA at 868 MHz.
- **Optional sky-window for GNSS:** thin section of the shell on the upper face to reduce GNSS attenuation while the tag is body-worn.

### Internal layout

- Tracker V2 board outline (dimensions known from `crates/heltec-wireless-tracker-v2-bsp` planning; verify on the physical board when units arrive)
- 1× 18650 cell holder vs soldered cell — soldered is smaller but eliminates user-replaceable cells (`production-concerns.md` §2 cold-charge concerns shift if cells aren't replaceable)
- buzzer placement (close to the buzzer port, not buried under the PCB)
- IPEX strain relief at the board side of the antenna pigtail (same problem class as gateway / `production-concerns.md` §3)
- visual identity: a flat face for a printed sticker or a recessed bezel for a printed-in identifier

### Tag identity surface

The enclosure is the right place to make a tag visually distinguishable. Options (decide):

- printed-in side colour band per node_id range
- printed-in raised number on the face (matches `node_id` from build-time config per `field-deployment-test-fleet-spike.md` §7)
- sticker pocket with a swappable printed insert
- combination

### Cross-spike implications (record, don't solve)

- `field-deployment-test-fleet-spike.md` §7: tag identity strategy ties to enclosure visual identity.
- `gateway-handheld-enclosure-spike.md`: shared sealing-technique research; reuse what works.
- TODO.md "Blocked": SOS button GPIO + button type — this spike chooses the **physical button** (sealed tactile vs membrane); GPIO/firmware-side is unblocked separately.
- `production-concerns.md` §2 (cold charging) and §3 (IPEX strain relief).

## Pass criteria

- IP target chosen (54 / 67 / fallback) with reasoning.
- Material chosen.
- Sealing strategy named for each surface: main seam, SOS button, buzzer port, USB-C, antenna path, optional GNSS sky-window.
- Internal layout sketch (text + dimensions): board placement, cell strategy (replaceable vs soldered), buzzer placement, strain relief, identity surface.
- Cross-spike implications recorded.

## Buzzer audibility — measurable bench criterion

**Pass criterion:** ≥80 dBA at 1 m on bench after enclosure sealing, measured with a phone SPL meter or equivalent rough instrument (no acoustic-lab certification needed). Measurement is taken in a quiet indoor space with the tag oriented so the buzzer port faces the meter.

Field audibility at 5 m in moderate wind remains an **observational note**, not the pass criterion — record the field result in the decision note as evidence, not as the gate.

## Fail criteria

- Buzzer audibility on bench drops below ≥80 dBA at 1 m through the chosen acoustic vent + wall thickness — fall back to a larger vent area, change vent material, or move buzzer placement.
- IPEX strain relief inside the smaller tag shell cannot be solved without an internal-antenna pivot — accept internal antenna at v1, document RF-loss expectation.
- The chosen visual-identity scheme requires per-tag print files (not just inserts) — adds operational cost; record explicitly.

## Fallback / next action

- If H1 holds: write the Fusion 360 modelling ticket with this brief. Print + body-worn test for 24 h before committing.
- If H0 (sleeve-dependent): document the tag-handling procedure clearly; sleeve is operational kit, not a v2 polish.

## Decision note template

```
Date:
IP target chosen: 54 / 67 / fallback — reason:
Material chosen: PETG / ASA / other — reason:

Form factor:
  long axis target: __ mm   thickness: __ mm   weight: __ g
  attachment: lanyard / belt clip / MOLLE / other:

Sealing strategy:
  main seam:    type:
  SOS button:   sealed tactile / silicone membrane / other:
  buzzer port:  acoustic mesh spec:
  USB-C:        panel-mount-with-cap / Pogo / other:
  antenna path: IPEX→SMA stub / internal antenna — wall thickness if internal:
  GNSS sky-window: yes / no — placement:

Internal layout:
  cell: replaceable holder / soldered:
  buzzer placement: near vent? yes:
  strain relief at IPEX: yes / N/A:
  identity surface: printed-in number / colour band / sticker pocket / combo:

Buzzer audibility:
  bench measurement (≥80 dBA at 1 m): __ dBA at 1 m
  field observation at 5 m in wind: ___

Operating-procedure caveats accepted:
  - cold charge: ___
  - cell service interval: ___
  - sleeve required for heavier rain (if H0 / IP54): ___

Cross-spike implications recorded:
  field-deployment §7 tag identity:           ___
  gateway enclosure shared techniques:         ___
  TODO.md SOS button physical decision:        ___
  production-concerns §2 (tag cold charge):    ___
  production-concerns §3 (IPEX strain relief): ___

Not implemented in this spike: CAD geometry, print profiles, prototype fabrication, GPIO assignment.

Next action:
```

## Cross-references

- `decisions/ADR-002-tag-hardware.md` — base tag ADR; this spike is the missing enclosure annex.
- `decisions/ADR-010-sos-encoding.md` — SOS button immediate-first-frame logic; physical button choice does not change that.
- `decisions/ADR-012-node-roles-and-sighting-semantics.md` — tag buzzer survives ADR-013 rollback.
- `production-concerns.md` §2 (cold charging) and §3 (IPEX strain relief).
- `spikes/field-deployment-test-fleet-spike.md` §7 — tag identity strategy.
- `spikes/gateway-handheld-enclosure-spike.md` — shared sealing techniques.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
