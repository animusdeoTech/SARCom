---
title: "Spike ticket — TAK / CoT integration: emit SARCOM tag positions to TAK-compatible clients"
status: open
type: spike-ticket
timebox: TBD (decide when scoped)
opened: 2026-05-05
updated: 2026-05-06 (handheld pivot — promoted from optional to architectural)
---

# Spike ticket: TAK / CoT integration

## PIVOT REFRAME 2026-05-06 — promoted from optional to architectural

The 2026-05-06 pivot to **local-first handheld gateway with opportunistic base-mode export** promotes this ticket from "potential extension that conflicts with ADR-008" to **part of v1 architecture**. The pivot context names "conditional outbound CoT/TAK export when WiFi exists and stable/external power is present" as a v1 feature.

What changes:

- **Open question 1 (does an outbound-only CoT emitter violate ADR-008?)** → **answered architecturally**: yes, an outbound-only LAN-bounded CoT emitter is now in scope. ADR-008 needs an amendment (or a new ADR) that distinguishes (a) "no inbound network surface", (b) "no internet-bound network calls", (c) "no cloud-hosted dependency" from (d) "outbound LAN multicast/unicast under explicit gate". The amendment itself is **not** drafted in this ticket; it is a follow-up to the doc audit (`spikes/handheld-pivot-doc-audit-spike.md`).
- **B1 (ADR-008 settles "no, CoT integration violates the spirit of the project")** → **moot**. The pivot already chose CoT export as in-scope. The fallback (separate `sarcom-tak-bridge` repo) is no longer needed for ADR alignment.
- **B9 (v1a is not yet working)** → **softened**: CoT export is part of v1, not a v1b/v2 deferred. Phase 1 (the multicast experiment) can run in parallel with v1a firmware bring-up; the integrated `cot_emitter` task lands once `gateway-runtime-task-architecture-spike.md` closes.
- **In-process vs separate daemon** → **answered**: in-process Tokio task inside the gateway binary, per the runtime-task spike. The runtime spike allocates a `cot_emitter` task slot and a `cot_gate` predicate task.
- **Export gate** → committed: the emitter only fires when **all** of these are true:
  1. **WiFi state**: associated, has-default-gateway, has-DHCP-lease (from `wifi_monitor` task — see runtime spike)
  2. **Power state**: external power present AND battery not in critical state (from `power_monitor` — see `gateway-handheld-power-architecture-spike.md` `POWER_GOOD` signal)
  3. **Manual enable flag**: a config-file flag default off; the operator opts in. (No kiosk UI surface; ADR-007 stays read-only.)
- **Transport** → committed (subject to Phase 1): UDP multicast on the LAN to a TAK group address, OR unicast TCP to a configured TAK Server address — both behind the gate. **No internet-routed destinations**; the emitter must drop messages whose destination is not RFC1918 / link-local / multicast. Check enforced in code, not by trust.
- **Data flow** → committed: the emitter is a **fire-and-forget read-only consumer of `tag_reports`** and a SQLite-backed replay log of what's been emitted. No retransmit logic in v1 — if the emit gate is open, send; if not, drop. (Replay-on-reconnect is a v2 follow-up; do not bundle.)
- **Mapping** → still open. CoT type for civilian-SAR-hiker, SOS-as-distress-event, stale handling, and PLI cadence mismatch (5 min vs TAK's 30–60 s) remain Phase 2 questions.
- **Phase 1 scope** → unchanged; the multicast experiment from the original ticket is the right cheap-truth-test.

The rest of this ticket below is the pre-pivot 2026-05-05 framing. Read it with the reframe above applied.

---

> This ticket is a **problem description**, not a plan. It describes what we want to know, what we already know, what is unknown, and what could go wrong. The actual implementation plan and integration code are out of scope here — they come after the open questions are surveyed and the ADR-008 friction is settled. **Update 2026-05-06: ADR-008 friction is now an explicit ADR amendment work item, not a blocker on this ticket.**

## Why this ticket exists

A potential career-investment-relevant extension to SARCOM is **emitting Cursor-on-Target (CoT) XML messages** so that any TAK-compatible client (ATAK on Android, iTAK on iOS, WinTAK on Windows) can see SARCOM tag positions on its tactical map. CoT is the de facto data-exchange format for tactical situational awareness in defence and increasingly in civilian SAR; TAK is the de facto client/server stack.

If SARCOM can emit CoT messages cleanly, this:
- Makes SARCOM **potentially interoperable with the TAK ecosystem.** The happy-path emitter is small, but real interop additionally requires verifying client rendering, stale behavior, SOS semantics, and transport assumptions — those are where the actual work sits, not in the XML emitter itself.
- Demonstrates familiarity with TAK / CoT / COP / PLI interoperability vocabulary relevant to defence-tech and SAR-adjacent systems.
- Gives SAR field-deployment scenarios a real interop story: a hut staff member with an Android phone running ATAK could see the same hiker positions the kiosk shows.

~~But: **this potentially conflicts with ADR-008**~~ **As of 2026-05-06**: this is now part of v1 architecture per the handheld pivot. ADR-008 needs amending to distinguish "no inbound surface" + "no cloud / internet-bound" from "outbound LAN under explicit gate". That amendment is the registrar's responsibility (`spikes/handheld-pivot-doc-audit-spike.md`), not this ticket's.

## Problem statement

We need to confidently answer:

> "Given the SARCOM gateway as it is designed (Rust binary on Yocto Linux, SQLite-backed `tag_reports`, native egui kiosk, no inbound network surface), can we add a thin **outbound-only CoT emitter** — a small daemon or in-process module that translates each new `tag_reports` row into a CoT XML message and sends it over TCP/UDP to a TAK Server (or directly to an ATAK client in unicast/multicast configurations) on the same local network — without violating the spirit of ADR-008 and without adding inbound attack surface to the gateway?"

We are NOT asking yet:
- How to receive CoT messages on the gateway (downlink — explicitly violates ADR-008).
- How to bridge SARCOM's protocol crate with TAK's federation server topology.
- How to build TAK plugins or modify ATAK/WinTAK clients.
- How to handle TAK PKI / client certificates at production scale.

Those are downstream questions that only matter after the answer to the question above is "yes" and ADR-008 has been re-examined.

## What we already know (general TAK / CoT background)

These are the broad strokes confirmed from public material; the spike will need to verify specifics in code.

- **TAK** = Team Awareness Kit (originally Tactical Assault Kit). Three primary clients: **ATAK** (Android), **iTAK** (iOS), **WinTAK** (Windows). All consume CoT messages. ATAK is mostly open-source via TAK.gov / civilian release.
- **CoT** = Cursor on Target. XML data format (a newer protobuf flavour also exists). Each message represents an "event" — a position update, a sensor detection, a free-text annotation, a drawing. CoT messages are normally pushed over **TCP** (with optional TLS) or **UDP multicast** on the LAN.
- **TAK Server** is the central broker that accepts CoT from clients and rebroadcasts to subscribed clients. Two main flavours:
  - **TAK Server (official, US-Gov)** — Java Spring stack, restricted access, TLS-required.
  - **FreeTAKServer (FTS)** — Python OSS, more permissive, easier to spin up on a Pi or laptop for local testing.
- **CoT can also be sent peer-to-peer** without a TAK Server: ATAK supports direct UDP multicast on the LAN (default group 239.2.3.1 port 6969). For a hut-local SARCOM-to-ATAK bridge this is the simplest topology — no server at all.
- **CoT type tree** follows MIL-STD-2525-style hierarchy. For a friendly ground unit (e.g. a SAR hiker tag) the type would typically be something like `a-f-G-U-C-I` (atomic, friendly, ground, unit, combat, infantry) or for civilian SAR `a-f-G-E-V-C` or similar — exact mapping for non-military hikers is one of the open questions.
- **Position updates in CoT** are called PLI (Position Location Information). Standard PLI cadence in TAK clients is 30-60s; SARCOM tags emit at 5-min cadence. Mismatch is real but not necessarily breaking — TAK clients render the last received PLI with its own staleness logic.

## What we think we'll need to do, in broad strokes

This is the **shape** of work, not the steps. Each line below is itself a smaller question that the spike will resolve.

- Settle the ADR-008 friction explicitly: is an outbound-only, opt-in, local-network-only CoT emitter compatible with the spirit of "no cloud, no downlink"? If yes, what are the safeguards (config flag default off, no inbound socket ever, etc.)?
- Choose a CoT transport for v1: TCP-to-TAK-Server, UDP multicast direct-to-client, or both behind a config switch.
- Choose CoT format: XML (legacy, dominant) or protobuf (newer, less universal). XML is safer for client compatibility.
- Decide where the emitter runs: in-process inside the gateway binary as an optional Tokio task, or as a separate daemon that polls SQLite. In-process is simpler operationally; separate daemon is cleaner architecturally.
- Investigate Rust ecosystem support: is there a CoT crate? An XML serialization story that handles the CoT schema cleanly (`quick-xml`, `serde-xml-rs`, hand-rolled)?
- Map SARCOM concepts to CoT concepts: hiker tag → friendly-ground-unit, SOS state → emergency / casualty event type, relay/drone-pod → friendly-asset, gateway → self/host.
- Figure out how to test interop: a single ATAK phone on the same WiFi as the gateway is the smallest credible test rig. FreeTAKServer adds complexity but enables multi-client testing.

Concrete code skeletons, message-template files, and integration testing methodology are explicitly **not** in this ticket. Those belong in a follow-up doc once the ADR-008 question is settled and the open questions below are surveyed.

## Phase 1 — minimum viable executable spike

**Timebox: 1 day. Hard stop.**

The ticket as written above is intentionally a survey across architectural decision (A), interop test (B), and semantic mapping (C). Surveying all three in one execution is too broad and risks turning into an interesting-defence-tech-encyclopedia rather than bewijsbaar technisch momentum. So before any of the broader open questions get touched, run the cheapest credible truth test:

**Goal:** emit one hardcoded SARCOM-like tag position as CoT XML over UDP multicast `239.2.3.1:6969` from a Rust binary on a Pi (or even Pieter's laptop) and verify it appears as a marker on an ATAK Android client on the same LAN.

**Hard non-goals for Phase 1:**
- TAK Server (FreeTAKServer or official) — multicast direct to client only
- TLS / X.509 / PKI
- SQLite watcher / `tag_reports` integration — single hardcoded message is enough
- SOS state mapping
- Relay or gateway CoT semantics
- Production crate selection — use whichever is fastest to get one packet on the wire (or hand-roll)
- Stale behavior verification
- Multi-client testing
- ADR-008 resolution — Phase 1 is exploratory and uses a config flag default-off; the architectural decision is deferred until Phase 1 either confirms or kills the integration

**Pass criterion (Phase 1):** an ATAK client on the same WiFi sees a marker appear on its tactical map within seconds of the Rust binary starting; the marker has a sensible icon and a position approximately where Pieter's garden is.

**If Phase 1 passes:** the broader spike (Phase 2 onward) is unblocked. The open questions, blockers, and "What 'answered' looks like" criteria below become the Phase 2 scope, properly re-timeboxed against the ADR-008 question.

**If Phase 1 fails:** the integration concept is in question at a fundamental level. Document specifically which layer failed (Rust binary cannot multicast? ATAK does not pick it up? message format wrong? wrong network interface? wrong port?) and decide before scoping Phase 2.

**Why scope it this tight:** The cheapest waarheidstest is "does anything talk to anything." Everything else (semantics, transport choice, persistence integration, SOS mapping, TLS) is downstream of that one bit.

## Open questions

### Architectural / decision questions (need Pieter)

1. **Does an outbound-only CoT emitter violate ADR-008?** ADR-008 forbids cloud, downlink, REST, WebSocket, phone app. CoT-over-TCP-to-TAK-Server is a network egress that does not handshake with the cloud and does not accept inbound commands — but it is also clearly not "pure offline kiosk." Decision needed: is this a v1 ADR amendment, a v2 deferred extension, or a fundamentally separate system?
2. **Local-only vs WAN-bridged?** A purely-local LAN CoT emitter (gateway and ATAK clients on same WiFi in the hut) is much closer to the offline-first ethos than emitting to a public TAK Server over the internet. Should v1-of-CoT be hard-restricted to LAN multicast / RFC1918 destinations only?
3. **In-process vs separate daemon?** In-process means CoT lives inside the gateway binary (Tokio task watching the SQLite write stream). Separate daemon means a small `sarcom-cot-bridge` binary that polls SQLite. Trade-offs: in-process is simpler to deploy but couples the kiosk-critical path to network I/O; separate daemon isolates failure modes but adds a process to manage.

### Format / protocol questions (need to verify with reference material)

4. **CoT XML vs protobuf?** XML is dominant, well-supported by all TAK clients. Protobuf is newer and faster but support varies. For v1 we likely want XML; verify protobuf is not strictly required for any target client.
5. **What is the right `type` attribute for a SARCOM hiker tag?** Civilian SAR is not the canonical TAK use case; the MIL-STD-2525 type tree may not have a clean SAR-civilian-hiker leaf. Investigate: do existing civilian SAR / search-and-rescue TAK deployments use a standard type, or is it organisation-specific?
6. **What CoT type signals SOS / distress / casualty?** Mapping SARCOM's SOS flag bit to a CoT event that ATAK renders distinctively (red banner, alert sound) is the most useful interop hook for the SAR use case. Verify which type/subtype TAK clients render as emergency.
7. **Stale handling in CoT — does it match SARCOM's freshness model?** CoT events have a `<event start="..." stale="...">` window. Does TAK ATAK actually drop a marker when stale, or just fade it? How does that interact with SARCOM's 22-min very-stale threshold?

### Implementation questions (need Rust ecosystem investigation)

8. **Is there a Rust CoT crate?** Checked briefly: there are scattered `cot-rs` and similar crates of unclear maintenance status. The spike must verify before assuming we have one to depend on.
9. **If no Rust crate exists, what is the cost of hand-rolling CoT XML emission?** CoT message structure is well-defined but the schema is large. `quick-xml` + a manual struct is feasible; `serde-xml-rs` may not handle CoT's attribute-heavy schema cleanly.
10. **TLS / PKI handling.** Production TAK Server uses client-certificate-authenticated TLS. Do we want to handle PKI in v1, or scope v1 to plaintext TCP / UDP multicast and defer TLS to v2?
11. **TAK Server choice for testing.** FreeTAKServer (Python OSS, easier setup, recent maintenance status TBD) vs official TAK Server (Java, restricted, gold-standard). For dev: FreeTAKServer or no server at all (multicast direct to ATAK).

### SAR-semantics questions (need real-world TAK deployment research)

12. **Is there an existing civilian SAR TAK convention** that civilian SAR teams (PSAR, Belgian Red Cross, mountain rescue orgs) follow? If yes, conform to it. If no, document our chosen mapping clearly.
13. **Should the CoT bridge emit events for relays and gateway too**, not just tag positions? An ATAK operator probably wants to see "where are my relays, what's their battery state" alongside "where are the hikers."

### Test-scope questions (affect the spike's own scope)

14. **What rig do we test on?** Minimum: one Android phone with ATAK on the same WiFi as a Pi running the SARCOM gateway. Better: add FreeTAKServer on a second Pi (or a Docker container on a laptop) and test multi-client.
15. **What counts as "this works"?** A SARCOM tag's position appears on the ATAK map, updates within (5+1) minutes of the tag's emission, with a sensible icon, and the SOS state correctly renders as a distress event. Pieter's call on the precise pass criterion.

## Potential blockers

Each blocker has a likelihood guess and a fallback. Likelihoods are gut-feel.

### Architectural

- **B1. ADR-008 settles "no, CoT integration violates the spirit of the project."** *Likelihood: medium-low.* If Pieter decides CoT integration is incompatible with SARCOM's offline-first identity, this ticket is closed and the work moves to a separate sister project. **Fallback:** build a separate `sarcom-tak-bridge` repository that is explicitly framed as an external adapter — keeps SARCOM's identity clean while still enabling the interop story.
- **B2. The interop story is portfolio-noise, not portfolio-leverage.** *Likelihood: low.* If recruiters at the target companies don't actually weight TAK/CoT vocabulary highly, the effort is misallocated. **Mitigation:** verify with recent Helsing / Anduril / Saronic / Skydio job postings whether TAK/CoT appears as desired skill before scoping the spike.

### Stack / ecosystem

- **B3. Rust CoT crates exist, but maturity / interoperability / auditability unknown.** *Likelihood: medium.* Crates surveyed (Pieter 2026-05-06): `cot_publisher` (claims multicast UDP + TCP/TLS destinations to TAK servers per docs.rs), `cot-proto` (uses `quick-xml` for CoT XML serialization/deserialization), plus `rustak` and `cottak`. None has been audited for interop correctness against current ATAK / WinTAK / TAK Server. **Fallback:** inspect crate source for spec compliance against captured CoT samples; if all crates fall short, hand-roll one minimal CoT XML message with `quick-xml` or a literal template. Either path is a viable OSS contribution opportunity (publish a hardened crate, or upstream patches to one of the existing four).
- **B4. CoT schema has under-documented edge cases.** *Likelihood: medium.* The CoT spec is partially-public; some attribute conventions are folklore in the ATAK plugin community rather than written down. May require reverse-engineering from sample messages. **Fallback:** capture CoT messages from a working TAK Server <-> ATAK pair and template against those; document our mapping explicitly.
- **B5. TLS / PKI is mandatory for non-trivial TAK Server testing.** *Likelihood: high if testing against official TAK Server; low if testing against FreeTAKServer or direct ATAK multicast.* **Fallback:** scope v1 to FreeTAKServer / multicast plaintext; defer TLS to a v2 follow-up.

### Operational

- **B6. ATAK installation friction.** *Likelihood: low.* ATAK is on Google Play and TAK.gov; relatively painless to install for testing. Worth verifying Pieter has an Android device available for the test rig.
- **B7. TAK Server (any flavour) install on a Pi is non-trivial.** *Likelihood: medium for FreeTAKServer (Python deps), high for official TAK Server (Java + Postgres + LDAP).* **Fallback:** test ATAK direct-multicast first (no server needed); only set up FreeTAKServer if multi-client testing is required.
- **B8. Update-rate mismatch (TAK 30-60s PLI cadence vs SARCOM 5-min cadence).** *Likelihood: low to be a hard blocker, but cosmetic.* ATAK clients render the last-received PLI; a 5-min-old position will look "stale" in TAK's visual conventions. **Fallback:** document the cadence in the operator-facing config; consider whether the bridge should re-emit the last-known position more frequently to keep markers visually fresh (cosmetic only — same data).

### Process

- **B9. v1a is not yet working.** *Likelihood: blocker by definition.* Per the project's anti-creep gate (see ADR-013 / TODO.md), v1b features depend on v1a passing. CoT integration is at minimum a v1b parallel item, possibly v2. Do not start this spike before v1a's garden test passes its acceptance criteria.

## What "answered" looks like for this spike

**Phase 1 exit (1 day, see "Phase 1" section above):** ATAK on the same LAN renders a marker for one hardcoded CoT message emitted by a minimal Rust binary. Yes/no answer.

**Full ticket exit (timebox TBD, gated on Phase 1 success and ADR-008 resolution):**

> "Can a SARCOM gateway emit CoT XML messages over TCP or UDP-multicast on a local network such that an ATAK client on the same network sees a SARCOM tag's position update on its tactical map within (5 + 1) minutes of the tag emission, with a sensible icon, and with the SOS flag correctly rendered as a distress / emergency event?"

If full-ticket answer is **yes**: open the implementation epic with concrete pass criteria. Decide separately whether to publish the CoT emitter as a Rust crate, write a public blog post, and / or contribute to FreeTAKServer.

If full-ticket answer is **no**: write a follow-up ticket describing exactly which layer failed (XML schema mismatch? transport? client rendering? Rust crate maturity gap?), and decide whether to drop CoT entirely, defer further, or pivot to a different interop format.

## Out of scope for this ticket

Listed here to prevent creep:

- **Receiving CoT messages on the gateway.** Inbound CoT would let TAK operators send commands / annotations to SARCOM. That is a deeper ADR-008 violation than emitting and is explicitly v2+.
- **TAK plugins (ATAK or WinTAK).** Modifying or extending TAK clients is a different scope entirely.
- **TAK Federation / multi-server topologies.** v1 of CoT integration is single-LAN single-emitter.
- **TAK PKI at production scale.** Plaintext on local LAN is acceptable for the spike; TLS / cert handling is v2.
- **Field-deployment certifications.** SAR org adoption (PSAR, Red Cross, mountain rescue) involves operational acceptance testing well beyond a code spike.
- **Replacing the SARCOM kiosk with ATAK.** The kiosk remains the primary UI per ADR-007. CoT bridge is an additional consumer, not a replacement.
- **Drone / C2 plane CoT messages.** If SARCOM ever extends toward drone C2 (per the 2026-05-05 conversation), CoT becomes a different problem (commands, ACKs, authenticated control) — separate ticket.

## Cross-references

- `decisions/ADR-008-no-cloud-no-downlink.md` — needs amendment per the 2026-05-06 pivot; the amendment is not drafted in this ticket
- `decisions/ADR-007-touchscreen-primary-ui.md` — kiosk remains primary UI; CoT does not displace it
- `decisions/ADR-013-multi-hop-flood-via-packet-id.md` — defines the wire data CoT bridge translates
- `dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md` — same-day session that surfaced career-investment framing
- `spikes/gateway-rx-bringup-spike.md` — sibling spike; gateway must be receiving packets reliably before CoT bridge has anything to emit
- `spikes/gateway-handheld-power-architecture-spike.md` — POWER_GOOD signal is part of the export gate
- `spikes/gateway-runtime-task-architecture-spike.md` — allocates `cot_gate` and `cot_emitter` tasks
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar; owns the ADR-008 amendment thread
- `crates/protocol/` — `Position` struct is the source data this bridge translates from

## Required next step before this spike can be timeboxed

**Sequencing under the 2026-05-06 pivot:**
- ADR-008 stance (open question 1) is **architecturally answered** in the pivot reframe; the ADR amendment work is owned by `spikes/handheld-pivot-doc-audit-spike.md`, not this ticket.
- Civilian-SAR TAK conventions (open question 12) remain Pieter's call — research this in Phase 2.
- v1a passing as a gate (B9) is **softened**: Phase 1 (cheap truth-test multicast experiment) can run in parallel with v1a firmware work; the integrated `cot_emitter` task lands after `gateway-runtime-task-architecture-spike.md` closes.

Once Phase 1 is timeboxed and run, the broader Phase 2 scope (semantics, transport choice, persistence integration, SOS mapping, TLS) is unblocked.
