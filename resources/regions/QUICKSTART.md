# Add a region — quickstart

Four commands from "I want to see this area" to "this area is on my kiosk-lab screen." Worked example: add **Bokrijk** (provincial park next to Terril Waterschei).

For schema reference, see `README.md` in this directory. This file is the doing-it path.

## 1. Get a bounding box

Open https://bboxfinder.com. Pan/zoom to the area, draw a box. The bottom of the page shows four numbers in `lon, lat, lon, lat` (W,S,E,N) order. For Bokrijk roughly:

```
5.380, 50.985, 5.450, 51.020
```

Keep it tight. `pmtiles extract` cost scales with area × zoom range.

## 2. Make the region directory

Copy `terril-waterschei` as your template (it's the working `protomaps_extract` shape), then drop the gitignored bake outputs from the copy:

```powershell
Copy-Item -Recurse resources\regions\terril-waterschei resources\regions\bokrijk
Remove-Item resources\regions\bokrijk\basemap.pmtiles, resources\regions\bokrijk\provenance.json -ErrorAction Ignore
```

## 3. Edit `resources\regions\bokrijk\region.toml`

Rewrite the comment block, set `name`, replace `[bounds]`, update `[view]` for sensible defaults, update `[source].upstream_url` + `source_extract_date` to a recent Protomaps daily (https://build.protomaps.com/YYYYMMDD.pmtiles — they keep ~1 week):

```toml
name        = "bokrijk"
description = "Domein Bokrijk, provincial park near Genk (BE)."

[bounds]
min_lon = 5.380
min_lat = 50.985
max_lon = 5.450
max_lat = 51.020

[view]
center_lon   = 5.415
center_lat   = 51.003
default_zoom = 14

[source]
kind                = "protomaps_extract"
upstream_url        = "https://build.protomaps.com/20260516.pmtiles"
tool                = "go-pmtiles@v1.30.2"
license             = "ODbL"
attribution         = "OpenStreetMap contributors, packaged by Protomaps"
source_extract_date = "2026-05-16"
```

## 4. Bake + run

```powershell
scripts\fetch-region.ps1 bokrijk
cargo run --release --manifest-path tools\sarcom-kiosk-lab\Cargo.toml
```

First time on a fresh machine: the bake script prompts before downloading the ~10 MB `go-pmtiles` CLI. Bake itself: ~8 seconds for a Belgian-municipality-sized bbox. Kiosk-lab auto-discovers the new region; use the header mode-switcher to pick it.

## 5. (Recommended for Flanders) Add a DHMV-II hillshade overlay

The Protomaps basemap shows generic roads / water / buildings but no terrain. For SAR operators a hillshade overlay derived from LIDAR -- relief showing slag heaps, valley contours, ridges -- gives immediate landmark recognition. For regions inside Flanders, use DHMV-II (Digitaal Hoogtemodel Vlaanderen II) as the source.

### Prerequisites

GDAL must be installed and on PATH. On Windows install one of:

- **OSGeo4W**: https://trac.osgeo.org/osgeo4w/ -- pick "Express Install" then "GDAL"
- **Conda Forge**: `conda install -c conda-forge gdal`

The bake script checks for `gdalwarp`, `gdaldem`, `gdal2tiles.py`, `gdal_translate` and errors with install instructions if any are missing. **No auto-install**: GDAL is operator-installed.

### Stage DHMV-II sheets in the local cache

DHMV-II is free but requires itsme/eID at the AGIV portal -- auto-download is impossible. You download manually once per sheet:

1. Open https://www.geopunt.be/ (or the AGIV download portal).
2. Find "Digitaal Hoogtemodel Vlaanderen II" -> "DSM" -> "raster" -> "1m".
3. Use the kaartblad map to pick the sheets covering your region. For Limburg / SARCom v1a regions: sheets k25 and k26 are the minimum for terril-waterschei.
4. Download the zip(s). Each is ~700 MB - 1 GB compressed.
5. Extract the `DHMVIIDSMRAS1m_kXX.tif` files into the cache directory:
   - Default: `%LOCALAPPDATA%\SARCom\lidar-cache\dhmv-ii\dsm\`
   - Override: `$env:SARCOM_LIDAR_CACHE = "D:\some\other\path"` (the cache root)
6. Capture the AGIV licence text into `<cache-root>\dhmv-ii\dsm\cache-manifest.json` under the `license_text` field (copy-paste from `Gebruik_DHMVIIDSMRAS1m.pdf` inside the zip). The bake script reads this into every `hillshade.provenance.json` for audit compliance. If absent, the script writes a "pending operator capture" placeholder.

### Reference in region.toml

Add an `[[overlays]]` entry (`terril-waterschei/region.toml` is the worked example):

```toml
[[overlays]]
kind   = "hillshade"
file   = "hillshade.pmtiles"
source = "dhmv_ii_dsm_1m"
```

### Bake

```powershell
scripts\fetch-region.ps1 <your-name>
```

The script clips the relevant DHMV sheets to your bbox, reprojects EPSG:31370 (Belgian Lambert 72) -> EPSG:3857 (Web Mercator), runs `gdaldem hillshade`, generates an XYZ tile pyramid via `gdal2tiles.py`, packs to PMTiles via `pmtiles convert`. Wall-clock: ~2-5 minutes for a Belgian-municipality-sized bbox; `gdal2tiles.py` is the slow step (~30-90s).

Output: `hillshade.pmtiles` (~5-15 MB) + `hillshade.provenance.json` next to it.

### What you'll see

The kiosk-lab loads the hillshade automatically -- any `[[overlays]]` entry of `kind = "hillshade"` whose file exists. It renders as a second walkers layer with 50% transparency above the Protomaps basemap. Terrain features become unmistakable: the Terril Waterschei slag heap shows as a distinct ~30-50 m mound rising above the surrounding flatland.

If the script says "DHMV-II sheets not staged in the cache", it prints which sheets it thinks you need and the cache directory path. Sheet lookup is approximate; verify against the AGIV portal map if the list looks off. Update the table at `$DhmvSheetWgs84Bboxes` in `scripts\fetch-region.ps1` if a sheet is missing or wrong.

## 6. Add an OSM overlay (auto-fetched or hand-drawn)

OSM overlays render through the `osm_vector.rs` primitive. Single `kind = "osm"` value in region.toml; the `source` field dispatches between two source variants (mirrors the hillshade `source = "..."` pattern). Both variants can coexist in one region -- declare two `[[overlays]]` blocks, declaration order is render order (later on top).

### 6a. Auto-fetched from Overpass (recommended for new regions)

Declare an `[[overlays]] kind = "osm" source = "overpass"` block and let `scripts\fetch-region.ps1` POST an Overpass QL query for your `[bounds]` bbox. The fetched OSM-XML lands at `<region>\osm-overpass.osm` (gitignored). No JOSM round-trip; refresh by re-running the bake script.

Minimum block:

```toml
[[overlays]]
kind   = "osm"
source = "overpass"
```

Both `features` and `endpoint` are optional. Defaults:

- **`features`**: a SARCom-relevant default declared once in `default_overpass_features()` at `tools\sarcom-kiosk-lab\src\map\region.rs` and mirrored in `$DefaultOverpassFeatures` at `scripts\fetch-region.ps1` -- `highway`, `waterway`, `natural=water`, `natural=wetland`, `landuse=reservoir`, `landuse=basin`, `landuse=brownfield`, `man_made=spoil_heap`. Override per region with a TOML array of bare tags (`"highway"`) and/or `key=value` selectors (`"natural=water"`):

  ```toml
  features = [
      "highway",
      "waterway",
      "natural=water",
      "landuse=brownfield",
      "man_made=spoil_heap",
  ]
  ```

- **`endpoint`**: the public Overpass instance `https://overpass-api.de/api/interpreter`. SARCom's deployment cadence (a handful of regions, occasional refresh) is well below the public instance's rate limits. For mass refresh, point `endpoint` at a self-hosted or partner Overpass.

`terril-waterschei` ships with an Overpass block as a worked example. The bake step:

```powershell
scripts\fetch-region.ps1 <your-name>
```

POSTs the query, writes `<region>\osm-overpass.osm` plus `<region>\osm-overpass.provenance.json` (carries the endpoint, the verbatim Overpass QL query, sha256, ODbL attribution). Wall-clock for a Belgian-municipality-sized bbox: ~2-5 seconds.

**Rate-limit behaviour.** HTTP 429 (rate-limited) and 504 (gateway-timeout) responses are caught explicitly: the script prints an operator-actionable message ("wait + retry" or "narrow features / split bbox / self-host endpoint") and exits non-zero. It does NOT auto-retry -- that exacerbates public-endpoint pressure.

**Failure is silent at render time.** If the Overpass fetch fails or the script hasn't been run yet, `osm-overpass.osm` is absent and the kiosk-lab logs a warning, renders without it. The basemap + hillshade + any other OSM overlays keep working.

### 6b. Hand-drawn .osm file (explicit override)

For regions where local knowledge beats auto-fetch (named structures Overpass doesn't tag, specific paths, hand-annotated landmarks), drop a `.osm` file in the region directory and add an `[[overlays]] kind = "osm" source = "file"` block:

```toml
[[overlays]]
kind   = "osm"
source = "file"
file   = "<region-name>.osm"   # path relative to resources/regions/<region-name>/
```

A `.osm` file is the JOSM / iD-editor save format -- ways (lines / polygons) tagged with `highway=*`, `waterway=*`, `natural=water`, etc. The kiosk-lab parses `highway` (major / minor), `waterway`, `natural=water|wetland`, `landuse=reservoir|basin` (see `tools\sarcom-kiosk-lab\src\map\osm_vector.rs` for the exact classifier). Unknown tags are ignored. If the file is missing the kiosk-lab logs a warning and renders without it -- no error.

`terril-waterschei` ships with one as a worked example: `resources\regions\terril-waterschei\terril-waterschei.osm`.

### Combining both source variants

Both blocks can coexist in one region.toml. Recommended ordering: `source = "overpass"` first, `source = "file"` second -- the bake fetches the broad auto-set, then the hand-drawn block paints on top with operator-specific detail. `terril-waterschei` ships with both blocks as the worked example.

**Z-order**: basemap -> hillshade overlay -> osm overlays (declaration order, later on top) -> markers -> region badge. Markers always render on top of everything.

## What you'll see, and why it might surprise you

- **The basemap shows neighbouring towns outside your bbox.** That's because `pmtiles extract` pulls whole parent tiles, and at low zoom (z0-z11) one parent tile covers tens of kilometres. Tighten the visible view with a higher `default_zoom`.
- **Your `default_zoom` may not be honoured.** If the kiosk-lab opens wider than you expected, that's either a `set_zoom` swallow in `tools\sarcom-kiosk-lab\src\map\pmtiles_map.rs` or you scrolled. Use walkers' zoom controls (mouse wheel) to recover.
- **The region badge is in the bottom-right corner** of the kiosk-lab map ("region: bokrijk") so you can confirm which region is rendering.

## Switching to a different source kind

- For a one-off public PMTiles (someone else's archive on a stable URL): use `[source].kind = "url_fixture"`. See `resources\regions\us-zipcodes-sample\region.toml` as the template.
- `planetiler_bake` and `pmtiles_convert` are stubbed in `scripts\fetch-region.ps1` and will error if you set them. Pick one of the two implemented kinds.
