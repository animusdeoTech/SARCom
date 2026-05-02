---
title: "ADR index"
status: living
type: adr-index
tags: [decisions, adr, index]
---

# Architecture Decision Records

One ADR per decision. Each ADR has Status, Date, Context, Decision, Consequences, Alternatives.

**Rule for any future Claude (or human) working here:** do NOT re-open an Accepted ADR inline. If context has genuinely changed, write a new ADR that *supersedes* the old one and set the old one's status to `Superseded by ADR-NNN`.

## Status board

| ID | Title | Status | Date |
|----|-------|--------|------|
| [001](ADR-001-firmware-language.md) | Rust everywhere (firmware + gateway + UI) | Accepted | 2026-04-22 |
| [002](ADR-002-tag-hardware.md) | Tag hardware: Wireless Tracker V2 | Accepted | 2026-04-22 |
| [003](ADR-003-relay-hardware.md) | Relay hardware: Wireless Tracker V2 + Solar Kit | Accepted | 2026-04-22 |
| [004](ADR-004-gateway-platform.md) | Gateway: RPi + Dragino HAT + Yocto | Accepted | 2026-04-22 |
| [005](ADR-005-map-and-ui.md) | Map & UI: native Rust kiosk, no web | Accepted | 2026-04-22 |
| [006](ADR-006-relay-has-gnss.md) | Relay has GNSS (commissioning + maintenance only) | Accepted | 2026-04-22 |
| [007](ADR-007-touchscreen-primary-ui.md) | Touchscreen is the only UI | Accepted | 2026-04-22 |
| [008](ADR-008-no-cloud-no-downlink.md) | No cloud, no downlink, pure uplink | Accepted | 2026-04-22 |
| [009](ADR-009-database-sqlite.md) | Database: SQLite | Accepted | 2026-04-22 |
| [010](ADR-010-sos-encoding.md) | SOS encoding: single band, flag bit, jittered cadence | Accepted | 2026-04-24 |
| [011](ADR-011-gateway-time-source.md) | Gateway time source: DS3231 RTC + opportunistic GPS | Accepted | 2026-04-24 |
| [012](ADR-012-node-roles-and-sighting-semantics.md) | Node roles, sighting semantics, v1a/v1b scope | Superseded in part by ADR-013, ADR-014 | 2026-04-25 |
| [013](ADR-013-multi-hop-flood-via-packet-id.md) | Multi-hop flood forwarding via packet_id dedup | Accepted | 2026-04-26 |
| [014](ADR-014-duty-cycle-budget-as-gate.md) | Duty-cycle budget table as mandatory protocol gate | Accepted | 2026-04-26 |

## ADR format

```
# ADR-NNN: <Title>
Status: Proposed | Accepted | Superseded by ADR-MMM
Date: YYYY-MM-DD

## Context
What problem, what constraints.

## Decision
What we chose.

## Consequences
What this locks in, what it closes off, what follow-up it triggers.

## Alternatives considered
The lanes we didn't pick and why.
```

Reference: [Michael Nygard, "Documenting Architecture Decisions" (2011)](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions).
