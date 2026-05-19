# KIOSK-005 — Gateway status surface — v1a mockup rationale

Single-design strict-ADR mockup. Four stacked 800×480 lab fixtures
showing the sidebar `gw-0` row in four states: healthy + charging,
healthy not charging, low battery, and battery unknown. The render-tick
indicator lives **in the sidebar gw-0 row** (NOT in the header) per
`tickets/KIOSK-005-gateway-status-surface.md:52, 55, 82-83`. Map areas
abstracted with three-layer label per `pmtiles_map.rs:82-125`. SVG at
`UX/mockups/KIOSK-005-gateway-status.svg`.

## Scenario summary (four states)

| State | Description | Battery icon fill | Charging icon | Primary glyph |
|---|---|---|---|---|
| 1 | Healthy, **charging**, 78% | GREEN, fill 78% | lightning bolt, GREEN | `●` (GREEN) |
| 2 | Healthy, not charging, 45% | AMBER, fill 45% | omitted | `●` (GREEN) |
| 3 | Low battery, 12% (not charging) | RED, fill 12% | omitted | `⚠` (AMBER) |
| 4 | Battery **unknown** (Option = None) | outline only, dim grey | omitted | `●` (GREEN) |

In every state the **render-tick pulse dot** is rendered at the
right edge of the gw-0 row with adjacent label `ui` in 8-9 pt
`TEXT_DIM`. The dot is `GREEN` (matches healthy state) and pulses on
every egui render frame.

The map area is abstracted in all four panels; the abstract label
names the three layers (basemap + LIDAR hillshade + OSM XML overlays)
per `pmtiles_map.rs:82-125`. KIOSK-001 owns the full map chrome render.

## Per-element source-of-truth citations

| Element | Citation |
|---|---|
| `battery_pct: Option<u8>` + `charging: Option<bool>` extension | `KIOSK-005:33-48, 78` |
| Battery icon ~24×10 px, outline + proportional fill | `KIOSK-005:80-88` |
| Gradient by threshold (GREEN / AMBER / RED) | `KIOSK-005:42-44, 84-87` |
| Charging icon lightning bolt, GREEN, ~8×10 px | `KIOSK-005:45, 90-94` |
| Charging icon shown only when `Some(true)` | `KIOSK-005:81, 94` |
| `BAT unknown` rendering (dim grey) | `KIOSK-005:46, 79` |
| Render-tick in sidebar gw-0 row | `KIOSK-005:52, 55, 82` |
| Render-tick label is `ui` | `KIOSK-005:53, 75-77, 83` |
| Existing gateway row code | `tools/sarcom-kiosk-lab/src/ui/sidebar.rs:268-302` |
| GatewayData base + KIOSK-005 ext | `tools/sarcom-kiosk-lab/src/data.rs:163-167` + `KIOSK-005:35-39` |
| Three-layer map render stack (abstracted in this mockup) | `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:82-125` |
| Overlay schema | `tools/sarcom-kiosk-lab/src/map/region.rs:25-71` |
| Hillshade implementation context | `dev-log/2026-05-16-lidar-overlay-implementation.md` |
| ADR-007 read-only invariant | `decisions/ADR-007-touchscreen-primary-ui.md:38-46` |
| 800×480 lab fixture (ADR-015-pending) | `tools/sarcom-kiosk-lab/README.md:53` + `README.md:36` |
| Palette constants | `tools/sarcom-kiosk-lab/src/ui/palette.rs:8-19` |
| No coord readout (KIOSK-002 deferred) | `tickets/README.md:30, 53, 71, 85` |

## Render-tick scope

**What `ui` proves:** the egui render loop is advancing. If the dot
stops pulsing, the operator can read "the UI froze" — which is
distinct from "no tag updates because no tag is moving."

**What `ui` does NOT prove:**

- It does **not** prove the LoRa receiver task is alive.
- It does **not** prove the gateway process is healthy.
- It does **not** prove any radio is decoding frames.

Those are deferred-pending-real-gateway signals per `KIOSK-005:57-64`.
**No mocked signal in this ticket. No fake radio. No fake process.**

## What is NOT rendered

- **NO header-side render-tick.** `KIOSK-005:55, 82, 92, 118`.
- **NO LoRa-RX liveness rendering.** `KIOSK-005:57-60`.
- **NO gateway-process liveness rendering.** `KIOSK-005:62-64`.
- **NO `BAT 0%` when state is unknown.** Use `BAT unknown`. `KIOSK-005:46, 79`.
- **NO `BAT —` placeholder.**
- **NO synthetic mocks of signals the real gateway doesn't yet
  expose.** `KIOSK-005:74`.
- **NO floating buttons** on the map.
- **NO coord readout** anywhere.

## What a reviewer verifies

- **Render-tick lives in the sidebar gw-0 row**, NOT in the header.
- **Render-tick label is `ui`**.
- **Battery icon uses a gradient by threshold**: GREEN / AMBER / RED
  / none, with fill width proportional to charge level.
- **Charging is `icon + colour change`** (lightning bolt GREEN).
- **Unknown battery renders as `BAT unknown`** — never `0%`, never `BAT —`.
- **Low-battery state (12%)** shows `⚠` glyph, RED battery, RED text.
- **Render-tick positioned identically in all four states.**
- **No header-side liveness indicator.**
- **No fake radio, no fake process** signal.
- Map area's abstract label names the three layers per
  `pmtiles_map.rs:82-125`.

## Open questions for Pieter

1. **Battery-icon thresholds (3-bucket vs 2-bucket).** Mockup locks
   ≥50% / 20-50% / <20%; `KIOSK-005:42-43` example is ≥40% / <20%
   (`KIOSK-005:44` says "implementer chooses").
2. **Render-tick visual treatment.** Mockup is static dot + margin
   annotation per `KIOSK-005:76`; SVG `<animate>` is the alternative.
3. **`gw-0` glyph in low-battery state.** Mockup flips `●` → `⚠`
   when battery is low; `KIOSK-003:39` keeps the gateway glyph tied
   to RTC.
