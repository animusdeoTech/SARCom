---
title: "SARCOM Troubleshooting Guide"
status: living
type: operations
tags: [operations, troubleshooting, gateway, relay, protocol]
---

# SARCOM Troubleshooting Guide

## First rule: the map must not lie

If time, GPS validity, CRC, or packet structure is uncertain, SARCOM should prefer “no marker”, “no-fix”, “stale”, or a visible warning over pretending certainty.

## Packet-level drop reasons

| Symptom / log reason | Meaning | Likely causes | What to check next |
|---|---|---|---|
| `CrcFail` | Frame bytes were received but CRC check failed | RF noise, wrong radio config, truncated payload, bad encode implementation | Verify both ends use same LoRa modem settings and frame layout; inspect raw bytes before CRC check |
| `UnknownType` | `TYPE` not recognized (or version rejected at relay mapping layer) | Sender not on v1 format, corrupted header byte, mixed firmware revisions | Confirm `TYPE=0x01` and `VER=0x01`; verify all nodes run matching protocol build |
| `Malformed` | Frame failed structural/decode checks not mapped to explicit reason | Bad length, bad magic, reserved bits, GPS/sentinel rule mismatch, short/long frame | Re-run decode locally and inspect exact `FrameError` (`BadLength`, `BadMagic`, `ReservedFlagBits`, etc.) |
| `Duplicate` | `(node_id, seq_nr)` already seen recently | Multiple relays hearing same frame, normal flood behavior, packet replay in test loop | Confirm duplicate timing is within seen-cache window; this is usually expected |
| `SelfEcho` | Relay heard its own `node_id` and dropped it | RF reflection, relay hearing rebroadcast of its own packet | Check antenna placement and nearby relays; behavior itself is correct |
| `ReservedFlagBits` | Reserved bits 3–7 in flags were non-zero | Out-of-spec sender, bit packing bug, stale test vectors | Inspect `flags` byte; ensure only GPS/SOS/BATT bits are used |
| `GpsValidSentinelMismatch` | GPS validity bit and coordinate sentinel usage disagree | Sender produced mixed sentinel/non-sentinel fields, wrong no-fix logic | Verify no-fix rule: GPS invalid => all sentinels, GPS valid => zero sentinels |
| `BadLength` | Frame size or `LEN` field invalid for v1 `POSITION` | Truncation, extra bytes, wrong parser assumptions | Confirm exact 22-byte frame and `LEN=16` |
| `BadMagic` | Magic byte mismatch | Not a SARCOM frame, garbage, wrong protocol source | Verify byte 0 is `0xA5` |
| `BadVersion` | Protocol version unsupported | Old/new protocol mismatch | Align all participants on v1 version (`0x01`) |

Notes:

- `ReservedFlagBits` and `GpsValidSentinelMismatch` are decode-level `FrameError` values.
- At relay decision level they can surface as generic `Malformed` unless logs include the lower-level decode reason.

## Why a tag does not appear on the map

Common causes:

- No packets received at gateway.
- Packets received but failing CRC or frame validation.
- Current packet has `GPS_VALID=0` and sentinels (tag alive, but no current marker).
- Gateway clock invalid, so freshness UI is suppressed or warning state is shown.
- Tag is stale/very stale according to freshness windows.
- Packet accepted previously, then repeats suppressed by dedup.
- `node_id` not mapped in future `nodes.toml` presentation config.

## Why a tag is in NO FIX

`NO FIX` means the packet says:

- `GPS_VALID=0`
- coordinates are sentinel values

Operationally:

- The latest valid fix (if any) should be shown separately from the latest report.
- A no-fix report still matters: it proves the tag is alive and in range.
- A no-fix SOS is still critical and must remain clearly marked as distress.

## Why SOS is visible but location is old or missing

This can be valid behavior:

- SOS entry sends first frame immediately and can intentionally skip GNSS acquisition.
- So the current report may be `SOS + no-fix`.
- Map can show last valid fix as historical/ghost context if available.
- Sidebar/banner should clearly state there is no current GPS fix.

## Why the same packet is not forwarded twice

This is expected loop control:

- Dedup key is `(node_id, seq_nr)`.
- Relay seen-cache window is 60 seconds.
- Multiple relays hearing the same packet is normal flood behavior.
- Duplicate suppression prevents echo storms.

## Clock / RTC problems

Time is operational data, not UI decoration.

- Primary clock source is DS3231 RTC (ADR-011).
- No NTP in v1.
- Bad clock corrupts `received_at`, freshness labels, and stale/SOS windows.
- Kiosk should show `RTC NOT SET` / time unavailable instead of false “X ago”.

First checks:

1. RTC module physically present.
2. Coin cell installed and healthy.
3. Boot-time system clock sanity.
4. `hwclock` read succeeds and is plausible.

## Radio bring-up checks

Start with basics before protocol blame:

- Frequency aligned (868.1 MHz).
- Spreading factor / bandwidth / coding rate consistent across participants.
- Syncword compatibility between SX1262 (tag/relay) and SX1276 (gateway) if private syncword is used in config.
- Antenna attached and matched.
- TX power configured to sane/legal values.
- Gateway Dragino HAT SPI/GPIO wiring and device access are correct.
- Do not depend on GNSS UART success to validate LoRa bring-up.

## Gateway database/map checks

- One accepted packet should create one `tag_reports` row.
- Duplicates should not create repeated rows within the dedup window.
- Sentinel coordinates must not render as normal map markers.
- Keep “latest report” and “latest valid fix” as separate concepts in read-model/UI logic.

## What not to debug in v1

Do not spend time hunting features that are intentionally absent:

- no acknowledgements to tags,
- no commands from gateway to tags,
- no per-hop path metadata,
- no gateway certainty about direct vs relayed receipt,
- no live tracking semantics,
- no cloud/web workaround for v1 kiosk correctness issues.

## Bring-up checklist

- [ ] Protocol tests pass.
- [ ] Canonical vectors match.
- [ ] Tag emits `POSITION`.
- [ ] Relay receives valid `POSITION`.
- [ ] Relay drops duplicate on second hearing.
- [ ] Relay forwards byte-identically.
- [ ] Gateway decodes same frame.
- [ ] Gateway writes SQLite row.
- [ ] Kiosk renders valid-fix / no-fix / SOS states correctly.
