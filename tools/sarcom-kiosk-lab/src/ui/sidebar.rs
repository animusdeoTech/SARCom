use crate::app::{KioskLabApp, Selection};
use crate::data::{
    freshness_for_relay, freshness_for_tag, Freshness, NodeData, NodeKind, SimState,
};
use crate::map::markers::node_display_color;
use crate::ui::palette::{
    freshness_color, AMBER, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM,
};
use crate::ui::{format_age, format_wall};
use eframe::egui;

/// Sidebar row — single row variant per
/// `dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md`. The kind-distinction
/// (icon glyph + label colour + age-suffix elision) is applied inside
/// `render_node_row` by looking up `sim.inventory[node_id]`.
struct Row<'a> {
    idx: usize,
    node: &'a NodeData,
}

impl KioskLabApp {
    pub(crate) fn show_sidebar(&mut self, ui: &mut egui::Ui, t: f64) {
        let sidebar_w = self.sidebar_width;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(sidebar_w);

            let rows = build_rows(&self.sim);
            for row in rows {
                let kind = self.sim.kind_for_id(row.node.node_id);
                let is_sel = self.selection.is(row.idx);
                let resp = render_node_row(ui, row.node, kind, is_sel, t);
                if resp.clicked() {
                    self.selection = if is_sel {
                        Selection::None
                    } else {
                        Selection::Node(row.idx)
                    };
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

/// Mission-first ordering inside the node list:
/// SOS → no-fix → stale/very-stale → normal tags → relays → gateway.
/// Priority is computed per node + inventory kind.
fn row_priority(node: &NodeData, kind: NodeKind) -> u8 {
    match kind {
        NodeKind::Tag => {
            if node.sos {
                return 0;
            }
            if !node.gps_valid {
                return 1;
            }
            match freshness_for_tag(node.last_seen_secs, false) {
                Freshness::VeryStale | Freshness::Stale => 2,
                _ => 3,
            }
        }
        NodeKind::Relay => 4,
        NodeKind::Gateway => 5,
    }
}

fn build_rows(sim: &SimState) -> Vec<Row<'_>> {
    let mut indices: Vec<usize> = (0..sim.nodes.len()).collect();
    indices.sort_by_key(|&i| {
        let n = &sim.nodes[i];
        (row_priority(n, sim.kind_for_id(n.node_id)), n.node_id)
    });
    indices
        .into_iter()
        .map(|i| Row {
            idx: i,
            node: &sim.nodes[i],
        })
        .collect()
}

/// Render any node — uniform layout. Kind affects:
///   - state-bullet colour (tag uses `node_display_color` per state;
///     relay uses `freshness_for_relay` thresholds; gateway is GREEN
///     because it's local).
///   - label colour (tag = TEXT_BRIGHT, relay = ORANGE, gateway = GREEN).
///   - age-suffix elision (gateway has `last_seen_secs == 0` sentinel →
///     no age line; gateway is local).
///
/// Field set is the same. No per-kind layout branch beyond presentation.
fn render_node_row(
    ui: &mut egui::Ui,
    node: &NodeData,
    kind: NodeKind,
    is_sel: bool,
    t: f64,
) -> egui::Response {
    let bg = if is_sel {
        egui::Color32::from_rgb(20, 30, 45)
    } else {
        egui::Color32::TRANSPARENT
    };

    let (dot, label_color) = match kind {
        NodeKind::Tag => (node_display_color(node), TEXT_BRIGHT),
        NodeKind::Relay => {
            let f = freshness_for_relay(node.last_seen_secs);
            (freshness_color(f), ORANGE)
        }
        NodeKind::Gateway => (GREEN, GREEN),
    };

    egui::Frame::NONE
        .fill(bg)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            // SOS sticky-alert prefix. Renders on any node where node.sos is
            // true — ADR-010 makes the behaviour tag-only at firmware level;
            // the UI does not defensively filter. A relay or gateway showing
            // SOS would be unusual state the operator should see, not silently
            // swallowed.
            if node.sos {
                let since = format!("since {}", format_wall(t - node.last_seen_secs as f64));
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

            // Primary line: bullet + label.
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(dot).size(11.0));
                ui.label(
                    egui::RichText::new(&node.label)
                        .color(label_color)
                        .strong()
                        .monospace()
                        .size(11.0),
                );
            });

            // last_seen line. Gateway elides this (it's local; sentinel-zero
            // `last_seen_secs` is not a meaningful age).
            if kind != NodeKind::Gateway {
                if !node.gps_valid {
                    // No-fix tag: GPS_VALID=0 + last-fix-age (if any).
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
                    if let Some(age) = node.last_valid_fix_age_secs {
                        ui.label(
                            egui::RichText::new(format!("  last fix {}", format_age(age)))
                                .color(TEXT_DIM)
                                .monospace()
                                .size(10.0),
                        );
                    }
                } else {
                    let frame_label = if kind == NodeKind::Relay {
                        "POSITION"
                    } else {
                        "last"
                    };
                    let line = format!(
                        "  {} {} · {}",
                        frame_label,
                        format_wall(t - node.last_seen_secs as f64),
                        format_age(node.last_seen_secs),
                    );
                    ui.label(
                        egui::RichText::new(line)
                            .color(TEXT_DIM)
                            .monospace()
                            .size(10.0),
                    );
                    let coord_suffix = match kind {
                        NodeKind::Relay => " · solar",
                        _ => "",
                    };
                    ui.label(
                        egui::RichText::new(format!(
                            "  {:.5}, {:.5}{}",
                            node.pos[0], node.pos[1], coord_suffix
                        ))
                        .color(TEXT_DIM)
                        .monospace()
                        .size(10.0),
                    );
                }
            }

            if node.battery_low {
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
