use eframe::egui;
use crate::data::NodeState;

pub const MAP_BG:      egui::Color32 = egui::Color32::from_rgb(10,  15,  20);
pub const HEADER_BG:   egui::Color32 = egui::Color32::from_rgb(7,   11,  16);
pub const SIDEBAR_BG:  egui::Color32 = egui::Color32::from_rgb(12,  17,  24);
pub const DIVIDER:     egui::Color32 = egui::Color32::from_rgb(26,  37,  48);
pub const TEXT_DIM:    egui::Color32 = egui::Color32::from_rgb(90,  106, 122);
pub const TEXT_BRIGHT: egui::Color32 = egui::Color32::from_rgb(192, 204, 216);
pub const ORANGE:      egui::Color32 = egui::Color32::from_rgb(240, 160, 48);
pub const GREEN:       egui::Color32 = egui::Color32::from_rgb(64,  208, 128);
pub const BLUE:        egui::Color32 = egui::Color32::from_rgb(50,  180, 255);
pub const RED:         egui::Color32 = egui::Color32::from_rgb(255, 60,  60);
pub const AMBER:       egui::Color32 = egui::Color32::from_rgb(234, 179, 8);
pub const GREY:        egui::Color32 = egui::Color32::from_rgb(107, 114, 128);

pub fn node_state_color(state: NodeState) -> egui::Color32 {
    match state {
        NodeState::Normal     => BLUE,
        NodeState::Stale      => ORANGE,
        NodeState::VeryStale  => egui::Color32::from_rgb(180, 83, 9),
        NodeState::NoFix      => GREY,
        NodeState::Sos        => RED,
        NodeState::LowBattery => AMBER,
    }
}
