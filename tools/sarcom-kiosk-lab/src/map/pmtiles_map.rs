use eframe::egui;
use walkers::{Map, MapMemory, PmTiles, Position, Style, lon_lat};

use crate::data::SimState;
use crate::map::osm_vector::OsmMap;
use crate::map::region::{Bounds, Region};
use crate::ui::palette::{BLUE, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM};

/// Walkers + PMTiles render path. Owns the walkers tile source for the
/// basemap, the optional LIDAR-derived hillshade tile source, and the
/// walkers map memory for one named region. The kiosk-lab's legacy
/// `MapMode::FakeGrid` / `MapMode::OsmVector` paths are unaffected.
///
/// Construction is keyed on a `Region` (parsed from `region.toml` by
/// `crate::map::region::load`). All center/zoom/bbox metadata flows through
/// the descriptor; nothing in this module hard-codes coordinates.
pub struct PmTilesMap {
    pmtiles: PmTiles,
    /// Second `walkers::PmTiles` layer carrying LIDAR-derived hillshade
    /// raster tiles, produced by `scripts/fetch-region.ps1` from a
    /// `[[overlays]]` entry of `kind = "hillshade"`. None when the region
    /// does not declare a hillshade overlay or the file is missing on
    /// disk. Rendered via `Map::with_layer` above the basemap layer with
    /// transparency < 1.0 so the basemap stays visible underneath.
    hillshade_pmtiles: Option<PmTiles>,
    map_memory: MapMemory,
    center: Position,
    bounds: Bounds,
    pub region_name: String,
}

impl PmTilesMap {
    /// True when this map carries a LIDAR-derived hillshade raster layer
    /// in addition to the basemap. Used by the bottom-status label so the
    /// footer reflects what is actually rendered rather than a hardcoded
    /// string.
    pub fn has_hillshade(&self) -> bool {
        self.hillshade_pmtiles.is_some()
    }

    /// Current walkers zoom for the bottom-status label.
    pub fn zoom(&self) -> f64 {
        self.map_memory.zoom()
    }

    pub fn from_region(region: &Region, egui_ctx: &egui::Context) -> Self {
        let (lon, lat) = region.center();
        let mut map_memory = MapMemory::default();
        if let Some(z) = region.view.default_zoom {
            // InvalidZoom is logged and ignored -- walkers' default zoom is a
            // sensible fallback.
            let _ = map_memory.set_zoom(z as f64);
        }
        // Walkers' Style::default() is empty (no layer rules) -- it renders
        // nothing for MVT vector tiles. The built-in protomaps_dark preset
        // matches the kiosk-lab palette and works against any tileset that
        // follows the Protomaps v4 basemap schema (roads / water / buildings /
        // landuse / boundaries / places source layers). For non-Protomaps
        // PMTiles (e.g. the us-zipcodes-sample fixture, which only has zcta
        // polygons), the style is a harmless no-op.
        let style = Style::protomaps_dark();

        // Optional hillshade layer -- a second raster PMTiles archive
        // baked by scripts/fetch-region.ps1 from a [[overlays]] entry of
        // kind = "hillshade". Walkers' Tile::new auto-detects raster vs
        // MVT from the tile's magic bytes (walkers/src/tiles.rs:108-130),
        // so PmTiles::new is sufficient for raster -- no style needed.
        let hillshade_pmtiles = region
            .hillshade_overlay_path()
            .map(|path| PmTiles::new(path, egui_ctx.clone()));

        Self {
            pmtiles: PmTiles::with_style(region.basemap_path(), style, egui_ctx.clone()),
            hillshade_pmtiles,
            map_memory,
            center: lon_lat(lon, lat),
            bounds: region.bounds.clone(),
            region_name: region.name.clone(),
        }
    }

    /// Render the walkers map widget. Z-order:
    ///   1. PMTiles basemap layer (with_layer transparency=1.0)
    ///   2. Optional hillshade raster layer (with_layer transparency=0.5),
    ///      LIDAR-derived terrain shading rendered above the basemap by
    ///      walkers' multi-layer tile draw at `walkers/src/map.rs:191-193`
    ///   3. Inside the `Map::show` closure:
    ///      a. Optional hand-drawn OSM-XML overlay via `draw_with_projector`
    ///         (terril shape, trails, water -- hand-annotated landmark
    ///         detail, kept as dev-fallback when LIDAR is unavailable or
    ///         insufficient)
    ///      b. SARCom sim markers (relay / gateway / tag)
    ///      c. Region badge (bottom-right corner)
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        sim: &SimState,
        osm_overlay: Option<&OsmMap>,
        t: f64,
    ) {
        // Hoist any &self field reads BEFORE we hand &mut borrows of
        // self.map_memory / self.pmtiles to walkers' Map.
        let bounds = self.bounds.clone();
        let region_name = self.region_name.clone();
        let mut map = Map::new(None, &mut self.map_memory, self.center);
        map = map.with_layer(&mut self.pmtiles, 1.0);
        if let Some(hs) = self.hillshade_pmtiles.as_mut() {
            // Transparency 0.5 keeps the basemap visible underneath while
            // making terrain relief clearly readable. Tunable at visual
            // review; if 0.5 muddies the basemap, drop to 0.35 -- if it
            // washes out the terrain, raise to 0.7.
            map = map.with_layer(hs, 0.5);
        }

        map.show(ui, |ui, _response, projector, _memory| {
            let painter = ui.painter();

            // 2. OSM vector overlay (if the active region declared one).
            //    Painted before markers so markers always stay on top.
            if let Some(osm) = osm_overlay {
                osm.draw_with_projector(painter, projector);
            }

            // Relay marker: orange cross.
            let relay_pos = normalized_to_lonlat(sim.relay.pos, &bounds);
            let relay_screen = projector.project(relay_pos).to_pos2();
            draw_cross(painter, relay_screen, ORANGE);
            painter.text(
                relay_screen + egui::vec2(12.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &sim.relay.label,
                egui::FontId::monospace(10.0),
                ORANGE,
            );

            // Gateway marker: green square outline.
            let gw_pos = normalized_to_lonlat(sim.gateway.pos, &bounds);
            let gw_screen = projector.project(gw_pos).to_pos2();
            let half = 7.0;
            let gw_rect = egui::Rect::from_min_max(
                egui::pos2(gw_screen.x - half, gw_screen.y - half),
                egui::pos2(gw_screen.x + half, gw_screen.y + half),
            );
            painter.rect_stroke(
                gw_rect,
                0,
                egui::Stroke::new(1.5, GREEN),
                egui::StrokeKind::Middle,
            );
            painter.text(
                gw_screen + egui::vec2(12.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &sim.gateway.label,
                egui::FontId::monospace(10.0),
                GREEN,
            );

            // Tag markers: filled circles.
            for tag in &sim.tags {
                let p = if tag.gps_valid {
                    tag.pos
                } else if let Some(lvf) = tag.last_valid_fix_pos {
                    lvf
                } else {
                    continue;
                };
                let pos = normalized_to_lonlat(p, &bounds);
                let screen = projector.project(pos).to_pos2();

                let color = if tag.sos {
                    RED
                } else if !tag.gps_valid {
                    GREY
                } else {
                    BLUE
                };

                if tag.sos {
                    let pulse = ((t * 2.5).sin() * 0.5 + 0.5) as f32;
                    let alpha = (pulse * 180.0) as u8;
                    painter.circle_stroke(
                        screen,
                        16.0,
                        egui::Stroke::new(
                            2.0,
                            egui::Color32::from_rgba_unmultiplied(255, 60, 60, alpha),
                        ),
                    );
                }

                painter.circle_filled(screen, 8.0, color);
                painter.text(
                    screen + egui::vec2(12.0, -5.0),
                    egui::Align2::LEFT_TOP,
                    &tag.label,
                    egui::FontId::monospace(10.0),
                    TEXT_BRIGHT,
                );
            }

            // Bottom-right corner: which region is active, for the spike's
            // visible "we are rendering THIS basemap" evidence.
            let outer = ui.clip_rect();
            painter.text(
                egui::pos2(outer.max.x - 8.0, outer.max.y - 8.0),
                egui::Align2::RIGHT_BOTTOM,
                format!("region: {}", region_name),
                egui::FontId::monospace(10.0),
                TEXT_DIM,
            );
        });
    }
}

/// Map a normalized `[0..1] x [0..1]` coordinate from the kiosk-lab sim to a
/// lat/lon `Position` inside the region's WGS84 bounding box. Sim convention
/// is `y=0` at the top of the rectangle, so we flip vertically.
fn normalized_to_lonlat(norm: [f32; 2], bounds: &Bounds) -> Position {
    let lon = bounds.min_lon + (norm[0] as f64) * (bounds.max_lon - bounds.min_lon);
    let lat = bounds.max_lat - (norm[1] as f64) * (bounds.max_lat - bounds.min_lat);
    lon_lat(lon, lat)
}

fn draw_cross(painter: &egui::Painter, p: egui::Pos2, color: egui::Color32) {
    let arm = 7.0;
    let stroke = egui::Stroke::new(2.0, color);
    painter.line_segment(
        [egui::pos2(p.x - arm, p.y), egui::pos2(p.x + arm, p.y)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(p.x, p.y - arm), egui::pos2(p.x, p.y + arm)],
        stroke,
    );
    painter.circle_filled(p, 2.0, color);
}
