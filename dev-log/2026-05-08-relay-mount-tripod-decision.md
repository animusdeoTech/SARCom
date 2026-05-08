---
date: 2026-05-08
type: dev-log
session-trigger: "Wooden-pole + Fusion-360-pole retired for v0/v1/v2; off-the-shelf plastic tripod chosen as the relay mount path"
---

# Decision note — relay mount: off-the-shelf plastic tripod for v0/v1/v2

## Verdict

The relay mount for **v0 (mama's garden)**, **v1 (Terril Waterschei)**, and **v2** is an **off-the-shelf plastic tripod** with a standard mount thread, plus an adapter between the tripod head and the Heltec Solar Kit enclosure. Two prior approaches are retired for v0/v1/v2:

- [ADR-003](../decisions/ADR-003-relay-hardware.md) §Decision + Order checklist: *"u-bolts or stainless hose clamps on a pressure-treated wooden pole"*.
- [bom.md](../bom.md) §"Relay pole — local build": *"Fusion 360 three-legged base + ground-stake, built at the woodworking shop"*.

ADR-003 hardware (Tracker V2 + Solar Kit + adhesive PCB standoffs + 3M VHB) is **unchanged**. Only the mount-to-ground path moves.

Tripod model + adapter dimensions + screw thread + load rating + folded length are owned by [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md). This dev-log commits the **path**, not the parts.

## Rationale

- **v0/v1 deployment context.** Mama's garden is a flat lawn with the relay 5–10 m from the gateway during bring-up; Terril Waterschei is a single-day field test. Neither is a winter-long unattended deployment. Both tolerate a portable tripod that can be carried in, set up, set down, and recovered at end of day.
- **Wooden-pole + hose-clamps approach is overkill** for v0/v1/v2. Pressure-treated lumber, ground-stake hole, hardware-store fastenings — sized for an unattended seasonal pole, which is a v3+ concern, not a v1/v2 concern.
- **Fusion-360 designed pole is overkill** for the same reason. The "do it once, do it right" framing in the prior bom.md was honest about wanting a designed component, but the component it imagined was for a deployment context the project no longer targets in v1/v2. Workshop time + sourcing pressure-treated stock + ground-anchor work are real costs, and they buy nothing v0/v1/v2 needs.
- **Off-the-shelf plastic tripod** with a standard 1/4"-20 or 3/8" mount head solves the mount problem with one printed/machined adapter. The adapter is the only fabricated component, small enough to print at hobby FDM precision without driving any of the spike's IP / sealing-flatness concerns (the Solar Kit shell is OEM-sealed; the adapter bolts to its underside).
- **Procurement deferred to spike.** Tripod model, adapter dimensions, load rating numbers, anti-rotation feature, transport-case fit — all spike scope.

## Consequences

- [ADR-003](../decisions/ADR-003-relay-hardware.md) §Decision body wording (pressure-treated pole, hose clamps, u-bolts) is factually retired for v0/v1/v2. A status banner is added pointing here; the body is preserved as history. The relay-hardware decision (Tracker V2 + Solar Kit + adhesive PCB standoffs + 3M VHB) is unchanged. A separate ADR-003 amendment ticket flips the §Decision wording when the spike returns a tripod + adapter selection.
- [bom.md](../bom.md) §"Relay pole — local build" is replaced with §"Relay tripod + Solar Kit adapter (selection per spike)". No SKUs committed here.
- [bom.md](../bom.md) §"Explicitly NOT ordering" gains the Fusion-360 designed pole alongside the existing hose-clamps + wooden-pole-from-hardware-store entries. Both approaches retired same date.
- [CLAUDE.md](../CLAUDE.md) §"What 'done' looks like for v1" pole clause replaced with the tripod framing.
- [TODO.md](../TODO.md) §"Right now" relay-pole row replaced with a tripod + adapter line citing the spike + this dev-log.
- [ARCHITECTURE.md](../ARCHITECTURE.md) §3 system-concept clause replaced with the tripod framing.
- [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md) narrows: §Hypothesis drops "fully-custom design" and "wooden-pole-and-clamps fallback"; remaining open scope is (a) tripod model selection, (b) Solar-Kit-to-tripod adapter spec, (c) transport-case sizing dependency on folded tripod length.
- **Audit trail.** Detective-audit row A3 (`dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md`) and handheld-pivot audit row (a) (`spikes/handheld-pivot-doc-audit-spike.md` + `dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`) are both resolved by this decision; the resolution lines reference this dev-log.
- **What does NOT change.** Solar Kit OEM IP67 enclosure; Tracker V2 mounting workaround (adhesive PCB standoffs + 3M VHB); 18650 battery topology; charge-controller routing (solar panel → Solar Kit charge controller → 18650 pack → V2 battery input — single charge path per ADR-003 §Consequences); IPEX→SMA pigtail. ADR-003 §Consequences §"Charge-controller routing" stays load-bearing.
- **What stays open for v3+.** A winter-long unattended deployment will need a different mount story. That conversation is not opened here.

## Open follow-ups

Owned by [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md):

- Tripod model selection — load rating, folded length, extended height, operating-temp envelope, stance / footprint, cost envelope.
- Solar-Kit-to-tripod adapter — print material (prototype + production), screw thread (1/4"-20 vs 3/8"), anti-rotation feature, mounting-plate dimensions matching the Solar Kit's underside.
- Transport-case sizing — case A internal dimensions must accept the folded tripod alongside the Solar Kit + assembled relay (per spike §Transport cases).

Not owned here:

- ADR-003 §Decision amendment ticket — separate follow-up once the spike returns the tripod + adapter selection. Status banner in this commit is informational, not a wording flip.
- Procurement (POs) — downstream of the spike's vendor / quote round.

## Cross-references

- [`spikes/physical-fabrication-brief-spike.md`](../spikes/physical-fabrication-brief-spike.md) — selection owner.
- [`decisions/ADR-003-relay-hardware.md`](../decisions/ADR-003-relay-hardware.md) — relay hardware unchanged; §Decision body language retired-but-preserved.
- [`dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md`](2026-05-06-doc-contradictions-and-blockers-audit.md) §A3 — pole-hardware three-way contradiction, resolved here.
- [`dev-log/2026-05-07-handheld-pivot-doc-audit-close.md`](2026-05-07-handheld-pivot-doc-audit-close.md) — handheld-pivot audit close; §bom.md checklist row updated alongside this commit.
- [`spikes/handheld-pivot-doc-audit-spike.md`](../spikes/handheld-pivot-doc-audit-spike.md) row (a) — pole-hardware contradiction registrar; resolution recorded alongside this commit.
