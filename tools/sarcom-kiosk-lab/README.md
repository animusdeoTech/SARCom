# SARCOM Kiosk Lab

Native Rust/egui desktop tool for designing and testing the SARCOM kiosk UI.  
No browser, no npm, no Electron. Matches the production stack (Rust + egui).

## Requirements

- Rust toolchain (stable, `x86_64-pc-windows-msvc`)
- Windows 10/11

## Run

```powershell
cd tools\sarcom-kiosk-lab
cargo run
```

Release build (no console window):

```powershell
cargo build --release
.\target\release\sarcom-kiosk-lab.exe
```

## What you get

800×480-ish window (resizable) with:

| Area | Description |
|---|---|
| **Header** | Scenario selector, GW online dot, clock, Edit toggle |
| **Map** | Dark grid, tag circles, relay diamond, gateway square, track lines |
| **Sidebar** | Active tags list, tag detail card, network status, sighting log |
| **Edit panel** | Floating window — layout tweaks, tag state overrides, save/load |

## Scenarios

Switch via the combobox in the header:

| Scenario | What it shows |
|---|---|
| Normal | One tag, normal state |
| SOS | One tag with SOS active (red pulse ring, banner) |
| Stale | Two tags: stale + very stale, one with low battery |
| No Fix | One tag with no GPS fix |
| Multi-Tag | Four tags: normal, SOS, stale, no-fix |

## Interacting

- **Click a tag** on the map or in the sidebar list to select it
- **Drag any marker** (tag, relay, gateway) to reposition it
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
