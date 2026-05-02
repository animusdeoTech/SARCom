---
title: "Production-grade concerns — post-v1 risks not yet engineered"
status: living
type: risk-register
tags: [production, post-v1, risk, hardware, environment]
date: 2026-04-25
---

# Production-grade concerns

Hardware and environmental risks that are **not** v0 / v0.5 / v1 blockers, but that must be answered before LoRa SAR can be deployed unsupervised on a mountain through a winter season. Problem statements only — no solutions, no v1 action items. Each concern is fact-checked against vendor datasheets and industry-standard specs; where a number is asserted, the source is cited inline so future-me can re-check it instead of trusting this file.

The trigger for writing this is Gemini's review of the v8 documentation set on 2026-04-24. Gemini's verdict that the architecture is "production-grade" is correct on the digital side (ADRs, protocol, persistence, time source). It is not yet correct on the physical side, and this file exists so that the gap is tracked rather than rediscovered later.

Scope of v1, for clarity: a single relay on a wooden pole in a Belgian garden, a gateway indoors near a window, a tag in a pocket. Service interval measured in minutes. None of the concerns below dominate the risk profile of that scenario. They start to dominate at "unattended on a pole through winter, three hours' walk from the hut, expected to last a season."

## 1. Adhesive-only relay mount, alpine thermal cycling

[ADR-003](decisions/ADR-003-relay-hardware.md) specifies that the Heltec Wireless Tracker V2 is fixed inside the Heltec Solar Kit enclosure with adhesive PCB standoffs and 3M VHB tape, because the Solar Kit's default bracket fits the V3/V4/T114 boards but not the Tracker V2. This is a deliberate workaround. For v1 it is fine. For multi-year alpine deployment it is the single largest unanswered mechanical question.

The popular framing — "VHB fails when it gets cold" — is too strong. 3M's own published technical data for the VHB family states operational use down to roughly −40°C for several VHB SKUs once cured, with thermal-shock and long-term-creep test data backing the claim. The mechanism that actually erodes a VHB bond on a deployed relay is not the absolute low temperature, it is **differential thermal expansion under repeated cycling combined with sustained shear loading**. The PCB substrate (FR-4) and the Solar Kit enclosure body (engineering plastic, exact resin not yet confirmed from Heltec docs) have different coefficients of thermal expansion. Each thermal cycle puts the bond line into shear. Across multiple years, with daily swings on the order of 30–50°C between sub-zero nights and a sun-loaded enclosure during the day, the cumulative shear cycles add up. UV ingress through translucent enclosure walls embrittles the exposed edges of the adhesive over time. Vibration from wind loading on the pole adds high-frequency cyclic load on top of the slow thermal cycle.

The failure mode is therefore not catastrophic single-event detachment, it is creep at the bond edges followed by progressive debond. By the time it manifests, an unsupervised relay has no fallback retention path: standoff and PCB simply fall away from the enclosure wall and the unit becomes a heavier-than-air object inside its own IP67 box, with the antenna pigtail as the only mechanical connection to anything fixed. That is a structural failure of the assembly, not just a degradation of LoRa link quality.

The production-grade question that must be answered before unattended deployment: does the unit retain board-to-enclosure mechanical fixation if the adhesive fully fails? In the current design, the answer is no.

## 2. 18650 cold-charging — lithium plating below 0°C

The BOM specifies Samsung INR18650-25R cells (or the equivalent Molicel P26A / LG HG2 fallbacks). ADR-003 wires one cell into the Heltec Solar Kit's onboard charge controller via the SH1.25-2 connector. The relay runs from this cell and is recharged opportunistically from the 5 W panel.

The Samsung INR18650-25R datasheet specifies a charging temperature window of 0°C to 50°C (some redistributor copies cite 0°C to 45°C; either way, sub-zero is out of spec). Heltec's Solar Kit product page lists charging temperature 0°C to 45°C for the kit as a system. Discharge specifications are far more permissive (typically −20°C to +75°C for the cell) — the temperature constraint is **specifically on the charge cycle**, not on operation in the cold.

The physics behind the spec is well established. At sub-zero cell temperatures, the lithium-ion intercalation kinetics at the graphite anode slow down to the point where the lithium ions arriving at the anode plate as **metallic lithium** on the anode surface rather than intercalating into the graphite lattice. Plated metallic lithium is electrically conductive and grows preferentially as dendrites. Each cold charge cycle leaves a thin layer of plated lithium that is partially recoverable in the next discharge but not fully. Cumulative plating reduces capacity, raises internal impedance, and — in the failure case — produces dendrites long enough to perforate the separator. A separator perforation is an internal short circuit, which in a Li-ion cell is the precursor to thermal runaway.

The open question that determines whether this is a real production risk for LoRa SAR specifically: **does the Heltec Solar Kit's charge controller enforce a low-temperature charge cutoff?** A controller that monitors a thermistor on the cell pack (NTC) and refuses to enable the charge path below a configured threshold is safe. A controller that simply enables charging whenever panel voltage exceeds the cell voltage by enough to push current is not safe in winter. Heltec's published documentation for the Solar Kit does not, as of 2026-04-25, clearly state which behaviour is implemented.

The realistic deployment scenario that exercises this: a Belgian-Alpine winter morning with clear skies and a sub-zero ambient. The panel begins producing meaningful current well before the cell warms to 0°C. If the controller has no cutoff, the cell is charged at perhaps −10°C to −5°C cell-internal temperature for an hour or more, every clear winter morning, for the deployment lifetime. None of that is visible from the gateway log. The damage accumulates silently and the failure surface is internal short — the worst Li-ion failure mode.

The production-grade question: confirm the Solar Kit's low-temperature charge behaviour empirically (cold-bench test, NTC behaviour observation, or Heltec engineering response) before any unattended winter deployment. Until that is confirmed, the operational envelope of the relay must be treated as undefined for sub-zero charging.

## 3. IPEX1.0 (u.FL) connector mating-cycle and strain-relief envelope

The Heltec Wireless Tracker V2 exposes its LoRa antenna and its GNSS antenna on IPEX1.0 (u.FL / MHF I) connectors. The BOM and ADR-003 specify an IPEX1.0→SMA female bulkhead pigtail that brings the LoRa antenna out to the Solar Kit's SMA bulkhead, where the external 868 MHz antenna mounts. This is a sound architecture; the constraint is on the IPEX side of the joint.

IPEX1.0 is an industry-standard miniature coaxial connector (Hirose U.FL, MHF I, AMC, IPX — multiple vendor names for what is effectively the same form factor). The connector is **rated by Hirose's own datasheet for approximately 30 mating cycles**. After that the spring contact deforms, the PCB-side ground ring loses preload, and impedance match degrades. This is a documented spec, not a tolerance variation between vendors.

For LoRa SAR, the mating-cycle limit is not the operational concern: a deployed relay is mated once during integration, and the enclosure is then sealed. What matters in the field is the **mechanical loading on the mated joint over years**. IPEX has no integrated strain relief on the PCB side. Any tension, peel load, snag, or persistent vibration applied to the pigtail propagates directly into the connector. Realistic field stressors include thermal-cycle-driven flex of the pigtail loop inside the enclosure, vibration of the antenna mount transferred along the cable, and any mechanical disturbance during a service visit (a technician moving the pigtail to inspect the cell, for example).

The failure modes are progressive: the centre contact backs out partially and impedance match drifts (gradual link-budget loss), the PCB-side ground ring lifts a pad (intermittent open), or the pigtail simply unmates from vibration once preload has been lost (full antenna disconnect). All three present at the gateway as either RSSI degradation over time or unexplained dropouts at a previously stable site, with no obvious correlation to weather or battery state.

The production-grade question: how is the pigtail mechanically immobilised inside the enclosure so that the IPEX joint never sees axial pull or lateral peel load, and what service procedure prevents the joint from being disturbed during routine maintenance? Garden v1 deployment, where the assembly is mated once and the enclosure is sealed for the duration of the test, does not exercise this surface. Multi-year mountain deployment with any service interventions does.

## 4. Gateway SD-card / unannounced power-loss resilience

[ADR-004](decisions/ADR-004-gateway-platform.md) specifies a Raspberry Pi running Yocto Linux as the gateway, with the [SQLite database](decisions/ADR-009-database-sqlite.md) in WAL mode on the SD card. Mains power at the hut is whatever the hut provides — at the Belgian garden v1 site that is consumer mains; at a mountain hut it is some combination of solar/wind/generator/mains, and the failure mode "someone unplugs the gateway, the breaker trips, the storm cuts the inverter" is realistic and recurring.

SQLite in WAL mode is durable against process crash and OS crash by design — that is the headline guarantee of WAL. It is **not** a guarantee against media-level failure modes. SD cards are NAND flash behind a small embedded controller that performs wear-levelling and (on better cards) some form of write atomicity. A sudden power loss during a flash program or erase cycle has two failure surfaces: the user data being written may be partially programmed, and the controller's own internal mapping tables may be in an inconsistent state. Higher-endurance / industrial SD cards reduce wear-out failures but do not categorically eliminate the power-loss-during-write class — only cards with explicit power-loss-protection (capacitor-backed write completion) do, and consumer SD cards generally do not advertise that feature.

The relevant production-grade scenario for LoRa SAR is not loss of recent sightings — those are inherently best-effort uplink data, and a few hours of gap is acceptable. The relevant scenario is **rootfs corruption from sudden power loss**, because rootfs corruption manifests as "the gateway does not boot" with no remediation path that hut staff can execute. The Pi is running a system that continuously touches the filesystem: kernel logs, journal, SQLite WAL checkpoints, the Rust binary's own logging. Any of those can be the write that is in flight when power is removed. With a read-write rootfs and no power-loss protection on the card, the failure rate is not zero per power cycle, and the failure is unrecoverable without re-imaging.

The current v1 stack does not address this. The Yocto image is a default read-write rootfs, the SD card is consumer-grade, and there is no UPS or capacitor-backed clean-shutdown path between mains and the Pi. That is acceptable for a desk and a garden where re-imaging is a five-minute operation. It is not acceptable for a hut where the gateway is the only piece of the stack that anyone interacts with directly, and where a rootfs corruption means a non-functional system until somebody carries a freshly imaged card up the mountain.

The production-grade question: what is the recovery path for an unannounced power loss to the gateway, and what fraction of unannounced power losses produces a non-bootable system? The hardening surface — read-only rootfs with a small writable partition for the SQLite database, A/B image with a known-good fallback, or a clean-shutdown UPS hat — is real engineering work that has not been scoped. The risk to capture today is that the digital side of the stack is being engineered for hut deployment while the bottom-of-stack appliance behaviour is still desk behaviour.

## What this file is not

This is a risk register, not a design backlog. None of the four items here belong in the v0 / v0.5 / v1 acceptance criteria in [ARCHITECTURE.md §15](ARCHITECTURE.md). They are recorded so that the path from "garden PoC works" to "shippable to a hut" has a known list of physical-layer questions that must be answered before that step, rather than being rediscovered the first winter.

When any of these is promoted to active work, it gets its own ADR (if a decision is being taken) or its own section in `ARCHITECTURE.md` (if the decision is already encoded in the design). This file then loses the corresponding section. Until then, the section stays here.

## References

- [ADR-002 — Tag hardware (Wireless Tracker V2)](decisions/ADR-002-tag-hardware.md) — IPEX1.0 antenna ports.
- [ADR-003 — Relay hardware (Wireless Tracker V2 + Solar Kit)](decisions/ADR-003-relay-hardware.md) — VHB mounting workaround, 18650 charge path.
- [ADR-004 — Gateway platform (Pi + Yocto)](decisions/ADR-004-gateway-platform.md) — SD-card storage path.
- [ADR-009 — Database (SQLite WAL)](decisions/ADR-009-database-sqlite.md) — WAL durability scope.
- 3M VHB Tape Family — Technical Data Sheets (operating temperature ranges, thermal-shock data).
- Samsung INR18650-25R — Cell datasheet (charge temperature 0–50°C, discharge −20–75°C).
- Heltec Solar Kit for Dev-board — Product page (charging temperature 0–45°C, default-bracket compatibility list).
- Hirose U.FL series — Connector datasheet (~30 mating cycles).
- SQLite — WAL durability documentation (process/OS crash, not media power-loss).
