use eframe::egui;
use crate::app::KioskLabApp;
use crate::ui::{format_age, palette::{ORANGE, GREEN, RED, AMBER, TEXT_DIM, TEXT_BRIGHT, DIVIDER}};
use crate::ui::palette::node_state_color;

impl KioskLabApp {
    pub(crate) fn show_sidebar(&mut self, ui: &mut egui::Ui, t: f64) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(self.sidebar_width);

            // ── Active Tags ───────────────────────────────────────────────
            sidebar_header(ui, "ACTIVE TAGS");
            for i in 0..self.sim.tags.len() {
                let tag     = &self.sim.tags[i];
                let color   = node_state_color(tag.state);
                let age_str = format_age(tag.last_seen_secs);
                let is_sel  = self.selected_tag == Some(i);

                let bg = if is_sel {
                    egui::Color32::from_rgb(20, 30, 45)
                } else {
                    egui::Color32::TRANSPARENT
                };

                let resp = egui::Frame::none()
                    .fill(bg)
                    .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("●").color(color).size(10.0));
                            ui.label(egui::RichText::new(&tag.label).color(TEXT_BRIGHT).strong().size(11.0));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new(&age_str).color(TEXT_DIM).monospace().size(10.0));
                                if tag.sos {
                                    ui.label(egui::RichText::new("SOS").color(RED).strong().size(9.0));
                                }
                            });
                        });
                    }).response;

                if resp.interact(egui::Sense::click()).clicked() {
                    self.selected_tag = if is_sel { None } else { Some(i) };
                }
            }

            ui.add_space(4.0);
            ui.painter().line_segment(
                [ui.min_rect().left_top()  + egui::vec2(0.0, ui.cursor().min.y - ui.min_rect().min.y),
                 ui.min_rect().right_top() + egui::vec2(0.0, ui.cursor().min.y - ui.min_rect().min.y)],
                egui::Stroke::new(1.0, DIVIDER),
            );

            // ── Tag Details ───────────────────────────────────────────────
            if let Some(idx) = self.selected_tag {
                if let Some(tag) = self.sim.tags.get(idx) {
                    sidebar_header(ui, "TAG DETAILS");
                    let color = node_state_color(tag.state);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(15, 22, 32))
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            ui.set_width(self.sidebar_width - 2.0);
                            kv(ui, "ID",        &format!("#{}", tag.node_id));
                            kv_color(ui, "STATE",    tag.state.label(), color);
                            kv(ui, "LAST SEEN", &format_age(tag.last_seen_secs));
                            kv_color(ui, "GPS",
                                if tag.gps_valid { "FIX" } else { "NO FIX" },
                                if tag.gps_valid { GREEN } else { AMBER });
                            kv_color(ui, "SOS",
                                if tag.sos { "ACTIVE" } else { "—" },
                                if tag.sos { RED } else { TEXT_DIM });
                            kv_color(ui, "BATTERY",
                                if tag.battery_low { "LOW" } else { "OK" },
                                if tag.battery_low { RED } else { GREEN });
                            kv(ui, "POS", &format!("{:.3}, {:.3}", tag.pos[0], tag.pos[1]));
                        });
                }
            }

            ui.add_space(4.0);

            // ── Network Status ────────────────────────────────────────────
            sidebar_header(ui, "NETWORK");
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(15, 22, 32))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.set_width(self.sidebar_width - 2.0);
                    ui.label(egui::RichText::new(&self.sim.relay.label).color(ORANGE).strong().size(11.0));
                    kv_color(ui, "STATUS", "ACTIVE", GREEN);
                    kv(ui, "POS", &format!("{:.3}, {:.3}",
                        self.sim.relay.pos[0], self.sim.relay.pos[1]));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(&self.sim.gateway.label).color(GREEN).strong().size(11.0));
                    kv_color(ui, "STATUS", "ONLINE", GREEN);
                    kv(ui, "POS", &format!("{:.3}, {:.3}",
                        self.sim.gateway.pos[0], self.sim.gateway.pos[1]));
                });

            // ── Sighting Log ──────────────────────────────────────────────
            if self.show_sighting_log {
                if let Some(idx) = self.selected_tag {
                    if let Some(tag) = self.sim.tags.get(idx) {
                        ui.add_space(4.0);
                        sidebar_header(ui, "SIGHTING LOG");
                        egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                            .show(ui, |ui| {
                                ui.set_width(self.sidebar_width - 2.0);
                                for s in tag.sightings.iter().take(8) {
                                    let mut line = format!(
                                        "{:>6}  seq={:<4}  {}",
                                        format_age(s.age_secs),
                                        s.seq,
                                        if s.gps_valid { "FIX" } else { "NOFIX" },
                                    );
                                    if s.sos { line.push_str("  SOS"); }
                                    let color = if s.sos { RED } else { TEXT_DIM };
                                    ui.label(egui::RichText::new(&line).color(color).monospace().size(9.0));
                                }
                                if tag.sightings.is_empty() {
                                    ui.label(egui::RichText::new("No sightings").color(TEXT_DIM).size(10.0));
                                }
                            });
                    }
                }
            }

            // ── Status message ────────────────────────────────────────────
            if !self.status_msg.is_empty() && t < self.status_expire {
                ui.add_space(8.0);
                ui.label(egui::RichText::new(&self.status_msg).color(GREEN).size(10.0));
            }
        });
    }
}

pub fn sidebar_header(ui: &mut egui::Ui, label: &str) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(7, 11, 16))
        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(label)
                .color(TEXT_DIM).size(9.0)
                .extra_letter_spacing(1.5));
        });
}

pub fn kv(ui: &mut egui::Ui, key: &str, val: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(key).color(TEXT_DIM).monospace().size(10.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(val).color(TEXT_BRIGHT).monospace().size(10.0));
        });
    });
}

pub fn kv_color(ui: &mut egui::Ui, key: &str, val: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(key).color(TEXT_DIM).monospace().size(10.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(val).color(color).monospace().strong().size(10.0));
        });
    });
}
