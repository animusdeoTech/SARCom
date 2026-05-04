---
title: "How the SARCOM Network Works"
status: living
type: explainer
tags: [explainer, network, protocol, operations]
---

# How the SARCOM Network Works

## The mission in one sentence

A hiker tag broadcasts GNSS position over LoRa, relays flood-forward those same packets, and a hut gateway stores and renders the latest sightings locally on its touchscreen — with no cloud, no phone app, and no commands sent back to tags.

## The three physical roles

### Tag

A tag is hiker-carried. It wakes on a timer, tries to get a GNSS fix, emits one `POSITION` frame, then sleeps again.

### Relay

A relay is a solar pole node. It validates incoming `POSITION` frames and rebroadcasts valid unseen frames byte-identically. It does not track path history inside packets.

### Gateway

The gateway is a Raspberry Pi with a Dragino HAT and touchscreen at the hut. It validates frames, deduplicates them, stores accepted reports in SQLite, and renders the map.

In v1, the gateway is also the workstation: the same Pi receives, stores, and displays. Nothing leaves that Pi.

## The 22-byte POSITION frame

The frame is deliberately tiny (22 bytes total) because shorter airtime protects battery and duty-cycle headroom.

| Field | Bytes | Purpose |
|---|---:|---|
| `MAGIC` | 1 | Frame signature (`0xA5`) |
| `VER` | 1 | Protocol version (`0x01`) |
| `TYPE` | 1 | Packet type (`POSITION`) |
| `LEN` | 1 | Payload length (`16`) |
| `node_id` | 1 | Sender identity |
| `seq_nr` | 4 | Monotonic per-node sequence |
| `flags` | 1 | GPS/SOS/battery status bits |
| `lat_e7` | 4 | Latitude × 10^7 |
| `lon_e7` | 4 | Longitude × 10^7 |
| `alt_m` | 2 | Altitude in meters |
| `CRC16` | 2 | Integrity check over bytes 0..19 |

CRC in SARCOM is an integrity check for accidental corruption on the radio link. It is not authentication.

## The flags byte

`flags` currently carries three meaningful bits:

- `GPS_VALID` (bit 0)
- `SOS` (bit 1)
- `BATT_LOW` (bit 2)

Bits 3–7 are reserved and must stay zero. If reserved bits are set, decode fails and the packet is rejected. This keeps v1 clean and prevents accidental unofficial format forks.

## The no-fix sentinel rule

This rule exists to prevent dangerous map lies.

- If `GPS_VALID=0`, all coordinate fields must be sentinel values.
- If `GPS_VALID=1`, no coordinate field may be a sentinel value.
- Mixed/partial sentinel packets are rejected.

Plain-language meaning: either the tag has a current fix, or it honestly says, “I am alive, but I do not know where I am right now.”

## What a relay does

`relay_decide` is the relay’s traffic cop. For each received frame, it applies this sequence:

1. Full-frame structural checks
2. `MAGIC` / `VER` / `TYPE` / `LEN` checks
3. CRC check
4. Reserved-flag and no-fix-sentinel consistency checks
5. Duplicate check in `seen_cache`
6. Self-echo check (`node_id == my_node_id`)
7. Enqueue byte-identical rebroadcast

If a packet was already seen, the relay drops it and does not rebroadcast it again.

`seen_cache` specifics in v1:

- Key: `(node_id, seq_nr)`
- Capacity: 32 entries
- Expiry: 60 seconds
- Eviction: ring-buffer oldest overwrite

This is how relay echo loops are prevented in v1 without extra forwarding metadata.

## Why byte-identical rebroadcast matters

Byte-identical forwarding is intentional:

- Keeps packets small.
- Prevents packet growth over multiple relays.
- Lets every receiver validate the same original bytes.
- Keeps forwarding free of path analytics payload.
- Protects duty-cycle budget.

## What the gateway does

The gateway performs the same protocol validation as a defense-in-depth boundary, then deduplicates before DB insert.

The gateway validates for itself even if the frame arrived through a relay. Relay validation is not trusted as a substitute for gateway validation.

For each accepted `POSITION`, it stores one report row in `tag_reports`, and the kiosk map is rendered from that local read model.

Important rendering rules:

- No-fix sentinel coordinates must never be rendered as a normal map marker.
- If the gateway clock is invalid, freshness text must be honest (`RTC NOT SET` / time-unavailable style), not fake relative times.

## What SARCOM deliberately does not know in v1

v1 keeps scope tight. It does not:

- know full radio path history,
- know hop count,
- know “via specific relay X” without a future reception-log layer,
- provide live tracking,
- send acknowledgements to the tag,
- send commands back to tags,
- use cloud services or phone apps.

## Normal heartbeat vs SOS

- Heartbeat cadence: 300–330 seconds.
- SOS is a flag in `POSITION`, not a separate packet family.
- On SOS entry, the first frame is immediate and is not blocked by GNSS acquisition.
- While SOS is active, cadence is 45–60 seconds.
- The audible buzzer belongs on the tag, not on relays.
- No separate SOS frequency in v1.

## One packet's journey

A tag emits sequence `42`.

A relay receives it, validates it, sees `(tag-1, 42)` is new, and forwards the exact same 22 bytes.

Any relay that hears `(tag-1, 42)` again within its own seen-cache window drops it as a duplicate instead of echoing it forever.

The gateway validates, deduplicates, stores one report, and the kiosk shows that latest sighting.

## Mental model

Tags shout tiny checked postcards; relays only repeat postcards that pass inspection and were not heard recently; the gateway pins accepted postcards to the map.
