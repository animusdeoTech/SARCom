use crate::data::{ScenarioKind, SimState, TagData};
use crate::map::{DragTarget, MapMode, OsmMap};
use crate::ui::{
    format_age_or_unavailable,
    palette::{HEADER_BG, MAP_BG, SIDEBAR_BG, TEXT_BRIGHT},
};
use eframe::egui;

pub struct KioskLabApp {
    pub(crate) scenario: ScenarioKind,
    pub(crate) sim: SimState,
    pub(crate) selected_tag: Option<usize>,
    pub(crate) show_track: bool,
    pub(crate) show_sighting_log: bool,
    pub(crate) show_edit: bool,
    pub(crate) edit_tag_idx: usize,
    pub(crate) sidebar_width: f32,
    pub(crate) drag_target: Option<DragTarget>,
    pub(crate) layout_path: String,
    pub(crate) status_msg: String,
    pub(crate) status_expire: f64,
    pub(crate) map_mode: MapMode,
    pub(crate) osm_map: OsmMap,
}

impl KioskLabApp {
    pub fn new() -> Self {
        let scenario = ScenarioKind::Normal;
        Self {
            scenario,
            sim: SimState::from_scenario(scenario),
            selected_tag: None,
            show_track: true,
            show_sighting_log: true,
            show_edit: false,
            edit_tag_idx: 0,
            sidebar_width: 320.0,
            drag_target: None,
            layout_path: "layout.json".into(),
            status_msg: String::new(),
            status_expire: 0.0,
            map_mode: MapMode::OsmVector,
            osm_map: OsmMap::load(),
        }
    }

    pub(crate) fn switch_scenario(&mut self, kind: ScenarioKind) {
        self.scenario = kind;
        self.sim = SimState::from_scenario(kind);
        self.selected_tag = None;
        self.drag_target = None;
        self.edit_tag_idx = 0;
    }

    fn set_status(&mut self, msg: impl Into<String>, t: f64) {
        self.status_msg = msg.into();
        self.status_expire = t + 3.0;
    }

    pub(crate) fn save_layout(&mut self, t: f64) {
        let file = LayoutFile {
            sidebar_width: self.sidebar_width,
            show_track: self.show_track,
            show_sighting_log: self.show_sighting_log,
            scenario: self.scenario,
            sim: self.sim.clone(),
        };
        match serde_json::to_string_pretty(&file) {
            Ok(json) => match std::fs::write(&self.layout_path, json) {
                Ok(_) => self.set_status(format!("Saved → {}", self.layout_path), t),
                Err(e) => self.set_status(format!("Save failed: {e}"), t),
            },
            Err(e) => self.set_status(format!("Serialise failed: {e}"), t),
        }
    }

    pub(crate) fn load_layout(&mut self, t: f64) {
        match std::fs::read_to_string(&self.layout_path) {
            Ok(json) => match serde_json::from_str::<LayoutFile>(&json) {
                Ok(file) => {
                    self.sidebar_width = file.sidebar_width;
                    self.show_track = file.show_track;
                    self.show_sighting_log = file.show_sighting_log;
                    self.scenario = file.scenario;
                    self.sim = file.sim;
                    self.selected_tag = None;
                    self.drag_target = None;
                    self.edit_tag_idx = 0;
                    self.set_status(format!("Loaded {}", self.layout_path), t);
                }
                Err(e) => self.set_status(format!("Parse failed: {e}"), t),
            },
            Err(e) => self.set_status(format!("Read failed: {e}"), t),
        }
    }
}

impl eframe::App for KioskLabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = SIDEBAR_BG;
        visuals.panel_fill = SIDEBAR_BG;
        visuals.override_text_color = Some(TEXT_BRIGHT);
        ctx.set_visuals(visuals);

        let t = ctx.input(|i| i.time);

        // Edit window (floating)
        if self.show_edit {
            let mut open = self.show_edit;
            egui::Window::new("Edit / Tweak")
                .open(&mut open)
                .default_pos([820.0, 50.0])
                .default_width(260.0)
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_edit_panel(ui, t);
                });
            self.show_edit = open;
        }

        // Header bar
        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(HEADER_BG)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                self.show_header(ui, t);
            });

        // SOS banner (conditionally shown). SARCOM-honest: no ACK, no
        // downlink, no operator-action wording — kiosk is read-only (ADR-007).
        let sos_tags: Vec<&TagData> = self.sim.tags.iter().filter(|t| t.sos).collect();
        if !sos_tags.is_empty() {
            let tag = sos_tags[0];
            let label = tag.label.clone();
            let no_fix = !tag.gps_valid;
            let clock_valid = self.sim.clock_valid;
            let last_frame = format_age_or_unavailable(tag.last_seen_secs, clock_valid);
            let last_fix = tag
                .last_valid_fix_age_secs
                .map(|a| format_age_or_unavailable(a, clock_valid));

            egui::TopBottomPanel::top("sos_banner")
                .frame(
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(180, 28, 28))
                        .inner_margin(egui::Margin::symmetric(14.0, 4.0)),
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("DISTRESS")
                                .color(egui::Color32::WHITE)
                                .strong()
                                .size(13.0),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new(&label)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new(format!("last frame {}", last_frame))
                                .color(egui::Color32::from_rgb(252, 165, 165)),
                        );
                        if no_fix {
                            ui.separator();
                            ui.label(
                                egui::RichText::new("NO CURRENT GPS FIX")
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            );
                            if let Some(lf) = last_fix {
                                ui.label(
                                    egui::RichText::new(format!("· last valid fix {}", lf))
                                        .color(egui::Color32::from_rgb(252, 165, 165)),
                                );
                            }
                        }
                    });
                });
        }

        // Sidebar
        egui::SidePanel::right("sidebar")
            .exact_width(self.sidebar_width)
            .frame(
                egui::Frame::none()
                    .fill(SIDEBAR_BG)
                    .inner_margin(egui::Margin::same(0.0)),
            )
            .show(ctx, |ui| {
                self.show_sidebar(ui, t);
            });

        // Map (central panel — show_map defined in map/mod.rs)
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(MAP_BG))
            .show(ctx, |ui| {
                self.show_map(ui, t);
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LayoutFile {
    sidebar_width: f32,
    show_track: bool,
    show_sighting_log: bool,
    scenario: ScenarioKind,
    sim: SimState,
}
