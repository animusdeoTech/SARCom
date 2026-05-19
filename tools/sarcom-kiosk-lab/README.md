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

The sidebar follows the canonical project model from
[ADR-013 §9](../../decisions/ADR-013-multi-hop-flood-via-packet-id.md)
and the v1a UI data-model collapse decision
([`dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`](../../dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md)):
**one uniform `NodeData` shape for every node**, with kind-distinction
(tag / relay / gateway) reduced to an inventory map
(`HashMap<u8, NodeKind>`) used for icon glyph + label colour only.
Each row carries a status bullet, the lowercase node label (`tag-1`,
`relay-1`, `gw-0`), and a state-driven summary. Same row template
across all kinds; gateway elides the `last_seen` line (gateway is
local, `last_seen_secs = 0` sentinel).

800×480-ish window (resizable) with:

| Area | Description |
|---|---|
| **Header** | `SARCOM` brand wordmark, scenario badge (combobox in dim text), centred "last RX" with freshness dot and relative-age string, monospace wall clock, Edit toggle |
| **Map** | Subdued OSM line-art on near-black slate, current-fix tag dots, ghost markers for last-valid-fix when no current fix, ✚ relay marker, house-outline gateway, 1 px dashed track lines |
| **Sidebar** | `▼ NODES (n)` — collapsible flat list of hikers, relays, gateway. `▼ NO FIX (n)` — collapsible list of hikers whose latest report had `GPS_VALID=0`. **Counters footer card** at the bottom: `POSITION rx today`, `via relay`, `direct`, `dedup'd`, `CRC fail` (gateway-side counters; see ADR-013 §10 — coverage telemetry is a v2+ deferral, the lab values are synthetic). |
| **Bottom strip** | Read-only hint plus `PMTiles · OSM · zoom 17`. Replaced by a red `● DISTRESS` band when any tag has `flags.SOS=1` |
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
must not pretend it can on a per-row basis. None of the following appear
in the UI:

- hop counts
- "via relay-X" annotations on individual tag rows
- inferred radio routes / path lines from gateway to tag
- per-packet RSSI/SNR or coverage maps

A reception-log / coverage layer is an explicit v2+ deferral
([ADR-013 §10](../../decisions/ADR-013-multi-hop-flood-via-packet-id.md)).

**Counters footer footnote.** The sidebar's gateway counters card
includes `via relay` and `direct` lines. These are **synthetic mockup
values** rendered from a `Counters` struct on `SimState`; the v1 wire
protocol does not let the gateway derive them. They are present as a
design exploration of where v2+ telemetry might surface in the kiosk —
not a claim that v1 can compute them.

## Freshness model

Stale thresholds are **cadence-derived**, not generic dashboard heuristics
(see [TODO.md](../../TODO.md) "Staleness thresholds are wrong").

| Source | Cadence | Fresh | Aging | Stale | Very stale |
|---|---|---|---|---|---|
| Tag heartbeat | 300–330 s | < 330 s | < 660 s | < 1320 s | ≥ 1320 s |
| Tag in SOS | 45–60 s jittered | < 180 s | — | — | ≥ 180 s |
| Relay POSITION | ~1800 s | < 1800 s | < 3600 s | ≥ 3600 s | — |

Relay status is phrased as `POSITION Xm ago` and never colored stale at
tag thresholds — a 14-minute-old relay POSITION frame is healthy, not late.
Same packet kind as tag POSITION per ADR-013 (one packet type); there is
no separate "self-announce" frame on the wire.

## Clock validity

[ADR-011](../../decisions/ADR-011-gateway-time-source.md) makes RTC
validity load-bearing for the gateway. When the gateway boots without
a valid RTC, every relative-time string ultimately becomes
`time unavailable` per `format_age_or_unavailable` at
[`src/ui/mod.rs:20-26`](src/ui/mod.rs). **Gateway-self status (RTC
freshness, battery, render-tick liveness) is not surfaced in the v1a
kiosk** per
[`tickets/KIOSK-005-gateway-status-surface.md`](../../tickets/KIOSK-005-gateway-status-surface.md)
(deferred from v1a). The gateway appears in the sidebar as just
another node.

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
