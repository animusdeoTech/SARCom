---
title: "Dev log -- osm_overpass overlay kind (auto-fetch from public Overpass API) joins the [[overlays]] schema"
date: 2026-05-16
type: dev-log
tags: [implementation, osm, overpass, regions, contract-extension, schema-enum-addition]
---

# 2026-05-16 -- osm_overpass overlay kind

Implementation ticket. Adds a new `osm_overpass` overlay kind alongside the existing `osm` (hand-drawn) and `hillshade` (LIDAR-derived) kinds. Closes the JOSM-round-trip friction surfaced earlier in the same day's session: most regions don't need hand-drawn OSM, they need the same Overpass-query slice everyone else uses. With this kind in the schema, a new region's OSM overlay is a `scripts\fetch-region.ps1 <name>` away instead of "open JOSM, draw the polygons, save the file."

## What changed

- **`tools/sarcom-kiosk-lab/src/map/region.rs`** -- third variant on the tagged `Overlay` enum:
  ```rust
  Overlay::OsmOverpass {
      features: Vec<String>,   // #[serde(default = "default_overpass_features")]
      endpoint: String,        // #[serde(default = "default_overpass_endpoint")]
  }
  ```
  Both fields default-via-fn, so a region.toml `[[overlays]] kind = "osm_overpass"` block with no further fields parses cleanly. New `osm_overpass_path()` accessor returns the conventional `<dir>/osm-overpass.osm` if the file exists, else `None` with a stderr warning (same soft-fallback shape as `osm_overlay_path()` and `hillshade_overlay_path()`). The fetched-file path is fixed by convention -- no `file` field is exposed to region.toml authors.
- **`tools/sarcom-kiosk-lab/src/app.rs`** -- new `osm_map_overpass: Option<OsmMap>` field on `KioskLabApp`. Loaded at startup via `region.osm_overpass_path()` through the same `OsmMap::load_from_path` entry point as the hand-drawn overlay (Overpass XML is standard OSM XML). Footer label gains an `"overpass"` token in the dynamic compose; PMTiles-mode label reads e.g. `PMTiles · hillshade · overpass · OSM · zoom 14` when all four are present.
- **`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`** -- `PmTilesMap::show` signature grows a third parameter (`osm_overpass: Option<&OsmMap>`); the closure paints the Overpass overlay BEFORE the hand-drawn overlay so explicit hand-annotated detail wins the z-fight when both are present. No new walkers API consulted -- this is a second call to the same `OsmMap::draw_with_projector` already used by the hand-drawn path.
- **`tools/sarcom-kiosk-lab/src/map/mod.rs`** -- the `show_map` dispatch passes `self.osm_map_overpass.as_ref()` alongside `self.osm_map.as_ref()` into `pm.show(...)`.
- **`scripts/fetch-region.ps1`** -- new `Invoke-OsmOverpassFetch` function. Reads the overlay block's `features` array (PowerShell module-scope defaults `$DefaultOverpassFeatures` / `$DefaultOverpassEndpoint` mirror the Rust-side defaults so unspecified fields land on the same values), reads `[bounds]` from region.toml, builds an Overpass QL query of the form
  ```
  [out:xml][timeout:25][bbox:S,W,N,E];
  (
    way["highway"];
    way["natural"="water"];
    ...
  );
  out body;
  >;
  out skel qt;
  ```
  POSTs to the endpoint via `Invoke-WebRequest`, writes the response body to `<region>\osm-overpass.osm`, writes `<region>\osm-overpass.provenance.json` (endpoint, bbox in both Overpass-native S,W,N,E and WGS84 ordering, features, verbatim query, sha256, bytes, fetch_seconds, ODbL license, OpenStreetMap attribution). HTTP 429 / 504 / other failures exit non-zero with operator-actionable messages; no auto-retry. Dispatch loop at the bottom of the script gets a new `'osm_overpass'` case alongside the existing `'osm'` no-op and `'hillshade'` GDAL branch.
- **`scripts/fetch-region.ps1` -- `Get-OverlayBlocks`** -- extended with a two-pass parser. Pass 1 captures TOML array values (`key = [ "a", "b", ... ]`, multi-line allowed) so the new `features` field deserializes correctly; pass 2 handles scalar k=v lines on the array-stripped body so a bare `[` is never mis-captured as a value. Existing `osm` and `hillshade` blocks still parse identically (verified by re-parsing terril-waterschei/region.toml after the edit).
- **`resources/regions/.gitignore`** -- added `**/osm-overpass.osm` and `**/osm-overpass.provenance.json` so the fetched product + sidecar stay out of git. The `[[overlays]] kind = "osm_overpass"` block in region.toml remains the durable artefact.
- **`resources/regions/terril-waterschei/region.toml`** -- new `[[overlays]] kind = "osm_overpass"` block added BEFORE the existing `kind = "osm"` block (matching the render z-order: overpass below, hand-drawn above). Worked-example feature set matches `default_overpass_features()`; endpoint omitted to exercise the default.
- **`resources/regions/README.md`** -- schema section gains the `osm_overpass` example alongside hillshade and osm; per-product provenance section gains the `osm-overpass.provenance.json` field list; z-order line updated.
- **`resources/regions/QUICKSTART.md`** -- new Step 6 ("(Recommended for new regions) Add an Overpass-fetched OSM overlay") inserted between the hillshade step and the hand-drawn step (which becomes Step 7). Documents the minimum block (`kind = "osm_overpass"` alone), the optional `features` and `endpoint` overrides, the default-feature list with its cite-locations in `region.rs` and `fetch-region.ps1`, the bake invocation, the rate-limit behaviour (429 / 504 surface with operator-actionable messages, no auto-retry), and the silent-render-time failure mode.

## Region-descriptor + provenance-sidecar contract extension

`spikes/field-deployment-package-shape-spike.md` (status `closed: 2026-05-16`) owns the region-descriptor and provenance-sidecar contracts at its `Cross-spike contracts produced or consumed` section. This implementation extends **both**, via the same dev-log-mechanism precedent set by `dev-log/2026-05-16-lidar-overlay-implementation.md` (no spike re-open):

- **Region-descriptor contract** -- `osm_overpass` is a new enum value in the `[[overlays]] kind = ...` tagged union. The contract's mandatory `{ name, bounds:WGS84, source-extract-date }` set is unchanged. The recommended `[[overlays]]` slot gains one more option.
- **Provenance-sidecar contract** -- `osm-overpass.provenance.json` is a new sidecar file with the same mandatory base set (`source-extract URL` = `endpoint`, `source-extract-date` derivable from `fetched_at_utc`, `region bounds:WGS84`, `sha256`, `bytes`) plus query-specific audit fields (`bbox_overpass`, `features`, `query`).

## Z-order, footer wording, fallback semantics

- **Z-order in PMTiles mode**: basemap (`with_layer α=1.0`) -> hillshade (`with_layer α=0.5`) -> osm_overpass (closure-painted) -> osm (closure-painted, on top of osm_overpass) -> markers -> region badge. Hand-drawn OSM wins the z-fight against Overpass because the hand-drawn block is the operator's explicit override.
- **Footer wording**: each overlay contributes one short token. `PMTiles` always; `hillshade` when `PmTilesMap::has_hillshade()`; `overpass` when `osm_map_overpass.is_some()`; `OSM` when `osm_map.is_some()`; `zoom N` from walkers' `MapMemory::zoom()`. Order in the label mirrors render z-order so the rightmost-readable tokens are the top-of-stack layers.
- **Fallback**: every overlay is `Option<...>` end-to-end. Missing files log a single stderr warning at startup; the kiosk-lab renders without them. A region with only `kind = "osm_overpass"` declared and the Overpass fetch not yet run shows basemap + markers, footer reads `PMTiles · zoom N`.

## Rust doc lookup posture

No fresh walkers / egui / serde API consulted. The implementation reuses the existing patterns from `dev-log/2026-05-16-lidar-overlay-implementation.md`:

- `serde(tag = "kind", rename_all = "snake_case")` -- adding a variant to an existing tagged enum, no new serde surface.
- `OsmMap::load_from_path` + `OsmMap::draw_with_projector` -- second call to the same primitives the hand-drawn overlay already uses; the OSM XML parser at `tools/sarcom-kiosk-lab/src/map/osm_vector.rs:272-357` parses Overpass output without modification (Overpass `[bbox:...]` emits `<bounds>` and `<node>` + `<way>` + `<nd>` + `<tag>` elements identical to JOSM-saved OSM XML).
- `walkers::Map::show` closure -- one extra `if let Some(osm) = osm_overpass { osm.draw_with_projector(...) }` call; no new walkers API.

Per the CLAUDE.md doc-lookup protocol, "validation and repair layer" is `cargo check`; ran clean after each Rust edit.

## What did NOT change

- **No protocol change.** SARCom's radio protocol is unaffected; osm_overpass is a kiosk-side visualization concern.
- **No new spike opened.** Extension goes through the dev-log mechanism per the closed sister-spike's contract-extension precedent. Parent `spikes/pmtiles-walkers-spike.md` and sister `spikes/field-deployment-package-shape-spike.md` stay `closed: 2026-05-16`.
- **No ADR re-opened.** ADR-005 (kiosk UI: native Rust GUI, walkers + PMTiles) and ADR-008 (no cloud, no downlink) are cited as-is: this overlay is an outbound HTTPS GET at bake time, not at runtime; the runtime kiosk-lab has no internet dependency. The bake step is a developer workstation activity, not a deployed-gateway activity. ADR-008's no-internet rule is for the deployed gateway; the bake-time workstation operates separately.
- **No new walkers feature added.** OsmMap rendering goes through the existing `draw_with_projector` closure-callback path inside `Map::show`; no `with_layer` change.
- **No JOSM toolchain change.** Hand-drawn OSM (`kind = "osm"`) stays as the explicit-override path; QUICKSTART Step 7 documents when to reach for it.

## Verification before declaring done

- `cargo check --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` -- clean.
- `cargo build --release --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` -- pending; runs as part of the operator-visual-verification step.
- ADR citations vs. contestation: `rg -n "ADR-005|ADR-007|ADR-008"` against touched files surfaces only the ADR-008 citation in this dev-log entry (the deployed-gateway rule); ADR-005 and ADR-007 are not contested. None of the kiosk-lab Rust files, region.toml, or scripts cite ADRs by number.
- `rg -n "TBD|___|pending close"` against this dev-log entry -- zero hits expected.

## Visual verification (operator-gated)

Verification flow expected to land after this entry commits:

1. Operator: `scripts\fetch-region.ps1 terril-waterschei` from a shell with internet access. Existing basemap + hillshade idempotency skips (sha matches); new `osm_overpass` step POSTs the query, expects ~few-KB of OSM XML back, writes `osm-overpass.osm` + provenance sidecar. Wall-clock ~2-5 s.
2. Operator: `cargo run --release --manifest-path tools\sarcom-kiosk-lab\Cargo.toml`. Footer reads `PMTiles · hillshade · overpass · OSM · zoom 14`; auto-fetched OSM lines paint below the hand-drawn lines, both visible above the hillshade.
3. Operator snips the kiosk-lab window manually. Programmatic capture path is NOT used (failed earlier in the day with wallpaper-artifact output; primary-screen capture is the documented escape hatch but the operator-snip is the durable contract).

## Cross-references

- Sister-spike contract owners: `spikes/field-deployment-package-shape-spike.md` `Cross-spike contracts produced or consumed` section, contracts (a) region descriptor min-fields and (b) provenance sidecar min-fields.
- Extension precedent: `dev-log/2026-05-16-lidar-overlay-implementation.md` and its post-close `## Hillshade load-vs-render gap (2026-05-16)` follow-up section.
- Defaults synchronization: Rust-side `default_overpass_features()` / `default_overpass_endpoint()` in `tools/sarcom-kiosk-lab/src/map/region.rs`; PowerShell-side `$DefaultOverpassFeatures` / `$DefaultOverpassEndpoint` in `scripts/fetch-region.ps1`. Same list, kept in sync by convention; the kiosk-lab's defaults are decorative only (it never reads `features` / `endpoint`) so the canonical source is the bake script.
- OSM XML parser unchanged: `tools/sarcom-kiosk-lab/src/map/osm_vector.rs:272-357`.
- Overpass API docs: https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL (consulted for the QL syntax `[out:xml][timeout:N][bbox:S,W,N,E];` header form and the `> ; out skel qt;` recursive-node-resolution idiom).
