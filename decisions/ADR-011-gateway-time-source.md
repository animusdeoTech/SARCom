---
title: "ADR-011: Gateway time source — Pi 5 built-in RTC, GPS opportunistic"
status: accepted
date: 2026-04-24
amended: 2026-05-04
type: adr
tags: [decision, gateway, time, rtc, gnss, kiosk]
---

# ADR-011: Gateway time source — Pi 5 built-in RTC, GPS opportunistic

**Status:** Accepted (amended 2026-05-04 — Pi 5 replaces Pi 4; DS3231 removed)
**Date:** 2026-04-24

## Context

The kiosk UI renders every sighting as "last seen X minutes ago" and colours markers by staleness. The gateway timestamps every `tag_reports` row with the moment of reception and uses those timestamps for freshness, deduplication windows, and SOS `DISTRESS_WINDOW` classification (see [ARCHITECTURE.md §7 / §11](../ARCHITECTURE.md)). Time is therefore a **primary sensor** in this system, not decoration.

The gateway is a Raspberry Pi on Yocto Linux (see [ADR-004](ADR-004-gateway-platform.md)), running fully offline:

- No internet. [ADR-008](ADR-008-no-cloud-no-downlink.md) closes that door on purpose.
- No NTP. No `systemd-timesyncd`. No phone-home to pool.ntp.org.
- **Pi 5 has a built-in battery-backed RTC** with a dedicated 2-pin JST battery header on the board. This was not present on Pi 3/4. The three original Pi 4 units died in storage (confirmed 2026-05-04); the gateway hardware is now Pi 5. The DS3231 external RTC module from the original BOM is therefore unnecessary and has been removed.

Without a battery in the RTC header, the Pi 5 still boots with whatever `fake-hwclock` last saved to disk — typically the shutdown-before-last timestamp. After a power cut without the RTC battery fitted, the kiosk comes up with a system time that can be hours, days, or weeks in the past. The consequences:

- **Every "last seen" string is wrong.** A sighting received two minutes ago may be rendered as "last seen 3 days ago" or "in the future." This is not a cosmetic bug; it breaks the single UI affordance the hut staff rely on.
- **Dedup and distress windows misfire.** If system time jumps forward or backward at boot, `(node_id, seq_nr)` freshness windows misclassify frames, and SOS `DISTRESS_WINDOW` cutoffs fire at the wrong moment.
- **SQLite rows get poisoned by bogus `received_at`.** Since the DB is append-only and the column is indexed, bad rows are not self-healing once time recovers.

This ADR exists because an earlier pass through [ARCHITECTURE.md §15](../ARCHITECTURE.md) (implementation roadmap) assumed "of course the Pi knows what time it is" without sourcing that time. That assumption is wrong on an offline appliance.

## Decision

**Primary: Pi 5 built-in RTC with battery backup via dedicated header. Secondary: opportunistic time from the Dragino HAT's Quectel L80-M39 GPS when it has a fix.**

### Primary: Pi 5 built-in RTC

- The Raspberry Pi 5 has an **on-board RTC** connected to the BCM2712 SoC, exposed via the standard Linux RTC interface (`/dev/rtc0`). No external I²C module needed.
- Battery: fit a compatible lithium coin cell to the 2-pin JST battery header on the Pi 5 board. Raspberry Pi sells an official "Raspberry Pi RTC Battery" for this header. A LIR2032 rechargeable cell with a JST pigtail also works; the Pi 5 has a trickle-charge circuit on the header. Do **not** wire a standard CR2032 directly unless the charge circuit is disabled — it is not rechargeable.
- Yocto image: the Pi 5 RTC is supported by the `meta-raspberrypi` BSP layer out of the box. Wire `hwclock --hctosys` at boot before any userland reads `CLOCK_REALTIME`. No `dtoverlay=i2c-rtc` needed.
- Accuracy: the Pi 5's built-in RTC is adequate for "last seen" semantics — expected drift well under 1 minute/day without GPS discipline.
- The DS3231 external module and CR2032 coin cell from the original BOM are **removed**. No I²C wiring, no stacking header concern for the RTC.

### Secondary: opportunistic GPS-disciplined time

- The Dragino HAT on the gateway carries a **Quectel L80-M39 GPS** (confirmed from physical inspection, 2026-05-04). When it has a fix, its PPS (pulse-per-second) output can discipline system time with sub-second accuracy via `gpsd` + `chrony` (`refclock PPS ...`, `refclock SHM 0`).
- Treat this as **opportunistic**, not primary. GPS fix reliability inside a mountain hut is uncertain — window placement, antenna view of sky, metal roof, snow load all affect it. The RTC must work without the GPS.
- When GPS is available and time-converged, it is allowed to step the RTC (`hwclock --systohc`) so the RTC itself stays disciplined. When GPS is not available, system time runs free from the last RTC read.
- **Known integration complexity on Pi 5:** the Pi 5 renames and reroutes UART devices compared to Pi 4 (RP1 southbridge). The serial console must be disabled on the UART that routes to the Dragino HAT's GPS pins, and the correct device node (`/dev/ttyAMA*`) verified before `gpsd` config. This is a bring-up task, not a design risk — it is predictable but will cost time. Do not assume it works on first boot.
- **Indoor GPS fix not possible** without external antenna and line-of-sight to sky. The kiosk unit's GPS_ANT SMA connector is currently empty. GPS integration is deferred to field bring-up; it is not a v1 lab blocker.

### What is NOT done

- **No NTP over the hut's WiFi, even if WiFi exists.** ADR-008 forbids the gateway from reaching out. That stance does not bend for time sync. If a hut operator later decides to add a local NTP server on their LAN, that is a future ADR.
- **No manual time-set UI on the kiosk.** [ADR-007](ADR-007-touchscreen-primary-ui.md) keeps the UI read-only. Time setup happens over SSH during commissioning, not through touchscreen modals.
- **No tag-provided time.** Tags have GNSS but their transmissions cannot discipline gateway time — they only arrive when a tag is within range, and tying gateway time to tag presence creates a circular failure mode (lose the tag, lose the clock). The relay's commissioning GNSS is similarly unsuitable.

## Consequences

- **BOM change.** DS3231 module and CR2032 removed. Add: 1× Raspberry Pi RTC Battery (or LIR2032 + JST pigtail) for the Pi 5 RTC header.
- **Yocto recipe.** No `dtoverlay=i2c-rtc` needed. Configure `hwclock --hctosys` at boot from `/dev/rtc0`. Add `gpsd` + `chrony` packages with a `refclock` config for the L80-M39 (GPS secondary path). Documented alongside [ADR-004](ADR-004-gateway-platform.md) during image build.
- **Commissioning step.** At first boot, after flashing the Yocto image to the SD card, the operator runs `date --set ...` followed by `hwclock --systohc` over SSH to prime the RTC. After that, the RTC retains time across power cycles via the JST battery.
- **Failure modes to test during bring-up.**
  - Pull power with RTC battery fitted → boot → verify system time is within ±5 s of actual.
  - Pull power with RTC battery missing → boot → verify the system falls back to `fake-hwclock` and the kiosk renders an explicit "clock not set" warning rather than pretending (UI: see [ARCHITECTURE.md §11](../ARCHITECTURE.md); add a "clock not set" banner state).
  - GPS fix lost mid-run → verify system time keeps running from the RTC rather than stepping backwards.
- **Kiosk contract.** The kiosk must refuse to render freshness strings ("last seen X minutes ago") if the system clock is unset or in a sentinel "clearly wrong" range (e.g. before the code's compile date). Silent fallback to wrong times is worse than a visible warning.
- **Opportunistic-GPS integration is v1+.** Getting the RTC right is v1 acceptance. Chrony/gpsd refclock discipline of the RTC is a v1 polish task — valuable, not blocking. UART bring-up on Pi 5 is a known non-trivial step; allow time for it.

## Alternatives considered

- **No RTC, trust `fake-hwclock` alone.** Rejected. `fake-hwclock` saves time periodically to disk and restores it at boot, but it drifts backwards across every power cut by exactly the uptime-since-last-save. On an appliance that sleeps and wakes, this is useless.
- **External I²C RTC (DS3231, PCF8523, etc.)** No longer needed — Pi 5 has one built in. These alternatives were valid for Pi 4 but are now moot.
- **GPS as primary time source.** Rejected: fix reliability inside the hut is not guaranteed. If the HAT antenna doesn't see sky, there is no time. RTC + opportunistic GPS is strictly better than GPS alone.
- **NTP over hut WiFi when available.** Rejected: violates [ADR-008](ADR-008-no-cloud-no-downlink.md). The design premise is that the appliance must work identically with and without external networks. If time depends on a network, the guarantee breaks.
- **Manual time-setting on the touchscreen.** Rejected: violates [ADR-007](ADR-007-touchscreen-primary-ui.md). The touchscreen is read-only.
- **Put the whole gateway on a UPS so it never loses power.** Rejected — UPSes fail too, and this ADR must cover the case where the gateway has been physically unplugged for weeks between hut seasons.

## Order checklist

- [ ] 1× Raspberry Pi RTC Battery (official, JST connector for Pi 5 RTC header) — or LIR2032 + matching JST pigtail
- [ ] Verify RTC header location on the Pi 5 board before seating the Dragino HAT (header is to the right of GPIO, should not conflict)

## References

- [DS3231 datasheet (Analog Devices / Maxim)](https://www.analog.com/media/en/technical-documentation/data-sheets/ds3231.pdf) — ±2 ppm accuracy, temperature-compensated crystal.
- [Raspberry Pi device-tree overlay: `i2c-rtc`](https://github.com/raspberrypi/linux/blob/rpi-6.6.y/arch/arm/boot/dts/overlays/README) — search "i2c-rtc".
- [chrony documentation: GPS PPS refclock](https://chrony-project.org/doc/refclock.html).
- [ADR-004](ADR-004-gateway-platform.md) — gateway platform (Pi + Dragino + Yocto).
- [ADR-007](ADR-007-touchscreen-primary-ui.md) — read-only touchscreen UI.
- [ADR-008](ADR-008-no-cloud-no-downlink.md) — no network sync.
