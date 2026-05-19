use crate::data::{NodeData, NodeKind, ScenarioKind, SimState};
use crate::map::region::{self, Region};
use crate::map::{DragTarget, MapMode, OsmMap, PmTilesMap};
use crate::ui::{
    format_age, format_wall,
    palette::{HEADER_BG, MAP_BG, SIDEBAR_BG, TEXT_BRIGHT, TEXT_DIM},
};
use eframe::egui;

pub struct KioskLabApp {
    pub(crate) scenario: ScenarioKind,
    pub(crate) sim: SimState,
    pub(crate) selected_tag: Option<usize>,
    pub(crate) show_track: bool,
    pub(crate) show_edit: bool,
    pub(crate) edit_tag_idx: usize,
    pub(crate) sidebar_width: f32,
    pub(crate) drag_target: Option<DragTarget>,
    pub(crate) layout_path: String,
    pub(crate) status_msg: String,
    pub(crate) status_expire: f64,
    pub(crate) map_mode: MapMode,
    /// Every `[[overlays]] kind = "osm"` block loaded as an `OsmMap`,
    /// in region.toml declaration order. Both source variants
    /// (`source = "file"` hand-drawn and `source = "overpass"` auto-
    /// fetched) load into the same vector; the renderer doesn't
    /// distinguish them. Later entries paint on top -- write the
    /// overpass block first and the hand-drawn block second so explicit
    /// hand-annotated detail wins the z-fight.
    pub(crate) osm_maps: Vec<OsmMap>,
    pub(crate) active_region: Option<Region>,
    pub(crate) pmtiles_map: Option<PmTilesMap>,
    pub(crate) view_offset: eframe::egui::Vec2,
    pub(crate) view_zoom: f32,
}

impl KioskLabApp {
    pub fn new() -> Self {
        let scenario = ScenarioKind::Normal;

        // Discover any region under resources/regions/ that has a fetched
        // basemap.pmtiles next to its region.toml; pick the SARCom test area
        // if available, else the first by name.
        let regions = region::discover(region::regions_root());
        let active_region = region::pick_default(&regions).cloned();

        // Load every `[[overlays]] kind = "osm"` block from the active
        // region in declaration order. Source variant (`"file"` /
        // `"overpass"`) is dispatch-side only; the kiosk-lab renders both
        // through the same OsmMap path. Failures (missing file, parse
        // error) log a stderr warning and skip that entry; the rest of
        // the stack keeps loading.
        let osm_maps: Vec<OsmMap> = active_region
            .as_ref()
            .map(|r| r.osm_overlay_paths())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|p| match OsmMap::load_from_path(&p) {
                Ok(m) => Some(m),
                Err(e) => {
                    eprintln!("[regions] osm overlay load failed at {}: {}", p.display(), e);
                    None
                }
            })
            .collect();

        // Default into the walkers + PMTiles path when at least one region is
        // ready; fall back to the egui-painted OSM-vector path otherwise so
        // the kiosk-lab stays usable on a fresh checkout (only useful if an
        // overlay is also present, since OsmVector renders the overlay
        // standalone).
        let default_mode = if active_region.is_some() {
            MapMode::PmTiles
        } else {
            MapMode::OsmVector
        };

        Self {
            scenario,
            sim: SimState::from_scenario(scenario),
            selected_tag: None,
            show_track: true,
            show_edit: false,
            edit_tag_idx: 0,
            sidebar_width: 320.0,
            drag_target: None,
            layout_path: "layout.json".into(),
            status_msg: String::new(),
            status_expire: 0.0,
            map_mode: default_mode,
            osm_maps,
            active_region,
            pmtiles_map: None,
            view_offset: egui::Vec2::ZERO,
            view_zoom: 1.0,
        }
    }

    pub(crate) fn switch_scenario(&mut self, kind: ScenarioKind) {
        self.scenario = kind;
        self.sim = SimState::from_scenario(kind);
        self.selected_tag = None;
        self.drag_target = None;
        self.edit_tag_idx = 0;
        self.view_offset = egui::Vec2::ZERO;
        self.view_zoom = 1.0;
    }

    fn set_status(&mut self, msg: impl Into<String>, t: f64) {
        self.status_msg = msg.into();
        self.status_expire = t + 3.0;
    }

    pub(crate) fn save_layout(&mut self, t: f64) {
        let file = LayoutFile {
            sidebar_width: self.sidebar_width,
            show_track: self.show_track,
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

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
                .show(&ctx, |ui| {
                    self.show_edit_panel(ui, t);
                });
            self.show_edit = open;
        }

        // Header bar
        egui::Panel::top("header")
            .frame(
                egui::Frame::NONE
                    .fill(HEADER_BG)
                    .inner_margin(egui::Margin::symmetric(10, 6)),
            )
            .show_inside(ui, |ui| {
                self.show_header(ui, t);
            });

        // Bottom status bar — built before the sidebar/central panels so
        // egui's layout reserves the space at the bottom of the viewport.
        // SARCOM-honest: no ACK, no downlink, no operator-action wording
        // (ADR-007 / ADR-008). When SOS is active the strip goes red and
        // wide; otherwise it stays as a thin "read-only" hint.
        let sos_tag: Option<&NodeData> = self
            .sim
            .nodes
            .iter()
            .find(|n| n.sos && self.sim.kind_for_id(n.node_id) == NodeKind::Tag);
        if let Some(tag) = sos_tag {
            let label = tag.label.clone();
            let last_frame = format_age(tag.last_seen_secs);
            // "since" is the wall-clock time of the most recent frame; lab is
            // synthetic so this stands in for the real distress-onset clock.
            let since = format!("since {}", format_wall(t - tag.last_seen_secs as f64));

            egui::Panel::bottom("bottom_status")
                .frame(
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(160, 28, 28))
                        .inner_margin(egui::Margin::symmetric(14, 5)),
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("●")
                                .color(egui::Color32::WHITE)
                                .strong()
                                .size(12.0),
                        );
                        ui.label(
                            egui::RichText::new("DISTRESS")
                                .color(egui::Color32::WHITE)
                                .strong()
                                .size(13.0),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "· {} · {} · flags.SOS=1 · last frame {}",
                                label, since, last_frame
                            ))
                            .color(egui::Color32::from_rgb(252, 220, 220))
                            .size(11.0),
                        );

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new("read-only · ack at the tag")
                                        .color(egui::Color32::from_rgb(252, 220, 220))
                                        .size(11.0),
                                );
                            },
                        );
                    });
                });
        } else {
            // Compose the bottom-status label from what is actually loaded.
            // Mode-aware (PMTiles / OSM / Fake grid); for PMTiles, include
            // "hillshade" when the LIDAR raster layer is present and the
            // OSM marker when the hand-drawn vector overlay is loaded;
            // zoom reflects walkers' MapMemory rather than a hardcoded
            // placeholder.
            let map_label = match self.map_mode {
                MapMode::FakeGrid => "Fake grid".to_string(),
                MapMode::OsmVector => "OSM".to_string(),
                MapMode::PmTiles => {
                    let mut parts: Vec<&str> = vec!["PMTiles"];
                    if self
                        .pmtiles_map
                        .as_ref()
                        .is_some_and(|p| p.has_hillshade())
                    {
                        parts.push("hillshade");
                    }
                    // One `OSM` token regardless of how many OSM overlays
                    // are stacked or which source variants they come from --
                    // the source distinction is an implementation detail,
                    // not an operator-facing facet.
                    if !self.osm_maps.is_empty() {
                        parts.push("OSM");
                    }
                    let zoom = self
                        .pmtiles_map
                        .as_ref()
                        .map(|p| p.zoom())
                        .unwrap_or(0.0);
                    format!("{} · zoom {:.0}", parts.join(" · "), zoom)
                }
            };

            egui::Panel::bottom("bottom_status")
                .frame(
                    egui::Frame::NONE
                        .fill(HEADER_BG)
                        .inner_margin(egui::Margin::symmetric(12, 3)),
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("read-only")
                                .color(TEXT_DIM)
                                .size(10.0),
                        );
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(map_label)
                                        .color(TEXT_DIM)
                                        .monospace()
                                        .size(10.0),
                                );
                            },
                        );
                    });
                });
        }

        // Sidebar
        egui::Panel::right("sidebar")
            .exact_size(self.sidebar_width)
            .frame(
                egui::Frame::NONE
                    .fill(SIDEBAR_BG)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show_inside(ui, |ui| {
                self.show_sidebar(ui, t);
            });

        // Map (central panel — show_map defined in map/mod.rs)
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(MAP_BG))
            .show_inside(ui, |ui| {
                self.show_map(ui, t);
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LayoutFile {
    sidebar_width: f32,
    show_track: bool,
    scenario: ScenarioKind,
    sim: SimState,
}
