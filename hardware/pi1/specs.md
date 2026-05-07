# Pi1 — specs

**Status: OUT OF ORDER (tested 2026-05-07).** This unit is retired from the gateway substrate candidate set. See [`../../dev-log/2026-05-07-pi4-retirement-substrate-decision.md`](../../dev-log/2026-05-07-pi4-retirement-substrate-decision.md).

**Role:** gateway/LoRa receiver (spare / development unit)

## Board
- **Model:** Raspberry Pi 4 Model B
- **Power:** USB-C 5V/3A
- **Video out:** 2× micro-HDMI
- **RAM:** unknown — verify with `cat /proc/meminfo` on first boot

## HAT
- **Model:** Dragino LoRa/GPS HAT for RPi v1.4
- **LoRa chip:** SX1276 (868/915 MHz variant — 868MHz checked on board)
- **GPS module:** Quectel L80-M39 (L80GR01A10S) — confirmed from photo 2026-05-04
- **SMA connectors:** LORA_ANT (left), GPS_ANT (right)

## Current state (2026-05-04)
- Bare on desk, no enclosure
- No antennas — both SMA connectors empty
- SD card present (32 GB)
- No PSU
- HAT seated on GPIO header with brass standoffs
