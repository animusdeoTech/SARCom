---
title: "Spike — Hardware datasheet / schematic inventory before bring-up"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-06
---

# Spike: Hardware datasheet / schematic inventory before bring-up

## Why this spike exists

`claude-rust-docs-spike.md` (resolved 2026-05-06) deferred deep chip-level reading to bring-up and named six datasheets to live in `resources/datasheets/`:

- SX1262 (tag, relay)
- SX1276 (Dragino HAT)
- UC6580 (GNSS on Tracker V2)
- Heltec Wireless Tracker V2 schematic
- DS3231 (gateway RTC)
- Dragino LoRa GPS HAT schematic

The pivot adds parts that need datasheets *before* a SKU is even chosen:

- Pi 5 GPIO / SPI / UART expansion path through the RP1
- candidate 5" DSI display (Pi Touch Display 2 controller, panel datasheet, ribbon pinout)
- Quectel L80-M39 (the GPS module on the Dragino HAT — was implicitly bundled with the HAT schematic but deserves its own row)
- battery / charger ICs once selected (BQ25xxx-class candidate, fuel gauge, NTC, etc. — *placeholder rows* until the power architecture spike picks parts)
- USB-C-PD trigger IC if used downstream of a fixed-input charger
- buzzer (small datasheet, but the GPIO drive parameters matter for the SOS audible-cue test)
- DSI controller IC for the chosen 5" panel

This spike commits the inventory list, the resolved-vs-pending status per row, the local path each datasheet lives at, and the rule that future bring-up sessions consult `resources/datasheets/` *before* reaching for web search per CLAUDE.md.

## Hypothesis / research question

**H1.** Every part the bring-up sessions touch has its datasheet committed to `resources/datasheets/` (or a stable upstream URL recorded if the PDF is too large to commit) **before** the bring-up session opens. The inventory is a small file with rows for every part, status `present` / `missing` / `pending-part-selection`, and a one-line "what we look up here" hint per row.

**H0.** Datasheets are fetched ad-hoc during bring-up, accepted that 30–60 minutes per session goes to "find the doc again". `claude-rust-docs-spike.md` already named this as wasteful.

## Scope fence

- **No part selection.** This spike does not pick the buck-converter IC or the fuel-gauge IC; it just allocates a row marked `pending` for each.
- **No deep reading.** The spike is *inventory*, not a study session. Deep reads happen in the bring-up sessions per CLAUDE.md.
- **No legal review.** Datasheet redistribution is the vendor's call; if a vendor restricts redistribution, the spike records the URL instead of committing the PDF.
- **No tooling for datasheet search.** Where to *look up* a part is a different question than "is the datasheet on disk." This spike does the latter.

## What to verify

### Inventory rows (each: present / missing / pending-part-selection)

| Part | Document | Local path | Status | Why we need it |
|---|---|---|---|---|
| SX1262 | full datasheet | `resources/datasheets/SX1262_datasheet.pdf` | present / missing | register map, SPI, modulation, CAD timing |
| SX1276 | full datasheet | `resources/datasheets/SX1276_datasheet.pdf` | present / missing | same for HAT chip |
| UC6580 | datasheet + NMEA guide | `resources/datasheets/UC6580_datasheet.pdf` | present / missing | NMEA sentence set, fix-state semantics, power gating |
| Heltec Wireless Tracker V2 | schematic + pinout | `resources/datasheets/heltec_wireless_tracker_v2_schematic.pdf` | present / missing | which GPIO is the SX1262 DIO, GNSS enable, IPEX assignments, free GPIOs for SOS button + buzzer |
| Heltec Solar Kit | mechanicals + charge-controller datasheet | `resources/datasheets/heltec_solar_kit.pdf` | present / missing | bracket compatibility, low-temp charge cutoff (per `production-concerns.md` §2 open question) |
| Quectel L80-M39 | datasheet | `resources/datasheets/quectel_L80-M39.pdf` | present / missing | NMEA, PPS pin, UART config, antenna requirements, cold/warm/hot start times |
| Dragino LoRa GPS HAT | schematic | `resources/datasheets/dragino_lora_gps_hat_schematic.pdf` | present / missing | SPI CS routing (incl. GPIO 25 defect identification), header pinout, L80 wiring |
| DS3231 | datasheet | `resources/datasheets/DS3231_datasheet.pdf` | present / missing | I²C register map, alarm pins, aging offset |
| Raspberry Pi 5 | board datasheet + RP1 datasheet | `resources/datasheets/raspberry_pi_5_datasheet.pdf` + `resources/datasheets/RP1.pdf` | present / missing | SPI/UART/GPIO/I²C peripheral routing, RP1 differences from Pi 4 |
| Raspberry Pi 4 | board datasheet | `resources/datasheets/raspberry_pi_4_datasheet.pdf` | present / missing | fallback substrate per substrate spike |
| Pi Touch Display 2 (5") | panel datasheet + DSI cable pinout | `resources/datasheets/pi_touch_display_2_5in.pdf` | pending-part-selection | candidate 5" panel; pivots if substrate spike chooses something else |
| Battery cell — pending part selection | datasheet | `resources/datasheets/battery_cell_TBD.pdf` | pending-part-selection | charge/discharge windows, NTC characteristic; chemistry/SKU chosen by `spikes/gateway-handheld-power-architecture-spike.md` |
| Tag piezo buzzer | datasheet | `resources/datasheets/buzzer.pdf` | present / missing | drive voltage, current, resonance, GPIO compatibility |
| Battery charger IC | datasheet | `resources/datasheets/charger_ic_TBD.pdf` | pending-part-selection | exposed by `gateway-handheld-power-architecture-spike.md` |
| Fuel gauge IC | datasheet | `resources/datasheets/fuel_gauge_TBD.pdf` | pending-part-selection | same |
| USB-C-PD trigger / sink IC | datasheet | `resources/datasheets/pd_trigger_TBD.pdf` | pending-part-selection | same |
| Buck converter to 5V/5A | datasheet | `resources/datasheets/buck_5v5a_TBD.pdf` | pending-part-selection | same |
| BLE/WiFi USB dongle (if needed) | datasheet | `resources/datasheets/usb_ble_wifi_TBD.pdf` | pending — only if substrate spike says onboard radios fail through-shell | exposed by `gateway-handheld-substrate-spike.md` |
| Sealed tactile button (gateway power + commissioning) | datasheet | `resources/datasheets/sealed_button_TBD.pdf` | pending-part-selection | exposed by `gateway-handheld-enclosure-spike.md` |
| Gore acoustic / pressure-equalisation vent | datasheet | `resources/datasheets/gore_vent_TBD.pdf` | pending-part-selection | exposed by gateway + tag enclosure spikes |
| SMA bulkhead with O-ring | datasheet | `resources/datasheets/sma_bulkhead_TBD.pdf` | pending-part-selection | exposed by enclosure spikes |

### Rules to commit alongside the inventory

1. CLAUDE.md `## Rust doc sources` already names datasheets in the per-crate table. Add: "before any GPIO/peripheral bring-up session, confirm the relevant row in `resources/datasheets/INVENTORY.md` is marked `present`. If `missing`, fetch and commit before bring-up. If `pending-part-selection`, the upstream spike must close before bring-up."
2. Heavy PDFs (Pi 5 datasheet, RP1 datasheet) commit to git; very heavy PDFs (>20 MB) record an upstream URL + a SHA-256 in the inventory file instead, with a fetch script in `scripts/fetch-datasheets.ps1` or `.sh`.
3. The inventory file lives at `resources/datasheets/INVENTORY.md` and is the source of truth. Each datasheet PDF in the directory has a row.
4. Deep-read deferral remains: this spike does not require anyone to read any datasheet now. It requires that the document is *available* when the bring-up session opens it.

### Cross-spike implications (record, don't solve)

- `gateway-handheld-substrate-spike.md`: confirms Pi 5 + RP1 + Pi Touch Display 2 + Dragino HAT datasheet rows.
- `gateway-handheld-power-architecture-spike.md`: as parts are picked, fills the `pending-part-selection` rows.
- `gateway-handheld-enclosure-spike.md`: bulkheads, vents, sealed buttons.
- `tag-handheld-enclosure-spike.md`: same.
- `production-concerns.md` §2: explicit Heltec Solar Kit cold-charge cutoff question — answered by the Solar Kit datasheet row when it's actually read.

## Pass criteria

- Inventory file `resources/datasheets/INVENTORY.md` drafted (or staged) with one row per part above.
- Each row marked `present` (file on disk, with SHA-256), `missing` (fetch action recorded), or `pending-part-selection` (linked back to the upstream spike).
- CLAUDE.md addition drafted: a one-paragraph "datasheet inventory rule" referencing `resources/datasheets/INVENTORY.md`. (Drafted only; not committed in this spike.)
- Fetch script noted as a follow-up if any datasheet is too large to commit directly.

## Fail criteria

- Vendor (e.g. Quectel) restricts redistribution and the URL itself requires a registration wall — record explicitly; the URL goes in the inventory but the bring-up session must do its own fetch.
- A `pending-part-selection` row's upstream spike has no candidate part within its own timebox — the inventory row stays `pending`; bring-up is gated until the upstream spike closes.

## Fallback / next action

- If H1 holds: write the inventory file and the CLAUDE.md addition; close.
- If H0 (continue ad-hoc): explicitly accept the cost in bring-up time and the recurrence of "where did I find that PDF last time" — do not silently drift back into the resolved spike's deferred state.

## Decision note template

```
Date:
H1 / H0 verdict:

Inventory file path: resources/datasheets/INVENTORY.md
Rows committed:
  SX1262:                     present / missing — action:
  SX1276:                     ___
  UC6580:                     ___
  Tracker V2 schematic:       ___
  Solar Kit datasheet:        ___
  Quectel L80-M39:            ___
  Dragino HAT schematic:      ___
  DS3231:                     ___
  Pi 5 + RP1:                 ___
  Pi 4:                       ___
  Pi Touch Display 2 (5"):    ___
  Battery cell:               pending — upstream: power spike
  Buzzer:                     ___
  Charger IC:                 pending — upstream: power spike
  Fuel gauge IC:              pending — upstream: power spike
  USB-C-PD trigger:           pending — upstream: power spike
  Buck 5V/5A:                 pending — upstream: power spike
  USB BLE/WiFi dongle:        pending — upstream: substrate spike (only if needed)
  Sealed button:              pending — upstream: gateway enclosure spike
  Gore vent:                  pending — upstream: enclosure spikes
  SMA bulkhead:               pending — upstream: enclosure spikes

Rules drafted:
  CLAUDE.md addition (text):
  Fetch script (path):

Cross-spike implications recorded:
  substrate:    ___
  power:        ___
  gateway encl: ___
  tag encl:     ___
  production-concerns §2:  ___

Not implemented in this spike: deep datasheet reads, part selection, large-PDF fetch script code.

Next action:
```

## Cross-references

- `spikes/claude-rust-docs-spike.md` — resolved; this spike implements its deferred datasheet inventory.
- `CLAUDE.md` — `## Rust doc sources` section; this spike adds a datasheet rule clause.
- `production-concerns.md` §2 — Solar Kit cold-charge cutoff is one of the datasheet questions.
- `spikes/gateway-handheld-substrate-spike.md`, `spikes/gateway-handheld-power-architecture-spike.md`, `spikes/gateway-handheld-enclosure-spike.md`, `spikes/tag-handheld-enclosure-spike.md` — sources of the `pending-part-selection` rows.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
