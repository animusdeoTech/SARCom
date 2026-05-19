pub mod fake_grid;
pub mod markers;
pub mod osm_vector;
pub mod pmtiles_map;
pub mod region;

pub use markers::DragTarget;
pub use osm_vector::OsmMap;
pub use pmtiles_map::PmTilesMap;

use crate::app::{KioskLabApp, Selection};
use crate::ui::palette::{BLUE, GREEN, GREY, MAP_BG, ORANGE, RED, TEXT_DIM};
use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    FakeGrid,
    OsmVector,
    PmTiles,
}

impl MapMode {
    pub fn label(&self) -> &'static str {
        match self {
            MapMode::FakeGrid => "Fake grid",
            MapMode::OsmVector => "OSM vector",
            MapMode::PmTiles => "PMTiles (walkers)",
        }
    }

    pub fn all() -> &'static [MapMode] {
        &[MapMode::FakeGrid, MapMode::OsmVector, MapMode::PmTiles]
    }
}

/// Pan + zoom view transform applied to every map primitive. `rect` is the
/// letterboxed map area in screen coordinates; `offset` and `zoom` are
/// applied around `rect.center()`.
#[derive(Clone, Copy)]
pub struct Viewport {
    pub rect: egui::Rect,
    pub offset: egui::Vec2,
    pub zoom: f32,
}

impl Viewport {
    pub fn apply(&self, p: egui::Pos2) -> egui::Pos2 {
        let c = self.rect.center();
        egui::pos2(
            c.x + (p.x - c.x) * self.zoom + self.offset.x,
            c.y + (p.y - c.y) * self.zoom + self.offset.y,
        )
    }

    pub fn unapply(&self, p: egui::Pos2) -> egui::Pos2 {
        let c = self.rect.center();
        egui::pos2(
            c.x + (p.x - c.x - self.offset.x) / self.zoom,
            c.y + (p.y - c.y - self.offset.y) / self.zoom,
        )
    }
}

impl KioskLabApp {
    pub(crate) fn show_map(&mut self, ui: &mut egui::Ui, t: f64) {
        // PMTiles mode hands the whole map area over to walkers, which manages
        // its own painter allocation, drag/zoom, and tile rendering. Branch
        // out before the egui-painted path's painter is allocated.
        if self.map_mode == MapMode::PmTiles {
            // Construct on first switch (or first frame). Bail to a hint label
            // if the operator hasn't run scripts/fetch-region.ps1 yet.
            if let Some(region) = self.active_region.clone() {
                // Split-borrow: self.osm_maps (shared, collected into a
                // `&[&OsmMap]` slice for the call) and self.pmtiles_map
                // (unique) are disjoint fields, so the borrow checker
                // accepts both in the same call to pm.show().
                let osm_overlays: Vec<&OsmMap> = self.osm_maps.iter().collect();
                let pm = self
                    .pmtiles_map
                    .get_or_insert_with(|| PmTilesMap::from_region(&region, ui.ctx()));
                pm.show(ui, &self.sim, &osm_overlays, t);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new(
                            "No region with basemap.pmtiles found under resources/regions/.\n\
                             Run: scripts/fetch-region.ps1 <region-name>",
                        )
                        .color(TEXT_DIM)
                        .size(12.0),
                    );
                });
            }
            return;
        }

        if self.map_mode == MapMode::OsmVector && self.osm_maps.is_empty() {
            // OsmVector renders the active region's first OSM overlay
            // standalone. With no overlays loaded, the legacy mode has
            // nothing to draw -- hint the operator at the configuration gap
            // rather than render a blank rectangle.
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(
                        "No OSM overlay for this region.\n\
                         Add an [[overlays]] kind = \"osm\" block to region.toml.",
                    )
                    .color(TEXT_DIM)
                    .size(12.0),
                );
            });
            return;
        }

        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let outer = response.rect;

        painter.rect_filled(outer, 0.0, MAP_BG);

        // Letterbox to a real-metre aspect for OSM (lat-stretched compensation).
        let mr = match self.map_mode {
            // self.osm_maps.is_empty() in OsmVector mode is handled by an
            // early-return higher in the function; if we reach this arm,
            // the first-index access is safe. Standalone OsmVector uses
            // the first OSM overlay's bounds for the letterbox.
            MapMode::OsmVector => self.osm_maps.first().unwrap().fit_rect(outer),
            MapMode::FakeGrid => outer,
            // PMTiles takes the early-return branch above; this arm is
            // structurally unreachable in current control flow.
            MapMode::PmTiles => unreachable!("PmTiles handled in early return"),
        };

        // Scroll-wheel zoom around the cursor — keeps the world point under
        // the cursor stationary while the rest of the map scales around it.
        if response.hovered() {
            let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_y.abs() > 0.01 {
                if let Some(cursor) = response.hover_pos() {
                    let factor = (scroll_y * 0.0015).exp();
                    let new_zoom = (self.view_zoom * factor).max(0.0001);
                    // Keep the world-point under cursor fixed:
                    //   cursor = c + (world - c) * z + offset
                    // Solve for new offset given new_zoom and the same world.
                    let c = mr.center();
                    let world = egui::pos2(
                        c.x + (cursor.x - c.x - self.view_offset.x) / self.view_zoom,
                        c.y + (cursor.y - c.y - self.view_offset.y) / self.view_zoom,
                    );
                    let new_offset = egui::vec2(
                        cursor.x - c.x - (world.x - c.x) * new_zoom,
                        cursor.y - c.y - (world.y - c.y) * new_zoom,
                    );
                    self.view_zoom = new_zoom;
                    self.view_offset = new_offset;
                }
            }
        }

        // Build view after the possible zoom update so the same-frame draw uses it.
        let view = Viewport {
            rect: mr,
            offset: self.view_offset,
            zoom: self.view_zoom,
        };

        // Drag start: marker if one is under the cursor, otherwise pan.
        if response.drag_started() {
            if let Some(ptr) = response.interact_pointer_pos() {
                self.drag_target =
                    Some(markers::find_closest(ptr, &self.sim, &view, 20.0).unwrap_or(DragTarget::Pan));
            }
        }
        if response.dragged() {
            match self.drag_target.clone() {
                Some(DragTarget::Pan) => {
                    self.view_offset += response.drag_delta();
                }
                Some(target) => {
                    if let Some(ptr) = response.interact_pointer_pos() {
                        markers::apply_drag(&mut self.sim, &target, ptr, &view);
                    }
                }
                None => {}
            }
        }
        if response.drag_stopped() {
            self.drag_target = None;
        }

        // Clip everything map-related to the outer rect so zoomed-out
        // content doesn't leak under the sidebar.
        let painter = painter.with_clip_rect(outer);

        match self.map_mode {
            MapMode::FakeGrid => fake_grid::draw(&painter, &view),
            // Standalone OsmVector mode draws every loaded OSM overlay
            // through the legacy Viewport-rect math, in declaration order
            // (later on top). Early-return above guarantees the slice is
            // non-empty so the letterbox math against `.first()` was safe.
            MapMode::OsmVector => {
                for osm in &self.osm_maps {
                    osm.draw(&painter, &view);
                }
            }
            MapMode::PmTiles => unreachable!("PmTiles handled in early return"),
        }

        if self.show_track {
            markers::draw_tracks(&painter, &self.sim, &view);
        }

        markers::draw_relay(&painter, &self.sim, &view);
        markers::draw_gateway(&painter, &self.sim, &view);
        markers::draw_tags(&painter, &self.sim, self.selection.idx(), t, &view);

        // Click-to-select uses the visible position. Iterates all nodes;
        // dispatch on kind happens inside `markers::node_visible_pos`. Per
        // dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md, the
        // selection is a single Node(usize) over `sim.nodes`.
        if response.clicked() {
            if let Some(ptr) = response.interact_pointer_pos() {
                let mut found = false;
                for (i, node) in self.sim.nodes.iter().enumerate() {
                    if let Some(p) = markers::node_visible_pos(node) {
                        if (ptr - markers::n2s(p, &view)).length() < 16.0 {
                            self.selection = if self.selection.is(i) {
                                Selection::None
                            } else {
                                Selection::Node(i)
                            };
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    self.selection = Selection::None;
                }
            }
        }

        // Cursor hint.
        if matches!(self.drag_target, Some(DragTarget::Pan)) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
        } else if self.drag_target.is_some() {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
        } else if let Some(hover) = response.hover_pos() {
            if markers::find_closest(hover, &self.sim, &view, 20.0).is_some() {
                ui.ctx()
                    .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            }
        }

        // North arrow (HUD — pinned to outer rect, not affected by viewport).
        painter.text(
            egui::pos2(outer.max.x - 16.0, outer.min.y + 14.0),
            egui::Align2::CENTER_CENTER,
            "N",
            egui::FontId::monospace(12.0),
            TEXT_DIM,
        );

        // Legend (HUD bottom-left). Per ADR-013 the v1 wire carries no path
        // metadata, so the legend deliberately has no "via relay" / "direct"
        // / "hop" entries.
        let mut lx = outer.min.x + 8.0;
        let ly = outer.max.y - 14.0;
        for (color, label) in &[
            (BLUE, "TAG"),
            (RED, "SOS"),
            (GREY, "NO FIX"),
            (ORANGE, "RELAY"),
            (GREEN, "GW"),
        ] {
            painter.circle_filled(egui::pos2(lx + 4.0, ly), 4.0, *color);
            painter.text(
                egui::pos2(lx + 10.0, ly),
                egui::Align2::LEFT_CENTER,
                *label,
                egui::FontId::monospace(9.0),
                TEXT_DIM,
            );
            lx += 60.0;
        }
    }
}
