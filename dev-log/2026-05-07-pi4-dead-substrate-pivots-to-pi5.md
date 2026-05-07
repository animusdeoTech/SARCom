---
title: "Pi 4 verdict — all three out of order; substrate moves to Pi 5"
date: 2026-05-07
type: dev-log
session-trigger: "Pieter pushed back: Pi 4 power-on test result is settled, not pending"
---

# Pi 4 verdict — out of order; substrate is Pi 5

> **Superseded by [`2026-05-07-pi4-retirement-substrate-decision.md`](2026-05-07-pi4-retirement-substrate-decision.md) (same day, canonical entry).** This file remains as historical record of the same verdict; the canonical entry is the one to read and link.

## What is settled

All three on-hand Raspberry Pi 4 Model B units (`hardware/pi1`, `hardware/pi2`, `hardware/pi3kiosk`) have been **tested by Pieter and confirmed out of order**. None boots cleanly against a known-good PSU + display + SD card.

This supersedes the [`dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md`](2026-05-06-doc-contradictions-and-blockers-audit.md) findings A10 (reframed) and recommendation #5, which both treated the "Pi 4s are bricked" claim as unverified and proposed the power-on test as a v1-blocking action item.

The 2026-05-06 audit's premise was correct at the time it was written — the test had not been formally recorded in `hardware/pi*/specs.md`. The test has since been done; this dev-log is the recording that the audit asked for.

## What this changes

- **The substrate-spike H0** (Pi 4 fallback) is empirically dead. The substrate decision is now between Pi 5 / CM5 / Pi-class-similar; Pi 4 is not a credible v1 fallback for SARCOM.
- **The "Power-on test the 3× Pi 4 Model B units" TODO** is removed from `TODO.md` — done, dead.
- **The substrate-spike unblocks.** It was waiting on this verdict; the verdict is in.
- **The Kiwi cart conversation re-opens as a real procurement question.** The 2026-05-06 audit's "if Pi 4s confirmed dead, *then* the Kiwi cart becomes a real procurement decision" condition is now satisfied. Whether the Kiwi cart specifically is the right cart, or whether a re-spec is needed (Pi 5 + reuse on-hand 7" DSI vs Pi 5 + new 5" panel vs CM5 + carrier), is the substrate-spike's job and ADR-015's downstream commitment.

## What this does NOT change

- ADR-004 still says "Pi 3B+ or 4". That wording is now stale, but ADR-004 is not edited in this dev-log. Supersession lands when ADR-015 is written. Audit close already enumerates ADR-004 as **Superseded-in-part by ADR-015**.
- The Heltec order (#110639, shipped 2026-05-07 per FedEx FIMS notification) is unaffected. The 10× Tracker V2 + 2× Solar Kit are tag/relay hardware, not gateway substrate.
- The Dragino HATs (3× on hand) are reported physically intact in [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](2026-05-05-first-entry-hardware-pi5-rppal.md). The HATs do not need to be re-ordered. Pi 5 + Dragino HAT compatibility research from that dev-log preempts the Pi-5-RP1 gotchas.
- The handheld pivot (2026-05-06) sits on top of Pi 5; nothing in the pivot framing breaks. The substrate spike now has a definite starting point (Pi 5) instead of a Pi-4-vs-Pi-5 fork.

## Doc updates landed in this commit

Same commit as this dev-log:

- `TODO.md`: removed "Power-on test the 3× Pi 4 Model B units" (`Right now`); rewrote the substrate-spike open-spikes line to reflect Pi 4 fallback is dead.
- `README.md`: "Hardware in hand" row updated — Pi 4s confirmed out of order, substrate moves to Pi 5; no more "power-on validation pending".
- `ARCHITECTURE.md` §10: substrate framing dropped Pi 4 from the open-options enumeration.

Spike body edits (substrate-spike H0 / Option 4 / fallback wording) are deferred to that spike's own close. They are not load-bearing on the v1 path now that the empirical answer is in; the spike's pass-criteria still produce the right ranking even with H0 dead.

## Source-of-truth

Pieter has stated the test result repeatedly across sessions; this dev-log entry records it as the canonical recorded verdict so future sessions stop re-opening the question. Future Claude: read this before writing any "verify Pi 4 status" or "Pi 4 fallback" line.
