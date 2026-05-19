use serde::{Deserialize, Serialize};

// Cadence-derived freshness. Normal heartbeat cadence 300–330 s per ADR-013.
// SOS cadence 45–60 s jittered per ADR-010.
// Relay POSITION cadence ~1800 s per ADR-006. Same packet kind as tag
// POSITION per ADR-013 — there is no separate "self-announce" frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Freshness {
    Fresh,
    Aging,
    Stale,
    VeryStale,
}

/// fresh < 330 s · aging < 660 s · stale < 1320 s · very stale >= 1320 s.
/// SOS path: stale if last frame > 180 s (3× expected cadence).
pub fn freshness_for_tag(age_secs: f32, sos: bool) -> Freshness {
    if sos {
        // SOS only has two states in this lab model: Fresh (link healthy)
        // and VeryStale (distress link stale). `VeryStale` here is reused
        // as the colour bucket for "SOS stale", NOT the heartbeat
        // very-stale meaning. If a future UI ever surfaces the freshness
        // enum as user-facing label text, split this into a dedicated
        // `Freshness::SosStale` variant first.
        if age_secs < 180.0 {
            Freshness::Fresh
        } else {
            Freshness::VeryStale
        }
    } else if age_secs < 330.0 {
        Freshness::Fresh
    } else if age_secs < 660.0 {
        Freshness::Aging
    } else if age_secs < 1320.0 {
        Freshness::Stale
    } else {
        Freshness::VeryStale
    }
}

/// Relay POSITION cadence ~1800 s; don't penalise at tag thresholds.
pub fn freshness_for_relay(age_secs: f32) -> Freshness {
    if age_secs < 1800.0 {
        Freshness::Fresh
    } else if age_secs < 3600.0 {
        Freshness::Aging
    } else {
        Freshness::Stale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScenarioKind {
    Normal,
    Sos,
    Stale,
    NoFix,
    MultiTag,
    SosNoFix,
}

impl ScenarioKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Sos => "SOS",
            Self::Stale => "Stale",
            Self::NoFix => "No Fix",
            Self::MultiTag => "Multi-Tag",
            Self::SosNoFix => "SOS + No Fix",
        }
    }

    pub fn all() -> &'static [ScenarioKind] {
        &[
            Self::Normal,
            Self::Sos,
            Self::Stale,
            Self::NoFix,
            Self::MultiTag,
            Self::SosNoFix,
        ]
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
            Self::Normal => "Normal",
            Self::Stale => "Stale (>11 min)",
            Self::VeryStale => "Very stale (>22 min)",
            Self::NoFix => "No GPS fix",
            Self::Sos => "SOS",
            Self::LowBattery => "Low battery",
        }
    }

    pub fn all() -> &'static [NodeState] {
        &[
            Self::Normal,
            Self::Stale,
            Self::VeryStale,
            Self::NoFix,
            Self::Sos,
            Self::LowBattery,
        ]
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
    /// Normalised [0,1] position. Only valid when gps_valid=true.
    /// Do NOT render as a current-position map marker when gps_valid=false.
    pub pos: [f32; 2],
    pub state: NodeState,
    pub last_seen_secs: f32,
    pub gps_valid: bool,
    pub sos: bool,
    pub battery_low: bool,
    pub track: Vec<[f32; 2]>,
    pub sightings: Vec<SightingEntry>,
    /// Last position with GPS_VALID=1. Rendered as ghost marker when gps_valid=false.
    #[serde(default)]
    pub last_valid_fix_pos: Option<[f32; 2]>,
    /// Age in seconds of last_valid_fix_pos. None when clock is invalid.
    #[serde(default)]
    pub last_valid_fix_age_secs: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
    /// Age of the most recent frame from this relay (POSITION; one packet type per ADR-013).
    #[serde(default)]
    pub last_seen_secs: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayData {
    pub node_id: u8,
    pub label: String,
    pub pos: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub tags: Vec<TagData>,
    pub relay: RelayData,
    pub gateway: GatewayData,
}

impl SimState {
    pub fn from_scenario(kind: ScenarioKind) -> Self {
        match kind {
            ScenarioKind::Normal => Self::normal(),
            ScenarioKind::Sos => Self::sos(),
            ScenarioKind::Stale => Self::stale(),
            ScenarioKind::NoFix => Self::no_fix(),
            ScenarioKind::MultiTag => Self::multi_tag(),
            ScenarioKind::SosNoFix => Self::sos_no_fix(),
        }
    }

    fn normal() -> Self {
        Self {
            tags: vec![make_tag(
                1,
                "tag-1",
                [0.55, 0.38],
                NodeState::Normal,
                8.0,
                true,
                false,
                false,
            )],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn sos() -> Self {
        Self {
            // 42 s: within the 45–60 s SOS cadence window
            tags: vec![make_tag(
                2,
                "tag-2",
                [0.60, 0.44],
                NodeState::Sos,
                42.0,
                true,
                true,
                false,
            )],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn stale() -> Self {
        Self {
            tags: vec![
                // 700 s crosses the 660 s aging→stale threshold
                make_tag(
                    1,
                    "tag-1",
                    [0.50, 0.33],
                    NodeState::Stale,
                    700.0,
                    true,
                    false,
                    false,
                ),
                // 1400 s crosses the 1320 s stale→very-stale threshold
                make_tag(
                    3,
                    "tag-3",
                    [0.65, 0.52],
                    NodeState::VeryStale,
                    1400.0,
                    true,
                    false,
                    true,
                ),
            ],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn no_fix() -> Self {
        let mut tag = make_tag(
            4,
            "tag-4",
            [0.58, 0.47],
            NodeState::NoFix,
            45.0,
            false,
            false,
            false,
        );
        tag.last_valid_fix_pos = Some([0.57, 0.45]);
        tag.last_valid_fix_age_secs = Some(480.0);
        Self {
            tags: vec![tag],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    fn multi_tag() -> Self {
        let mut nf = make_tag(
            4,
            "tag-4",
            [0.70, 0.48],
            NodeState::NoFix,
            30.0,
            false,
            false,
            false,
        );
        nf.last_valid_fix_pos = Some([0.69, 0.46]);
        nf.last_valid_fix_age_secs = Some(210.0);
        Self {
            tags: vec![
                make_tag(
                    1,
                    "tag-1",
                    [0.50, 0.28],
                    NodeState::Normal,
                    5.0,
                    true,
                    false,
                    false,
                ),
                make_tag(
                    2,
                    "tag-2",
                    [0.62, 0.41],
                    NodeState::Sos,
                    42.0,
                    true,
                    true,
                    false,
                ),
                make_tag(
                    3,
                    "tag-3",
                    [0.48, 0.54],
                    NodeState::Stale,
                    700.0,
                    true,
                    false,
                    true,
                ),
                nf,
            ],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }

    /// SOS active, current report GPS_VALID=0, previous valid fix available.
    /// `pos` is NOT rendered as a map marker. Ghost is drawn at last_valid_fix_pos.
    fn sos_no_fix() -> Self {
        let mut tag = make_tag(
            2,
            "tag-2",
            [0.60, 0.44],
            NodeState::Sos,
            55.0,
            false,
            true,
            false,
        );
        tag.last_valid_fix_pos = Some([0.58, 0.42]);
        tag.last_valid_fix_age_secs = Some(380.0);
        Self {
            tags: vec![tag],
            relay: default_relay(),
            gateway: default_gateway(),
        }
    }
}

fn default_relay() -> RelayData {
    RelayData {
        node_id: 101,
        label: "relay-1".into(),
        pos: [0.55, 0.60],
        last_seen_secs: Some(840.0), // 14 min: well within relay's ~1800 s POSITION cadence
    }
}

fn default_gateway() -> GatewayData {
    GatewayData {
        node_id: 200,
        label: "gw-0".into(),
        pos: [0.30, 0.65],
    }
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
    let track = if gps_valid {
        make_track(pos)
    } else {
        Vec::new()
    };
    let cadence = if sos { 50.0_f32 } else { 330.0 };
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
        sightings: make_sightings(node_id, last_seen_secs, gps_valid, sos, cadence),
        last_valid_fix_pos: None,
        last_valid_fix_age_secs: None,
    }
}

fn make_track(end: [f32; 2]) -> Vec<[f32; 2]> {
    let offsets: [[f32; 2]; 8] = [
        [-0.060, -0.050],
        [-0.048, -0.038],
        [-0.036, -0.030],
        [-0.026, -0.022],
        [-0.018, -0.015],
        [-0.010, -0.009],
        [-0.004, -0.003],
        [0.0, 0.0],
    ];
    offsets
        .iter()
        .map(|o| {
            [
                (end[0] + o[0]).clamp(0.02, 0.98),
                (end[1] + o[1]).clamp(0.02, 0.98),
            ]
        })
        .collect()
}

fn make_sightings(
    seq_base: u8,
    last_seen_secs: f32,
    gps_valid: bool,
    sos: bool,
    cadence: f32,
) -> Vec<SightingEntry> {
    (0..8u32)
        .rev()
        .map(|i| SightingEntry {
            seq: (seq_base as u32) * 10 + (8 - i),
            age_secs: last_seen_secs + i as f32 * cadence,
            gps_valid: if i == 3 { false } else { gps_valid },
            sos: sos && i < 2,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Heartbeat cadence 300–330 s. A healthy tag must NOT flag stale before
    // the next expected frame. These boundary checks lock that in.
    #[test]
    fn tag_freshness_thresholds_match_heartbeat_cadence() {
        assert_eq!(freshness_for_tag(0.0, false), Freshness::Fresh);
        assert_eq!(freshness_for_tag(329.0, false), Freshness::Fresh);
        assert_eq!(freshness_for_tag(330.0, false), Freshness::Aging);
        assert_eq!(freshness_for_tag(659.0, false), Freshness::Aging);
        assert_eq!(freshness_for_tag(660.0, false), Freshness::Stale);
        assert_eq!(freshness_for_tag(1319.0, false), Freshness::Stale);
        assert_eq!(freshness_for_tag(1320.0, false), Freshness::VeryStale);
        assert_eq!(freshness_for_tag(99999.0, false), Freshness::VeryStale);
    }

    // SOS cadence 45–60 s; stale at >180 s (3× expected).
    #[test]
    fn sos_freshness_uses_separate_threshold() {
        assert_eq!(freshness_for_tag(60.0, true), Freshness::Fresh);
        assert_eq!(freshness_for_tag(179.0, true), Freshness::Fresh);
        assert_eq!(freshness_for_tag(180.0, true), Freshness::VeryStale);
        // SOS at 600s would be "Aging" under tag rules; must be VeryStale here.
        assert_eq!(freshness_for_tag(600.0, true), Freshness::VeryStale);
    }

    // Relay POSITION cadence ~1800 s — must NOT use tag thresholds.
    #[test]
    fn relay_freshness_does_not_borrow_tag_thresholds() {
        // 14 min is "very stale" for a tag, but Fresh for a relay.
        assert_eq!(freshness_for_relay(840.0), Freshness::Fresh);
        assert_eq!(freshness_for_relay(1799.0), Freshness::Fresh);
        assert_eq!(freshness_for_relay(1800.0), Freshness::Aging);
        assert_eq!(freshness_for_relay(3600.0), Freshness::Stale);
    }

    // SOS+NoFix: current pos must NOT be advertised as a valid fix.
    // Ghost rendering relies on (gps_valid=false, last_valid_fix_pos=Some).
    #[test]
    fn sos_no_fix_scenario_has_ghost_data() {
        let s = SimState::from_scenario(ScenarioKind::SosNoFix);
        let tag = &s.tags[0];
        assert!(tag.sos);
        assert!(!tag.gps_valid);
        assert!(tag.last_valid_fix_pos.is_some());
        assert!(tag.last_valid_fix_age_secs.is_some());
    }
}
