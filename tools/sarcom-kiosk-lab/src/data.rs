use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScenarioKind {
    Normal,
    Sos,
    Stale,
    NoFix,
    MultiTag,
}

impl ScenarioKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal   => "Normal",
            Self::Sos      => "SOS",
            Self::Stale    => "Stale",
            Self::NoFix    => "No Fix",
            Self::MultiTag => "Multi-Tag",
        }
    }
    pub fn all() -> &'static [ScenarioKind] {
        &[Self::Normal, Self::Sos, Self::Stale, Self::NoFix, Self::MultiTag]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeState {
    Normal,
    Stale,
    VeryStale,
    NoFix,
    Sos,
    LowBattery,
}

impl NodeState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal     => "Normal",
            Self::Stale      => "Stale (>2 min)",
            Self::VeryStale  => "Very stale (>10 min)",
            Self::NoFix      => "No GPS fix",
            Self::Sos        => "SOS",
            Self::LowBattery => "Low battery",
        }
    }
    pub fn all() -> &'static [NodeState] {
        &[Self::Normal, Self::Stale, Self::VeryStale, Self::NoFix, Self::Sos, Self::LowBattery]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SightingEntry {
    pub seq: u32,
    pub age_secs: f32,
    pub gps_valid: bool,
    pub sos: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
    pub state: NodeState,
    pub last_seen_secs: f32,
    pub gps_valid: bool,
    pub sos: bool,
    pub battery_low: bool,
    pub track: Vec<[f32; 2]>,
    pub sightings: Vec<SightingEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub tags:    Vec<TagData>,
    pub relay:   RelayData,
    pub gateway: GatewayData,
}

impl SimState {
    pub fn from_scenario(kind: ScenarioKind) -> Self {
        match kind {
            ScenarioKind::Normal   => Self::normal(),
            ScenarioKind::Sos      => Self::sos(),
            ScenarioKind::Stale    => Self::stale(),
            ScenarioKind::NoFix    => Self::no_fix(),
            ScenarioKind::MultiTag => Self::multi_tag(),
        }
    }

    fn normal() -> Self {
        Self {
            tags: vec![make_tag(1, "TAG-01", [0.55, 0.38], NodeState::Normal, 8.0, true, false, false)],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn sos() -> Self {
        Self {
            tags: vec![make_tag(2, "TAG-02", [0.60, 0.44], NodeState::Sos, 3.0, true, true, false)],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn stale() -> Self {
        Self {
            tags: vec![
                make_tag(1, "TAG-01", [0.50, 0.33], NodeState::Stale,    180.0, true,  false, false),
                make_tag(3, "TAG-03", [0.65, 0.52], NodeState::VeryStale, 840.0, true,  false, true),
            ],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn no_fix() -> Self {
        let mut tag = make_tag(4, "TAG-04", [0.58, 0.47], NodeState::NoFix, 45.0, false, false, false);
        tag.track.clear();
        Self {
            tags: vec![tag],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn multi_tag() -> Self {
        let mut nf = make_tag(4, "TAG-04", [0.70, 0.48], NodeState::NoFix, 30.0, false, false, false);
        nf.track.clear();
        Self {
            tags: vec![
                make_tag(1, "TAG-01", [0.50, 0.28], NodeState::Normal,    5.0,   true,  false, false),
                make_tag(2, "TAG-02", [0.62, 0.41], NodeState::Sos,       2.0,   true,  true,  false),
                make_tag(3, "TAG-03", [0.48, 0.54], NodeState::Stale,     200.0, true,  false, true),
                nf,
            ],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }
}

fn default_relay() -> RelayData {
    RelayData { node_id: 101, label: "RELAY-01".into(), pos: [0.55, 0.60] }
}

fn default_gateway() -> GatewayData {
    GatewayData { node_id: 200, label: "GATEWAY".into(), pos: [0.30, 0.65] }
}

fn make_tag(
    node_id: u8,
    label: &str,
    pos: [f32; 2],
    state: NodeState,
    last_seen_secs: f32,
    gps_valid: bool,
    sos: bool,
    battery_low: bool,
) -> TagData {
    let track = make_track(pos);
    let sightings = make_sightings(node_id, last_seen_secs, gps_valid, sos);
    TagData {
        node_id,
        label: label.into(),
        pos,
        state,
        last_seen_secs,
        gps_valid,
        sos,
        battery_low,
        track,
        sightings,
    }
}

fn make_track(end: [f32; 2]) -> Vec<[f32; 2]> {
    // 8-point track wandering toward `end`
    let offsets: [[f32; 2]; 8] = [
        [-0.060, -0.050], [-0.048, -0.038], [-0.036, -0.030],
        [-0.026, -0.022], [-0.018, -0.015], [-0.010, -0.009],
        [-0.004, -0.003], [0.0, 0.0],
    ];
    offsets.iter().map(|o| [
        (end[0] + o[0]).clamp(0.02, 0.98),
        (end[1] + o[1]).clamp(0.02, 0.98),
    ]).collect()
}

fn make_sightings(seq_base: u8, last_seen_secs: f32, gps_valid: bool, sos: bool) -> Vec<SightingEntry> {
    (0..8u32).rev().map(|i| SightingEntry {
        seq:       (seq_base as u32) * 10 + (8 - i),
        age_secs:  last_seen_secs + i as f32 * 30.0,
        gps_valid: if i == 3 { false } else { gps_valid },
        sos:       sos && i < 2,
    }).collect()
}
