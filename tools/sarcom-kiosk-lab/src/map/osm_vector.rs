use eframe::egui::{self, epaint};
use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};

// One extra level up compared to src/osm.rs: src/map/ → src/ → sarcom-kiosk-lab/ → tools/ → SARCom/
const OSM_XML: &str = include_str!("../../../../resources/osm/terril_waterschei.osm");

struct Bounds {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

impl Bounds {
    fn to_screen(&self, lat: f64, lon: f64, rect: egui::Rect) -> egui::Pos2 {
        let nx = ((lon - self.min_lon) / (self.max_lon - self.min_lon)) as f32;
        let ny = 1.0 - ((lat - self.min_lat) / (self.max_lat - self.min_lat)) as f32;
        egui::pos2(
            rect.min.x + nx * rect.width(),
            rect.min.y + ny * rect.height(),
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WayKind {
    HighwayMajor,
    HighwayMinor,
    Water,
}

struct Way {
    refs: Vec<i64>,
    kind: WayKind,
    closed: bool,
}

pub struct OsmMap {
    bounds: Bounds,
    nodes: HashMap<i64, (f64, f64)>,
    ways: Vec<Way>,
}

impl OsmMap {
    pub fn load() -> Self {
        parse(OSM_XML)
    }

    /// Letterbox `outer` to a sub-rect whose aspect matches the geographic
    /// area in real metres (i.e., longitude pre-multiplied by cos(mean_lat)).
    /// This forces a true 2D top-down rendering — features are no longer
    /// horizontally stretched by the rect's wider aspect at ~51 °N.
    pub fn fit_rect(&self, outer: egui::Rect) -> egui::Rect {
        let mean_lat = (self.bounds.min_lat + self.bounds.max_lat) * 0.5;
        let cos_lat = mean_lat.to_radians().cos();
        let geo_w = ((self.bounds.max_lon - self.bounds.min_lon) * cos_lat) as f32;
        let geo_h = (self.bounds.max_lat - self.bounds.min_lat) as f32;
        if geo_w <= 0.0 || geo_h <= 0.0 {
            return outer;
        }
        let geo_aspect = geo_w / geo_h;
        let outer_aspect = outer.width() / outer.height();
        if outer_aspect > geo_aspect {
            // outer is wider than geo — letterbox left/right
            let w = outer.height() * geo_aspect;
            let off = (outer.width() - w) * 0.5;
            egui::Rect::from_min_size(
                egui::pos2(outer.min.x + off, outer.min.y),
                egui::vec2(w, outer.height()),
            )
        } else {
            // outer is taller than geo — letterbox top/bottom
            let h = outer.width() / geo_aspect;
            let off = (outer.height() - h) * 0.5;
            egui::Rect::from_min_size(
                egui::pos2(outer.min.x, outer.min.y + off),
                egui::vec2(outer.width(), h),
            )
        }
    }

    pub fn draw(&self, painter: &egui::Painter, rect: egui::Rect) {
        let nodes = &self.nodes;
        let bounds = &self.bounds;
        let to_pt = |id: &i64| -> Option<egui::Pos2> {
            nodes
                .get(id)
                .map(|(lat, lon)| bounds.to_screen(*lat, *lon, rect))
        };

        for way in self.ways.iter().filter(|w| w.closed) {
            let fill = match way.kind {
                WayKind::Water => egui::Color32::from_rgba_unmultiplied(25, 70, 130, 130),
                _ => continue,
            };
            let pts: Vec<egui::Pos2> = way.refs.iter().filter_map(|id| to_pt(id)).collect();
            if pts.len() < 3 {
                continue;
            }
            painter.add(egui::Shape::Path(epaint::PathShape {
                points: pts,
                closed: true,
                fill,
                stroke: epaint::PathStroke::NONE,
            }));
        }

        for way in self
            .ways
            .iter()
            .filter(|w| !w.closed && w.kind == WayKind::Water)
        {
            let pts: Vec<egui::Pos2> = way.refs.iter().filter_map(|id| to_pt(id)).collect();
            if pts.len() < 2 {
                continue;
            }
            let s = egui::Stroke::new(
                1.5,
                egui::Color32::from_rgba_unmultiplied(50, 120, 200, 170),
            );
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], s);
            }
        }

        for way in self.ways.iter().filter(|w| w.kind == WayKind::HighwayMajor) {
            let pts: Vec<egui::Pos2> = way.refs.iter().filter_map(|id| to_pt(id)).collect();
            if pts.len() < 2 {
                continue;
            }
            let s = egui::Stroke::new(
                1.5,
                egui::Color32::from_rgba_unmultiplied(120, 120, 120, 190),
            );
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], s);
            }
        }

        for way in self.ways.iter().filter(|w| w.kind == WayKind::HighwayMinor) {
            let pts: Vec<egui::Pos2> = way.refs.iter().filter_map(|id| to_pt(id)).collect();
            if pts.len() < 2 {
                continue;
            }
            let s = egui::Stroke::new(0.7, egui::Color32::from_rgba_unmultiplied(80, 80, 80, 150));
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], s);
            }
        }
    }
}

fn classify(tags: &[(String, String)]) -> Option<WayKind> {
    for (k, v) in tags {
        match k.as_str() {
            "highway" => {
                return Some(match v.as_str() {
                    "primary" | "secondary" | "tertiary" | "residential" | "living_street" => {
                        WayKind::HighwayMajor
                    }
                    _ => WayKind::HighwayMinor,
                })
            }
            "waterway" => return Some(WayKind::Water),
            "natural" => match v.as_str() {
                "water" | "wetland" => return Some(WayKind::Water),
                _ => {}
            },
            "landuse" => match v.as_str() {
                "reservoir" | "basin" => return Some(WayKind::Water),
                _ => {}
            },
            _ => {}
        }
    }
    None
}

fn parse(xml: &str) -> OsmMap {
    let mut bounds = Bounds {
        min_lat: 0.0,
        max_lat: 0.0,
        min_lon: 0.0,
        max_lon: 0.0,
    };
    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    let mut ways: Vec<Way> = Vec::new();

    let mut in_way = false;
    let mut cur_refs: Vec<i64> = Vec::new();
    let mut cur_tags: Vec<(String, String)> = Vec::new();

    for event in EventReader::from_str(xml) {
        let Ok(event) = event else { continue };
        match event {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let get = |key: &str| -> Option<&str> {
                    attributes
                        .iter()
                        .find(|a| a.name.local_name == key)
                        .map(|a| a.value.as_str())
                };
                match name.local_name.as_str() {
                    "bounds" => {
                        let p = |k| get(k).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                        bounds = Bounds {
                            min_lat: p("minlat"),
                            max_lat: p("maxlat"),
                            min_lon: p("minlon"),
                            max_lon: p("maxlon"),
                        };
                    }
                    "node" => {
                        if let (Some(id), Some(lat), Some(lon)) = (
                            get("id").and_then(|v| v.parse::<i64>().ok()),
                            get("lat").and_then(|v| v.parse::<f64>().ok()),
                            get("lon").and_then(|v| v.parse::<f64>().ok()),
                        ) {
                            nodes.insert(id, (lat, lon));
                        }
                    }
                    "way" => {
                        in_way = true;
                        cur_refs.clear();
                        cur_tags.clear();
                    }
                    "nd" if in_way => {
                        if let Some(r) = get("ref").and_then(|v| v.parse::<i64>().ok()) {
                            cur_refs.push(r);
                        }
                    }
                    "tag" if in_way => {
                        if let (Some(k), Some(v)) = (get("k"), get("v")) {
                            cur_tags.push((k.to_string(), v.to_string()));
                        }
                    }
                    _ => {}
                }
            }
            XmlEvent::EndElement { name } if name.local_name == "way" && in_way => {
                in_way = false;
                if let Some(kind) = classify(&cur_tags) {
                    if cur_refs.len() >= 2 {
                        let closed = cur_refs.first() == cur_refs.last();
                        ways.push(Way {
                            refs: cur_refs.clone(),
                            kind,
                            closed,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    OsmMap {
        bounds,
        nodes,
        ways,
    }
}
