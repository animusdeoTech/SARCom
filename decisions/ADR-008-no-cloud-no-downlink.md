---
title: "ADR-008: No cloud, no downlink, pure uplink"
status: accepted
date: 2026-04-22
supersedes: "ADR-008 (2026-04-19): Offline-first with opportunistic cloud sync"
type: adr
tags: [decision, architecture, cloud, downlink, scope]
---

# ADR-008: No cloud, no downlink, pure uplink

**Status:** Accepted
**Date:** 2026-04-22
**Supersedes:** the 2026-04-19 "offline-first with opportunistic cloud sync" stance. That version left the door open to sync; this one closes it for v1.

## Context

Earlier doc versions hedged on cloud sync ("opportunistic", "when connectivity allows") and on downlink control ("compatible with the design, planned evolution"). Hedging becomes soft scope creep: partial implementations, half-wired seams, docs that promise things the code doesn't do.

The deployment scenario also forces the point. Picture: system installed in a mountain hut, relays strapped to poles along a trail. A snowstorm hits. WiFi dies. 4G dies. The relays are still up, the tags are still beaconing, the gateway still receives packets, and the kiosk still shows where people are. If any of those depended on a cloud handshake, the system would fail in the exact moment it matters most.

## Decision

**v1 scope:**

- **No cloud backend.** No internet-hosted server that this system depends on.
- **No REST API, no web dashboard, no mobile app.** Everything the operator sees lives on the kiosk.
- **No downlink.** The tag's transmit schedule is autonomous. The gateway does not command the tag. Pure uplink.
- **"Offline" is the normal state**, not a degraded mode. The system assumes zero internet and happens to work with it if present.

## Consequences

- **System diagram simplifies.** There is no server and no arrow leaving the gateway. The gateway is a terminal node for data.
- **Protocol simplifies.** No RX windows on the tag, no ACKs, no retransmit schedules. Fire-and-forget is the entire semantics.
- **Dedup happens once**, at the gateway, on `(tag_id, seq_nr)`. No server-side second dedup because there is no server.
- **No auth, no encryption in v1.** Nothing leaves the local machine; the threat model for a kiosk-in-a-hut is physical access, not network attack. Add-later list: HMAC per packet, key management, tile auth.
- **Phone access and cloud sync are v2+.** They remain on the future list; no v1 code or doc should imply they are close.
- **Future-compatibility discipline (without building it):** pick v1 options that do not *close the door* to a later HTTP surface or later cloud sync. Concretely: SQLite as the single source of truth with monotonic row IDs; protocol versioned by a `VER` byte; gateway receiver isolated behind a trait so a future HTTP publisher can sit alongside it. But we do not spend v1 engineering budget building these seams.

## Alternatives considered

- **Offline-first with opportunistic cloud sync** (2026-04-19). Rejected: "opportunistic" unspecified and unverified.
- **Limited downlink for operator commands** (cadence change, SOS escalation). Rejected for v1: adds tag RX windows, burning battery; adds addressing; adds ACK-ambiguity in a broadcast topology. Real design cost, no mission-critical v1 gain.
- **Full cloud backend + web dashboard from v1.** Rejected: appliance-in-a-hut doesn't need it, and building it distracts from radio-chain and physical-deployment work.
