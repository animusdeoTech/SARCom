use crate::app::KioskLabApp;
use crate::data::NodeState;
use crate::map::MapMode;
use crate::ui::palette::{GREEN, TEXT_DIM};
use eframe::egui;

impl KioskLabApp {
    pub(crate) fn show_edit_panel(&mut self, ui: &mut egui::Ui, t: f64) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── Map Background ────────────────────────────────────────────
            ui.collapsing("Map Background", |ui| {
                ui.radio_value(&mut self.map_mode, MapMode::FakeGrid, "Fake grid");
                ui.radio_value(&mut self.map_mode, MapMode::OsmVector, "OSM vector");
            });

            ui.separator();

            // ── Layout ────────────────────────────────────────────────────
            ui.collapsing("Layout", |ui| {
                // Min 280: below this the INFRA / SYSTEM rows start clipping
                // (kv helpers wrap to a second line, but we still need room
                // for the indented value).
                ui.add(
                    egui::Slider::new(&mut self.sidebar_width, 280.0..=420.0).text("sidebar width"),
                );
                ui.checkbox(&mut self.show_track, "Show track");
                ui.checkbox(&mut self.show_sighting_log, "Show sighting log");
            });

            ui.separator();

            // ── Tag Tweak ─────────────────────────────────────────────────
            ui.collapsing("Tag Tweak", |ui| {
                if self.sim.tags.is_empty() {
                    ui.label(egui::RichText::new("No tags in this scenario.").color(TEXT_DIM));
                    return;
                }
                self.edit_tag_idx = self.edit_tag_idx.min(self.sim.tags.len() - 1);

                let cur_label = self.sim.tags[self.edit_tag_idx].label.clone();
                egui::ComboBox::from_label("Tag")
                    .selected_text(&cur_label)
                    .show_ui(ui, |ui| {
                        for i in 0..self.sim.tags.len() {
                            let lbl = self.sim.tags[i].label.clone();
                            ui.selectable_value(&mut self.edit_tag_idx, i, lbl);
                        }
                    });

                let tag = &mut self.sim.tags[self.edit_tag_idx];

                let cur_state = tag.state;
                egui::ComboBox::from_label("State")
                    .selected_text(cur_state.label())
                    .show_ui(ui, |ui| {
                        for &s in NodeState::all() {
                            if ui.selectable_value(&mut tag.state, s, s.label()).clicked() {
                                tag.sos = matches!(s, NodeState::Sos);
                                tag.gps_valid = !matches!(s, NodeState::NoFix);
                                tag.battery_low = matches!(s, NodeState::LowBattery);
                            }
                        }
                    });

                ui.add(
                    egui::Slider::new(&mut tag.last_seen_secs, 0.0..=1800.0).text("last seen (s)"),
                );
                ui.checkbox(&mut tag.gps_valid, "GPS valid");
                ui.checkbox(&mut tag.sos, "SOS active");
                ui.checkbox(&mut tag.battery_low, "Battery low");
            });

            ui.separator();

            // ── Save / Load ───────────────────────────────────────────────
            ui.collapsing("Save / Load", |ui| {
                ui.label(
                    egui::RichText::new(&self.layout_path)
                        .color(TEXT_DIM)
                        .monospace()
                        .size(10.0),
                );
                ui.horizontal(|ui| {
                    if ui.button("Save layout").clicked() {
                        self.save_layout(t);
                    }
                    if ui.button("Load layout").clicked() {
                        self.load_layout(t);
                    }
                });
                ui.add_space(4.0);
                ui.label(egui::RichText::new(
                    "PNG export: use Windows Snipping Tool\n(Win+Shift+S). Native capture TODO."
                ).color(TEXT_DIM).size(9.0));
            });

            // ── Status ────────────────────────────────────────────────────
            if !self.status_msg.is_empty() && t < self.status_expire {
                ui.separator();
                ui.label(
                    egui::RichText::new(&self.status_msg)
                        .color(GREEN)
                        .size(10.0),
                );
            }
        });
    }
}
