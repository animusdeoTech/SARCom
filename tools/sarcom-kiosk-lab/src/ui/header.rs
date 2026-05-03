use eframe::egui;
use crate::app::KioskLabApp;
use crate::data::ScenarioKind;
use crate::ui::palette::{ORANGE, GREEN, TEXT_DIM};

impl KioskLabApp {
    pub(crate) fn show_header(&mut self, ui: &mut egui::Ui, t: f64) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("SARCOM").color(ORANGE).strong().size(13.0));
            ui.label(egui::RichText::new("KIOSK LAB").color(TEXT_DIM).size(10.0));

            ui.separator();

            let old = self.scenario;
            egui::ComboBox::from_label("")
                .selected_text(self.scenario.label())
                .width(100.0)
                .show_ui(ui, |ui| {
                    for &kind in ScenarioKind::all() {
                        ui.selectable_value(&mut self.scenario, kind, kind.label());
                    }
                });
            if self.scenario != old {
                let new = self.scenario;
                self.switch_scenario(new);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let secs = t as u64;
                let time_str = format!("{:02}:{:02}:{:02}",
                    (secs / 3600) % 24, (secs / 60) % 60, secs % 60);
                ui.label(egui::RichText::new(time_str).color(TEXT_DIM).monospace().size(10.0));

                ui.label(egui::RichText::new("● GW ONLINE").color(GREEN).size(10.0));

                ui.separator();

                let edit_label = if self.show_edit { "■ Edit" } else { "Edit" };
                if ui.button(egui::RichText::new(edit_label).size(11.0)).clicked() {
                    self.show_edit = !self.show_edit;
                }
            });
        });
    }
}
