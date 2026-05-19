---
title: "self-ann field name reified an invented frame type — third subtyping-fetish instance"
date: 2026-05-19
status: incident-recorded
type: dev-log
---

# `self-ann` field name reified an invented frame type — third subtyping-fetish instance

## What surfaced

During a re-run of the mockup orchestrator (`tickets/mockup-prompts/00-RUN-ALL.md`), Claude produced KIOSK-008 and re-validated the five existing per-ticket mockups (KIOSK-001/003/004/005/006). Pieter, reviewing the result, spotted the string `self-ann 14 m` / `self-ann 65 m` on relay rows across the mockups and called it out:

> "ge hebt nu een netwerk dat zonder routing table POSITION packets broadcast in een soort repeater pattern. ge hebt een broadcaster die position packets uitroept en ge hebt repeaters/relays die packets forwarden zonder acks zonder tables zonder iets. en nu gaat ge ffkes zo in de ux flows voor een of andere reden ookal staat er overal in de docs dat nie de bedoeling is gaat ge self-ann en weet ik veel wat allemaal batterij status enzo van relays via een... wat? een apart protocol met een nieuwe sturen naar de gateway? what the fuck?"

The string `self-ann` lexically implies a "self-announce" frame kind separate from POSITION. **No such frame exists.** Per `decisions/ADR-013-multi-hop-flood-via-packet-id.md`, the v1 wire protocol carries exactly one packet type (POSITION). A relay surfaces its presence by broadcasting POSITION packets with its own `node_id`; `nodes.toml` assigns the role. There is no second protocol, no second frame, no separate flow.

## Root cause

The wrong wording originated in `tools/sarcom-kiosk-lab/src/data.rs` (pre-incident state):

```rust
/// Age of the most recent relay self-announce frame received by the gateway.
#[serde(default)]
pub self_ann_age_secs: Option<f32>,
```

Field name + doc-comment together reified a frame kind that does not exist on the wire. The field's actual semantic is "age of the most recent POSITION packet received from this relay's `node_id`" — mechanically identical to `TagData.last_seen_secs:135`. **Lexical** subtyping, not structural, but same failure mode as the two prior instances.

Twelve relay-row strings across the v1a per-ticket mockups (`KIOSK-001/003/004/005/006`) plus six strings in the legacy umbrella inherited the wording uncritically when Claude generated the mockups from the data model.

The schema-extension-discipline three-question check in `CLAUDE.md` had the answers required to catch this:

| Question | Answer for `self_ann_age_secs` |
|---|---|
| 1. Is the distinction operator-visible at runtime, or only provenance? | Only provenance — operator sees "last heard X min ago" for both tag and relay |
| 2. Does an existing type already use an attribute-pattern for this category? | Yes — `TagData.last_seen_secs` |
| 3. Does the new variant force every downstream consumer into an extra branch? | Yes — every UI/log/test path has a parallel branch |

All three answers point to "attribute on existing primitive, not new type." The check existed; it was not run when the field was first authored, and it was not re-run when the mockup orchestrator inherited the wording.

## Why this is the third instance

1. **LoRa relay-role near-miss.** Nearly added `drone_relay` / `fixed_relay` as wire-level POSITION subtypes. Correct shape: one `relay` role with `nodes.toml` config per ADR-013.
2. **OSM overlay subtypes (2026-05-16).** Shipped `osm_overpass` as a peer kind of `osm`; collapsed to `kind = "osm"` + `source = "file" | "overpass"`. See `dev-log/2026-05-16-osm-overlay-collapse-subtypes.md`.
3. **`self_ann_age_secs` (2026-05-19, this entry).** Field name + doc-comment + 18 downstream operator-facing strings.

Pieter's diagnosis verbatim:

> "DIT IS EEN MENTALITEITSPROBLEEM, EEN ATTITUDEPROBLEEM, NIET EEN GEBREK AAN DOCUMENTATIE OF NIET GOED GENOEG GEPROMPTED DOOR PIETER PROBLEEM."

The documentation is in place (CLAUDE.md schema-extension-discipline §, ADR-013, `MEMORY.md` `feedback-no-subtyping-fetish`). The check exists. What fails is running it in the proposal each time, particularly when the wording is **inherited from existing code** rather than freshly authored — that is exactly the case where the reflex skips the check.

## Mental-model lock

Recorded explicitly so the rule survives the next session boundary:

- **POSITION is the single network primitive.** Anything a node ever tells the gateway — node_id, position, timestamp (gateway-filled per `dev-log/2026-05-19-gateway-rx-timestamps-as-position-field.md`), battery, flags, future fields — is a **field on POSITION**, not a new frame.
- **Tag vs relay is a `nodes.toml` config distinction**, not a wire-level distinction. Per ADR-013 there is no wire-level role enum.
- **The simplicity is the strength.** No routing, no discovery, no handshake, no per-role frame types, no acknowledgements — preserve it. Reflexes that reach for new packet types / new flows / new protocol concepts when a node-attribute appears are wrong by default for SARCOM.

## Corrective work landed

Commit `67ed961` (pushed to `origin/main`):

**Code:**

- `tools/sarcom-kiosk-lab/src/data.rs:154-156` — `RelayData.self_ann_age_secs` → `last_seen_secs`. Doc-comment → `"Age of the most recent frame from this relay (POSITION; one packet type per ADR-013)."`
- `tools/sarcom-kiosk-lab/src/data.rs:5-6, 40, 463` — cadence comments and test comments: `self-announce` → `POSITION cadence`. Module-header comment now explicitly states: `"Same packet kind as tag POSITION per ADR-013 — there is no separate 'self-announce' frame."`
- `tools/sarcom-kiosk-lab/src/data.rs:351` — `default_relay()` initializer field name updated.
- `tools/sarcom-kiosk-lab/src/ui/sidebar.rs:193, 214, 217, 230` — field access × 2; UI strings: `"  self-ann {} · {}"` → `"  POSITION {} · {}"`; `"  self-ann — · no frame rx"` → `"  POSITION — · no frame rx"`.
- `tools/sarcom-kiosk-lab/README.md:148-153` — freshness table row `Relay self-announce` → `Relay POSITION`; paragraph rewritten to state "Same packet kind as tag POSITION per ADR-013 (one packet type); there is no separate 'self-announce' frame on the wire."

`cargo check` + `cargo test --no-run` clean after rename.

**Mockups (18 strings):**

| File | Substitutions |
|---|---|
| `UX/mockups/KIOSK-001-map-scale-north.svg` | 1 (relay-1 marker label) |
| `UX/mockups/KIOSK-003-sidebar-row-redesign.svg` + `.md` | 4 (sidebar row + state table Healthy + Overdue) |
| `UX/mockups/KIOSK-004-selection-detail-sidebar.svg` + `.md` | 5 (relay-2 marker, state strip, detail-row label, field annotation cite, field-table row) — field-name citation in annotation: `self_ann_age_secs` → `last_seen_secs` |
| `UX/mockups/KIOSK-005-gateway-status.svg` | 4 (relay-1 row × 4 panels) |
| `UX/mockups/KIOSK-006-sos-strip.svg` | 2 (relay-1 row × 2 panels) |
| `UX/mockups/v1a-operator-map-mockup.svg` + `.md` | 6 (header comment + 4× `self-ann 14 m` + 2× `self-ann 65 m` + table + tekst) |

All seven touched SVGs (six active + umbrella) re-validated `[PASS]`. KIOSK-008's mockup, produced fresh in the same session, never carried the wrong wording.

## What changed in memory

`memory/feedback_no_subtyping_fetish.md` updated:

- Third instance added to the catalogue.
- Mental-model lock made explicit ("POSITION is the single primitive; tag vs relay is a `nodes.toml` config distinction, not a wire-level distinction").
- Title/description sharpened to emphasise running the three-question check IN the proposal even when wording is inherited from existing code — especially then, because the code itself may have already failed the check.

`memory/MEMORY.md` index line updated to reflect the third instance + the stronger framing.

## What this entry does NOT change

- No ADR is amended. ADR-013 already said this.
- No new ticket is opened.
- No mockup is regenerated beyond the string substitutions.
- The dev-log entry `2026-05-19-gateway-rx-timestamps-as-position-field.md` is unaffected — it is about a different POSITION-field topic (gateway-RX-filled timestamps), not about the protocol-frame-kind question this entry is about.

## Cross-references

- `decisions/ADR-013-multi-hop-flood-via-packet-id.md` — v1 protocol = one packet type (POSITION); no wire-level role enum.
- `CLAUDE.md` Schema-extension discipline § — the three-question check.
- `dev-log/2026-05-16-osm-overlay-collapse-subtypes.md` — second instance.
- `memory/feedback_no_subtyping_fetish.md` — three-instance catalogue + mental-model lock.
- Commit `67ed961` — `feat(ux,kiosk-lab): v1a per-ticket mockups + relay last_seen rename`.
