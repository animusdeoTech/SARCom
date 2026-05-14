---
title: "Spike — Gateway runtime task architecture"
status: open
type: spike
timebox: 0.5 day
opened: 2026-05-06
amended: 2026-05-14 (cot_gate predicate: 3 inputs → 2; power_monitor no longer exposes POWER_GOOD)
---

# Spike: Gateway runtime task architecture

## 2026-05-14 signal-contract correction — POWER_GOOD retired

The pending-ADR-016 CoT/TAK export gate has been re-scoped to **"WiFi + manual opt-in"** (2 inputs). This is a signal-contract change for two tasks in the table below:

- **`cot_gate`** no longer reads `POWER_GOOD`. Its inputs reduce to `watch<WifiState>` (from `wifi_monitor`) + a config-file flag (read once at startup, or watched via inotify if hot-reload is wanted). Output `watch<ExportEnabled>` is unchanged in shape; only the predicate composing it changes.
- **`power_monitor`** no longer exposes `POWER_GOOD`. The task still exists (it still reads VBUS-droop on the Pi 5 USB-C-PD input to raise `SHUTDOWN_REQUEST` for clean shutdown), but its `watch<PowerState>` no longer carries an external-power-present bit. The `BATTERY_STATE` / `CHARGE_STATE` fields previously named in the power-arch spike's signal contract were already declared not-firmware-readable in the 2026-05-08 verdict; they are now formally removed from this task's outputs.
- **`shutdown_orchestrator`** is unchanged. The low-VBUS-detect → clean-shutdown sequence still runs through `SHUTDOWN_REQUEST`; that signal survives the amendment.

Concrete effect on this spike:

- Task table row `cot_gate` "Triggered by" column: gate inputs are now WiFi state + config flag (was: WiFi + POWER_GOOD + config flag).
- Task table row `power_monitor`: `watch<PowerState>` carries VBUS-droop / SHUTDOWN_REQUEST trigger only; no external-power-present bit.
- H1 / H0 verdicts and channel contracts are unchanged.

See `spikes/gateway-handheld-power-architecture-spike.md` 2026-05-14 amendment, `spikes/tak-cot-integration-spike.md` 2026-05-14 gate-language correction, and `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md` for the originating session.

## Why this spike exists

The pivot adds tasks the original "Pi + Dragino HAT, single Tokio runtime, kiosk fullscreen" framing in ADR-004 / ARCHITECTURE.md §10 / §17 doesn't enumerate. With the handheld + base-mode-export design, the gateway binary must concurrently:

- read LoRa RX (polled on Pi 5 RP1 per dev-log 2026-05-05)
- validate frames (`crates/protocol`)
- write to SQLite (`tag_reports`, single writer)
- read from SQLite for the kiosk render (read-only handle)
- run the kiosk UI (egui + walkers, fullscreen)
- maintain RTC + opportunistic GPS time discipline (gpsd / chrony on Linux side; the Rust binary just reads system time)
- run a BLE central role for commissioning (intermittent — see `ble-commissioning-scope-spike.md`)
- monitor WiFi association / DHCP lease state for base-mode export gate
- monitor power-good / battery state / charge state from the power architecture (see `gateway-handheld-power-architecture-spike.md`)
- run the CoT/TAK emit gate (see updated `tak-cot-integration-spike.md`)
- run the CoT/TAK emitter when gate is open
- handle low-battery shutdown cleanly (no SQLite WAL corruption, no truncated CoT mid-message)
- handle the power button + commissioning button (debounce, semantics)

The runtime architecture is open: which tasks share async runtime, which use channels vs shared state, which are the same Tokio task vs separate, which lock SQLite write vs read, what blocks the UI thread.

This spike scopes the **task split + channel contracts**; it does not implement them.

## Hypothesis / research question

**H1.** Single `tokio` multi-threaded runtime, ~10 logical tasks, fan-in to a single `db_writer` task via an `mpsc` channel for all writes, multiple read-only SQLite handles (per UI render frame OR per CoT-emit cycle, opened/closed cheap-ish), `egui` runs as a *blocking thread* via `eframe`'s native target with a UI-side `mpsc` receiver fed by a `db_reader` notifier — keeps `egui` off the Tokio threadpool, avoids the "render blocked by I/O" failure mode.

**H0.** Single-threaded `tokio` (`current_thread`) is sufficient for the load and simplifies reasoning; UI runs on the same thread via cooperative yields. Likely too fragile under WiFi association + BLE central + LoRa RX polling at the same time; documented as fallback only.

## Scope fence

- **No code.** Spike output is a task list with channel contracts and an explicit "consumer side" / "producer side" per channel.
- **No SQLite migration.** Schema is per ADR-009 / ARCHITECTURE.md §10; this spike does not move it.
- **No protocol-crate changes.** Validation is a pure function call from RX path to writer; the spike does not redesign the protocol surface.
- **No `walkers` / `egui` integration code.** The pmtiles spike covers map; this spike just confirms egui runs off the Tokio worker pool.
- **No BLE state-machine implementation.** That's BLE-commissioning's downstream ticket.
- **No CoT XML schema work.** Lives in the tak-cot spike; this spike just allocates the emitter task slot.

## What to verify

### Task list (each row gets a contract)

> Defaults below are working hypotheses, not decisions. The spike verifies the task split, channel capacities, backpressure policy, and UI-thread shape; the table is a starting point for refinement.

| Task | Triggered by | Reads from | Writes to | Notes |
|---|---|---|---|---|
| `lora_rx` | polled SX1276 status | SPI | `mpsc<RawFrame>` to `validate` | hot loop, low-latency; runs polled per dev-log 2026-05-05 RP1 note |
| `validate` | `mpsc<RawFrame>` | — | `mpsc<ValidatedPosition>` to `db_writer`; structured logs | calls `crates/protocol`; drops with reason |
| `db_writer` | `mpsc<ValidatedPosition>` (also from `relay_self_position` and others) | — | SQLite `tag_reports` (single writer) | enforces recent-window dedup at INSERT |
| `db_reader_notify` | SQLite `update_hook` | SQLite read-only | broadcast channel to `kiosk` and `cot_emitter` | minimises per-render queries |
| `kiosk` | `eframe`'s native loop | broadcast from `db_reader_notify`; SQLite read-only | display | runs as a blocking thread off Tokio worker pool |
| `nmea_time` | UART from L80 GNSS | `/dev/ttyAMA*` | system time discipline via gpsd/chrony (Linux-side, not in-process); *reads* time for `received_at` | Linux-side daemon does the discipline; the Rust task only reads system clock and validates |
| `clock_validator` | timer | system clock + RTC sentinel | `watch<ClockState>` | drives the "RTC NOT SET" banner per ARCHITECTURE.md §11 / ADR-011 |
| `wifi_monitor` | netlink / dbus / polled `iw` | wpa_supplicant state | `watch<WifiState>` | reports associated, has-default-gateway, has-DHCP-lease |
| `power_monitor` | GPIO/I²C from power IC | power architecture signals | `watch<PowerState>` | per `gateway-handheld-power-architecture-spike.md` signal contract |
| `cot_gate` | gate inputs (per `tak-cot-integration-spike.md`) | — | `watch<ExportEnabled>` | Consumes the gate contract from `spikes/tak-cot-integration-spike.md`; this spike allocates the task slot and wiring only. |
| `cot_emitter` | `watch<ExportEnabled>` (only when true) + `db_reader_notify` | SQLite read-only | UDP multicast / TCP unicast to TAK | gated; never sends if `ExportEnabled=false` |
| `ble_central` | manual operator trigger (button or kiosk?) | — | BLE peripherals (relay/tag) | intermittent; *not* always-on; surfaces structured BLE-read results to a buffer |
| `button_handler` | GPIO interrupts | — | `mpsc<ButtonEvent>` | power button + commissioning button; debounced; routes to `shutdown_orchestrator` and `ble_central` |
| `shutdown_orchestrator` | low-battery from `power_monitor` OR power-button long-press | — | initiates clean shutdown: stops `db_writer`, flushes WAL, signals `cot_emitter` to drop in-flight messages, signals systemd shutdown | mandatory for SD-card / WAL safety per `production-concerns.md` §4 + power spike |

### Channel contract (each entry: type / capacity / backpressure policy)

- `mpsc<RawFrame>` — bounded(64); drop-oldest on overflow; logs DROP_OVERFLOW
- `mpsc<ValidatedPosition>` — bounded(64); drop-oldest on overflow; logs DROP_OVERFLOW
- `watch<ClockState>` — single-writer (`clock_validator`), many readers
- `watch<WifiState>` — single-writer (`wifi_monitor`), many readers
- `watch<PowerState>` — single-writer (`power_monitor`), many readers
- `watch<ExportEnabled>` — single-writer (`cot_gate`), one reader (`cot_emitter`)
- `mpsc<ButtonEvent>` — bounded(8); does not drop on overflow (button events are rare and significant)

### SQLite policy

- **Single writer**: `db_writer` task only.
- **Many readers**: `kiosk`, `cot_emitter`, ad-hoc maintenance queries. All open with WAL mode + read-only flag (`?mode=ro`).
- **Reader pattern:** open per task; long-lived handle.
- **Connection per write batch:** `db_writer` keeps a single writer connection, prepares dedup INSERT once per startup.
- **No reader holds the writer's lock.** Verify `update_hook` does not hold the writer lock while broadcasting.

### Shutdown semantics (clean shutdown checklist)

- `power_monitor` reports critical battery → `shutdown_orchestrator` triggered.
- `cot_emitter` finishes any in-flight write OR closes mid-message after a 1-second grace.
- `db_writer` flushes WAL via `PRAGMA wal_checkpoint(TRUNCATE)`.
- BLE central drops any active connection.
- `kiosk` is told to render a "shutting down" splash for 2 s.
- systemd shutdown signalled; OS handles the rest.

### UI blocking-avoidance verification

- `egui` is *immediate-mode* and re-renders ~60 Hz. Verify (or design) that the render loop does **no SQLite query** — instead it consumes a snapshot view-model produced by `db_reader_notify`.
- The `walkers` PMTiles read happens in a background task, not on the render thread (verified separately by the pmtiles spike retarget).

### Cross-spike implications (record, don't solve)

- `gateway-handheld-power-architecture-spike.md`: signal contract → `power_monitor`.
- `ble-commissioning-scope-spike.md`: trigger surface → `ble_central` activation.
- `tak-cot-integration-spike.md` (updated): `cot_gate` predicate + emit cadence.
- `pmtiles-walkers-spike.md`: confirms render thread shape.
- `gateway-rx-bringup-spike.md`: polled-RX behavior on RP1 → `lora_rx` task shape.
- ADR-009: WAL + recent-window dedup; this spike does not change either.

## Pass criteria

- Task table filled in (one row per task, with trigger / reads / writes / notes).
- Channel contract drafted (one entry per channel, with type / capacity / backpressure policy).
- SQLite single-writer / multi-reader policy committed.
- Shutdown semantics ordered list committed.
- UI-blocking-avoidance design rule stated.
- Cross-spike implications recorded.

## Fail criteria

- Polled-RX `lora_rx` cannot meet the timing budget without dedicating an OS thread (Tokio is too coarse) — recorded as a constraint and pushed to the bringup spike. The runtime spike accepts it as "lora_rx is a `std::thread` blocking task feeding an `mpsc::sync_channel` to validate"; documented explicitly.
- BLE central + WiFi monitor + LoRa polling on the same single-board runtime overshoot the Pi 5's CPU budget under peak — escalate to substrate spike (drop substrate option) or to a dual-runtime split.
- SQLite WAL checkpoint behavior under low-battery shutdown cannot be verified without bench testing — accept as an open follow-up; do not silently assume "it'll be fine".

## Fallback / next action

- If H1 holds: write task-shaped implementation tickets per task. Each ticket targets one task at a time, validated against the channel contract.
- If H0 (single-threaded fallback): downgrade explicitly; warn that BLE + WiFi + LoRa RX concurrent is the failure surface to test first.

## Decision note template

```
Date:
H1 / H0 verdict:

Task table (filled in):
  lora_rx: ___
  validate: ___
  db_writer: ___
  db_reader_notify: ___
  kiosk: ___
  nmea_time: ___
  clock_validator: ___
  wifi_monitor: ___
  power_monitor: ___
  cot_gate: ___
  cot_emitter: ___
  ble_central: ___
  button_handler: ___
  shutdown_orchestrator: ___

Channel contracts:
  mpsc<RawFrame>:           bounded(__), backpressure: ___
  mpsc<ValidatedPosition>:  bounded(__), backpressure: ___
  watch<ClockState>:        ___
  watch<WifiState>:         ___
  watch<PowerState>:        ___
  watch<ExportEnabled>:     ___
  mpsc<ButtonEvent>:        bounded(__), backpressure: ___

SQLite policy:
  single writer = db_writer:           confirmed
  read-only handles per reader:        confirmed
  no reader holds writer lock:         verified / TBD
  writer connection lifetime:          ___

Shutdown semantics (ordered):
  1.
  2.
  3.
  4.
  5.

UI blocking avoidance rule:
  ___ (statement)

Cross-spike implications recorded:
  power architecture:    ___
  ble commissioning:     ___
  tak-cot:               ___
  pmtiles:               ___
  rx bringup:            ___

Not implemented in this spike: code, task implementations, BLE state machine, CoT XML.

Next action:
```

## Cross-references

- `decisions/ADR-004-gateway-platform.md` — single-binary statement preserved; task split is *inside* the binary.
- `decisions/ADR-009-database-sqlite.md` — single writer / WAL preserved.
- `decisions/ADR-011-gateway-time-source.md` — gpsd/chrony Linux-side; Rust binary reads system clock.
- `production-concerns.md` §4 — clean shutdown is a daily concern on battery, not a once-in-a-blue-moon mains-loss concern.
- `spikes/gateway-handheld-power-architecture-spike.md` — ~~POWER_GOOD~~ `[CORRECTED 2026-05-14 — POWER_GOOD retired]` / SHUTDOWN_REQUEST contracts.
- `spikes/ble-commissioning-scope-spike.md` — BLE central activation surface.
- `spikes/tak-cot-integration-spike.md` — cot_gate predicate.
- `spikes/pmtiles-walkers-spike.md` — UI render thread shape.
- `spikes/gateway-rx-bringup-spike.md` — polled-RX timing.
- `spikes/handheld-pivot-doc-audit-spike.md` — registrar.
