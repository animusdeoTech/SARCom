---
title: "Spike — Duty-cycle ADR-014 gate enforcement + desk measurement workflow"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-06
---

# Spike: Duty-cycle ADR-014 gate enforcement + desk measurement workflow

## Why this spike exists

ADR-014 makes the duty-cycle budget table (ARCHITECTURE.md §13) a **mandatory protocol gate** — every protocol change must update the table. The pivot adds new airtime-relevant variables:

- the synthetic POSITION source (per `fake-position-injector-spike.md`) shares the 1% sub-band M budget when used during real bench tests
- relay duty-cycle runtime enforcement is mentioned in ADR-014 §Consequences as "a firmware concern" but is unscheduled
- handheld + base-sync-export adds *no* on-air airtime (CoT/TAK is LAN-side, not LoRa) — but the spike must say so explicitly so future contributors don't conflate "outbound CoT" with "duty cycle"

ADR-014 also says all airtime values come from "a single canonical LoRa airtime calculator." That calculator is *not yet committed to the repo*. Without a committed source-of-truth tool, future contributors will compute airtime three different ways with three different rounding conventions and reproduce the exact failure ADR-014 was written to prevent.

This spike scopes:

1. how the airtime calculator becomes a committed artifact
2. what fields the firmware must log on every TX so duty-cycle can be measured against the design-time table
3. desk-measurement scenarios for verifying the table within ±10% (per ARCHITECTURE.md §15 v1a hard gates)
4. how the fake-tag cadence stays inside budget when shared with production traffic
5. how every ADR / commit enforces the gate operationally

## Hypothesis / research question

**H1.** A small Rust CLI in `tools/airtime/` (or a notebook + a script) that takes SF/BW/CR/preamble/header/CRC/LDRO and a payload length, prints airtime in ms; committed with unit tests against the published Semtech / TTN reference values. Firmware logs (relay + tag + fake-tag) emit `TX seq=__ len=__ airtime_ms=__ duty_h_pct=__` per transmission; a desk-side log analyser sums airtime over rolling 1-hour windows. Pre-merge gate: when an ADR or commit changes one of (packet size, cadence, retransmit, hop limit), CI runs a check that the budget table in ARCHITECTURE.md §13 has been edited in the same commit (a grep guard, not a deep semantic check). Cheap, sufficient.

**H0.** Continue tracking duty cycle in prose, accept that the failure mode (mis-computed airtime) recurs, and add runtime relay self-throttle as the only enforcement. ADR-014 already names this as insufficient ("failed three times in a row").

## Scope fence

- **No firmware coding.** Spike scopes log fields, not the logging code.
- **No CI configuration.** Spike scopes the *check rule*, not the GitHub Actions YAML.
- **No bench measurement run.** Spike defines the scenarios; running them is the v1a acceptance gate per ARCHITECTURE.md §15.
- **No ADR-014 amendment.** Spike output may *propose* an ADR-014 amendment to add the calculator + log-fields requirement, but does not write it.
- **No re-litigation of the §13 table values.** Numbers are correct per ADR-014; the spike adds the *gate*, not new rows.

## What to verify

### Calculator source-of-truth

- Pick the artifact: a Rust crate (`tools/airtime/`, exporting a `airtime_ms(SF, BW, CR, preamble, header_explicit, crc_on, ldro, payload_len)` function), or a Python notebook in `tools/airtime/notebook.ipynb`, or both. Default candidate: small Rust crate, because it's reusable as a library by the firmware to compute its own airtime for logging.
- Cross-check it against:
  - Semtech AN1200.13 LoRa Modulation Basics (their formula)
  - The Things Network airtime calculator (https://www.thethingsnetwork.org/airtime-calculator)
  - One published independent third-party calculator
- Ship at least the canonical v1 row (22 B / SF10 / BW 125 kHz / CR 4/5 / preamble 8 / explicit header / CRC on / LDRO off → ~371 ms) as a unit test that pins the answer to within ±1 ms.

### Firmware TX log fields

Per transmission (relay self-announce + relay rebroadcast + tag heartbeat + tag SOS + fake-tag), the firmware emits a structured log line containing:

- `node_id`
- `seq_nr`
- frame `len` (always 22 in v1, but logged so future packet types are visible)
- `airtime_ms` — computed by the firmware itself using the same calculator
- `duty_h_pct` — rolling 1-hour airtime as % of the 1% sub-band M cap
- `tx_reason` — one of `HEARTBEAT`, `SOS`, `REBROADCAST`, `SELF_ANNOUNCE`, `FAKE_TAG_SCENARIO`
- `cad_result` — `clear` / `busy_retry_n` / `busy_forced` (relay only)
- `dropped_on_budget` — bool; if true, the TX was queued but not emitted because doing so would breach the 1% cap

The relay's BLE health surface (`field-deployment-test-fleet-spike.md` §4) already includes the rolling 1-hour airtime and the dropped-on-budget counter; this spike confirms the same fields are also in the serial log so non-BLE bench debugging works.

### Desk-measurement scenarios

For ARCHITECTURE.md §15 v1a hard gate "Duty-cycle behaviour matches §13 table within ±10% measured":

- **Tag heartbeat sustained 1 hour:** measure tag's TX airtime/h. Expected ≈ 4.5 s, ±0.45 s.
- **Tag SOS sustained 1 hour:** measure tag's TX airtime/h. Expected ≈ 29.7 s, ±3.0 s.
- **Relay rebroadcast under one heartbeat-only tag for 1 hour:** measure relay's TX airtime/h. Expected ≈ 4.5 s.
- **Relay rebroadcast under one SOS tag for 1 hour:** measure relay's TX airtime/h. Expected ≈ 29.7 s.
- **Relay self-POSITION over 1 hour:** measure relay's TX airtime/h. Expected ≈ 0.74 s.
- **Combined: relay rebroadcast + relay self-POSITION under one heartbeat tag for 1 hour:** sum should be ≈ 5.2 s.

For each scenario: log capture path, compare against the calculator-derived target, accept ±10%.

### Fake-tag cadence inside budget

`fake-position-injector-spike.md` deliberately defers a hard answer. This spike commits a number:

- A fake-tag in heartbeat-equivalent mode adds the same airtime as a real tag heartbeat (~0.12% per source).
- A fake-tag in SOS-equivalent mode adds the same airtime as a real tag SOS (~0.82%).
- **Two simultaneous SOS sources (e.g. one real tag in SOS + one fake tag in SOS) puts the relay at ~1.64% — over budget.** Per ADR-014, do not run that combination at the relay. Bench-only fake tests must either (a) not exercise the relay simultaneously with a real-tag SOS, or (b) move the fake tag to a separate test channel (which becomes a recorded v1a deviation per `fake-position-injector-spike.md`).

### CI / pre-merge gate

A grep-style check, not a semantic one:

- If a commit touches any of `crates/protocol/src/frame.rs`, `crates/protocol/src/position.rs`, files under `firmware/{tag,relay}/`, or any ADR file, AND the commit does not also touch `ARCHITECTURE.md` (specifically the §13 table line range), the check fails with a message pointing at ADR-014.
- The check is advisory in the sense that "ARCHITECTURE.md §13 was unchanged" can be a legitimate result if the change does not affect airtime — but the check forces the contributor to write a one-line "no airtime impact" justification in the commit body to bypass.
- Implementation lives in a small `scripts/check-airtime-gate.ps1` or `.sh`; CI integration is the follow-up ticket.

### Cross-spike implications (record, don't solve)

- `fake-position-injector-spike.md`: this spike commits the duty-cycle accounting numbers and the multi-source over-budget rule.
- `gateway-runtime-task-architecture-spike.md`: gateway has no on-air TX, but the firmware-side TX log fields above are consumed by the gateway's `validate` task for completeness logging.
- `field-deployment-test-fleet-spike.md` §4: BLE counter list already includes rolling 1-hour airtime and dropped-on-budget; this spike confirms the same fields appear in the serial log too.

## Pass criteria

- Calculator artifact named (location + language + cross-checked references); unit test for the canonical v1 row committed as the artifact's pin (or staged for follow-up commit).
- TX log field list committed.
- Desk-measurement scenario table filled (six scenarios above + expected values + tolerance).
- Fake-tag cadence inside-budget rules committed (per-source budget, multi-source over-budget warning, separate-channel deviation policy).
- CI / pre-merge gate rule drafted as a one-liner check (grep-style); follow-up ticket filed for actual CI implementation.
- Cross-spike implications recorded.

## Fail criteria

- The Rust airtime calculator does not match Semtech AN1200.13 + TTN within ±1 ms — investigate the discrepancy before publishing the artifact; do not ship a calculator that disagrees with reference sources.
- Bench measurement on real hardware deviates from the calculator by >10% — open a follow-up spike on what's wrong (firmware airtime accounting? CAD overhead? something else?). Do not silently accept the deviation.
- Multi-source over-budget rule gets pushed back as "we'll just throttle at runtime" without a design-time gate — explicitly call this insufficient per ADR-014's "failed three times" pattern; design-time + runtime are both required.

## Fallback / next action

- If H1 holds: file follow-up tickets for (a) commit `tools/airtime/`, (b) wire firmware TX logging once `firmware/{tag,relay}/` exist, (c) add the CI gate script.
- If H0 (continue with prose): explicitly accept the historical failure pattern and add a re-review trigger ("if a fourth budget-table failure ships, re-open this spike").

## Decision note template

```
Date:
H1 / H0 verdict:

Calculator artifact:
  location:
  language: Rust crate / Python notebook / both:
  cross-checks performed: Semtech AN1200.13 / TTN calculator / third party:
  canonical v1 row pinned (22 B / SF10 / 125 kHz / CR 4/5 / pre 8 / hdr exp / CRC on / LDRO off → ~371 ms): yes / no:
  unit test committed (or staged): ___:

TX log fields (final list):
  node_id:
  seq_nr:
  len:
  airtime_ms:
  duty_h_pct:
  tx_reason:
  cad_result (relay):
  dropped_on_budget:

Desk-measurement scenarios:
  tag heartbeat sustained 1h:        target ≈ 4.5 s,    tolerance ±10%:
  tag SOS sustained 1h:              target ≈ 29.7 s,   tolerance ±10%:
  relay rebroadcast under heartbeat: target ≈ 4.5 s,    tolerance ±10%:
  relay rebroadcast under SOS:       target ≈ 29.7 s,   tolerance ±10%:
  relay self-POSITION 1h:            target ≈ 0.74 s,   tolerance ±10%:
  relay rebroadcast + self combined: target ≈ 5.2 s,    tolerance ±10%:

Fake-tag rules:
  per-source budget:
  multi-source over-budget warning:
  separate test-channel deviation policy:

Pre-merge gate:
  rule (one line):
  artifact (script path):
  CI integration follow-up ticket filed:

Cross-spike implications recorded:
  fake-position-injector:    ___
  runtime task architecture: ___
  field-deployment §4:       ___
  ADR-014 amendment proposed (calculator + log fields)? yes / no:

Not implemented in this spike: airtime crate code, firmware TX logging, CI YAML, bench measurement run.

Next action:
```

## Cross-references

- `decisions/ADR-014-duty-cycle-budget-as-gate.md` — base rule; this spike adds the artifacts that operationalise it.
- `decisions/ADR-010-sos-encoding.md` — SOS cadence; the budget calculator validates it.
- `ARCHITECTURE.md` §13 — budget table; gate target.
- `ARCHITECTURE.md` §15 v1a — hard gate "Duty-cycle behaviour matches §13 table within ±10% measured".
- `spikes/fake-position-injector-spike.md` — synthetic source's airtime contribution.
- `spikes/gateway-runtime-task-architecture-spike.md` — firmware log path and gateway-side aggregation.
- `spikes/field-deployment-test-fleet-spike.md` §4 — BLE health surface (same fields).
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
