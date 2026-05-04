# Pi3kiosk — specs

**Role:** production gateway kiosk (this is the one that goes in the hut)

## Board
- **Model:** Raspberry Pi 4 Model B (micro-HDMI visible on sideview — verify model text on boot)
- **Power:** USB-C 5V/3A
- **Video out:** 2× micro-HDMI + DSI (active, connected to screen)
- **RAM:** unknown — verify with `cat /proc/meminfo` on first boot

## HAT
- **Model:** Dragino LoRa/GPS HAT for RPi v1.4
- **LoRa chip:** SX1276 (868/915 MHz variant — 868MHz checked on board)
- **GPS module:** Quectel L80-M39 (L80GR01A10S) — confirmed from photo 2026-05-04
- **SMA connectors:** LORA_ANT (left), GPS_ANT (right)
- **Antenna:** 1× SMA antenna present on LORA_ANT — confirmed from photo

## Screen
- **Model:** Official Raspberry Pi 7" DSI Touchscreen
- **Controller board:** Raspberry Pi Display V1.1
- **Connection:** DSI ribbon cable, connected
- **Mounting:** Pi + HAT physically mounted on screen back panel via standoffs

## Current state (2026-05-04)
- Most complete assembly — screen + Pi + HAT + DSI cable all connected
- 1 LoRa antenna present
- GPS_ANT empty
- No PSU
- SD card status: not confirmed from photos — check on first boot
