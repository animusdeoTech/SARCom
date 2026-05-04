# SARCOM Kiosk Lab

Native Rust/egui desktop tool for designing and testing the SARCOM kiosk UI.  
No browser, no npm, no Electron. Matches the production stack (Rust + egui).

## Requirements

- Rust toolchain (stable, `x86_64-pc-windows-msvc`)
- Windows 10/11

## Run

This tool is intentionally a **standalone Cargo workspace** — it is not a
member of the SARCOM root workspace (which only carries `crates/protocol`).
That keeps the kiosk lab's `eframe` dependency stack out of the firmware /
gateway build graph. Run cargo commands either from inside the tool
directory or with an explicit `--manifest-path`:

```powershell
cd tools\sarcom-kiosk-lab
cargo run

# or, from the repo root:
cargo check --manifest-path tools\sarcom-kiosk-lab\Cargo.toml
cargo test  --manifest-path tools\sarcom-kiosk-lab\Cargo.toml
```

`cargo test` from the root workspace will *not* pick this tool up — that
is deliberate.

Release build (no console window):

```powershell
cargo build --release
.\target\release\sarcom-kiosk-lab.exe
```

## What you get

Sidebar defaults to **320 px**. The `Edit → Layout` slider clamps to
**300–420 px**.

INFRA, SYSTEM, and TAG DETAILS render as **full-width status cards**
(the inner Frame is forced to `ui.available_width()` so the card
background spans the sidebar instead of shrinking to text width).
Inside each card, key/value status rows use a **compact two-column
layout**: a fixed-width dim monospace key cell (~80 px) on the left, a
flexible value cell on the right with `Label::wrap()` enabled, both
left-aligned. Long values like `time unavailable` or `none on record`
fall back to wrapping inside the value cell — no `right_to_left`
layout, which was the original right-edge clipping source under
`SidePanel` + `ScrollArea`. The HIKERS list rows stay inline (compact,
short content with badges on the right).

800×480-ish window (resizable) with:

| Area | Description |
|---|---|
| **Header** | Scenario selector, GW online dot, clock (or `RTC NOT SET` warning), Edit toggle |
| **Map** | Dark grid, current-fix tag markers, ghost markers for last-valid-fix when no current fix, relay diamond, gateway square, track lines |
| **Sidebar** | `HIKERS` (mission-sorted) → `TAG DETAILS` → `INFRA` (relay self-announce) → `SYSTEM` (gateway/RTC/radio) → `SIGHTING LOG` |
| **Edit panel** | Floating window — layout tweaks, tag state overrides, save/load |

## Scenarios

Switch via the combobox in the header:

| Scenario | What it shows |
|---|---|
| Normal | One tag, normal heartbeat (8 s ago) |
| SOS | One tag with SOS active (red pulse ring, `DISTRESS` banner) |
| Stale | Two tags crossing real cadence-derived thresholds: stale (>11 min) + very stale (>22 min) |
| No Fix | One tag with current `GPS_VALID=0`; ghost marker at last valid fix |
| Multi-Tag | Four tags: normal, SOS, stale, no-fix-with-ghost |
| Clock Invalid | `RTC NOT SET` header banner; all relative time strings are replaced with `time unavailable` (per ADR-011). Map markers still render — ordering is synthetic. |
| SOS + No Fix | Tag is alive and distressed but current report has `GPS_VALID=0`. **No current-position marker is fabricated.** A ghost marker is drawn at the last valid fix with red SOS pulse emphasis. Sidebar shows last-frame age and last-valid-fix age separately. |

## Interacting

- **Click a tag** on the map or in the sidebar list to select it
- **Drag any marker** (tag, relay, gateway) to reposition it
  - For a no-fix tag that has a last valid fix, the visible marker *is*
    the ghost — dragging it moves `last_valid_fix_pos`, not a current
    position (a current position would be a lie when `gps_valid=false`).
- **Edit panel** (header → Edit button):
  - Sidebar width slider
  - Show/hide track lines and sighting log
  - Tag Tweak: change any tag's state, last-seen time, GPS/SOS/battery flags
  - Save / Load layout JSON

## Layout JSON

Positions and settings are saved to `layout.json` in the working directory (i.e., `tools\sarcom-kiosk-lab\layout.json` when run via `cargo run`).

```powershell
# Save from within the app: Edit → Save / Load → Save layout
# Or load a previously saved file: Edit → Save / Load → Load layout
```

## Screenshot

Use Windows Snipping Tool (`Win+Shift+S`) or:

```powershell
# Full window capture (requires PowerShell 7 + Windows.Graphics.Capture)
# Or just Snip — it's one keypress
```

Native screenshot export is not implemented; the snipping tool is fast enough for mockup work.

## Limitations (first pass)

- Map is a fake dark grid — no real tiles. `walkers` + PMTiles integration is deferred.
- Clock counts elapsed seconds since process start, not wall time.
- No real LoRa data — all state is synthetic from the selected scenario.
- PNG export requires Snipping Tool.

## SARCOM v1 truth — what this lab deliberately does NOT show

The kiosk lab mirrors what the v1 protocol and gateway actually know. Per
[ADR-013](../../decisions/ADR-013-multi-hop-flood-via-packet-id.md) the wire
carries one packet type (`POSITION`) with **no FORWARD envelope, no hop
count, no path array, and no per-hop RSSI/SNR**. The gateway therefore
cannot tell "via relay-1" from "direct" for a given packet, so the kiosk
must not pretend it can. None of the following appear in the UI:

- hop counts
- "via relay-X" annotations on tag rows
- direct vs. via-relay counters
- inferred radio routes / path lines from gateway to tag
- per-packet RSSI/SNR or coverage maps

A reception-log / coverage layer is an explicit v2+ deferral
([ADR-013 §10](../../decisions/ADR-013-multi-hop-flood-via-packet-id.md)).

## Freshness model

Stale thresholds are **cadence-derived**, not generic dashboard heuristics
(see [TODO.md](../../TODO.md) "Staleness thresholds are wrong").

| Source | Cadence | Fresh | Aging | Stale | Very stale |
|---|---|---|---|---|---|
| Tag heartbeat | 300–330 s | < 330 s | < 660 s | < 1320 s | ≥ 1320 s |
| Tag in SOS | 45–60 s jittered | < 180 s | — | — | ≥ 180 s |
| Relay self-announce | ~1800 s | < 1800 s | < 3600 s | ≥ 3600 s | — |

Relay status is phrased as `self-ann Xm ago` and never colored stale at
tag thresholds — a 14-minute-old relay self-announce is healthy, not late.

## Clock validity

[ADR-011](../../decisions/ADR-011-gateway-time-source.md) makes RTC
validity load-bearing for the UI. When the gateway boots without a valid
RTC, every relative-time string in the kiosk is replaced with
`time unavailable`. The header shows `⚠ RTC NOT SET` and the right side
shows `CLOCK INVALID` instead of wall time. Map markers still render
(ordering is whatever the DB hands back), but the kiosk does not invent
"42 s ago" from a free-running tick.

## SOS + No GPS fix

The most operator-critical SARCOM case: the tag is alive and distressed
(SOS bit set in the most recent `POSITION`) but `GPS_VALID=0` so the
current coordinates in that frame are sentinel. The kiosk handles this
honestly:

- the current `pos` is **not** rendered as a map marker
- a ghost marker is drawn at `last_valid_fix_pos` with a dashed outer
  ring, faded fill, label `<TAG> last fix`, sub-label `NO FIX · <age>`
- the SOS pulse ring is still drawn over the ghost, in red
- the `DISTRESS` banner reads
  `DISTRESS · <tag> · last frame <age> · NO CURRENT GPS FIX · last valid fix <age>`
- the sidebar tag-details card shows `GPS: NO FIX`, `LAST FIX: <age>`,
  `AT: <coords>` — it never fakes a current position from sentinel data
