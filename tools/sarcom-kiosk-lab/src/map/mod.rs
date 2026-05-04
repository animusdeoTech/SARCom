pub mod fake_grid;
pub mod markers;
pub mod osm_vector;

pub use markers::DragTarget;
pub use osm_vector::OsmMap;

use crate::app::KioskLabApp;
use crate::ui::palette::{BLUE, GREEN, GREY, MAP_BG, ORANGE, RED, TEXT_DIM};
use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    FakeGrid,
    OsmVector,
}

impl KioskLabApp {
    pub(crate) fn show_map(&mut self, ui: &mut egui::Ui, t: f64) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let mr = response.rect;

        painter.rect_filled(mr, 0.0, MAP_BG);

        match self.map_mode {
            MapMode::FakeGrid => fake_grid::draw(&painter, mr),
            MapMode::OsmVector => self.osm_map.draw(&painter, mr),
        }

        // Drag handling — uses tag_visible_pos so ghost markers can be moved
        // (lab-only convenience; the real gateway is read-only).
        if response.drag_started() {
            if let Some(ptr) = response.interact_pointer_pos() {
                self.drag_target = markers::find_closest(ptr, &self.sim, mr, 20.0);
            }
        }
        if response.dragged() {
            if let (Some(target), Some(ptr)) =
                (self.drag_target.clone(), response.interact_pointer_pos())
            {
                markers::apply_drag(&mut self.sim, &target, ptr, mr);
            }
        }
        if response.drag_stopped() {
            self.drag_target = None;
        }

        // Tracks
        if self.show_track {
            markers::draw_tracks(&painter, &self.sim.tags, mr);
        }

        // Node markers
        markers::draw_relay(&painter, &self.sim, mr);
        markers::draw_gateway(&painter, &self.sim, mr);
        markers::draw_tags(
            &painter,
            &self.sim.tags,
            self.selected_tag,
            t,
            mr,
            self.sim.clock_valid,
        );

        // Click-to-select uses the visible position (ghost for no-fix).
        if response.clicked() {
            if let Some(ptr) = response.interact_pointer_pos() {
                let mut found = false;
                for (i, tag) in self.sim.tags.iter().enumerate() {
                    if let Some(p) = markers::tag_visible_pos(tag) {
                        if (ptr - markers::n2s(p, mr)).length() < 16.0 {
                            self.selected_tag = if self.selected_tag == Some(i) {
                                None
                            } else {
                                Some(i)
                            };
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    self.selected_tag = None;
                }
            }
        }

        // Cursor hint
        if self.drag_target.is_some() {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
        } else if let Some(hover) = response.hover_pos() {
            if markers::find_closest(hover, &self.sim, mr, 20.0).is_some() {
                ui.ctx()
                    .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            }
        }

        // North arrow
        painter.text(
            egui::pos2(mr.max.x - 16.0, mr.min.y + 14.0),
            egui::Align2::CENTER_CENTER,
            "N",
            egui::FontId::monospace(12.0),
            TEXT_DIM,
        );

        // Legend (bottom-left). NOTE: legend deliberately has no "via relay"
        // / "direct" / "hop" entries — v1 protocol carries no path metadata
        // (ADR-013), so the kiosk must not imply per-packet path knowledge.
        let mut lx = mr.min.x + 8.0;
        let ly = mr.max.y - 14.0;
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
