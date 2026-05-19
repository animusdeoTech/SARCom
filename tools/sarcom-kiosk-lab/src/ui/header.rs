use crate::app::KioskLabApp;
use crate::data::ScenarioKind;
use crate::map::MapMode;
use crate::ui::palette::{BRAND_ORANGE, GREEN, GREY, TEXT_BRIGHT, TEXT_DIM};
use crate::ui::{format_age, format_wall};
use eframe::egui;

impl KioskLabApp {
    pub(crate) fn show_header(&mut self, ui: &mut egui::Ui, t: f64) {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("SARCOM")
                    .color(BRAND_ORANGE)
                    .strong()
                    .size(14.0),
            );

            ui.add_space(8.0);

            // Scenario badge -- combobox styled as a dim badge so the lab
            // can switch scenarios without making the header look like a
            // real-product control.
            let old = self.scenario;
            egui::ComboBox::from_id_salt("scenario")
                .selected_text(
                    egui::RichText::new(self.scenario.label())
                        .color(TEXT_DIM)
                        .size(11.0),
                )
                .width(140.0)
                .show_ui(ui, |ui| {
                    for &kind in ScenarioKind::all() {
                        ui.selectable_value(&mut self.scenario, kind, kind.label());
                    }
                });
            if self.scenario != old {
                let new = self.scenario;
                self.switch_scenario(new);
            }

            ui.add_space(8.0);

            // Map-mode badge -- lets the Phase-1 spike run switch between the
            // walkers PMTiles render path and the legacy egui-painted modes
            // without rebuilding. Adjacent to the scenario combobox; same dim
            // styling so it doesn't read like a real-product control.
            egui::ComboBox::from_id_salt("map_mode")
                .selected_text(
                    egui::RichText::new(self.map_mode.label())
                        .color(TEXT_DIM)
                        .size(11.0),
                )
                .width(160.0)
                .show_ui(ui, |ui| {
                    for &mode in MapMode::all() {
                        ui.selectable_value(&mut self.map_mode, mode, mode.label());
                    }
                });

            // Right-side cluster: wall clock only. ADR-007 read-only kiosk —
            // no Edit button, no settings, no toggles. Built first
            // (right-to-left layout) so the centre slot can use the
            // remaining width.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format_wall(t))
                        .color(TEXT_BRIGHT)
                        .monospace()
                        .size(13.0),
                );

                ui.add_space(20.0);

                // Centre slot: "last RX [dot] HH:MM:SS · Nm ago"
                if let Some((age, fresh)) = newest_rx(&self.sim) {
                    let dot_color = if fresh { GREEN } else { GREY };
                    ui.label(
                        egui::RichText::new(format_age(age))
                            .color(TEXT_DIM)
                            .monospace()
                            .size(11.0),
                    );
                    ui.label(egui::RichText::new("●").color(dot_color).size(10.0));
                    ui.label(
                        egui::RichText::new("last RX")
                            .color(TEXT_DIM)
                            .size(11.0),
                    );
                }
            });
        });
    }
}

/// Most recent (smallest age) tag last-seen across the scenario, plus a
/// freshness flag (fresh = under tag heartbeat cadence, ~330 s).
fn newest_rx(sim: &crate::data::SimState) -> Option<(f32, bool)> {
    use crate::data::NodeKind;
    sim.nodes
        .iter()
        .filter(|n| sim.kind_for_id(n.node_id) == NodeKind::Tag)
        .map(|n| n.last_seen_secs)
        .fold(None::<f32>, |acc, s| match acc {
            Some(min) if min < s => Some(min),
            _ => Some(s),
        })
        .map(|age| (age, age < 330.0))
}
