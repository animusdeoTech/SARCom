---
title: "ADR-011: Gateway time source — DS3231 RTC, GPS opportunistic"
status: accepted
date: 2026-04-24
type: adr
tags: [decision, gateway, time, rtc, ds3231, gnss, kiosk]
---

# ADR-011: Gateway time source — DS3231 RTC, GPS opportunistic

**Status:** Accepted
**Date:** 2026-04-24

## Context

The kiosk UI renders every sighting as "last seen X minutes ago" and colours markers by staleness. The gateway timestamps every `tag_reports` row with the moment of reception and uses those timestamps for freshness, deduplication windows, and SOS `DISTRESS_WINDOW` classification (see [ARCHITECTURE.md §7 / §11](../ARCHITECTURE.md)). Time is therefore a **primary sensor** in this system, not decoration.

The gateway is a Raspberry Pi on Yocto Linux (see [ADR-004](ADR-004-gateway-platform.md)), running fully offline:

- No internet. [ADR-008](ADR-008-no-cloud-no-downlink.md) closes that door on purpose.
- No NTP. No `systemd-timesyncd`. No phone-home to pool.ntp.org.
- No on-chip RTC. The Pi 3/4/5 have no battery-backed real-time clock; they rely on the network or an external module.

Without intervention, the Pi boots with whatever `fake-hwclock` last saved to disk — typically the shutdown-before-last timestamp. After a power cut, the kiosk comes up with a system time that can be hours, days, or weeks in the past. The consequences:

- **Every "last seen" string is wrong.** A sighting received two minutes ago may be rendered as "last seen 3 days ago" or "in the future." This is not a cosmetic bug; it breaks the single UI affordance the hut staff rely on.
- **Dedup and distress windows misfire.** If system time jumps forward or backward at boot, `(node_id, seq_nr)` freshness windows misclassify frames, and SOS `DISTRESS_WINDOW` cutoffs fire at the wrong moment.
- **SQLite rows get poisoned by bogus `received_at`.** Since the DB is append-only and the column is indexed, bad rows are not self-healing once time recovers.

This ADR exists because an earlier pass through [ARCHITECTURE.md §15](../ARCHITECTURE.md) (implementation roadmap) assumed "of course the Pi knows what time it is" without sourcing that time. That assumption is wrong on an offline appliance.

## Decision

**Primary: DS3231 I²C RTC module with coin cell, persistent across power loss. Secondary: opportunistic time from the Dragino HAT's Quectel L80-M39 GPS when it has a fix.**

### Primary: DS3231 RTC

- Add a **DS3231 RTC module with CR2032 battery holder** to the BOM (see [../bom.md](../bom.md) §Gateway time source).
- Physical connection: Pi I²C bus on the GPIO header (SDA/SCL), 3.3 V and GND. Verify the Dragino HAT leaves the I²C pins free; if it doesn't, route through the HAT's pass-through header or use a stacking header.
- Yocto image: enable `i2c-dev` + `rtc-ds3231` kernel modules, add a device-tree overlay (`dtoverlay=i2c-rtc,ds3231`), wire `fake-hwclock` → `hwclock` → `ds3231` ordering in the boot sequence so that `hwclock --hctosys` runs on boot from the RTC before any userland reads `clock_gettime(CLOCK_REALTIME)`.
- Accuracy: ±2 ppm from -40 °C to +85 °C (manufacturer spec). Worst case drift ~1 minute/year, more than good enough for "last seen" semantics.
- Coin cell: CR2032. Holds RTC time for ~5–10 years with no main power. Order one with the module regardless of whether the module ships with one — some variants do, many don't.

### Secondary: opportunistic GPS-disciplined time

- The Dragino HAT on the gateway carries a **Quectel L80-M39 GPS** (confirmed from physical inspection, 2026-05-04). When it has a fix, its PPS (pulse-per-second) output can discipline system time with sub-second accuracy via `gpsd` + `chrony` (`refclock PPS ...`, `refclock SHM 0`).
- Treat this as **opportunistic**, not primary. GPS fix reliability inside a mountain hut is uncertain — window placement, antenna view of sky, metal roof, snow load all affect it. The RTC must work without the GPS.
- When GPS is available and time-converged, it is allowed to step the RTC (`hwclock --systohc`) so the RTC itself stays disciplined. When GPS is not available, system time runs free from the last RTC read.

### What is NOT done

- **No NTP over the hut's WiFi, even if WiFi exists.** ADR-008 forbids the gateway from reaching out. That stance does not bend for time sync. If a hut operator later decides to add a local NTP server on their LAN, that is a future ADR.
- **No manual time-set UI on the kiosk.** [ADR-007](ADR-007-touchscreen-primary-ui.md) keeps the UI read-only. Time setup happens over SSH during commissioning, not through touchscreen modals.
- **No tag-provided time.** Tags have GNSS but their transmissions cannot discipline gateway time — they only arrive when a tag is within range, and tying gateway time to tag presence creates a circular failure mode (lose the tag, lose the clock). The relay's commissioning GNSS is similarly unsuitable.

## Consequences

- **BOM addition.** DS3231 module + CR2032 coin cell. Covered in [../bom.md](../bom.md).
- **Yocto recipe addition.** `meta-raspberrypi` device-tree overlay for `i2c-rtc,ds3231`, `hwclock` integration in the boot sequence, `gpsd` + `chrony` packages with a `refclock` config for the L80-M39. Documented alongside [ADR-004](ADR-004-gateway-platform.md) during image build.
- **Commissioning step.** At first boot, after flashing the Yocto image to the SD card, the operator runs `date --set ...` followed by `hwclock --systohc` over SSH to prime the RTC. After that, the RTC retains time across power cycles via its coin cell.
- **Failure modes to test during bring-up.**
  - Pull power with the coin cell in place → boot → verify the kernel reads the RTC and system time is within ±5 s of actual.
  - Pull power with the coin cell removed → boot → verify the system falls back to `fake-hwclock` and the kiosk renders an explicit "clock not set" warning rather than pretending (UI: see [ARCHITECTURE.md §11](../ARCHITECTURE.md); add a "clock not set" banner state).
  - GPS fix lost mid-run → verify system time keeps running from the RTC rather than stepping backwards.
- **Kiosk contract.** The kiosk must refuse to render freshness strings ("last seen X minutes ago") if the system clock is unset or in a sentinel "clearly wrong" range (e.g. before the code's compile date). Silent fallback to wrong times is worse than a visible warning.
- **Opportunistic-GPS integration is v1+.** Getting the RTC right is v1 acceptance. Chrony/gpsd refclock discipline of the RTC is a v1 polish task — valuable, not blocking.

## Alternatives considered

- **No RTC, trust `fake-hwclock` alone.** Rejected. `fake-hwclock` saves time periodically to disk and restores it at boot, but it drifts backwards across every power cut by exactly the uptime-since-last-save. On an appliance that sleeps and wakes, this is useless.
- **PCF8523, DS1307, or other cheaper I²C RTCs.** PCF8523 is fine (±5 ppm, slightly cheaper). DS1307 is not — it drifts ±20 ppm and lacks temperature compensation. The DS3231's temperature-compensated crystal oscillator is worth the extra euro on an appliance that will see the thermal range of a mountain hut.
- **GPS as primary time source.** Rejected: fix reliability inside the hut is not guaranteed. If the HAT antenna doesn't see sky, there is no time. RTC + opportunistic GPS is strictly better than GPS alone.
- **NTP over hut WiFi when available.** Rejected: violates [ADR-008](ADR-008-no-cloud-no-downlink.md). The design premise is that the appliance must work identically with and without external networks. If time depends on a network, the guarantee breaks.
- **Manual time-setting on the touchscreen.** Rejected: violates [ADR-007](ADR-007-touchscreen-primary-ui.md). The touchscreen is read-only.
- **Put the whole gateway on a UPS so it never loses power.** Rejected — UPSes fail too, and this ADR must cover the case where the gateway has been physically unplugged for weeks between hut seasons.

## Order checklist

- [ ] 1× DS3231 RTC module (with CR2032 battery holder, I²C, pin-compatible with Pi GPIO header)
- [ ] 1× CR2032 coin cell (order one regardless of whether the module ships with one)
- [ ] Verify I²C pin availability on the Pi GPIO header once the Dragino HAT is seated — if blocked, source a stacking header or a short I²C extender

## References

- [DS3231 datasheet (Analog Devices / Maxim)](https://www.analog.com/media/en/technical-documentation/data-sheets/ds3231.pdf) — ±2 ppm accuracy, temperature-compensated crystal.
- [Raspberry Pi device-tree overlay: `i2c-rtc`](https://github.com/raspberrypi/linux/blob/rpi-6.6.y/arch/arm/boot/dts/overlays/README) — search "i2c-rtc".
- [chrony documentation: GPS PPS refclock](https://chrony-project.org/doc/refclock.html).
- [ADR-004](ADR-004-gateway-platform.md) — gateway platform (Pi + Dragino + Yocto).
- [ADR-007](ADR-007-touchscreen-primary-ui.md) — read-only touchscreen UI.
- [ADR-008](ADR-008-no-cloud-no-downlink.md) — no network sync.
