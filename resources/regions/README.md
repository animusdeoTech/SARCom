---
title: "Field-deployment region directory"
status: living
type: meta
tags: [regions, pmtiles, bake, deployment-package]
---

# `resources/regions/`

One subdirectory per **named geographic area** the SARCOM gateway needs a basemap for. Format and contents are inherited from the pending sister sub-spike `spikes/field-deployment-package-shape-spike.md` (not yet open) and are **provisional / recommended-not-frozen** until that spike closes.

> **Want to add a region right now?** Read `QUICKSTART.md` (in this directory) — one screen, one worked example, four commands. This README is the spec/schema reference; the quickstart is the doing-it path.

## The "name an area, toolchain has what it needs" contract

```
resources/regions/<region-name>/
    region.toml          # machine-readable descriptor (committed)
    provenance.json      # write-on-fetch metadata (gitignored)
    basemap.pmtiles      # binary tile archive       (gitignored)
    [optional sidecars]  # DEM clip, 3D massing, etc. — sister-spike scope
```

To go from "I need tiles for this area" to "the kiosk-lab can render it":

```powershell
scripts\fetch-region.ps1 <region-name>
```

That command reads `region.toml`, consults `[source]`, materialises `basemap.pmtiles` + `provenance.json` next to it. The kiosk-lab loads `resources/regions/<region-name>/basemap.pmtiles` directly. No manual fidgety steps; rerunning is idempotent (skips fetch if the existing file's sha256 matches `region.toml`).

## Adding a region (the workflow)

The schema below documents *what* a `region.toml` looks like. The steps here document *how* to produce one for a new geographic area.

1. **Pick a bounding box in WGS84 decimal degrees.** Tools that give you four numbers (min_lon, min_lat, max_lon, max_lat): [bboxfinder.com](https://bboxfinder.com), OpenStreetMap's *Share → Manually select a different area*, [geojson.io](https://geojson.io), or any GIS tool. Keep the bbox tight — `pmtiles extract` cost scales with area × zoom range.

2. **Pick a kebab-case name.** Lowercase, hyphens. The name is both the directory name under `resources/regions/<name>/` and the `name` field inside `region.toml`.

3. **Pick a `[source].kind`.** The default for a SARCom-area region is `protomaps_extract` (clips a bbox out of a Protomaps daily planet build via HTTP range requests — no full-planet download). For a one-off public PMTiles, `url_fixture` (downloads a stable URL). `planetiler_bake` and `pmtiles_convert` are stubs — they error if set.

4. **For `protomaps_extract`: pick a recent daily build.** `https://build.protomaps.com/YYYYMMDD.pmtiles` — Protomaps keeps ~1 week of dailies. Set both `upstream_url` and `source_extract_date` to the same date.

5. **Copy an existing region as a template.** For `protomaps_extract`, copy `terril-waterschei/`; for `url_fixture`, copy `us-zipcodes-sample/`.
   ```powershell
   Copy-Item -Recurse resources\regions\terril-waterschei resources\regions\<your-name>
   # drop the gitignored bake outputs from the copy:
   Remove-Item resources\regions\<your-name>\basemap.pmtiles  -ErrorAction Ignore
   Remove-Item resources\regions\<your-name>\provenance.json  -ErrorAction Ignore
   ```
   Then edit `resources\regions\<your-name>\region.toml`: rewrite the leading comment block, set `name`, `[bounds]`, `[view]` (optional defaults), and the `[source]` fields per the schema below.

6. **Bake.**
   ```powershell
   scripts\fetch-region.ps1 <your-name>
   ```
   First time on any machine: the script prompts before downloading the ~10 MB `go-pmtiles` CLI to `tools\bin\`. After that, a Belgian-municipality-sized bbox at z0-z15 takes ~8 seconds and ~3 MB of wire transfer. Rerunning is idempotent — skips fetch if the existing `basemap.pmtiles` sha256 matches.

7. **Run the kiosk-lab.**
   ```powershell
   cargo run --release --manifest-path tools\sarcom-kiosk-lab\Cargo.toml
   ```
   It auto-discovers any directory under `resources/regions/` that contains both `region.toml` and `basemap.pmtiles`. `terril-waterschei` is the preferred default; otherwise the first region by name. Use the header mode-switcher to flip between regions or back to the legacy OSM-vector renderer.

**What lands on disk:** `region.toml` (committed), `basemap.pmtiles` (gitignored, ~MB), `provenance.json` (gitignored, audit trail).

## `region.toml` schema (provisional)

Minimum fields per the pmtiles-walkers spike's extensibility lens (Axes 2 + 4 amendments at `spikes/pmtiles-walkers-spike.md:142-143`): name, bounds in WGS84, source-extract dates.

```toml
name        = "kebab-case-name"           # also the directory name
description = "free-form English summary"

# Geographic bounds — WGS84 lon/lat. Required.
[bounds]
min_lon = ...
min_lat = ...
max_lon = ...
max_lat = ...

# Default view at first load. Optional; sister sub-spike may move or formalise.
[view]
center_lon  = ...
center_lat  = ...
default_zoom = N            # integer

# How basemap.pmtiles is produced. Exactly one [source] block.
[source]
kind = "url_fixture"        # one of: url_fixture | protomaps_extract | planetiler_bake | pmtiles_convert
# Fields below depend on kind. See `scripts/fetch-region.ps1` for the matrix.

# When kind = "url_fixture":
url           = "https://..."           # stable public PMTiles URL
expected_sha256 = "..."                 # optional integrity pin; verified at fetch
license       = "..."                   # data licence (e.g. ODbL, public-domain, CC-BY)
attribution   = "..."                   # required by most OSM-derived data
source_extract_date = "YYYY-MM-DD"      # when the upstream extract was generated

# When kind = "protomaps_extract":
# upstream_url   = "https://..."        # planet-scale PMTiles to clip from
# tool           = "pmtiles@1.X.Y"      # go-pmtiles CLI required
# source_extract_date = "YYYY-MM-DD"

# When kind = "planetiler_bake":
# osm_extract_url = "https://download.geofabrik.de/..."
# tool            = "planetiler@0.X.Y"  # JDK 21+ required
# profile         = "openmaptiles"      # or custom
# source_extract_date = "YYYY-MM-DD"

# Overlays rendered on top of the PMTiles basemap. Typed array per the
# LIDAR-overlay rework documented in
# dev-log/2026-05-16-lidar-overlay-implementation.md (migrated from the
# earlier single `[overlay] osm_file = "..."` shape; one-shot
# migration, no backward compat). Z-order: basemap -> hillshade (if
# present) -> osm overlays (declaration order, later on top) -> markers
# -> badge. Each entry is optional; missing files fall back gracefully
# with a warning.

# Hillshade overlay -- LIDAR-derived terrain shading. The `source` field
# dispatches the bake recipe; today only `dhmv_ii_dsm_1m` (DHMV-II 1 m
# DSM, Flanders) is implemented. See "Step 5" in QUICKSTART.md for the
# bake workflow.
[[overlays]]
kind   = "hillshade"
file   = "hillshade.pmtiles"             # gitignored; produced by bake
source = "dhmv_ii_dsm_1m"                # one of: dhmv_ii_dsm_1m (Flanders)

# OSM overlay. Single `kind = "osm"` value; the `source` field
# dispatches between hand-drawn and auto-fetched (same pattern as
# hillshade above). Both source variants render through the same
# osm_vector.rs primitive; multiple `[[overlays]] kind = "osm"` blocks
# can coexist in one region and paint in declaration order. See "Step 6"
# in QUICKSTART.md for the workflow.
#
# source = "overpass": auto-fetched from the public Overpass API at bake
#   time. Fetched file lands at <region>/osm-overpass.osm (gitignored).
#   Optional `features = [...]` (default in default_overpass_features at
#   tools/sarcom-kiosk-lab/src/map/region.rs) and `endpoint = "..."`
#   (default https://overpass-api.de/api/interpreter).
[[overlays]]
kind   = "osm"
source = "overpass"
# features = [ "highway", "waterway", "natural=water", ... ]   # optional, sensible default
# endpoint = "https://overpass-api.de/api/interpreter"          # optional, public default

# source = "file": hand-drawn OSM-XML committed to the repo alongside
#   region.toml. Required `file = "..."` path relative to the region dir.
#   Useful when local knowledge beats auto-fetch (named landmarks,
#   hand-tagged trail attributes, building footprints Overpass doesn't
#   carry).
[[overlays]]
kind   = "osm"
source = "file"
file   = "<region-name>.osm"             # path relative to the region directory
```

The `kind` enum is **not frozen**. The sister sub-spike `spikes/field-deployment-package-shape-spike.md` may rename, split, or otherwise restructure these. For now: pragmatic, append-only.

## Per-product provenance sidecars (write-on-fetch)

Generated by `scripts/fetch-region.ps1`. **One sidecar per baked artefact**, per the contract extension at `dev-log/2026-05-16-lidar-overlay-implementation.md` (extends the sister-spike's single-file provenance shape):

- `basemap.provenance.json` -- captured at basemap fetch / bake time. Fields: region, fetched_at_utc, source_kind, source_url or source_upstream_url, source_extract_date, bbox_wgs84, sha256, bytes, bake_seconds, tool, recipe, license, attribution.
- `hillshade.provenance.json` -- captured at LIDAR hillshade bake time. Additional fields beyond basemap: source_kaartbladen (NGI sheet list), source_cache_dir, crs_src ("EPSG:31370" for DHMV-II), crs_dst ("EPSG:3857"), license_text (verbatim from the AGIV license PDF, captured in the operator's cache-manifest.json at sheet-staging time).
- `osm-overpass.provenance.json` -- captured at Overpass-fetch time for any `[[overlays]] kind = "osm" source = "overpass"` block. Additional fields beyond basemap: endpoint (the Overpass URL used), bbox_overpass (S,W,N,E -- Overpass-native ordering, distinct from WGS84 `min_lon,min_lat,max_lon,max_lat`), features (the verbatim selector list as fetched), query (the literal Overpass QL POST body), fetch_seconds. License is `ODbL` and attribution is `OpenStreetMap contributors`. The `source = "file"` variant produces no sidecar -- the hand-drawn `.osm` is its own audit trail.

Example `basemap.provenance.json` for the `url_fixture` source kind:

```json
{
  "region": "<name>",
  "fetched_at_utc": "2026-05-16T13:42:11Z",
  "source_kind": "url_fixture",
  "source_url": "https://...",
  "source_extract_date": "YYYY-MM-DD",
  "sha256": "...",
  "bytes": 20926494,
  "tool": "scripts/fetch-region.ps1"
}
```

This is the Axis 4 amendment evidence: "source-extract URL, source-extract date, region bounds (WGS84)" -- bounds are in `region.toml`, URL and date land in the per-product sidecar.

## Reproducibility

Re-running `scripts\fetch-region.ps1 <region-name>` is idempotent. If `basemap.pmtiles` exists with a matching sha256 (against `expected_sha256` in region.toml, or against the last-recorded `sha256` in provenance.json), the script is a no-op. If sha256 has drifted, the script writes a `provenance-mismatch.txt` and exits non-zero; the human decides whether to re-pin or re-fetch.

For `url_fixture`, reproducibility is bounded by the upstream host's uptime — that's the Phase 1 shortcut. For `protomaps_extract` and `planetiler_bake`, reproducibility is bounded by the OSM/Protomaps daily-build still being available.

## Why provisional

`spikes/pmtiles-walkers-spike.md`'s extensibility lens (Axes 1-6, classifications at lines 211-257) treats the deployment-package shape as a sister-sub-spike decision (`spikes/field-deployment-package-shape-spike.md`). This directory's conventions are the minimum shape that satisfies the lens-derived pass criteria at `spikes/pmtiles-walkers-spike.md:141-144` without freezing schema, container format, or aux-layer surface. Expect the sister sub-spike to refine.
