---
title: "Pi 4 retirement — substrate decision"
date: 2026-05-07
type: dev-log
session-trigger: "Pi 4 power-on tests resolved; Pi 5 procurement is now live"
---

# Pi 4 retirement — substrate decision

## Verdict

All three on-hand Raspberry Pi 4 Model B units (`hardware/pi1`, `hardware/pi2`, `hardware/pi3kiosk`) have been **tested by Pieter and confirmed out of order**. None boots cleanly against a known-good PSU + display + SD card. The verdict is terminal: **the Pi 4 substrate path is retired**.

The "Pi 4 power-on test pending" doctrine that survived across multiple sessions is also retired. Future Claude: do not propose a power-on test, do not list Pi 4 as a substrate candidate, do not write "fall back to Pi 4" in any new doc. The empirical answer is in.

This entry supersedes [`dev-log/2026-05-07-pi4-dead-substrate-pivots-to-pi5.md`](2026-05-07-pi4-dead-substrate-pivots-to-pi5.md) — which records the same verdict but was written before this canonical entry and uses non-canonical filename framing — and finalises the dev-log audit A10 follow-up.

## Consequences

- **`spikes/gateway-handheld-substrate-spike.md` is unblocked.** It was waiting on the Pi 4 power-on result. The verdict is in; the spike can run.
- **Substrate candidate set:** Pi 5 (4 / 8 / 16 GB) / Pi 5 + USB SX1276 dongle / CM5 / Zero 2W. **Pi 4 is dropped.** The substrate spike's H0 (Pi 4 fallback) is empirically eliminated; H0 is restated in this same commit as "fall back to CM5 (compute) or Zero 2W (compute floor) and accept reduced compute headroom".
- **Kiwi cart (or equivalent Pi 5 stack) is now a live procurement decision**, not "voor later". The 2026-05-06 audit's gating condition ("if all three Pi 4s are confirmed dead, *then* the Kiwi cart becomes a real procurement decision") is satisfied.
- **ADR-004 §Decision is factually superseded** on the substrate row ("Raspberry Pi (3B+ or 4, whichever has healthy ports and a working SD slot)"). ADR-015 will formalise the supersession. Until ADR-015 lands, ADR-004 carries a Status banner referencing this dev-log; the banner is added in this same commit as the only ADR-004 edit.
- **Yocto + Dragino + single-binary stance survives.** The pivot does not move those. The `meta-rust` recipe story, the `i2c-rtc,ds3231` device-tree overlay, the `lora-phy` SX127x driver — all unchanged. Only the SBC class moves.
- **dev-log audit A10 RESOLVED.** [`dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md`](2026-05-06-doc-contradictions-and-blockers-audit.md) §A10 (and its priority-list row #5) is annotated in this same commit with a "RESOLVED 2026-05-07" note pointing here.

## What this does NOT change

- The Heltec order #110639 (10× Tracker V2 + 2× Solar Kit, shipped 2026-05-07 via FedEx FIMS #871462208618) is unaffected. Tag and relay hardware live on Heltec; gateway substrate lives on Pi 5.
- The Dragino HATs (3× on hand) are physically intact per [`dev-log/2026-05-05-first-entry-hardware-pi5-rppal.md`](2026-05-05-first-entry-hardware-pi5-rppal.md). They do not need to be re-ordered. The Pi-5-RP1 compatibility research from that dev-log preempts the gotchas (UART syntax change `enable_uart=1` → `dtparam=uart0`, GPIO chip routing through PCIe, `rppal` → `rpi-pal` swap).
- The 7" DSI touchscreen on hand is mechanically and electrically compatible with Pi 5 (DSI ribbon and power); whether it is *the* display class for the handheld is open per the substrate spike + pmtiles-walkers retarget.
- The handheld pivot (2026-05-06) sits on top of Pi 5, not Pi 4; the pivot framing is unchanged.

## Open follow-ups (procurement, not architecture)

These are decisions for the substrate spike close + a small procurement note. They are explicitly **not** ADR-015 itself; they are the inputs ADR-015 commits to.

- **Pi 5 RAM variant.** 4 GB / 8 GB / 16 GB. Driver: peak memory under `egui` + `walkers` + `lora-phy` + `tokio` + SQLite + BLE central + WiFi monitor + CoT emitter (per `spikes/gateway-runtime-task-architecture-spike.md`). 4 GB is likely sufficient for v1; 8 GB is the safe pick if the Pi 5 BOM gap is small.
- **Display reuse vs new panel.** Reuse the on-hand 7" DSI 1024×600 OR order a 5"-class touch panel (Pi Touch Display 2 or third-party). Driver: handheld form factor envelope (per `spikes/gateway-handheld-substrate-spike.md` + `spikes/gateway-handheld-enclosure-spike.md`). Reuse is cheaper and unlocks v0/v0.5 desk bring-up immediately; a 5" panel matches the handheld pivot but adds cost + lead time.
- **Pi 5 PSU + cooler.** Official Raspberry Pi 27 W USB-C-PD PSU (5V/5A) is the desk-bring-up requirement; on the handheld, the battery + buck topology from `spikes/gateway-handheld-power-architecture-spike.md` replaces the wall PSU. Active cooler is required for Pi 5 under sustained load — passive heatsink alone will throttle. The handheld enclosure spike picks the heat path.
- **EU supplier.** Kiwi Electronics (NL), Pimoroni (UK), or Reichelt / Welectron / Berrybase (DE). Driver: in-stock RAM variant + delivery time + VAT clarity. Kiwi assembled the original cart; revisit whether that cart's exact line items are still the right BOM.
