---
title: "Spike — PMTiles + walkers on egui/Pi"
status: open
type: spike
timebox: 1 day
---

# Spike: PMTiles + walkers on egui / Raspberry Pi

## Why this spike exists

ARCHITECTURE.md bets on `egui + walkers + local .pmtiles` as the production basemap. That bet has never been tested on the target hardware. The kiosk-lab currently uses an egui-painted fake grid and an OSM-vector renderer — both are good enough for UI iteration but say nothing about whether the production basemap stack works on the Pi's GPU. This spike validates or kills that dependency before any production kiosk work commits to it.

## Hypotheses

**H1 (keep):** `egui + walkers` renders a local `.pmtiles` file smoothly enough on both Windows and Raspberry Pi for an 800×480 kiosk with markers, pan, and zoom.

**H0 (fallback):** PMTiles via walkers is too fragile, too slow, or too much glue code on the Pi GPU stack. We fall back to one of the fallback options below.

## Timebox

**1 day hard stop.** No extensions. If there is no local PMTiles visible in an egui window by end of day: H0 wins.

```
0.5 h — pin walkers version, confirm .pmtiles local-file API
1.5 h — obtain a small valid .pmtiles for the test area (Terril / Bokrijk)
2.0 h — Windows: cargo run renders the map in eframe
2.0 h — Pi: build and run, observe rendering and GPU behaviour
1.0 h — add a marker overlay and basic pan/zoom
1.0 h — write the decision note
```

## What to verify

1. Can walkers open a local `.pmtiles` file without a webserver?
2. Does it render in a native `eframe` window on Windows?
3. Does the same binary build and run on the Pi without GPU/driver issues?
4. Can SARCOM markers be drawn on top of the tile layer?
5. Is 800×480 usable — tiles don't degrade the sidebar/header layout?
6. Is the tile load latency acceptable at startup?
7. How much glue code is needed — is it self-contained enough to later live in `gateway/src/ui`?

## Pass criteria (keep PMTiles)

- Local `.pmtiles` renders in egui on Windows without network
- Same on Pi — no GPU crash, no blank map, subjectively usable framerate
- Markers can be drawn on top
- Code volume is small enough to eventually move into `gateway/src/ui`

## Fail criteria (fallback)

Any one of:
- walkers cannot open a local `.pmtiles` without a server
- Pi rendering fails or is visually unusable
- Styling/vector rendering turns into a separate project
- Getting a valid `.pmtiles` for the test area takes more than 1.5 h

## Fallback options

**Option A — egui-painted OSM vector (current kiosk-lab approach)**
Keep the fake-grid/OSM-vector renderer for v0 and v0.5. Revisit PMTiles when production kiosk work actually starts. Costs: no real basemap context for field tests.

**Option B — custom raster tile provider (MBTiles or pre-rendered PNG tiles)**
More work than PMTiles but fully controllable. Worth doing if walkers is the problem but egui rendering itself is fine.

**Option C — defer basemap entirely**
v0 and v0.5 acceptance criteria do not require a real basemap — a dot on a coordinate grid is sufficient to prove the pipeline. Basemap becomes a v1a concern.

## Decision note template

```
Date:
Result: H1 / H0
Evidence: [what rendered, what didn't, Pi GPU behaviour]
Decision: keep PMTiles / fallback A / fallback B / fallback C
Rationale:
Next action:
```
