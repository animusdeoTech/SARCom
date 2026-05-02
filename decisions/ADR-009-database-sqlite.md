---
title: "ADR-009: Database — SQLite"
status: accepted
date: 2026-04-22
type: adr
tags: [decision, database, sqlite, persistence]
---

# ADR-009: Database — SQLite (not PostgreSQL)

**Status:** Accepted
**Date:** 2026-04-22

## Context

The gateway persists `POSITION` packets — the single v1 wire packet type per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md). (Earlier drafts of this ADR also referenced a separate `RELAY_INFO` packet from ADR-012; that packet type is rolled back. Tags and relays both self-report as POSITION; rows for both land in the same `tag_reports` table.) The gateway is the only writer — a single Rust task draining a channel from the LoRa RX thread — and the only reader — the kiosk UI rendering markers. Workload:

- Hundreds to a few thousand rows per day under current planning assumptions
- One process, one machine, one disk
- No concurrent writers from separate processes
- No distributed query, no network client (see [ADR-008](ADR-008-no-cloud-no-downlink.md))
- Must run offline, survive power cycles, be backed up by copying one file

## Decision

**SQLite.** Single file at `/var/lib/lora-sar/sightings.db` on the Pi. Accessed from Rust via `rusqlite` or `sqlx`-with-SQLite — choice deferred to the persistence crate implementation.

> The DB file is named `sightings.db` for historical/conceptual continuity; the v1 table is `tag_reports` per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md). A future schema change may rename the file; the table name is authoritative.

## Consequences

- **No DB server process.** No `postgresql.service`, no `pg_hba.conf`, no port open, no `postgres` user, no vacuum cron. Yocto image shrinks, attack surface shrinks, failure modes shrink.
- **Backups are trivial.** `cp sightings.db sightings.db.bak` when idle, or `sqlite3 sightings.db ".backup '/mnt/usb/backup.db'"` for a consistent online copy without stopping the gateway.
- **One writer, one reader, no contention.** WAL mode gives the UI thread snapshot reads while the LoRa thread writes. Standard SQLite idiom.
- **Schema** lives in a `persistence` crate next to `protocol`. Migrations applied idempotently on gateway startup.
- **`seq_nr` width and dedup policy.** Wire `seq_nr` is `u32` in the `protocol` crate, big-endian (see [ARCHITECTURE.md §7](../ARCHITECTURE.md)). An earlier sketch used `u16`; at the planned 300 s heartbeat cadence, a `u16` wraps in ~227 days, and at a 45 s minimum SOS interval in ~34 days. Combined with a **permanent** `UNIQUE INDEX idx_dedup ON tag_reports(node_id, seq_nr)` — which was also in an earlier sketch — the gateway would start silently dropping real new reports as "duplicates" after the first wrap. That is a system-killing drift bug that only appears months into deployment. The fix is threefold and all three parts are non-negotiable:
  1. Wire-level `seq_nr` is `u32`. A `u32` at the worst-case minimum interval still wraps only on geological timescales; at that horizon the appliance has been re-flashed many times.
  2. Dedup is enforced over a **recent-window** filter (e.g. last 24 h of rows per `node_id`), not a permanent table-wide constraint. Implementation: a narrow covering index on `(node_id, seq_nr, received_at DESC)`, and an `INSERT ... WHERE NOT EXISTS (SELECT 1 FROM tag_reports WHERE node_id = ? AND seq_nr = ? AND received_at > NOW - 86400)`. The `UNIQUE INDEX` from the earlier sketch is **removed**.
  3. Dedup is keyed on `(node_id, seq_nr)` — single `tag_reports` table for tags and relays alike, per [ADR-013](ADR-013-multi-hop-flood-via-packet-id.md). (The earlier `RELAY_INFO`-with-`(relay_id, seq_nr)` formulation from ADR-012 is rolled back.)
- **Future cloud sync not blocked (though not built).** `sqlite3 .dump | psql` is a trivial export path. `tag_reports` has a monotonic integer PK and a `received_at` timestamp — both sufficient for incremental sync to whatever Postgres schema shows up in v2.
- **Spatial queries.** Not needed in v1 — the kiosk does bounding-box filtering on the Pi, and SQLite handles a few thousand lat/lon rows trivially. PostGIS is a v3+ concern if ever.

## Alternatives considered

- **PostgreSQL.** Rejected. Buys nothing at this scale: no concurrent writers, no network clients, no spatial index needs. Costs a server daemon, RAM, an admin surface, and a second failure mode. Right choice for a cloud backend — which v1 doesn't have.
- **Flat CSV or JSONL log.** Rejected: no indexed reads, no transactional safety, manual corruption handling.
- **Embedded K/V (sled, redb).** Rejected: we actually want SQL — range queries on `(node_id, received_at)` are the primary read pattern, and SQLite is the most battle-tested embedded SQL store by orders of magnitude.
- **DuckDB.** Rejected for v1: analytical workload fit, not the transactional write path we need.
