//! `region.toml` reader for the kiosk-lab.
//!
//! Implements the consumer half of the convention documented in
//! `resources/regions/README.md`. Reads the minimum fields the kiosk-lab
//! needs (name, bounds in WGS84, optional view center+zoom). The `[source]`
//! block is producer-side only; this module ignores it.
//!
//! Format choice (TOML) is **recommended-not-frozen** per the deferral to
//! `spikes/field-deployment-package-shape-spike.md`.

use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    pub name: String,
    /// Free-form description from region.toml. Parsed for round-trip
    /// completeness; not used by the kiosk-lab today (sister-sub-spike scope).
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
    pub bounds: Bounds,
    #[serde(default)]
    pub view: View,
    /// Typed array of overlays rendered on top of the PMTiles basemap.
    /// Extends the sister-spike's `Region descriptor min-fields contract`
    /// with one recommended `[[overlays]]` array; spec at
    /// `resources/regions/README.md` schema section. Each entry is a tagged
    /// enum: `kind = "osm"` for hand-drawn OSM XML, `kind = "hillshade"`
    /// for a baked raster PMTiles archive produced from LIDAR.
    ///
    /// Migrated from the earlier single `[overlay]` block (see
    /// `dev-log/2026-05-16-lidar-overlay-implementation.md`); the array
    /// shape is backward-incompatible with the old single-block shape on
    /// purpose -- one-shot migration of the single existing region.
    #[serde(default)]
    pub overlays: Vec<Overlay>,
    /// Filled in by the loader; absolute path to the region directory.
    #[serde(skip)]
    pub dir: PathBuf,
}

/// Tagged-enum overlay entry per `[[overlays]]` block in `region.toml`.
/// The TOML form is `kind = "osm" | "hillshade"` plus per-kind fields.
/// `kind = "osm"` dispatches further on a nested `source` field
/// (`"file"` for hand-drawn committed XML, `"overpass"` for auto-fetched
/// Overpass output), mirroring the hillshade `source = "..."` pattern.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Overlay {
    /// OSM XML overlay rendered through `osm_vector.rs`. The nested
    /// `OsmSource` variant carries the source-specific fields (path for
    /// hand-drawn, fetch parameters for Overpass). Both source variants
    /// render through the same `OsmMap::draw_with_projector` primitive;
    /// the kiosk-lab does not distinguish them at render time.
    Osm(OsmSource),
    /// LIDAR-derived hillshade raster, packaged as a raster PMTiles archive.
    /// Rendered as a second `walkers::PmTiles` layer via `with_layer` with
    /// transparency below 1.0. Source field names the bake-recipe input
    /// kind (e.g. `"dhmv_ii_dsm_1m"`); the producer half is
    /// `scripts/fetch-region.ps1`.
    Hillshade {
        /// Path relative to the region directory.
        file: String,
        /// Bake-recipe input kind. Documented in
        /// `resources/regions/README.md`; the kiosk-lab itself does not
        /// dispatch on this field (it's producer-side metadata kept for
        /// the audit trail).
        #[allow(dead_code)]
        source: String,
    },
}

/// Nested tagged-enum dispatch for `[[overlays]] kind = "osm"` blocks.
/// The TOML form keeps `source = "file" | "overpass"` plus per-source
/// fields at the same block level as `kind` (serde flattens the inner
/// enum's tag onto the outer level for newtype variants).
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum OsmSource {
    /// Hand-drawn OSM XML committed to the repo. Explicit-override path:
    /// painted ABOVE `Overpass` blocks in declaration order so operator-
    /// tagged detail wins the z-fight where both cover the same area.
    File {
        /// Path relative to the region directory.
        file: String,
    },
    /// Auto-fetched OSM features for the region's bbox, sourced from the
    /// public Overpass API at bake time. Recommended default for new
    /// regions; the fetched file is written by `scripts/fetch-region.ps1`
    /// to the conventional `<region>/osm-overpass.osm` path (gitignored).
    /// Both fields default to sensible values, so the minimum region.toml
    /// entry is `[[overlays]] kind = "osm" source = "overpass"`.
    Overpass {
        /// OSM tag selectors to fetch (either bare tag like `"highway"`
        /// or `key=value` like `"natural=water"`). Producer-side; the
        /// kiosk-lab never reads this field.
        #[serde(default = "default_overpass_features")]
        #[allow(dead_code)]
        features: Vec<String>,
        /// Overpass API endpoint URL. Producer-side; the kiosk-lab never
        /// reads this field.
        #[serde(default = "default_overpass_endpoint")]
        #[allow(dead_code)]
        endpoint: String,
    },
}

/// Default OSM feature selectors for an `osm` overlay with
/// `source = "overpass"`. Mirrors the tag classifier in `osm_vector.rs`
/// (`highway`, `waterway`, `natural=water`, etc.) so the auto-fetched
/// features render with the same colour scheme as hand-drawn ones.
/// Producer-side default reflected in `scripts/fetch-region.ps1`'s
/// `$DefaultOverpassFeatures` constant; kept in sync by convention.
fn default_overpass_features() -> Vec<String> {
    vec![
        "highway".into(),
        "waterway".into(),
        "natural=water".into(),
        "natural=wetland".into(),
        "landuse=reservoir".into(),
        "landuse=basin".into(),
        "landuse=brownfield".into(),
        "man_made=spoil_heap".into(),
    ]
}

/// Default Overpass API endpoint. Public instance with conservative rate
/// limits; for mass refresh (many regions in a short window) override per
/// region with a self-hosted or partner endpoint.
fn default_overpass_endpoint() -> String {
    "https://overpass-api.de/api/interpreter".into()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bounds {
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct View {
    pub center_lon: Option<f64>,
    pub center_lat: Option<f64>,
    pub default_zoom: Option<u32>,
}

impl Region {
    pub fn basemap_path(&self) -> PathBuf {
        self.dir.join("basemap.pmtiles")
    }

    /// Absolute paths to every `[[overlays]] kind = "osm"` block whose
    /// resolved file exists on disk, in declaration order. Both source
    /// variants (`source = "file"` and `source = "overpass"`) flow through
    /// this single accessor; the kiosk-lab doesn't distinguish them at
    /// render time. Later entries paint on top per the loop in
    /// `pmtiles_map.rs::show`. Soft-fallback: missing files log a warning
    /// rather than crashing (the operator may not have run the bake yet,
    /// or the Overpass API was unavailable at bake time).
    pub fn osm_overlay_paths(&self) -> Vec<PathBuf> {
        self.overlays
            .iter()
            .filter_map(|o| match o {
                Overlay::Osm(OsmSource::File { file }) => {
                    let candidate = self.dir.join(file);
                    if candidate.exists() {
                        Some(candidate)
                    } else {
                        eprintln!(
                            "[regions] {}: osm source=file overlay {} not found; rendering without",
                            self.name,
                            candidate.display()
                        );
                        None
                    }
                }
                Overlay::Osm(OsmSource::Overpass { .. }) => {
                    let candidate = self.dir.join("osm-overpass.osm");
                    if candidate.exists() {
                        Some(candidate)
                    } else {
                        eprintln!(
                            "[regions] {}: osm source=overpass overlay {} not found; rendering without",
                            self.name,
                            candidate.display()
                        );
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Absolute path to the first `[[overlays]]` entry of `kind = "hillshade"`
    /// whose file exists on disk, plus the `source` string (audit trail).
    /// Returns None if no hillshade entry is declared or the declared file
    /// is missing. Soft-fallback: missing files log a warning.
    pub fn hillshade_overlay_path(&self) -> Option<PathBuf> {
        self.overlays.iter().find_map(|o| match o {
            Overlay::Hillshade { file, .. } => {
                let candidate = self.dir.join(file);
                if candidate.exists() {
                    Some(candidate)
                } else {
                    eprintln!(
                        "[regions] {}: hillshade overlay file {} not found; rendering without",
                        self.name,
                        candidate.display()
                    );
                    None
                }
            }
            _ => None,
        })
    }

    pub fn center(&self) -> (f64, f64) {
        let lon = self
            .view
            .center_lon
            .unwrap_or_else(|| 0.5 * (self.bounds.min_lon + self.bounds.max_lon));
        let lat = self
            .view
            .center_lat
            .unwrap_or_else(|| 0.5 * (self.bounds.min_lat + self.bounds.max_lat));
        (lon, lat)
    }
}

/// Read and parse the region descriptor at `<dir>/region.toml`. Returns the
/// parsed Region with `dir` populated, or an io/parse error.
pub fn load(dir: impl AsRef<Path>) -> Result<Region, RegionLoadError> {
    let dir = dir.as_ref().to_path_buf();
    let toml_path = dir.join("region.toml");
    let text = std::fs::read_to_string(&toml_path).map_err(|e| RegionLoadError::Io {
        path: toml_path.clone(),
        source: e,
    })?;
    let mut region: Region = toml::from_str(&text).map_err(|e| RegionLoadError::Parse {
        path: toml_path,
        source: e,
    })?;
    region.dir = dir;
    Ok(region)
}

/// Scan `resources/regions/` for subdirectories that contain both
/// `region.toml` and a fetched `basemap.pmtiles`. Returns the parsed
/// regions in name-sorted order; entries that fail to parse are logged to
/// stderr and skipped.
pub fn discover(regions_root: impl AsRef<Path>) -> Vec<Region> {
    let root = regions_root.as_ref();
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return found;
    };
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        if !dir.join("basemap.pmtiles").exists() {
            continue;
        }
        match load(&dir) {
            Ok(r) => found.push(r),
            Err(e) => {
                eprintln!("[regions] skipping {}: {}", dir.display(), e);
            }
        }
    }
    found.sort_by(|a, b| a.name.cmp(&b.name));
    found
}

/// Pick a default region from the discovered set. Preference order:
///   1. `terril-waterschei` (SARCom v1a test area)
///   2. First entry by name (fallback for fresh setups where only the
///      Phase-1 fixture exists).
pub fn pick_default(regions: &[Region]) -> Option<&Region> {
    regions
        .iter()
        .find(|r| r.name == "terril-waterschei")
        .or_else(|| regions.first())
}

#[derive(Debug)]
pub enum RegionLoadError {
    Io { path: PathBuf, source: std::io::Error },
    Parse { path: PathBuf, source: toml::de::Error },
}

impl std::fmt::Display for RegionLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "io error reading {}: {}", path.display(), source),
            Self::Parse { path, source } => write!(f, "parse error in {}: {}", path.display(), source),
        }
    }
}

impl std::error::Error for RegionLoadError {}

/// Repository-relative path to the regions root. Resolved against
/// `CARGO_MANIFEST_DIR` so it works regardless of cwd at run time.
pub fn regions_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("resources")
        .join("regions")
}
