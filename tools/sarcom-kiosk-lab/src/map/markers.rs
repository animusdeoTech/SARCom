use crate::data::{freshness_for_tag, NodeData, NodeKind, SimState};
use crate::map::Viewport;
use crate::ui::format_age;
use crate::ui::palette::{freshness_color, AMBER, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM};
use eframe::egui;

/// Drag target — index into `sim.nodes`. The kind-of-thing-being-dragged is
/// looked up via `sim.inventory[node_id]` at apply time. No per-kind variants.
#[derive(Clone, PartialEq)]
pub enum DragTarget {
    Node(usize),
    /// Empty-area drag — used to pan the viewport.
    Pan,
}

pub fn n2s(norm: [f32; 2], view: &Viewport) -> egui::Pos2 {
    let world = egui::pos2(
        view.rect.min.x + norm[0] * view.rect.width(),
        view.rect.min.y + norm[1] * view.rect.height(),
    );
    view.apply(world)
}

fn s2n(pos: egui::Pos2, view: &Viewport) -> [f32; 2] {
    let world = view.unapply(pos);
    [
        ((world.x - view.rect.min.x) / view.rect.width()).clamp(0.0, 1.0),
        ((world.y - view.rect.min.y) / view.rect.height()).clamp(0.0, 1.0),
    ]
}

/// Visible position for a node — `pos` when `gps_valid`, else `last_valid_fix_pos`.
/// Same shape applies to tags, relays, and gateway. For relays/gateway in v1a
/// fixtures, `gps_valid=true` and `pos` is always meaningful.
pub fn node_visible_pos(node: &NodeData) -> Option<[f32; 2]> {
    if node.gps_valid {
        Some(node.pos)
    } else {
        node.last_valid_fix_pos
    }
}

/// Per-state colour from `freshness_color` (Decisions pinned #8 in KIOSK-008).
/// Applies to any node kind that surfaces a freshness-driven dot colour.
pub fn node_display_color(node: &NodeData) -> egui::Color32 {
    if node.sos {
        return RED;
    }
    if !node.gps_valid {
        return GREY;
    }
    freshness_color(freshness_for_tag(node.last_seen_secs, false))
}

pub fn find_closest(
    ptr: egui::Pos2,
    sim: &SimState,
    view: &Viewport,
    threshold: f32,
) -> Option<DragTarget> {
    let mut best: Option<(f32, DragTarget)> = None;

    for (i, node) in sim.nodes.iter().enumerate() {
        // For tags use visible-pos (handles ghost at last_valid_fix_pos);
        // for relays and gateway the pos is always meaningful.
        let p = match sim.kind_for_id(node.node_id) {
            NodeKind::Tag => node_visible_pos(node),
            _ => Some(node.pos),
        };
        if let Some(p) = p {
            let d = (ptr - n2s(p, view)).length();
            if d < threshold && best.as_ref().map_or(true, |(bd, _)| d < *bd) {
                best = Some((d, DragTarget::Node(i)));
            }
        }
    }

    best.map(|(_, t)| t)
}

pub fn apply_drag(sim: &mut SimState, target: &DragTarget, ptr: egui::Pos2, view: &Viewport) {
    let norm = s2n(ptr, view);
    match target {
        DragTarget::Node(i) => {
            let kind = sim.kind_for_id(sim.nodes[*i].node_id);
            let node = &mut sim.nodes[*i];
            // Tags: respect gps_valid / last_valid_fix mode. Relays/gateway:
            // always mutate `pos` directly.
            if kind == NodeKind::Tag {
                if node.gps_valid {
                    node.pos = norm;
                } else if node.last_valid_fix_pos.is_some() {
                    node.last_valid_fix_pos = Some(norm);
                } else {
                    node.pos = norm;
                }
            } else {
                node.pos = norm;
            }
        }
        DragTarget::Pan => {
            // Pan is handled at the show_map level by adjusting view_offset
            // directly from the per-frame drag delta. apply_drag is a no-op.
        }
    }
}

/// 1 px dashed track of recent valid-fix points. Sentinel/no-fix coordinates
/// never make it here — we only draw when `gps_valid` AND the node carries
/// ≥2 track points. Relays / gateway have empty tracks in v1a fixtures, so
/// they self-skip via the `< 2` check; no kind-gate needed.
pub fn draw_tracks(painter: &egui::Painter, sim: &SimState, view: &Viewport) {
    for node in &sim.nodes {
        if !node.gps_valid || node.track.len() < 2 {
            continue;
        }
        let base = node_display_color(node);
        let color = egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), 110);
        let pts: Vec<egui::Pos2> = node.track.iter().map(|p| n2s(*p, view)).collect();
        for w in pts.windows(2) {
            draw_dashed_segment(painter, w[0], w[1], color);
        }
    }
}

fn draw_dashed_segment(painter: &egui::Painter, a: egui::Pos2, b: egui::Pos2, color: egui::Color32) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1.0 {
        return;
    }
    let dash = 4.0_f32;
    let gap = 3.0_f32;
    let step = dash + gap;
    let n = (len / step).ceil() as i32;
    let ux = dx / len;
    let uy = dy / len;
    let stroke = egui::Stroke::new(1.0, color);
    for i in 0..n {
        let s = (i as f32) * step;
        let e = (s + dash).min(len);
        let p0 = egui::pos2(a.x + ux * s, a.y + uy * s);
        let p1 = egui::pos2(a.x + ux * e, a.y + uy * e);
        painter.line_segment([p0, p1], stroke);
    }
}

/// Find the first node of a given kind. v1a sim has at most one of each
/// non-tag kind; tag callers should iterate instead.
fn first_node_of(sim: &SimState, kind: NodeKind) -> Option<&NodeData> {
    sim.nodes
        .iter()
        .find(|n| sim.kind_for_id(n.node_id) == kind)
}

/// Relay marker: small ✚ pole/cross.
pub fn draw_relay(painter: &egui::Painter, sim: &SimState, view: &Viewport) {
    let Some(relay) = first_node_of(sim, NodeKind::Relay) else {
        return;
    };
    let rp = n2s(relay.pos, view);
    let arm = 7.0_f32;
    let stroke = egui::Stroke::new(2.0, ORANGE);
    painter.line_segment(
        [
            egui::pos2(rp.x - arm, rp.y),
            egui::pos2(rp.x + arm, rp.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(rp.x, rp.y - arm),
            egui::pos2(rp.x, rp.y + arm),
        ],
        stroke,
    );
    painter.circle_filled(rp, 2.0, ORANGE);
    painter.text(
        rp + egui::vec2(12.0, 0.0),
        egui::Align2::LEFT_CENTER,
        &relay.label,
        egui::FontId::monospace(10.0),
        ORANGE,
    );
}

/// Gateway marker: plain square outline.
pub fn draw_gateway(painter: &egui::Painter, sim: &SimState, view: &Viewport) {
    let Some(gw) = first_node_of(sim, NodeKind::Gateway) else {
        return;
    };
    let gp = n2s(gw.pos, view);
    let half = 7.0_f32;
    let body = egui::Rect::from_min_max(
        egui::pos2(gp.x - half, gp.y - half),
        egui::pos2(gp.x + half, gp.y + half),
    );
    painter.rect_stroke(body, 0, egui::Stroke::new(1.5, GREEN), egui::StrokeKind::Middle);
    painter.text(
        gp + egui::vec2(12.0, 0.0),
        egui::Align2::LEFT_CENTER,
        &gw.label,
        egui::FontId::monospace(10.0),
        GREEN,
    );
}

/// Draw tag dots (and ghosts for no-fix tags). Dispatches via inventory.kind;
/// relays/gateway are drawn separately via `draw_relay` / `draw_gateway`.
pub fn draw_tags(
    painter: &egui::Painter,
    sim: &SimState,
    selected_node: Option<usize>,
    t: f64,
    view: &Viewport,
) {
    for (i, node) in sim.nodes.iter().enumerate() {
        if sim.kind_for_id(node.node_id) != NodeKind::Tag {
            continue;
        }
        let is_sel = selected_node == Some(i);

        if node.gps_valid {
            draw_current_marker(painter, node, is_sel, t, view);
        } else if node.last_valid_fix_pos.is_some() {
            draw_ghost_marker(painter, node, is_sel, t, view);
        }
    }
}

fn draw_current_marker(
    painter: &egui::Painter,
    node: &NodeData,
    is_sel: bool,
    t: f64,
    view: &Viewport,
) {
    let sp = n2s(node.pos, view);
    let color = node_display_color(node);

    if node.sos {
        let pulse = ((t * 2.5).sin() * 0.5 + 0.5) as f32;
        let ring_alpha = (pulse * 180.0) as u8;
        painter.circle_stroke(
            sp,
            16.0,
            egui::Stroke::new(
                2.0,
                egui::Color32::from_rgba_unmultiplied(255, 60, 60, ring_alpha),
            ),
        );
    }

    let radius = if is_sel { 10.0 } else { 8.0 };
    painter.circle_filled(sp, radius, color);
    if is_sel {
        painter.circle_stroke(
            sp,
            radius + 2.0,
            egui::Stroke::new(1.5, egui::Color32::WHITE),
        );
    }
    painter.circle_stroke(
        sp,
        radius + 6.0,
        egui::Stroke::new(
            0.5,
            egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 60),
        ),
    );

    painter.text(
        sp + egui::vec2(14.0, -5.0),
        egui::Align2::LEFT_TOP,
        &node.label,
        egui::FontId::monospace(11.0),
        TEXT_BRIGHT,
    );
    painter.text(
        sp + egui::vec2(14.0, 7.0),
        egui::Align2::LEFT_TOP,
        &format_age(node.last_seen_secs),
        egui::FontId::monospace(9.0),
        TEXT_DIM,
    );
}

fn draw_ghost_marker(
    painter: &egui::Painter,
    node: &NodeData,
    is_sel: bool,
    t: f64,
    view: &Viewport,
) {
    let Some(lvf) = node.last_valid_fix_pos else {
        return;
    };
    let gp = n2s(lvf, view);
    let radius = if is_sel { 10.0 } else { 8.0 };

    if node.sos {
        let pulse = ((t * 2.5).sin() * 0.5 + 0.5) as f32;
        let ring_alpha = (pulse * 160.0) as u8;
        painter.circle_stroke(
            gp,
            18.0,
            egui::Stroke::new(
                2.0,
                egui::Color32::from_rgba_unmultiplied(255, 60, 60, ring_alpha),
            ),
        );
    }

    let fill = if node.sos {
        egui::Color32::from_rgba_unmultiplied(255, 60, 60, 90)
    } else {
        egui::Color32::from_rgba_unmultiplied(GREY.r(), GREY.g(), GREY.b(), 90)
    };
    painter.circle_filled(gp, radius, fill);

    let dash_color = egui::Color32::from_rgba_unmultiplied(220, 220, 220, 160);
    let r = radius + 4.0;
    for k in 0..12 {
        if k % 2 == 1 {
            continue;
        }
        let a0 = (k as f32) * std::f32::consts::TAU / 12.0;
        let a1 = ((k + 1) as f32) * std::f32::consts::TAU / 12.0;
        let p0 = egui::pos2(gp.x + r * a0.cos(), gp.y + r * a0.sin());
        let p1 = egui::pos2(gp.x + r * a1.cos(), gp.y + r * a1.sin());
        painter.line_segment([p0, p1], egui::Stroke::new(1.0, dash_color));
    }

    if is_sel {
        painter.circle_stroke(
            gp,
            radius + 2.0,
            egui::Stroke::new(1.5, egui::Color32::WHITE),
        );
    }

    let label_color = egui::Color32::from_rgba_unmultiplied(
        TEXT_BRIGHT.r(),
        TEXT_BRIGHT.g(),
        TEXT_BRIGHT.b(),
        170,
    );
    painter.text(
        gp + egui::vec2(14.0, -5.0),
        egui::Align2::LEFT_TOP,
        &format!("{} last fix", node.label),
        egui::FontId::monospace(10.0),
        label_color,
    );

    let sub = match node.last_valid_fix_age_secs {
        Some(age) => format!("NO FIX · {}", format_age(age)),
        None => "NO FIX".to_string(),
    };
    painter.text(
        gp + egui::vec2(14.0, 7.0),
        egui::Align2::LEFT_TOP,
        &sub,
        egui::FontId::monospace(9.0),
        AMBER,
    );
}
