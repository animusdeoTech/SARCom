---
title: "Spike — PMTiles + walkers on egui/Pi"
status: open
type: spike
timebox: 1 day
updated: 2026-05-06 (handheld pivot — retarget to 5" / Pi 5 handheld)
---

# Spike: PMTiles + walkers on egui / Raspberry Pi

## PIVOT NOTE 2026-05-06 — retarget to 5" handheld

The 2026-05-06 pivot retargets this spike from **800×480 7" wall-mount kiosk** to **5-inch handheld**. Concrete changes:

- **Display target:** the working candidate panel is the **Pi Touch Display 2 (5")** at **720×1280 portrait / 1280×720 landscape**, but the substrate spike (`spikes/gateway-handheld-substrate-spike.md`) has not committed to that panel yet. Treat 5" / 1280×720 landscape as the working bet; revisit if the substrate spike picks differently.
- **Pi GPU target:** **Pi 5 + RP1 + V3D / DRM/KMS stack** is the working substrate (was Pi 3B+/4 + VC4/V3D). Different driver stack; the previous "fall back to `iced`/`slint` if the Pi's graphics drivers misbehave" story still applies but the failure surface is different.
- **800×480 small-screen branch is dropped** unless the substrate spike picks a smaller panel (Pi Zero 2W substrate option). All references below to 800×480 should be read as "5" / 1280×720 landscape" unless the substrate verdict changes.
- **Acceptance now includes a handheld-orientation question:** native 720×1280 portrait vs rotated 1280×720 landscape. The kiosk-lab currently sizes for landscape; the production pivot may want either. Ad-hoc check during the spike: render at both orientations, screenshot, decide which is more readable for hut staff at arm's length.
- **Marker-overlay UX needs to fit a smaller display.** Touch targets sized for finger-on-handheld (≥9 mm = ~32 pixels at typical Pi 5 panel DPI). Sidebar/header layout from the kiosk-lab may need a stack rather than side-by-side at 5".
- **Scope fence change:** this spike is no longer just "does PMTiles render"; it also asks "does it render *legibly* on the chosen handheld panel at the chosen orientation".

The original spike body below remains substantive and useful for the Windows-offline branch. Treat all "Pi" references as "Pi 5 unless the substrate spike says otherwise" and all "800×480" as "5" / 1280×720 landscape unless overridden".

---

## Why this spike exists

ARCHITECTURE.md bets on `egui + walkers + local .pmtiles` as the production basemap. That bet has never been tested on the target hardware. The kiosk-lab currently uses an egui-painted fake grid and an OSM-vector renderer — both are good enough for UI iteration but say nothing about whether the production basemap stack works on the Pi's GPU. This spike validates or kills that dependency before any production kiosk work commits to it.

## Scope fence

The experiment runs **inside `tools/sarcom-kiosk-lab/`** unless that is technically impossible — e.g., walkers requires an egui major kiosk-lab cannot adopt without a destabilising upgrade (see Version-compatibility precheck below).

If a separate throwaway crate is used, the decision note must:
- explain the technical reason kiosk-lab could not host the experiment, and
- name the modules (kiosk-lab `src/map/*`, future `gateway/src/ui/*`) that would later receive the proven code.

The point is to test PMTiles in the same eframe binary that already runs the kiosk UI, not in a clean-room demo that proves nothing about the production stack.

## No production rewrite

This spike does **not**:
- rewrite the kiosk UI architecture,
- move code into `gateway/src/ui/` or any other production module,
- introduce new map abstractions unless strictly required to isolate the PMTiles experiment from the existing `src/map/{fake_grid,osm_vector,markers,mod}.rs`.

If isolation needs a new abstraction, keep it inside kiosk-lab, mark it experimental, and record in the decision note what would have to change for it to graduate to production.

## Version-compatibility precheck

Before any code is written, inspect:
- `tools/sarcom-kiosk-lab/Cargo.toml`
- `tools/sarcom-kiosk-lab/Cargo.lock`

Record current versions of `eframe`, `egui`, and `walkers` (walkers will be absent on first run — that itself is a finding to record). As of 2026-05-06: kiosk-lab pins `eframe 0.29.1` and `egui 0.29.1`; walkers is not yet a dependency.

Resolve the chosen `walkers` version and the egui version it depends on:

- If it matches kiosk-lab's pinned egui, proceed.
- If it requires a different egui major, **stop and record the blast radius** before changing anything: how many files in `tools/sarcom-kiosk-lab/src/` import `egui::*`, which kiosk-lab features (`header`, `sidebar`, `edit_panel`, `palette`, `fake_grid` map, `osm_vector` map, `markers`) would need a coordinated update, and whether breaking changes in egui's widget API hit them.
- A required egui upgrade is a **first-class spike result**, not a setup chore. It can flip the answer to H0 even if PMTiles itself works in isolation.

Do not blindly bump egui across kiosk-lab to satisfy walkers. Record the cost; bring it back to the user before continuing.

## Hypotheses

**H1 (keep):** `egui + walkers` renders a local `.pmtiles` file smoothly enough on both Windows and Raspberry Pi (Pi 5 per substrate-spike close) for a **5" handheld panel at 1280×720 landscape** (working candidate: Pi Touch Display 2 5") with markers, pan, and zoom, at finger-friendly touch-target sizes (≥9 mm hit areas).

**H0 (fallback):** PMTiles via walkers is too fragile, too slow, or too much glue code on the Pi GPU stack. We fall back to one of the fallback options below.

## Timebox

**1 day hard stop.** No extensions. If there is no local PMTiles visible **offline** in an egui window by end of day: H0 wins.

```
0.5 h — version-compatibility precheck (kiosk-lab eframe/egui vs chosen walkers)
0.5 h — pin walkers version, confirm .pmtiles local-file API
1.5 h — obtain a small valid .pmtiles for the test area (Terril / Bokrijk)
1.5 h — Windows: cargo run inside kiosk-lab renders the map with host offline
2.0 h — Pi: build, run, observe rendering and GPU behaviour, record evidence
1.0 h — add a marker overlay and basic pan/zoom
0.5 h — styling (hard cap; see Styling cap)
0.5 h — write the decision note
```

## What to verify

1. Can walkers open a local `.pmtiles` file without **any** HTTP tile source — i.e., no `HttpTiles`, no online fallback, no implicit network fetch (including no localhost server in front of the file)?
2. Does the resulting map render in a native `eframe` window on Windows with the host **fully offline** (Wi-Fi off, or known-unreachable URL configured to prove no silent fallback)?
3. Does the same binary build and run on the Pi without GPU/driver issues?
4. Can SARCOM markers be drawn on top of the local PMTiles layer?
5. Is the 5" handheld target (1280×720 landscape; 720×1280 portrait as alternative) usable — tiles don't degrade the sidebar/header layout, touch targets are reachable with a thumb at handheld grip?
6. Is the tile load latency acceptable at startup (cold open of the `.pmtiles` file)?
7. How much glue code is needed to wire walkers into the existing kiosk-lab `src/map/` module structure? Is it self-contained enough to later live in `gateway/src/ui/`?

## Pass criteria (keep PMTiles)

- Local `.pmtiles` renders in egui on Windows **with the host offline** — no `HttpTiles`, no online tile server, no localhost shim in the pass path
- Same on Pi — no GPU crash, no blank map, subjectively usable framerate (see Pi evidence requirements)
- Markers can be drawn on top of the local tile layer
- Code volume is small enough to eventually move into `gateway/src/ui`
- The Version-compatibility precheck did not require a destabilising egui upgrade across kiosk-lab (or, if it did, the upgrade itself passed a kiosk-lab smoke run — header / sidebar / edit panel / palette / existing maps still build and render)

## Fail criteria (fallback)

Any one of:
- walkers cannot open a local `.pmtiles` without a server, or its only documented offline path requires `HttpTiles` against a localhost server (that is not "offline" in the sense the kiosk needs)
- Pi rendering fails or is visually unusable
- Styling / vector rendering turns into a separate project (see Styling cap)
- Getting a valid `.pmtiles` for the test area takes more than 1.5 h
- walkers requires an egui major that would force a kiosk-lab-wide upgrade with breaking changes the existing UI cannot absorb in this timebox

## Pi evidence requirements

The decision note must record, verbatim, the following for the Pi run. A Pi result without these fields is not a Pi result and cannot satisfy H1.

- **Pi model + OS + display:** e.g., "Raspberry Pi 5 4 GB, Yocto image rev XYZ, Pi Touch Display 2 5" at 1280×720 landscape" or "Pi 5, Raspberry Pi OS Bookworm aarch64, Pi Touch Display 2 5"" — be specific.
- **Exact run command:** the literal command line used.
- **Window opens?** yes / no.
- **Map tiles visible?** yes / no / partial — describe what's missing if partial.
- **Pan/zoom responsiveness:** good / tolerable / bad. "Tolerable" means a handheld operator could live with it; "bad" is a fail.
- **Touch targets sized for finger?** yes / no / N/A (if no touch test in this run).
- **GPU / wgpu / glutin / driver errors:** copy the exact stderr lines, even if the window still opened.
- **Screenshot or photo path:** path under `resources/` or `dev-log/` if captured. Phone photo is acceptable for the Pi screen.

## Styling cap

Use whatever default or example styling walkers ships with. Do not spend more than **30 minutes** customising the map style (colours, fonts, label placement, vector layer selection).

If styling becomes the hard part — the default style is unreadable on the 5" handheld panel at the chosen orientation, or vector rendering needs a custom shader / sprite sheet to look acceptable — that is **evidence for H0 or fallback B**, not a problem to keep grinding on inside the spike.

Record time spent on styling in the decision note.

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
Scope:
  - ran inside tools/sarcom-kiosk-lab/? yes / no
  - if no, why kiosk-lab could not host:
  - target modules for graduation (kiosk-lab src/map/*, gateway/src/ui/*):
Version-compatibility precheck:
  - kiosk-lab eframe/egui (current pin):
  - walkers (chosen):
  - walkers' required egui:
  - blast radius if an egui upgrade was needed:
Offline proof:
  - Windows: rendered local .pmtiles with host offline? yes / no
  - HttpTiles touched anywhere in the pass path? yes / no
Pi evidence (verbatim):
  - Pi model + OS + display + orientation:
  - Exact run command:
  - Window opens:
  - Map tiles visible:
  - Pan/zoom responsiveness:
  - Touch targets sized for finger:
  - GPU / wgpu / glutin / driver errors:
  - Screenshot / photo path:
Markers on top: yes / no
Styling time spent: __ minutes (cap: 30)
Decision: keep PMTiles / fallback A / fallback B / fallback C
Rationale:
Next action:
```
