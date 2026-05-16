---
title: "Dev log -- collapse osm + osm_overpass into one `osm` kind with `source` dispatch (corrective refactor)"
date: 2026-05-16
type: dev-log
tags: [implementation, regions, schema, corrective-refactor, audit-trail, subtyping-failure-mode]
---

# 2026-05-16 -- OSM overlay collapse: subtypes -> source dispatch

Corrective refactor of `dev-log/2026-05-16-osm-overpass-overlay-implementation.md`. That entry stays as the historical record of the design mistake; this entry documents the correction and the recurring failure mode behind it.

## Context

Earlier in the same day, an implementation ticket added `osm_overpass` as a third enum variant alongside `osm` and `hillshade` in the `[[overlays]]` typed-array schema (the schema itself extended into `[[overlays]]` typed-array form by `dev-log/2026-05-16-lidar-overlay-implementation.md`). Pieter reviewed the implementation before running the bake or relaunching the kiosk-lab, and called the shape wrong: both `osm` and `osm_overpass` produce the same renderable artefact (OSM XML, parsed by `tools/sarcom-kiosk-lab/src/map/osm_vector.rs:272-357`) from different sources -- they are not peer kinds. The right shape mirrors hillshade: one `kind = "osm"` value with a `source` field that dispatches the bake-time and load-time behaviour.

## The design mistake

The just-landed schema had three peer variants:

- `kind = "osm"` -- hand-drawn OSM XML, `file = "..."` committed to repo.
- `kind = "hillshade"` -- LIDAR raster PMTiles, `file = "..." source = "..."` dispatched on `source`.
- `kind = "osm_overpass"` -- auto-fetched OSM XML, `features = [...] endpoint = "..."`.

Hillshade already established the correct **kind + source-dispatch** pattern for "same renderable artefact, different source provenance". OSM should look identical:

- `kind = "osm" source = "file" file = "..."`
- `kind = "osm" source = "overpass" features = [...] endpoint = "..."`

Reading hillshade right would have given the answer. The mistake was reaching for a new top-level enum variant when the shape already in the file said "use a nested dispatch field."

## Recurring failure mode -- "subtyping fetish"

This is the second instance Pieter has caught the reviewer (Claude in Cowork) reaching for **new enum variants or wire-level subtypes** when the right answer is **a configuration attribute on an existing type**. The prior instance was a near-miss on the LoRa protocol: `drone_relay` and `fixed_relay` were nearly added as wire-level subtypes of the relay role, instead of being modelled as one `relay` role with a presentation/configuration attribute resolved gateway-side. The accepted shape is captured in `decisions/ADR-013-multi-hop-flood-via-packet-id.md` -- no wire-level role enum, role-as-presentation is `nodes.toml` config keyed on `node_id`. `CLAUDE.md` quotes this verbatim under the don't-re-open-these block: *"No wire-level role enum. Node presentation (hiker / relay / drone) is gateway config in `nodes.toml`, keyed on `node_id`."*

The OSM case rhymes structurally:

| Case | Wrong shape (variant proliferation) | Right shape (one type + attribute) |
|---|---|---|
| LoRa POSITION packet | `role = drone_relay / fixed_relay` byte on the wire | One `relay` role, `nodes.toml` carries the presentation tag gateway-side |
| `[[overlays]]` array | `kind = "osm_overpass"` peer of `kind = "osm"` | `kind = "osm"` + `source = "overpass" \| "file"` |

In both cases the "subtype" doesn't change how the receiver renders / decodes; only where the artefact came from. That's a configuration attribute, not a new type. Recording the pattern here so the audit trail captures it as a recurring vigilance item, not a one-off slip. Watch for: a new variant whose only difference from an existing variant is *where the data was sourced*. That's a `source = ...` field, not a new top-level kind.

## What collapsed

Hard removal of the `osm_overpass` string from the codebase and schema. Nothing in the world depended on the just-landed shape -- the bake had not been re-run and the kiosk-lab had not been re-launched against the new dispatch -- so no compat shim or deprecation alias was retained.

- **`tools/sarcom-kiosk-lab/src/map/region.rs`** -- `Overlay::OsmOverpass { features, endpoint }` variant removed. `Overlay::Osm { file: String }` replaced with `Overlay::Osm(OsmSource)` where the new nested tagged enum (`#[serde(tag = "source", rename_all = "snake_case")]`) carries the two per-source field sets:
  ```rust
  pub enum OsmSource {
      File { file: String },
      Overpass {
          #[serde(default = "default_overpass_features")] features: Vec<String>,
          #[serde(default = "default_overpass_endpoint")] endpoint: String,
      },
  }
  ```
  `default_overpass_features()` and `default_overpass_endpoint()` are still used (via the inner enum's `#[serde(default = ...)]` attributes). `osm_overlay_path()` and `osm_overpass_path()` accessors collapsed into one `osm_overlay_paths() -> Vec<PathBuf>` that iterates both source variants in declaration order and resolves each to its on-disk path (with the per-variant soft-fallback when the file is absent).
- **`tools/sarcom-kiosk-lab/src/app.rs`** -- `osm_map: Option<OsmMap>` and `osm_map_overpass: Option<OsmMap>` collapsed into one `osm_maps: Vec<OsmMap>`. Loader at startup iterates `region.osm_overlay_paths()`, loads each, pushes the successful ones (failures log + continue). Footer label emits a single `OSM` token whenever `!osm_maps.is_empty()`, regardless of source variant or count.
- **`tools/sarcom-kiosk-lab/src/map/pmtiles_map.rs`** -- `show()` signature gives up its two separate `Option<&OsmMap>` parameters in favour of one `osm_overlays: &[&OsmMap]` slice. The closure replaces the two `if let Some(osm) = ...` blocks with one `for osm in osm_overlays { osm.draw_with_projector(...) }` loop. Declaration order = render order (later on top).
- **`tools/sarcom-kiosk-lab/src/map/mod.rs`** -- the PMTiles dispatch builds a `Vec<&OsmMap>` view of `self.osm_maps` and passes it to `pm.show(...)`. Standalone `MapMode::OsmVector` updated similarly: `self.osm_maps.is_empty()` gates the early-return, `self.osm_maps.first().unwrap()` is the letterbox source, and the draw arm loops every entry.
- **`scripts/fetch-region.ps1`** -- dispatch loop loses the top-level `'osm_overpass'` case. The `'osm'` case now sub-dispatches on `$overlay.source`: `'file'` warns + skips if the declared file is missing (kiosk-lab side handles render-time fallback); `'overpass'` calls the existing `Invoke-OsmOverpassFetch` with the block's `features` / `endpoint` (defaults handled inside the function). `$DefaultOverpassFeatures` and `$DefaultOverpassEndpoint` constants and `Invoke-OsmOverpassFetch` itself unchanged.
- **`resources/regions/terril-waterschei/region.toml`** -- the two `[[overlays]]` blocks (one `kind = "osm_overpass"`, one `kind = "osm"`) replaced with two blocks under the collapsed shape: `kind = "osm" source = "overpass"` (Overpass features list inlined) and `kind = "osm" source = "file" file = "terril-waterschei.osm"`. Declaration order preserves the prior z-order intent (overpass first, file second; hand-drawn paints on top).
- **`resources/regions/README.md`** -- schema section's OSM block collapsed: one `kind = "osm"` worked example with the two `source = ...` sub-variants documented side-by-side, mirroring how hillshade is already documented. Per-product provenance section's `osm-overpass.provenance.json` paragraph updated to note `source = "file"` produces no sidecar.
- **`resources/regions/QUICKSTART.md`** -- Steps 6 and 7 collapsed into one Step 6 ("Add an OSM overlay (auto-fetched or hand-drawn)") with two sub-sections (6a Overpass, 6b hand-drawn). New paragraph at the bottom documents combining both variants in one region.toml.

## What stays

- **The per-product file convention.** `<region>/osm-overpass.osm` + `<region>/osm-overpass.provenance.json` are still the on-disk names for the overpass-fetched product. The `.gitignore` patterns (`**/osm-overpass.osm`, `**/osm-overpass.provenance.json`) added in the prior pass keep working unchanged.
- **`Invoke-OsmOverpassFetch` behaviour.** Same Overpass QL query shape (`[out:xml][timeout:25][bbox:S,W,N,E];` header, `way[<selector>]` union body, `out body; >; out skel qt;` footer), same HTTP 429 / 504 / other error handling (operator-actionable message, exit non-zero, no auto-retry), same provenance sidecar fields.
- **Default constants.** `default_overpass_features()` / `default_overpass_endpoint()` on the Rust side and `$DefaultOverpassFeatures` / `$DefaultOverpassEndpoint` on the PowerShell side keep their values (the canonical SARCom-relevant feature list and the public Overpass endpoint URL); kept in sync by convention.
- **The sister sub-spike's typed `[[overlays]]` array contract.** Unchanged. The contract at `spikes/field-deployment-package-shape-spike.md` `Cross-spike contracts produced or consumed` section commits to *typed-array shape* + *per-product provenance sidecars*; neither the array shape nor the sidecar shape changed. Only the OSM-internal dispatch shape moved from "peer variant of osm" to "nested source dispatch under osm".

## Verification

- `cargo check --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` -- clean.
- `cargo build --release --manifest-path tools/sarcom-kiosk-lab/Cargo.toml` -- pending; runs as part of the operator-visual-verification step.
- `rg -n "osm_overpass" tools/ scripts/ resources/regions/` -- only allowed hits are file naming conventions (`osm-overpass.osm`, `osm-overpass.provenance.json`) which remain the on-disk names; zero hits as a `kind` value or Rust identifier.
- `rg -n "OsmOverpass"` -- zero hits (the Rust variant was removed).
- `rg -n "osm_map_overpass"` -- zero hits (the field was renamed away).
- `rg -n "ADR-005|ADR-007|ADR-008|ADR-013"` against touched files -- citation-only (ADR-013 cited in this dev-log entry for the role-presentation precedent; ADR-007 / ADR-008 pre-existing in `app.rs:245` SOS-strip comment, unchanged). Zero contestation.
- `rg -n "TBD|___|pending close"` against this dev-log -- zero hits.

## Operator-gated next steps

1. `scripts\fetch-region.ps1 terril-waterschei` from a shell with internet access. Basemap + hillshade idempotency should still skip; the new `kind = "osm" source = "overpass"` block exercises the collapsed dispatch and fetches the Overpass features.
2. Relaunch kiosk-lab. Footer expected: `PMTiles · hillshade · OSM · zoom 14`. Single `OSM` token regardless of how many OSM overlays are stacked. Both source variants render on the map (auto-fetched below, hand-drawn on top per region.toml declaration order).
3. Manual window snip + commit if Pieter wants the verification artefact.

## Cross-references

- Prior implementation (the design mistake this corrects): `dev-log/2026-05-16-osm-overpass-overlay-implementation.md`.
- Pattern this rework conforms to: `dev-log/2026-05-16-lidar-overlay-implementation.md`'s `Hillshade { file, source }` enum entry and its bake-script `'hillshade' { switch ($source) ... }` dispatch.
- Recurring failure-mode precedent: `decisions/ADR-013-multi-hop-flood-via-packet-id.md` -- "No wire-level role enum. Node presentation (hiker / relay / drone) is gateway config in `nodes.toml`, keyed on `node_id`." Also captured in `CLAUDE.md`'s don't-re-open-these block.
- Sister-spike contract this collapse preserves: `spikes/field-deployment-package-shape-spike.md` `Cross-spike contracts produced or consumed` section, contracts (a) region descriptor min-fields and (b) provenance sidecar min-fields. Both unchanged; only the OSM-internal dispatch shape moved.
