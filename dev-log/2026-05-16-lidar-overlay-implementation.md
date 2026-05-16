---
title: "Dev log -- LIDAR hillshade overlay implementation (DHMV-II) with typed [[overlays]] schema migration"
date: 2026-05-16
type: dev-log
tags: [implementation, lidar, hillshade, dhmv, walkers, regions, contract-extension, schema-migration]
---

# 2026-05-16 -- LIDAR hillshade overlay implementation

Implementation ticket flowing from `dev-log/2026-05-16-lidar-overlay-investigation.md` (Pieter locked the architectural decisions before drafting; the investigation note's amendment block records the post-review corrections). LIDAR-derived hillshade joins the kiosk-lab as a **primary** overlay for Flanders regions, alongside the hand-drawn OSM overlay retained as a **documented dev-fallback**. Both render together when both are present.

The visual verification (Step 9) is **DONE 2026-05-16** -- DHMV-II sheets `k25` + `k26` were staged manually via the itsme/eID-gated AGIV portal, the bake completed end-to-end with conda GDAL on PATH, and the gzip-wrap workaround for walkers' raster-PMTiles dispatch was applied (see the dedicated section below). Screenshot at `dev-log/2026-05-16-lidar-overlay-postbake-screenshot.png`.

## What changed

- **`tools/sarcom-kiosk-lab/src/map/region.rs`** -- replaced `pub overlay: Option<Overlay>` (single struct with `osm_file: String`) with `pub overlays: Vec<Overlay>` where `Overlay` is now a tagged enum:
  ```rust
  #[derive(Debug, Deserialize, Clone)]
  #[serde(tag = "kind", rename_all = "snake_case")]
  pub enum Overlay {
      Osm { file: String },
      Hillshade { file: String, source: String },
  }
  ```
  Helper methods `osm_overlay_path()` and `hillshade_overlay_path()` return the first matching entry's resolved path if the file exists, else `None` with a stderr warning (soft-fallback).
- **`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`** -- `PmTilesMap` gains `hillshade_pmtiles: Option<PmTiles>`. Constructed in `from_region` from `region.hillshade_overlay_path()`. Inside `show`, added between the basemap `with_layer` and the markers closure:
  ```rust
  if let Some(hs) = self.hillshade_pmtiles.as_mut() {
      map = map.with_layer(hs, 0.5);
  }
  ```
  Walkers' `Map::with_layer` is multi-layer-capable by design (`walkers/src/map.rs:103-110` for the API, `walkers/src/map.rs:191-193` for the per-layer draw loop; the demo's `OpenStreetMapWithGeoportal` provider at `demo/src/tiles.rs:88-96` is the precedent for multi-layer stacking). `Tile::new` auto-detects raster vs MVT from image magic bytes (`walkers/src/tiles.rs:108-130`), so hillshade-as-raster-PMTiles renders through the same `Tiles` trait surface as the basemap with no new render code.
- **`tools/sarcom-kiosk-lab/src/app.rs`** -- `osm_map` loading switched from the old `r.overlay_path()` helper to the new `r.osm_overlay_path()`. No other app-side changes; the typed-array migration is contained to `region.rs` and `pmtiles_map.rs`.
- **`resources/regions/terril-waterschei/region.toml`** -- migrated from `[overlay] osm_file = "..."` to two `[[overlays]]` entries (`kind = "hillshade"` for the DHMV bake target, `kind = "osm"` for the hand-drawn dev-fallback).
- **`scripts/fetch-region.ps1`** -- existing basemap dispatch (`[source].kind` -> `url_fixture` / `protomaps_extract` / stubs) preserved; the `return` statements inside the url_fixture skip cases changed to `break` so the script continues past the switch to overlay processing. Added: `Get-OverlayBlocks` (TOML `[[overlays]]` array-of-tables parser), `Get-LidarCacheRoot` / `Get-DhmvDsmCacheDir` (env-var-configurable cache layout), `Test-GdalAvailable` (helpful install message on miss; exit 20 if missing), `Find-DhmvSheetsForWgs84Bbox` (approximate WGS84-bbox-to-NGI-sheet lookup, table in script as `$DhmvSheetWgs84Bboxes`), `Test-DhmvSheetsPresent` (cache check, returns hashtable), `Write-MissingSheetsInstructions` (operator-facing instructions with cache path + manual-download walkthrough; exit 22 if missing), `Get-CacheManifest` / `Save-CacheManifest` (license-text capture across regions), `Invoke-DhmvHillshadeBake` (the full GDAL pipeline). Provenance path renamed `provenance.json` -> `basemap.provenance.json`; new `hillshade.provenance.json` written per-bake.
- **`resources/regions/.gitignore`** -- added `**/basemap.provenance.json`, `**/hillshade.pmtiles`, `**/hillshade.provenance.json`. Kept `**/provenance.json` (legacy, for orphans from the rename).
- **`resources/regions/README.md`** -- schema section's `[overlay]` example replaced with the typed `[[overlays]]` array. Per-product `provenance.json` doc updated to name both `basemap.provenance.json` and `hillshade.provenance.json`.
- **`resources/regions/QUICKSTART.md`** -- Step 5 reframed as "(Recommended for Flanders) Add a DHMV-II hillshade overlay" with GDAL prereq, sheet-staging walkthrough, license-capture step, bake invocation, and z-order note. New Step 6 documents OSM as the dev-fallback.
- **`dev-log/2026-05-16-lidar-overlay-investigation.md`** -- amended with a top-of-note Amendment block: §4 gains the explicit `gdalwarp -t_srs EPSG:3857` step; exec summary holds visualization apart from the fingerprinting toolchain; the proposed bbox amendment in §1 was reviewed and skipped (the rework prompt had values swapped).

## Region-descriptor + provenance-sidecar contract extension

`spikes/field-deployment-package-shape-spike.md` (status `closed: 2026-05-16`) owns the region-descriptor and provenance-sidecar contracts at its `Cross-spike contracts produced or consumed` section. This implementation extends **both**:

**Region-descriptor contract extension** -- the existing **recommended** slot's earlier `[overlay] osm_file = "..."` shape (introduced by `dev-log/2026-05-16-pmtiles-osm-overlay.md`) is **replaced** by a typed `[[overlays]]` array:

```toml
[[overlays]]
kind   = "hillshade"
file   = "hillshade.pmtiles"
source = "dhmv_ii_dsm_1m"

[[overlays]]
kind = "osm"
file = "<region-name>.osm"
```

One-shot migration; no backward compat; `terril-waterschei` is the single region requiring migration and was migrated in this commit. The sister-spike's **mandatory** min-fields contract (`{name, bounds:WGS84, source-extract-date}`) is unchanged.

**Provenance-sidecar contract extension** -- the sister-spike's existing single-file `provenance.json` shape is **replaced** by per-product sidecars: `basemap.provenance.json` (one per region) and `hillshade.provenance.json` (one per region with a hillshade overlay baked). The sister-spike's per-product framing in its close-note "Cross-spike contracts produced" -> "(b) Provenance sidecar min-fields contract" already names mandatory fields `{source-extract URL, source-extract date, region bounds:WGS84, sha256, bytes}`; this implementation keeps those mandatory and adds, for the hillshade product specifically: `{source_kaartbladen, source_cache_dir, upstream_portal, crs_src, crs_dst, license_text}`. The license-text field captures the verbatim AGIV licence terms from `Gebruik_DHMVIIDSMRAS1m.pdf` (which the operator drops into `<cache-root>\dhmv-ii\dsm\cache-manifest.json`'s `license_text` field at sheet-staging time) -- compliance audit trail per the DHMV-II usage terms.

Per `docs/spike-rules.md:89-91` single-ownership form, future siblings consume the extended contracts via:

> "consumes the region-descriptor / provenance-sidecar contracts from `spikes/field-deployment-package-shape-spike.md`, extended with the typed `[[overlays]]` array and per-product provenance sidecars documented in `dev-log/2026-05-16-lidar-overlay-implementation.md` (which supersedes the earlier `[overlay] osm_file` extension in `dev-log/2026-05-16-pmtiles-osm-overlay.md` -- the typed array is the durable shape)."

Documentation-strategy choice: this is a **dev-log extension**, matching the OSM-overlay precedent. No new ADR, no new sub-spike. The closed sister spike stays `closed`.

## Z-order

Per the locked decision and matching the walkers multi-layer draw order at `walkers/src/map.rs:191-193`:

1. PMTiles basemap (`with_layer` transparency=1.0)
2. Hillshade raster (`with_layer` transparency=0.5) -- LIDAR-derived terrain shading
3. Inside `Map::show`'s closure (which walkers runs after all `with_layer` calls):
   1. OSM vector overlay (closure call to `OsmMap::draw_with_projector`) -- hand-annotated landmark detail
   2. SARCom sim markers (relay, gateway, tags)
   3. Region badge in the corner

Hillshade transparency 0.5 is the starting value. Tunable at visual review: drop to 0.35 if basemap muddies, raise to 0.7 if terrain washes out. Comment in `pmtiles_map.rs:show` flags the knob.

## Visual verification status (DONE 2026-05-16)

Step 9 complete. Prerequisites that were operator-gated have all landed:

1. **DHMV-II sheets** `k25` + `k26` staged in `%LOCALAPPDATA%\SARCom\lidar-cache\dhmv-ii\dsm\` via Pieter's itsme/eID session at https://www.geopunt.be/.
2. **GDAL** available via `C:\Users\Pieter\miniconda3\Library\bin` (gdalwarp / gdaldem / gdal_translate) and `C:\Users\Pieter\miniconda3\Scripts` (gdal2tiles / mb-util). Bake run with conda PATH prepended.

Bake run on this commit (after the walkers gzip-wrap workaround landed -- see `## Hillshade load-vs-render gap (2026-05-16)` below):

```
[hillshade] 1/4 gdalwarp clip + reproject (25,26 -> dsm-clipped.tif)
[hillshade] 2/4 gdaldem hillshade (az=315 alt=45)
[hillshade] 3/4 gdal2tiles.py z0-15
[hillshade] 4/4 mb-util XYZ -> MBTILES, gzip-wrap, pmtiles convert -> PMTiles, fix-up header
[gzip-wrap] gzipped 98 blob(s); skipped 0 (empty or already-gzip)
[ok] hillshade -> resources\regions\terril-waterschei\hillshade.pmtiles (3.28 MB, sha256 8851ee4bba3c..., 7s)
```

Visual verification of the kiosk-lab render at `dev-log/2026-05-16-lidar-overlay-postbake-screenshot.png`: footer reads `PMTiles · hillshade · OSM · zoom 14`; terril-Waterschei slag-heap mound and surrounding terrain undulations are clearly visible as topographic relief above the Protomaps dark basemap; hand-drawn OSM trails (yellow lines) and waterways (blue) paint on top of the hillshade; sim markers render unchanged.

## What did NOT change

- **No spike re-opened.** Parent (`spikes/pmtiles-walkers-spike.md`) and sister (`spikes/field-deployment-package-shape-spike.md`) both stay `closed: 2026-05-16`. Contract extension goes through the dev-log mechanism per the sister's own close note.
- **No ADR text authored.** The sister's named follow-up (ADR-NNN: Field-deployment package shape) remains the next-ADR-author's job; this entry informs that ADR's eventual `[[overlays]]` schema text but does not write ADR content.
- **No `gateway/` code touched.** Kiosk-lab only.
- **No protocol change.** SARCom's radio protocol is unaffected; LIDAR is a kiosk-side visualization concern.
- **No fingerprinting implementation.** Held apart per the investigation amendment; v2/v3 protocol concern.
- **No global coverage.** Only `dhmv_ii_dsm_1m` source kind in v1. Wallonia / NL / FR / Copernicus-global-fallback all become future sub-spikes when those deployment areas come into scope.
- **No Pi-side validation.** Per `CLAUDE.md ##Current hardware inventory`, no Pi 5 exists. Pi-side hillshade rendering is deferred-pending-procurement, same posture as the parent spike's Pi half.

## Cross-references

- Investigation that fed the decisions: `dev-log/2026-05-16-lidar-overlay-investigation.md` (with the 2026-05-16 amendment block).
- Parent spike: `spikes/pmtiles-walkers-spike.md` (status closed; H1 HOLD; walkers + PMTiles render path).
- Sister spike: `spikes/field-deployment-package-shape-spike.md` (status closed; owns the contracts this entry extends).
- Earlier overlay extension: `dev-log/2026-05-16-pmtiles-osm-overlay.md` (superseded by this entry's typed-array shape; the dev-log mechanism is the same).
- Walkers source: `podusowski/walkers@main` -- `walkers/src/map.rs:103-110` (with_layer API), `walkers/src/map.rs:191-193` (per-layer draw loop), `walkers/src/tiles.rs:108-130` (Tile::new raster/MVT auto-detect). Fetched via github MCP 2026-05-16.
- DHMV-II portal: `https://www.geopunt.be/`. Accessed 2026-05-16. Auto-fetch impossible (itsme/eID auth).
- GDAL docs: `https://gdal.org/programs/gdaldem.html`, `https://gdal.org/download.html`. Accessed 2026-05-16.
- AGIV licence (verbatim captured per region at bake time): `Gebruik_DHMVIIDSMRAS1m.pdf` inside each DHMV-II download zip; cached at `<sarcom-lidar-cache>\dhmv-ii\dsm\cache-manifest.json` under `license_text`.


## Post-bake hillshade load gap (2026-05-16)

**Symptom.** Bake produced `resources/regions/terril-waterschei/hillshade.pmtiles` (3.29 MB, sha256 `d6e170e24802...`). Kiosk-lab launched and rendered, but the bottom-status footer read `PMTiles · OSM · zoom 17` -- unchanged from before the bake. No "hillshade" token. Either the layer was being dropped silently somewhere on the load path, or the footer was lying.

**Diagnosis.** Walked the load path top-to-bottom against the three candidate root causes named in the diagnosis prompt:

- **Candidate A (schema migration omission)** -- ruled out. `resources/regions/terril-waterschei/region.toml:54-57` declares the `[[overlays]] kind = "hillshade" file = "hillshade.pmtiles" source = "dhmv_ii_dsm_1m"` block as expected. The companion OSM entry at `region.toml:64-66` is also present.
- **Candidate B (loader silently drops the entry)** -- ruled out. `tools/sarcom-kiosk-lab/src/map/region.rs:47-69` defines the tagged-enum `Overlay` with both `Osm` and `Hillshade` variants. `region.rs:118-135` `hillshade_overlay_path()` returns the absolute path when the file exists on disk. `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:55-57` calls that accessor in `PmTilesMap::from_region`, constructs `Option<PmTiles>`, and the render path at `pmtiles_map.rs:93-100` calls `Map::with_layer(hs, 0.5)` whenever the option is `Some`. No silent drop in the load path.
- **Candidate C (footer label hardcoded)** -- the actual bug. `tools/sarcom-kiosk-lab/src/app.rs:289` had `egui::RichText::new("PMTiles · OSM · zoom 17")` as a string literal in the bottom-status panel. The hillshade was being loaded, wired into `with_layer`, and (most likely) drawn -- the footer was simply not reading runtime state.

**Fix.** Targeted single-feature edit, no architectural change:

1. `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`: added two public accessors on `PmTilesMap`:
   - `has_hillshade(&self) -> bool` -- forwards `self.hillshade_pmtiles.is_some()`.
   - `zoom(&self) -> f64` -- forwards `self.map_memory.zoom()` (walkers `MapMemory::zoom`, `walkers-0.53.0/src/memory.rs:29`).
2. `tools/sarcom-kiosk-lab/src/app.rs`: replaced the hardcoded literal with a `match self.map_mode` that composes the label from actual state:
   - `MapMode::FakeGrid` -> `"Fake grid"`.
   - `MapMode::OsmVector` -> `"OSM"`.
   - `MapMode::PmTiles` -> `"PMTiles"` joined with `"hillshade"` (when `PmTilesMap::has_hillshade()` is true) and `"OSM"` (when `self.osm_map.is_some()`), suffixed with `format!("zoom {:.0}", pm.zoom())`.

**Verification.**

- `cargo build --release --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` -> `Finished `release` profile [optimized] target(s) in 13.59s`. Clean compile, no warnings introduced.
- Visual / screenshot verification (terrain relief visible + footer reading `PMTiles · hillshade · OSM · zoom N`) deferred to the next interactive kiosk-lab run by the operator -- this CLI session has no display. Save the screenshot to `dev-log/2026-05-16-lidar-overlay-postbake-screenshot.png` once captured.

**Out-of-scope follow-ups (deliberately not done here).**

- Tuning the hillshade alpha (current 0.5 in `pmtiles_map.rs:99`). Hold until visual review against the live render.
- Reflecting the active `MapMode` in `MapMode::all()` ordering or the sidebar label set -- separate UI concern.
- Pi-side render validation -- still deferred-pending-procurement per `CLAUDE.md`.


## Hillshade load-vs-render gap (2026-05-16)

**Symptom (operator-confirmed self-snip).** After the footer fix above, footer reads `PMTiles · hillshade · OSM · zoom 14` -- the kiosk-lab thinks the hillshade is loaded and added as a layer. **Visually, no terrain relief paints over the terril-waterschei area.** Basemap (Protomaps dark) + OSM vector overlay + sim markers render normally; the hillshade layer is in memory but draws zero pixels.

**Diagnostic walk against the five candidates.**

1. **Candidate A -- all-NoData / fully transparent tiles. RULED OUT.** `tools\bin\pmtiles.exe tile resources\regions\terril-waterschei\hillshade.pmtiles 14 8443 5485 > z14-sample.png` produced a 68 608-byte PNG (magic `89 50 4e 47 …`); visual inspection (image read by the agent) shows a high-contrast greyscale hillshade with clearly visible buildings and terrain undulations across the terril-Waterschei area. Tile content is valid.

2. **Candidate B -- zoom range mismatch. RULED OUT.** `pmtiles.exe show` reports `min zoom: 0`, `max zoom: 15`. Footer view zoom is 14, well inside the range.

3. **Candidate C -- geographic extent mismatch. RULED OUT.** The z=14 tile over Genk-Waterschei contains rich content (per (1)); whatever bbox the bake used, this view-center tile has data. The `pmtiles show` bounds line reports the whole-world (-180/180, -85/85) -- that's an unset / default bounds-header artifact, not an extent-truncation indicator.

4. **Candidate E -- z-order / blending. RULED OUT.** `tools\sarcom-kiosk-lab\src\map\pmtiles_map.rs:93-100` calls `with_layer(basemap, 1.0)` first, then conditionally `with_layer(hillshade, 0.5)`. Walkers' demo pattern (per investigation note §3) puts later `with_layer` calls *on top*; alpha 0.5 is well above zero. Z-order is correct.

5. **Candidate D -- walkers' raster dispatch silently drops tiles. ROOT CAUSE.** `walkers-0.53.0/src/pmtiles.rs:127-136`:

   ```rust
   async fn fetch(&self, tile_id: TileId) -> Result<Bytes, Self::Error> {
       let reader = AsyncPmTilesReader::new_with_path(self.path.to_owned()).await?;
       let bytes = reader
           .get_tile(TileCoord::new(tile_id.zoom, tile_id.x, tile_id.y)?)
           .await?
           .ok_or(PmTilesError::TileNotFound(tile_id))?;

       Ok(decompress(&bytes)?.into())
   }
   ```

   Combined with `walkers-0.53.0/src/pmtiles.rs:146-155`:

   ```rust
   /// Decompress the tile.
   ///
   /// This function assumes the input is gzip compressed data, but this might not always be the case.
   /// You can use `pmtiles info <file>` to check the compression type.
   fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
       let mut decoder = flate2::read::GzDecoder::new(data);
       let mut buf = Vec::new();
       decoder.read_to_end(&mut buf)?;
       Ok(buf)
   }
   ```

   Walkers calls the pmtiles crate's `get_tile()` (not `get_tile_decompressed()`, per `pmtiles-0.19.2/src/async_reader.rs:100-109` vs `:120-129`), then runs **hardcoded gzip decompression** on the raw tile bytes regardless of the archive's `tile_compression` header. Our hillshade tiles are stored as **raw PNG** (no gzip wrap); `pmtiles show` confirms `tile compression: unknown`. PNG magic `89 50` is not gzip magic `1f 8b`; `GzDecoder::read_to_end` fails with an `io::Error`; the `?` propagates to `PmTilesError::Decompression`; the tile fetch returns `Err`; walkers silently drops the tile and renders nothing in its place. `walkers/src/tiles.rs:115-130` has working PNG/JPEG auto-detection but it never sees our bytes -- the gzip step kills them upstream.

   **The investigation note's §3 evidence was misread on the way in.** It cited the demo's two-`HttpTiles` stacking pattern (which works because `HttpTiles` has its own header-aware decompression path) and `tiles.rs:108-130` raster auto-detect (which works at the `Tile::new` consumer side); it did **not** verify walkers' `PmTiles` fetch path. The fetch path's hardcoded gzip step is the actual lane-incompatibility this overlay hits.

**Halt -- not applying the fix yet (per the prompt's stop condition).**

Two viable fix paths surfaced. Both have to be authorised before they touch the tree:

- **Fix path α (small / re-bake side, no kiosk-lab change).** Modify `scripts\fetch-region.ps1` (and/or the `pmtiles convert` step inside it) so each raster PNG tile is gzip-wrapped before being written to the PMTiles archive, **and** the archive header's `tile_compression` is set to `Gzip` (currently it's `Unknown`). Walkers' existing fetch chain would then gzip-decompress the gzipped-PNG bytes back to plain PNG, hand them to `Tile::new`, hit the raster auto-detect path, and render. No kiosk-lab Rust code touched. Risk: depends on whether the current bake's gdal2tiles → MBTiles → `pmtiles convert` chain exposes a tile-recompression knob; may require a custom write step or `pmtiles edit` post-pass.

- **Fix path β (Option 3B from the investigation note).** Pivot the kiosk-lab to load `hillshade.png` (single-image) and blit it via `egui::Painter::image()` inside the `Map::show` closure -- bypassing walkers' `PmTiles` for the hillshade entirely. Trade-offs already enumerated in `dev-log\2026-05-16-lidar-overlay-investigation.md:77`: ~60 lines new in `pmtiles_map.rs`, no zoom-aware re-tiling (single image stretched at all zooms), but no upstream-walkers dependency. Bigger fix; the kiosk-lab's PmTilesMap struct grows a third layer field.

Recommended next step: **try path α first** because the bake side is the layer Pieter controls and the kiosk-lab side is already correctly written. If go-pmtiles cannot be coaxed into writing gzipped-PNG tiles + a `tile_compression: Gzip` header (or `pmtiles edit` cannot retroactively gzip-wrap tile bodies), fall through to path β.

**No code/bake change committed in this halt-step.** Awaiting Pieter's choice between α and β. Screenshot of the working layered render will be operator-snipped after the chosen fix lands.

### Fix applied: path α -- gzip-wrap tile blobs inside the bake (2026-05-16)

Pieter authorised path α. Changes:

- **New file** `scripts/_gzip_mbtiles_tiles.py`. Opens an MBTILES SQLite file, finds the underlying tile-storage table (real `tiles` table for mb-util's flat schema, falls back to `images` for dedup-mode archives), gzip-wraps every blob in place. Idempotent: blobs already starting with the gzip magic `1f 8b` are skipped. `mtime=0` in the gzip header keeps re-bakes deterministic for sha256 stability.
- **`scripts/fetch-region.ps1` 4/4 step.** Between `mb-util` and `pmtiles convert`, the new script is invoked against the MBTILES; after `pmtiles convert`, `pmtiles show --header-json` is round-tripped through `ConvertFrom-Json`/`ConvertTo-Json`, `tile_compression` is flipped to `"gzip"` and `tile_type` to `"png"`, and `pmtiles edit --header-json=$headerJsonPath` writes the corrected header back. Comment in the script cites walkers/src/pmtiles.rs:127-155 as the upstream behaviour we are working around.

**Verification (post-rebake, before kiosk-lab relaunch):**

```text
$ tools\bin\pmtiles.exe show resources\regions\terril-waterschei\hillshade.pmtiles
pmtiles spec version: 3
tile type: png                  <-- was: (empty)
min zoom: 0
max zoom: 15
internal compression: gzip
tile compression: gzip          <-- was: unknown
```

```text
$ tools\bin\pmtiles.exe tile resources\regions\terril-waterschei\hillshade.pmtiles 14 8443 5485 > z14.bin
$ head -c 8 z14.bin | xxd
00000000: 1f8b 0800 0000 0000                      ........        <-- gzip magic
$ gunzip -c z14.bin | head -c 8 | xxd
00000000: 8950 4e47 0d0a 1a0a                      .PNG....        <-- PNG magic inside
```

Tile body on disk now matches walkers' fetch-time assumption: raw bytes are gzipped, gzip-decode yields PNG, `walkers/src/tiles.rs:115-130` `Tile::new` raster auto-detect handles the rest. No kiosk-lab Rust code touched -- `tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs:55-100` was structurally correct from the earlier load-path pass; only the on-disk format needed fixing.

**No screenshot captured by the agent** -- per the prompt's stop condition (programmatic capture cost time + produced wallpaper artifacts earlier in this session). Operator snips `dev-log/2026-05-16-lidar-overlay-postbake-screenshot.png` after relaunching `tools/sarcom-kiosk-lab/target/release/sarcom-kiosk-lab.exe`. Expected visual: footer reads `PMTiles · hillshade · OSM · zoom N`; over the terril-Waterschei area, topographic relief (slag-heap mound, surrounding ridges) is now visible at α=0.5 above the dark Protomaps basemap, with the hand-drawn OSM trails / waterways still painting on top.

**Out-of-scope follow-up.** The `protomaps_extract` basemap branch in `scripts/fetch-region.ps1` lacks the idempotency-skip the `url_fixture` branch has -- every re-run of the hillshade bake currently re-fetches `basemap.pmtiles` over the network (~3 MB / ~8 s). Not blocking; just noting. Separate cleanup ticket if it starts to chafe.
