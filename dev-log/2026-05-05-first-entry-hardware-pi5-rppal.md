---
title: "Dev log — 2026-05-05 — Heltec order placed; Kiwi gateway-stack cart drafted (not ordered); rppal retirement; Pi 5 + Dragino HAT research"
date: 2026-05-05
type: dev-log
session-trigger: hardware procurement during Cargoplot interview prep
corrections:
  - "2026-05-06: Pieter clarified that ONLY the Heltec order #110639 was placed on 2026-05-05. The Kiwi gateway-stack cart was drafted but never checked out. Earlier wording in this entry conflated 'cart' with 'ordered'; corrected throughout. Findings 1, 3 and the TODOs generated from this session that depended on Pi 5 / 5\" Touch Display 2 / 64 GB SD being on hand have been re-flagged as conditional on the Kiwi cart actually being placed."
---

# Dev log — 2026-05-05

> **STATUS UPDATE 2026-05-07:** The Pi 4 verification step referenced repeatedly in this entry is RESOLVED — all three Pi 4 Model B units tested out of order. Findings 1 + 3 ("conditional on Pi 5 being acquired") are now LIVE; Pi 5 is the working substrate candidate. See [`2026-05-07-pi4-retirement-substrate-decision.md`](2026-05-07-pi4-retirement-substrate-decision.md). The body below is preserved as historical context — do NOT re-derive from it that Pi 4 is on the table.

First entry. From now on, sessions that produce non-trivial findings, decisions, or follow-ups should land here as dated entries — easier to grep through later than scattered chat history, and easier to extract into ADRs / TODO items / spikes when patterns crystallise.

## Context

Today's session was supposed to be a 25-minute prep for a Cargoplot kennismakingsgesprek (Amsterdam, Go backend role). **One hardware order was placed** (Heltec, see below). **A second cart was drafted** (Kiwi gateway stack) **but not checked out**. Several findings and TODOs surfaced during the session — some are valid regardless of the Kiwi cart's status, others are conditional on the Kiwi cart actually being placed; this entry now distinguishes the two.

## Hardware activity today

### Heltec — order #110639 ($403.40, Bancontact, ordinary delivery from CN) — **PLACED**

| Qty | Item | Use |
| --- | --- | --- |
| 10 | Wireless Tracker V2 (ESP32-S3 + SX1262 + UC6580 GNSS, 863–870 MHz) | Tag firmware targets per [ADR-002](../decisions/ADR-002-tag-hardware.md); also the relay MCU per [ADR-003](../decisions/ADR-003-relay-hardware.md) |
| 2 | Solar Kit for Dev-board (868 MHz, LoRa + 2.4G antenna interface) | Relay power + enclosure base per [ADR-003](../decisions/ADR-003-relay-hardware.md) |

ETA: 2–4 weeks (~2026-05-19 to 2026-06-02). Bracket-fit issue from ADR-003 still applies — Solar Kit's default bracket does not fit Tracker V2 form factor; use 3M VHB tape + adhesive PCB standoffs.

This deviates from the [TODO.md "Right now"](../TODO.md) row that originally said 3× Tracker V2 + 1× Solar Kit. Actual order is **10× Tracker V2 + 2× Solar Kit** — bigger fleet, supports v1a + v1b + reserve units. TODO.md should be updated to reflect what was actually ordered (not blocking, but worth a small edit pass for accuracy).

### Kiwi Electronics — €337.49 incl BTW — **DRAFT CART, NOT ORDERED (and not yet decided)**

> ⚠ **Status as of 2026-05-06: this cart was prepared during the 2026-05-05 session but Pieter did NOT check out and has NOT decided whether to.** Earlier text in this entry leaked downstream language that treated the items below as substrate (Pi 5, 5" Touch Display 2, 64 GB SDs, 45 W PSUs). They are not. They are a cart that may or may not be placed. Every finding or TODO below that depends on these items now carries an explicit "*conditional on the Kiwi cart being placed*" tag.

| Qty | Item | Notes |
| --- | --- | --- |
| 2 | Raspberry Pi 5 — 2 GB | *Cart only.* Floated as a replacement for the existing Pi 4s if Pieter decides those are unrecoverable. Pi 4 status (bricked? unreliable? working but unverified?) is itself unconfirmed beyond a same-session claim — see Finding 4 follow-up. |
| 2 | Raspberry Pi 45 W USB-C PSU (EU) | *Cart only.* Pi 5 needs more current than Pi 4; would only be needed if Pi 5s are ordered. |
| 2 | **Raspberry Pi Touch Display 2 — 5 inch** | *Cart only.* This is what was originally written up as "5"-vs-7" deviation". With no 5" display in hand and no decision to order one, **there is no live deviation** — the docs continue to assume 7", and the kiosk-lab continues to size for 800×480 until that changes. See "Open question (revised)" below. |
| 2 | 64 GB microSD with Raspberry Pi OS 64-bit | *Cart only.* First-boot sanity-check medium only; real deployment image is Yocto per [ADR-004](../decisions/ADR-004-gateway-platform.md). |

ETA if ordered: 3–5 working days from NL. **Decision to place this cart is itself an open item** and is NOT implied by anything in the session.

### Already on hand (clarified during this session)

- **3× Dragino LoRa/GPS HAT** on Pieter's desk. Each HAT carries **both** the SX1276 LoRa transceiver **and** the L80 GNSS module on a single board. This means the HAT covers two architectural roles simultaneously:
  - Gateway LoRa receive path (CLAUDE.md "Gateway / kiosk" tools row)
  - Opportunistic GPS/PPS time discipline per [ADR-011](../decisions/ADR-011-gateway-time-source.md)
- The HATs are physically intact; only the original Pi 4 hosts were dead. **No new LoRa/GPS HAT ordering needed.**

## Findings

### Finding 1 — Pi 5 + Dragino LoRa/GPS HAT compatibility: works, but RP1 changes the setup non-trivially *(conditional research — only relevant if the Kiwi cart is placed and Pi 5 becomes the gateway substrate)*

> *This finding was researched on 2026-05-05 in anticipation of Pi 5 being ordered. The Kiwi cart that would have placed it is **not** ordered as of 2026-05-06. The notes below remain useful — they preempt Pi-5-specific gotchas if/when a Pi 5 is acquired — but they are NOT load-bearing on the v0/v0.5/v1 path. The currently-on-hand gateway substrate is the existing Pi 4s; their actual operability is itself unverified (see Finding 4 follow-up).*

**Background.** Pi 5's RP1 southbridge routes all GPIO/SPI/I²C/UART through PCIe instead of the SoC's direct memory-mapped peripherals. Most HATs designed for Pi 4 and earlier work electrically but need software/config adjustments. Dragino's official manual lists Pi 2 / Pi 3 compatibility only — neither Pi 4 nor Pi 5 is documented as supported, but the hardware is standard SPI + GPIO + UART.

**Concrete Pi 5 gotchas to handle during gateway bring-up:**

1. **GPIO interrupts can be flaky on Pi 5 with default `lgpio` setup.** [RadioLib issue #1200](https://github.com/jgromes/RadioLib/issues/1200) documents an SX1262 RX failure on Pi 5: "transmit works, receive doesn't, the interrupt seems to never fire." Workaround there was switching back to wiringPi. For the SARCOM Rust + `lora-phy` stack: validate **polled RX mode** before interrupt-driven RX. If polled works and interrupts don't, that is a known Pi 5 routing issue, not a HAT or radio fault.

2. **The RP1 GPIO controller is `gpiochip4`**, not `gpiochip0`. Code that hardcodes `/dev/gpiochip0` will silently address the wrong controller on Pi 5. `cat /sys/kernel/debug/gpio` shows the mapping.

3. **UART syntax changed.** The Pi 4 idiom `enable_uart=1` is invalid on Pi 5. To enable UART0 on GPIO 14/15 for the L80 GPS:
   ```
   # /boot/firmware/config.txt
   dtparam=uart0
   dtparam=spi=on
   ```
   And clean `/boot/firmware/cmdline.txt` — remove `console=serial0,115200` so the kernel console doesn't fight `gpsd` for the UART. `gpsd` device path is `/dev/ttyAMA0` (not `/dev/serial0`).

4. **Dragino LoRa/GPS HAT older revisions had an SPI CS routing defect** — chip-select wired to physical pin 22 (BCM GPIO 25) instead of pin 24 (CE0). Affected boards need software-CS via GPIO 25 — pass an extra `OutputPin` to `lora-phy`. **Check the rev silkscreen on the 3× HATs in inventory before debugging "SPI is dead" on first boot.**

5. **Pin assignment must follow the Dragino schematic, not the SX127x package datasheet.** [RadioLib discussion #1187](https://github.com/jgromes/RadioLib/discussions/1187) shows multiple users wiring chip-package pin numbers and getting init failure (`-2`). For Dragino HATs the working pinout follows the silkscreen, with DIO0 wired to Pi physical pin 7 (BCM GPIO 4) and DIO1 needing a manual jumper to physical pin 16 (BCM GPIO 23) on most revisions.

**Sources consulted (2026-05-05):**
- [RadioLib discussion #1187 — SX1262 + Pi 5 working config](https://github.com/jgromes/RadioLib/discussions/1187)
- [RadioLib issue #1200 — Pi 5 SX1262 RX interrupt failure](https://github.com/jgromes/RadioLib/issues/1200)
- [Pi 5 UART config — forums.raspberrypi.com](https://forums.raspberrypi.com/viewtopic.php?t=359132)
- [Pi 5 GPIO controller mapping — forums.raspberrypi.com](https://forums.raspberrypi.com/viewtopic.php?t=360558)
- [Dragino LoRa/GPS HAT user manual](https://www.dragino.com/downloads/downloads/LoRa-GPS-HAT/LoRa_GPS_HAT_UserManual_v1.0.pdf) — original Pi 2/3 baseline
- [Meshtastic firmware issue #7242 — Dragino SX1276 v1.4 Linux native attempts](https://github.com/meshtastic/firmware/issues/7242)

### Finding 2 — `rppal` was retired 2025-07-01; `rpi-pal` is the maintained successor

**Background.** [`rppal`](https://lib.rs/crates/rppal) was the de facto Rust GPIO/SPI/I²C/UART access library for Raspberry Pi. As of **2025-07-01 the upstream repo was archived** with no new features and no bug fixes. [`rpi-pal`](https://github.com/rpi-pal/rpi-pal) is a community fork that picked up maintenance — same API surface, drop-in replacement, **explicitly Pi 5 compatible**.

**Why this matters for SARCOM.** CLAUDE.md, ARCHITECTURE.md, ADR-004, and `docs/claude-code-setup.md` all reference `rppal`. The gateway crate hasn't been written yet, so this is a free swap — but it must happen **before** any gateway code commits to a `rppal` dependency. `linux-embedded-hal` continues to work the same way (it sits on top of either crate via the `embedded-hal` traits).

**Decision taken this session:** swap all `rppal` references to `rpi-pal` across:
- `CLAUDE.md` (line 65)
- `ARCHITECTURE.md` (line 693)
- `docs/claude-code-setup.md` (line 61)
- `decisions/ADR-004-gateway-platform.md` (line 39) — inline edit acceptable; the *decision* (Yocto + Pi + Dragino HAT) is unchanged, only the technical hint about which Rust crate to verify pin numbers against

This is **not** a new ADR — the gateway-platform decision stands. It's a maintenance correction to a technical reference inside the Accepted decision.

### Finding 3 — Display deviation: NOT a live deviation (cart not placed)

> **Corrected 2026-05-06.** The earlier text below claimed the 5" Touch Display 2 was "ordered today". It was not — it was added to the Kiwi gateway-stack cart and the cart was never checked out. With no 5" display in hand and no decision to order one, **there is no active 5"-vs-7" deviation**. The docs (ADR-005, ADR-007, README, CLAUDE.md, ARCHITECTURE.md) continue to assume 7"/800×480. The kiosk-lab continues to target 800×480. The PMTiles-walkers spike continues to size for 800×480.
>
> The original analysis is preserved below for context — it accurately captures what *would* need to happen *if* the 5" path were chosen — but it is no longer driving any doc edit, ADR write, or kiosk-lab change. Decision #3 from the 2026-05-06 audit-question batch ("Accept 5" now and cascade docs") is **retracted**; the substrate decision (5" vs 7" vs the existing Pi 3B+/4 + 7" DSI on `pi3kiosk`) is itself an open item.

**Original write-up (preserved, not authoritative):**

The 5" Raspberry Pi Touch Display 2 was *in the Kiwi cart* (2× units). The architecture, ADR-005, ADR-007, README.md, CLAUDE.md, ARCHITECTURE.md, and the kiosk-lab UI mockups all assume a **7" display** as the kiosk surface.

5" would be materially different in usable real estate:
- Touch Display 2 5": 720 × 1280 pixels native (portrait), but typically used in landscape → 1280 × 720
- 7" Touch Display: 800 × 480 pixels — the size assumed by the kiosk-lab in `tools/sarcom-kiosk-lab` and the spike work in `spikes/pmtiles-walkers-spike.md`

Implications *if* 5" were chosen:
- Higher pixel density on 5" (~294 ppi vs ~133 ppi on 7"), so glyphs need to be ~2× larger in CSS-pixel terms to remain readable at hut glance distance
- More pixel headroom in landscape (1280×720 vs 800×480) → can fit more sidebar info, but at the cost of physical size when mounted in a hut
- All existing UI mockups and spike work were sized for 800×480

**Open question (revised, 2026-05-06):** the existing 7" DSI Touchscreen on `pi3kiosk` per `hardware/pi3kiosk/specs.md` is the on-hand kiosk surface. Unless Pieter places the Kiwi cart AND decides to repurpose those Pi 5 + 5" units as the production kiosk, the 7" DSI stays as the v1 kiosk substrate per ADR-005 / ADR-007. No doc edits required.

### Finding 4 — Inventory clarification: Pieter's existing HATs cover both LoRa and GPS roles

Two architectural roles previously assumed to need separate hardware are actually covered by Pieter's existing 3× Dragino LoRa/GPS HATs:
- Gateway LoRa SX1276 receive path
- Opportunistic GPS/PPS time discipline per ADR-011

Earlier in this session I incorrectly listed a separate "Dragino Quectel L80-M39 GPS/PPS HAT" in the still-needed BOM. That was wrong — the L80 (different module, but same UART + NMEA + PPS interface) is on-board the existing HATs. The combined HAT means one PCB carries both peripherals on the same Pi.

This does **not** change ADR-011 — the architectural roles are unchanged; the inventory line items are. `bom.md` and TODO.md should be re-read with this understanding before placing additional orders.

## TODOs generated from this session

In rough priority order. **Reorganised 2026-05-06** so that items conditional on the unplaced Kiwi cart are clearly separated from items that are valid regardless.

### Now — valid regardless of Kiwi cart

- [ ] **Swap `rppal` → `rpi-pal` across the four affected files** (CLAUDE.md, ARCHITECTURE.md, docs/claude-code-setup.md, ADR-004). *Claimed done in this session*; verified 2026-05-06 that **CLAUDE.md was missed** — the swap is still pending on CLAUDE.md:65. See [the 2026-05-06 audit](2026-05-06-doc-contradictions-and-blockers-audit.md) finding A1.
- [ ] **Check the silkscreen rev on the 3× Dragino LoRa/GPS HATs.** Note any boards that have the SPI CS-on-pin-22 defect — those will need software CS via GPIO 25 in the gateway crate. Findings should land in `hardware/pi{1,2,3kiosk}/specs.md`.
- [ ] **Update TODO.md "Right now" Heltec order line** — actual order was 10× Tracker V2 + 2× Solar Kit, not 3× + 1×. Reflect what was placed.
- [ ] **Verify the on-hand Pi 4 status.** The same-session claim "the Pi 4s on Pieter's desk are bricked" is unverified beyond that one sentence. `hardware/pi{1,2,3kiosk}/specs.md` describe the boards as physical objects without a working/non-working flag. Power one up against any micro-HDMI display + USB-C PSU + a known-good SD card; record the result. This is the actual gateway-substrate question, not Pi 5.
- [ ] **Source DS3231 RTC + CR2032** (already on TODO.md "Right now" parallel order list — keep as-is).

### Conditional — only relevant if the Kiwi cart is placed

> Each of the items below depends on Pi 5 / 5" Touch Display 2 / 64 GB SD / 45 W PSU actually being acquired. None of them block v0/v0.5/v1 on the existing Pi 4 + 7" DSI substrate.

- [ ] **Decide whether to place the Kiwi cart at all.** Option (a) place as drafted; option (b) revise (e.g. Pi 5 + 7" 800×480 instead of 5"); option (c) drop entirely and bring up v0 on existing Pi 4 + 7" DSI; option (d) defer until the Pi 4 verification step above produces a result. Until this is resolved, the items below are speculative.
- [ ] **(Conditional)** Source Pi 5 active cooler. Pi 5 thermally throttles fast under SPI + display + SQLite + LoRa load. Only matters if Pi 5 is actually ordered.
- [ ] **(Conditional)** Pi 5 first-boot validation checklist (write into a new `spikes/pi5-hat-bringup-spike.md` before plugging anything in):
  - [ ] `pinctrl get 14`, `pinctrl get 15` — confirm GPIO 14/15 in ALT mode for UART
  - [ ] `cat /sys/kernel/debug/gpio` — confirm `gpiochip4` = RP1 controller
  - [ ] `gpsmon /dev/ttyAMA0` — NMEA strings appearing? UART path good.
  - [ ] SPI loopback test (MOSI ↔ MISO bridged) with `linux-embedded-hal` + `rpi-pal` before bringing up SX1276
  - [ ] SX1276 in **polled RX** before interrupt-driven RX — isolate radio/SPI from interrupt-routing concerns
  - [ ] Verify L80 PPS pin (if exposed on the HAT) into `chrony` for sub-second time discipline
- [ ] **(Conditional)** Pi 5 + Yocto build experiment. Yocto's `meta-raspberrypi` Pi 5 support landed in scarthgap (2024); confirm the active branch supports Pi 5 GPIO/SPI peripherals via device tree before committing. If staying on Pi 4, this becomes a Pi 4 + Yocto experiment instead — `meta-raspberrypi` covers Pi 4 from kirkstone onward.

## Open questions requiring Pieter's call

- **Gateway substrate: existing Pi 4s or Pi 5 (Kiwi cart)?** Drives whether to place the Kiwi cart, and whether ADR-004's "3B+ or 4" wording is current or needs an addendum. Verify the Pi 4s first, then decide.
- ~~Display: 5" or 7"?~~ **Resolved 2026-05-06: not a live deviation; no 5" exists; docs continue to assume 7" DSI.** Re-opens only if the Kiwi cart is placed with 5" displays.

## Doc updates applied this session

- ~~`CLAUDE.md` — `rppal` → `rpi-pal` in the Gateway / kiosk tools row~~ **NOT APPLIED** — verified 2026-05-06 that CLAUDE.md:65 still reads `rppal`. The edit was claimed but never committed. Tracked as audit finding A1.
- `ARCHITECTURE.md` — `rppal` → `rpi-pal` in the Dragino-pin-numbering caveat ✓
- `docs/claude-code-setup.md` — `rppal` → `rpi-pal` in the listed crates ✓
- `decisions/ADR-004-gateway-platform.md` — `rppal` → `rpi-pal` in the technical hint about pin verification (decision unchanged) ✓
- `dev-log/` — directory created, this entry committed as the first dev log

## Open source contribution opportunities surfaced by this session

These all hang naturally off the SARCOM work and would land as portfolio-visible OSS work — directly supporting the career-investment framing of the project. Roughly ordered by feasibility × impact:

1. **`dragino-lora-gps-hat` BSP-style crate.** No Rust crate currently exposes the Dragino LoRa/GPS HAT pinout, SX1276 SPI client, and L80 UART NMEA stream as ergonomic structs for a Pi gateway. SARCOM will end up writing this code internally — extracting it into a small standalone crate is a 1–2 day effort and fills a real gap. Bonus: ships with a `examples/pi5-bringup` showing the polled-RX → interrupt-RX progression.

2. **`lora-phy` Pi 5 / Linux gateway example.** The [`lora-rs/lora-rs`](https://github.com/lora-rs/lora-rs) repo has good MCU examples (esp-hal, embassy) but thin Linux-host examples. A documented "SX1276 on Raspberry Pi 5 with Dragino HAT, polled and interrupt-driven" example would be genuinely useful and is a natural side-effect of the SARCOM gateway crate. PR-able.

3. **`rpi-pal` Pi 5 documentation / `gpiochip` auto-detection.** rpi-pal is young; if SARCOM hits any Pi 5 quirks (gpiochip routing, interrupt edge detection), upstream them. Small surface, high reuse.

4. **Dragino wiki / community Pi 5 setup notes.** Dragino's wiki only covers Pi 2/3. A clean, dated "How to bring up the Dragino LoRa/GPS HAT on Raspberry Pi 5 with Yocto + Rust" walkthrough lands in the spaces hobbyists actually search. Markdown, not code.

5. **Meshtastic Linux native YAML for Dragino SX1276 v1.4.** Issue [#7242](https://github.com/meshtastic/firmware/issues/7242) is open. Limited utility because Meshtastic's custom preamble is poorly supported on SX1276, but the YAML-config contribution is small and demonstrates protocol-level Meshtastic familiarity — useful for embedded recruiters.

6. **`meta-raspberrypi` / `meta-rust` Yocto recipe contributions.** SARCOM will produce a Yocto image for Pi 5 with a Rust gateway binary, gpsd, chrony, DS3231 device-tree overlay, and PMTiles tooling. Once that recipe is stable, fragments are upstreamable. Yocto contributions are visible to embedded-Linux recruiters specifically.

Pick one or two when SARCOM v1a is in shape — premature to do these before the gateway crate exists, but the *opportunity* is real and worth holding in view. Item 1 (`dragino-lora-gps-hat` crate) is probably the highest-leverage if the goal is "concrete, project-coupled OSS contribution that interviewers can read."

## Process note (for future dev log entries)

Going forward, dev log entries should follow this rough shape:
- Date in filename (`YYYY-MM-DD-short-slug.md`)
- YAML frontmatter (title, date, type, session-trigger)
- Context (one paragraph)
- Findings (each with sources)
- TODOs generated
- Doc updates applied (or open)
- Open questions for Pieter

Frequency: write one whenever a session produces non-trivial findings worth grepping for later. Skip when sessions are routine implementation work — those belong in commits, not dev logs.
