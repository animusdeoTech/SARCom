use crate::app::KioskLabApp;
use crate::data::{freshness_for_relay, freshness_for_tag, Freshness, TagData};
use crate::map::markers::tag_display_color;
use crate::ui::format_age_or_unavailable;
use crate::ui::palette::{
    freshness_color, AMBER, DIVIDER, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM,
};
use eframe::egui;

impl KioskLabApp {
    pub(crate) fn show_sidebar(&mut self, ui: &mut egui::Ui, t: f64) {
        let clock_valid = self.sim.clock_valid;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(self.sidebar_width);

            // ── HIKERS ────────────────────────────────────────────────────
            sidebar_header(ui, "HIKERS");

            // Sort indices by mission priority. Original indexing is preserved
            // so `selected_tag` keeps pointing at the same tag across re-sorts.
            let mut sorted: Vec<usize> = (0..self.sim.tags.len()).collect();
            sorted.sort_by_key(|&i| hiker_priority(&self.sim.tags[i]));

            if sorted.is_empty() {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("  (no hikers)")
                        .color(TEXT_DIM)
                        .size(10.0),
                );
                ui.add_space(4.0);
            }

            for i in sorted {
                let tag = &self.sim.tags[i];
                let dot = tag_display_color(tag);
                let is_sel = self.selected_tag == Some(i);
                let bg = if is_sel {
                    egui::Color32::from_rgb(20, 30, 45)
                } else {
                    egui::Color32::TRANSPARENT
                };
                let age_str = format_age_or_unavailable(tag.last_seen_secs, clock_valid);

                let resp = egui::Frame::none()
                    .fill(bg)
                    .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("●").color(dot).size(10.0));
                            ui.label(
                                egui::RichText::new(&tag.label)
                                    .color(TEXT_BRIGHT)
                                    .strong()
                                    .size(11.0),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(&age_str)
                                            .color(TEXT_DIM)
                                            .monospace()
                                            .size(10.0),
                                    );
                                    if tag.battery_low {
                                        ui.label(
                                            egui::RichText::new("BATT")
                                                .color(AMBER)
                                                .strong()
                                                .size(9.0),
                                        );
                                    }
                                    if !tag.gps_valid {
                                        ui.label(
                                            egui::RichText::new("NOFIX")
                                                .color(GREY)
                                                .strong()
                                                .size(9.0),
                                        );
                                    }
                                    if tag.sos {
                                        ui.label(
                                            egui::RichText::new("SOS")
                                                .color(RED)
                                                .strong()
                                                .size(9.0),
                                        );
                                    }
                                },
                            );
                        });
                    })
                    .response;

                if resp.interact(egui::Sense::click()).clicked() {
                    self.selected_tag = if is_sel { None } else { Some(i) };
                }
            }

            divider_line(ui);

            // ── TAG DETAILS (selected) ────────────────────────────────────
            if let Some(idx) = self.selected_tag {
                if let Some(tag) = self.sim.tags.get(idx) {
                    sidebar_header(ui, "TAG DETAILS");
                    let dot = tag_display_color(tag);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(15, 22, 32))
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            ui.set_width(self.sidebar_width - 2.0);
                            kv(ui, "ID", &format!("#{}", tag.node_id));
                            kv_color(ui, "STATE", tag.state.label(), dot);
                            kv(
                                ui,
                                "LAST SEEN",
                                &format_age_or_unavailable(tag.last_seen_secs, clock_valid),
                            );
                            kv_color(
                                ui,
                                "GPS",
                                if tag.gps_valid { "FIX" } else { "NO FIX" },
                                if tag.gps_valid { GREEN } else { AMBER },
                            );
                            // Show last valid fix only when current GPS is invalid;
                            // otherwise the current marker is the answer.
                            if !tag.gps_valid {
                                if let Some(age) = tag.last_valid_fix_age_secs {
                                    kv(
                                        ui,
                                        "LAST FIX",
                                        &format_age_or_unavailable(age, clock_valid),
                                    );
                                    if let Some(p) = tag.last_valid_fix_pos {
                                        kv(ui, "  AT", &format!("{:.3}, {:.3}", p[0], p[1]));
                                    }
                                } else {
                                    kv_color(ui, "LAST FIX", "none on record", TEXT_DIM);
                                }
                            }
                            kv_color(
                                ui,
                                "SOS",
                                if tag.sos { "ACTIVE" } else { "—" },
                                if tag.sos { RED } else { TEXT_DIM },
                            );
                            kv_color(
                                ui,
                                "BATTERY",
                                if tag.battery_low { "LOW" } else { "OK" },
                                if tag.battery_low { RED } else { GREEN },
                            );
                            // Current pos is meaningless when gps_valid=false; hide it.
                            if tag.gps_valid {
                                kv(ui, "POS", &format!("{:.3}, {:.3}", tag.pos[0], tag.pos[1]));
                            }
                        });
                }
            }

            ui.add_space(4.0);

            // ── INFRA ─────────────────────────────────────────────────────
            sidebar_header(ui, "INFRA");
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(15, 22, 32))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.set_width(self.sidebar_width - 2.0);
                    ui.label(
                        egui::RichText::new(&self.sim.relay.label)
                            .color(ORANGE)
                            .strong()
                            .size(11.0),
                    );
                    // Relay self-announce uses its own freshness rule (~1800 s cadence).
                    // Phrase it as relay-only — never apply tag thresholds here.
                    match self.sim.relay.self_ann_age_secs {
                        Some(age) => {
                            let age_str = format_age_or_unavailable(age, clock_valid);
                            let color = if clock_valid {
                                freshness_color(freshness_for_relay(age))
                            } else {
                                TEXT_DIM
                            };
                            // Key already reads SELF-ANN; the value is just
                            // the relay-cadence age. Don't repeat "self-ann"
                            // in the value or the row goes >25 chars and the
                            // 240–280 px sidebar clips.
                            kv_color(ui, "SELF-ANN", &age_str, color);
                        }
                        None => {
                            kv_color(ui, "SELF-ANN", "no frame rx", TEXT_DIM);
                        }
                    }
                    kv(
                        ui,
                        "POS",
                        &format!("{:.3}, {:.3}", self.sim.relay.pos[0], self.sim.relay.pos[1]),
                    );
                });

            ui.add_space(4.0);

            // ── SYSTEM ────────────────────────────────────────────────────
            sidebar_header(ui, "SYSTEM");
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(15, 22, 32))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.set_width(self.sidebar_width - 2.0);
                    ui.label(
                        egui::RichText::new(&self.sim.gateway.label)
                            .color(GREEN)
                            .strong()
                            .size(11.0),
                    );
                    kv_color(ui, "GATEWAY", "ONLINE", GREEN);
                    kv_color(ui, "RADIO", "RX", GREEN);
                    kv_color(
                        ui,
                        "RTC",
                        if clock_valid { "OK" } else { "NOT SET" },
                        if clock_valid { GREEN } else { AMBER },
                    );
                });

            // ── SIGHTING LOG ──────────────────────────────────────────────
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
                                    let age_part =
                                        format_age_or_unavailable(s.age_secs, clock_valid);
                                    let mut line = format!(
                                        "{:>16}  seq={:<4}  {}",
                                        age_part,
                                        s.seq,
                                        if s.gps_valid { "FIX" } else { "NOFIX" },
                                    );
                                    if s.sos {
                                        line.push_str("  SOS");
                                    }
                                    let color = if s.sos { RED } else { TEXT_DIM };
                                    ui.label(
                                        egui::RichText::new(&line)
                                            .color(color)
                                            .monospace()
                                            .size(9.0),
                                    );
                                }
                                if tag.sightings.is_empty() {
                                    ui.label(
                                        egui::RichText::new("No sightings")
                                            .color(TEXT_DIM)
                                            .size(10.0),
                                    );
                                }
                            });
                    }
                }
            }

            // ── Status message ────────────────────────────────────────────
            if !self.status_msg.is_empty() && t < self.status_expire {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&self.status_msg)
                        .color(GREEN)
                        .size(10.0),
                );
            }
        });
    }
}

/// Mission-first ordering: SOS → no-fix → stale/very-stale → normal.
fn hiker_priority(tag: &TagData) -> u8 {
    if tag.sos {
        return 0;
    }
    if !tag.gps_valid {
        return 1;
    }
    match freshness_for_tag(tag.last_seen_secs, false) {
        Freshness::VeryStale | Freshness::Stale => 2,
        _ => 3,
    }
}

fn divider_line(ui: &mut egui::Ui) {
    ui.add_space(4.0);
    let y = ui.cursor().min.y;
    ui.painter().line_segment(
        [
            egui::pos2(ui.min_rect().left(), y),
            egui::pos2(ui.min_rect().right(), y),
        ],
        egui::Stroke::new(1.0, DIVIDER),
    );
}

pub fn sidebar_header(ui: &mut egui::Ui, label: &str) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(7, 11, 16))
        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(label)
                    .color(TEXT_DIM)
                    .size(9.0)
                    .extra_letter_spacing(1.5),
            );
        });
}

pub fn kv(ui: &mut egui::Ui, key: &str, val: &str) {
    kv_color(ui, key, val, TEXT_BRIGHT);
}

/// Key/value row that gracefully handles long values.
///
/// - If `key`, a 12 px gap, and `val` all fit in `available_width()`, the
///   row is a single line: key on the left, value right-aligned.
/// - Otherwise the value wraps to an indented second line. We never let the
///   value draw beyond the sidebar (the old `right_to_left` layout did).
pub fn kv_color(ui: &mut egui::Ui, key: &str, val: &str, color: egui::Color32) {
    let font = egui::FontId::monospace(10.0);
    let key_w = monospace_width(ui.ctx(), key, &font);
    let val_w = monospace_width(ui.ctx(), val, &font);
    let avail = ui.available_width();
    let gap_min = 12.0;
    // 6 px slack absorbs egui's internal item-spacing in horizontal layouts.
    let fits_one_line = key_w + gap_min + val_w + 6.0 <= avail;

    let key_rich = egui::RichText::new(key)
        .color(TEXT_DIM)
        .monospace()
        .size(10.0);
    let val_rich = egui::RichText::new(val)
        .color(color)
        .monospace()
        .strong()
        .size(10.0);

    if fits_one_line {
        ui.horizontal(|ui| {
            ui.label(key_rich);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(val_rich);
            });
        });
    } else {
        ui.label(key_rich);
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            // wrap() lets values longer than the sidebar break across lines
            // instead of being painted past the right edge.
            ui.add(egui::Label::new(val_rich).wrap());
        });
    }
}

fn monospace_width(ctx: &egui::Context, text: &str, font: &egui::FontId) -> f32 {
    ctx.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
            .rect
            .width()
    })
}
