use crate::app::KioskLabApp;
use crate::data::{
    freshness_for_relay, freshness_for_tag, Freshness, GatewayData, RelayData, TagData,
};
use crate::map::markers::tag_display_color;
use crate::ui::palette::{
    freshness_color, AMBER, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM,
};
use crate::ui::{format_age, format_wall};
use eframe::egui;

enum Row<'a> {
    Hiker { idx: usize, tag: &'a TagData },
    Relay(&'a RelayData),
    Gateway(&'a GatewayData),
}

impl KioskLabApp {
    pub(crate) fn show_sidebar(&mut self, ui: &mut egui::Ui, t: f64) {
        let sidebar_w = self.sidebar_width;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(sidebar_w);

            let rows = build_rows(&self.sim);
            for row in rows {
                match row {
                    Row::Hiker { idx, tag } => {
                        let is_sel = self.selected_tag == Some(idx);
                        let resp = render_hiker_row(ui, tag, is_sel, t);
                        if resp.clicked() {
                            self.selected_tag = if is_sel { None } else { Some(idx) };
                        }
                    }
                    Row::Relay(relay) => render_relay_row(ui, relay, t),
                    Row::Gateway(gw) => render_gateway_row(ui, gw),
                }
            }

            if !self.status_msg.is_empty() && t < self.status_expire {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&self.status_msg)
                        .color(GREEN)
                        .size(10.0),
                );
            }

            ui.add_space(8.0);
        });
    }
}

/// Mission-first ordering inside the node list: SOS → no-fix → stale/very-stale → normal,
/// with relays and the gateway pinned at the bottom.
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

fn build_rows(sim: &crate::data::SimState) -> Vec<Row<'_>> {
    let mut tag_idxs: Vec<usize> = (0..sim.tags.len()).collect();
    tag_idxs.sort_by_key(|&i| (hiker_priority(&sim.tags[i]), sim.tags[i].node_id));

    let mut rows: Vec<Row<'_>> = Vec::with_capacity(sim.tags.len() + 2);
    for i in tag_idxs {
        rows.push(Row::Hiker {
            idx: i,
            tag: &sim.tags[i],
        });
    }
    rows.push(Row::Relay(&sim.relay));
    rows.push(Row::Gateway(&sim.gateway));
    rows
}

fn render_hiker_row(
    ui: &mut egui::Ui,
    tag: &TagData,
    is_sel: bool,
    t: f64,
) -> egui::Response {
    let bg = if is_sel {
        egui::Color32::from_rgb(20, 30, 45)
    } else {
        egui::Color32::TRANSPARENT
    };
    let dot = tag_display_color(tag);

    egui::Frame::NONE
        .fill(bg)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            if tag.sos {
                let since = format!("since {}", format_wall(t - tag.last_seen_secs as f64));
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("⚠ SOS")
                            .color(RED)
                            .strong()
                            .monospace()
                            .size(10.0),
                    );
                    ui.label(
                        egui::RichText::new(format!("· {}", since))
                            .color(RED)
                            .monospace()
                            .size(10.0),
                    );
                });
            }

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(dot).size(11.0));
                ui.label(
                    egui::RichText::new(&tag.label)
                        .color(TEXT_BRIGHT)
                        .strong()
                        .monospace()
                        .size(11.0),
                );
            });

            if !tag.gps_valid {
                ui.label(
                    egui::RichText::new("  last —")
                        .color(TEXT_DIM)
                        .monospace()
                        .size(10.0),
                );
                ui.label(
                    egui::RichText::new("  GPS_VALID=0 · sentinels")
                        .color(GREY)
                        .monospace()
                        .size(10.0),
                );
                if let Some(age) = tag.last_valid_fix_age_secs {
                    ui.label(
                        egui::RichText::new(format!("  last fix {}", format_age(age)))
                            .color(TEXT_DIM)
                            .monospace()
                            .size(10.0),
                    );
                }
            } else {
                let line = format!(
                    "  last {} · {}",
                    format_wall(t - tag.last_seen_secs as f64),
                    format_age(tag.last_seen_secs),
                );
                ui.label(
                    egui::RichText::new(line)
                        .color(TEXT_DIM)
                        .monospace()
                        .size(10.0),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "  {:.5}, {:.5}",
                        tag.pos[0], tag.pos[1]
                    ))
                    .color(TEXT_DIM)
                    .monospace()
                    .size(10.0),
                );
            }

            if tag.battery_low {
                ui.label(
                    egui::RichText::new("  BATT LOW")
                        .color(AMBER)
                        .strong()
                        .monospace()
                        .size(9.5),
                );
            }
        })
        .response
        .interact(egui::Sense::click())
}

fn render_relay_row(ui: &mut egui::Ui, relay: &RelayData, t: f64) {
    let dot = match relay.last_seen_secs {
        Some(age) => freshness_color(freshness_for_relay(age)),
        None => GREY,
    };

    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(dot).size(11.0));
                ui.label(
                    egui::RichText::new(&relay.label)
                        .color(ORANGE)
                        .strong()
                        .monospace()
                        .size(11.0),
                );
            });

            match relay.last_seen_secs {
                Some(age) => {
                    let line = format!(
                        "  POSITION {} · {}",
                        format_wall(t - age as f64),
                        format_age(age),
                    );
                    ui.label(
                        egui::RichText::new(line)
                            .color(TEXT_DIM)
                            .monospace()
                            .size(10.0),
                    );
                }
                None => {
                    ui.label(
                        egui::RichText::new("  POSITION — · no frame rx")
                            .color(TEXT_DIM)
                            .monospace()
                            .size(10.0),
                    );
                }
            }

            ui.label(
                egui::RichText::new(format!(
                    "  {:.5}, {:.5} · solar",
                    relay.pos[0], relay.pos[1]
                ))
                .color(TEXT_DIM)
                .monospace()
                .size(10.0),
            );
        });
}

fn render_gateway_row(ui: &mut egui::Ui, gw: &GatewayData) {
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(GREEN).size(11.0));
                ui.label(
                    egui::RichText::new(&gw.label)
                        .color(GREEN)
                        .strong()
                        .monospace()
                        .size(11.0),
                );
            });
            ui.label(
                egui::RichText::new("  RPi5 · Dragino HAT")
                    .color(TEXT_DIM)
                    .monospace()
                    .size(10.0),
            );
            ui.label(
                egui::RichText::new("  RTC: DS3231 ok")
                    .color(GREEN)
                    .monospace()
                    .size(10.0),
            );
        });
}
