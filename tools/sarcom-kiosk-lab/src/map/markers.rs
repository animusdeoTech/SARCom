use crate::data::{freshness_for_tag, SimState, TagData};
use crate::ui::format_age_or_unavailable;
use crate::ui::palette::{freshness_color, AMBER, GREEN, GREY, ORANGE, RED, TEXT_BRIGHT, TEXT_DIM};
use eframe::egui;

#[derive(Clone, PartialEq)]
pub enum DragTarget {
    Tag(usize),
    Relay,
    Gateway,
}

pub fn n2s(norm: [f32; 2], rect: egui::Rect) -> egui::Pos2 {
    egui::pos2(
        rect.min.x + norm[0] * rect.width(),
        rect.min.y + norm[1] * rect.height(),
    )
}

fn s2n(pos: egui::Pos2, rect: egui::Rect) -> [f32; 2] {
    [
        ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0),
        ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0),
    ]
}

/// The map position the user sees for a tag, or `None` if no marker is drawn.
/// - gps_valid: current `pos`
/// - !gps_valid + last_valid_fix_pos: ghost at last valid fix
/// - !gps_valid + no last valid fix: nothing on map
pub fn tag_visible_pos(tag: &TagData) -> Option<[f32; 2]> {
    if tag.gps_valid {
        Some(tag.pos)
    } else {
        tag.last_valid_fix_pos
    }
}

/// SOS / no-fix / freshness color for a tag's marker. Centralised so map
/// and sidebar agree on what a tag "looks like right now".
pub fn tag_display_color(tag: &TagData) -> egui::Color32 {
    if tag.sos {
        return RED;
    }
    if !tag.gps_valid {
        return GREY;
    }
    freshness_color(freshness_for_tag(tag.last_seen_secs, false))
}

pub fn find_closest(
    ptr: egui::Pos2,
    sim: &SimState,
    rect: egui::Rect,
    threshold: f32,
) -> Option<DragTarget> {
    let mut best: Option<(f32, DragTarget)> = None;

    let mut check = |dist: f32, target: DragTarget| {
        if dist < threshold && best.as_ref().map_or(true, |(d, _)| dist < *d) {
            best = Some((dist, target));
        }
    };

    for (i, tag) in sim.tags.iter().enumerate() {
        if let Some(p) = tag_visible_pos(tag) {
            check((ptr - n2s(p, rect)).length(), DragTarget::Tag(i));
        }
    }
    check((ptr - n2s(sim.relay.pos, rect)).length(), DragTarget::Relay);
    check(
        (ptr - n2s(sim.gateway.pos, rect)).length(),
        DragTarget::Gateway,
    );

    best.map(|(_, t)| t)
}

pub fn apply_drag(sim: &mut SimState, target: &DragTarget, ptr: egui::Pos2, rect: egui::Rect) {
    let norm = s2n(ptr, rect);
    match target {
        DragTarget::Tag(i) => {
            // For no-fix tags the ghost (last_valid_fix_pos) is the visible marker;
            // dragging it moves that ghost. For valid-fix tags, drag moves `pos`.
            let tag = &mut sim.tags[*i];
            if tag.gps_valid {
                tag.pos = norm;
            } else if tag.last_valid_fix_pos.is_some() {
                tag.last_valid_fix_pos = Some(norm);
            } else {
                tag.pos = norm;
            }
        }
        DragTarget::Relay => sim.relay.pos = norm,
        DragTarget::Gateway => sim.gateway.pos = norm,
    }
}

pub fn draw_tracks(painter: &egui::Painter, tags: &[TagData], rect: egui::Rect) {
    for tag in tags {
        if !tag.gps_valid || tag.track.len() < 2 {
            continue;
        }
        let base = tag_display_color(tag);
        let color = egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), 80);
        let pts: Vec<egui::Pos2> = tag.track.iter().map(|p| n2s(*p, rect)).collect();
        for w in pts.windows(2) {
            painter.line_segment([w[0], w[1]], egui::Stroke::new(1.5, color));
        }
        for (i, p) in pts.iter().enumerate() {
            if i < pts.len() - 1 {
                painter.circle_filled(*p, 2.0, color);
            }
        }
    }
}

pub fn draw_relay(painter: &egui::Painter, sim: &SimState, rect: egui::Rect) {
    let rp = n2s(sim.relay.pos, rect);
    let d = 8.0_f32;
    painter.add(egui::Shape::convex_polygon(
        vec![
            egui::pos2(rp.x, rp.y - d),
            egui::pos2(rp.x + d, rp.y),
            egui::pos2(rp.x, rp.y + d),
            egui::pos2(rp.x - d, rp.y),
        ],
        ORANGE,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 15, 20)),
    ));
    painter.text(
        rp + egui::vec2(12.0, 0.0),
        egui::Align2::LEFT_CENTER,
        &sim.relay.label,
        egui::FontId::monospace(10.0),
        ORANGE,
    );
}

pub fn draw_gateway(painter: &egui::Painter, sim: &SimState, rect: egui::Rect) {
    let gp = n2s(sim.gateway.pos, rect);
    let s = 8.0_f32;
    painter.rect_filled(
        egui::Rect::from_center_size(gp, egui::vec2(s * 2.0, s * 2.0)),
        1.0,
        GREEN,
    );
    painter.rect_stroke(
        egui::Rect::from_center_size(gp, egui::vec2(s * 2.0, s * 2.0)),
        1.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 15, 20)),
    );
    painter.text(
        gp + egui::vec2(12.0, 0.0),
        egui::Align2::LEFT_CENTER,
        &sim.gateway.label,
        egui::FontId::monospace(10.0),
        GREEN,
    );
}

pub fn draw_tags(
    painter: &egui::Painter,
    tags: &[TagData],
    selected_tag: Option<usize>,
    t: f64,
    rect: egui::Rect,
    clock_valid: bool,
) {
    for (i, tag) in tags.iter().enumerate() {
        let is_sel = selected_tag == Some(i);

        if tag.gps_valid {
            draw_current_marker(painter, tag, is_sel, t, rect, clock_valid);
        } else if tag.last_valid_fix_pos.is_some() {
            draw_ghost_marker(painter, tag, is_sel, t, rect, clock_valid);
        }
        // !gps_valid && no last_valid_fix_pos: deliberately no map marker.
        // Sidebar/no-fix list is the only honest place to surface this tag.
    }
}

fn draw_current_marker(
    painter: &egui::Painter,
    tag: &TagData,
    is_sel: bool,
    t: f64,
    rect: egui::Rect,
    clock_valid: bool,
) {
    let sp = n2s(tag.pos, rect);
    let color = tag_display_color(tag);

    if tag.sos {
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
        &tag.label,
        egui::FontId::monospace(11.0),
        TEXT_BRIGHT,
    );
    if clock_valid {
        painter.text(
            sp + egui::vec2(14.0, 7.0),
            egui::Align2::LEFT_TOP,
            &format_age_or_unavailable(tag.last_seen_secs, true),
            egui::FontId::monospace(9.0),
            TEXT_DIM,
        );
    }
}

/// Ghost marker drawn at last_valid_fix_pos for a tag whose current report
/// has GPS_VALID=0. Visually distinct from current-fix marker:
///   * filled circle has low alpha
///   * dashed-style outer ring (broken arc strokes)
///   * label reads "<TAG> last fix" not just "<TAG>"
///   * SOS pulse ring still drawn so distress remains prominent
fn draw_ghost_marker(
    painter: &egui::Painter,
    tag: &TagData,
    is_sel: bool,
    t: f64,
    rect: egui::Rect,
    clock_valid: bool,
) {
    let Some(lvf) = tag.last_valid_fix_pos else {
        return;
    };
    let gp = n2s(lvf, rect);
    let radius = if is_sel { 10.0 } else { 8.0 };

    if tag.sos {
        // SOS state is still load-bearing even when current GPS is invalid.
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

    let fill = if tag.sos {
        egui::Color32::from_rgba_unmultiplied(255, 60, 60, 90)
    } else {
        egui::Color32::from_rgba_unmultiplied(GREY.r(), GREY.g(), GREY.b(), 90)
    };
    painter.circle_filled(gp, radius, fill);

    // Dashed outer ring: 8 short arc segments simulated as offset chords.
    // egui has no native dashed-stroke; this hand-rolled version is enough
    // to read as "ghost / last-known" at kiosk distance.
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
        &format!("{} last fix", tag.label),
        egui::FontId::monospace(10.0),
        label_color,
    );

    let sub = match tag.last_valid_fix_age_secs {
        Some(age) => format!("NO FIX · {}", format_age_or_unavailable(age, clock_valid)),
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
