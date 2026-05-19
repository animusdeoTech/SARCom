use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Inventory kind — the ONLY distinction between tag / relay / gateway in v1a.
/// Maps `node_id → NodeKind` for icon glyph + colour assignment in the UI.
/// Per dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md: this is the
/// gateway-internal inventory table (SQLite-backed in the real gateway binary),
/// modelled here as a HashMap. Not part of the wire protocol; not a field on
/// NodeData (which carries only POSITION-derived state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Tag,
    Relay,
    Gateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SightingEntry {
    pub seq: u32,
    pub age_secs: f32,
    pub gps_valid: bool,
    pub sos: bool,
}

/// Uniform node shape: POSITION-derived state for any node in the network.
/// Per dev-log/2026-05-19-v1a-ui-data-model-collapse-nodedata.md, the v1a UI
/// data model is one uniform NodeData. Tag vs Relay vs Gateway is an inventory
/// distinction (see [`NodeKind`] + `SimState.inventory`), not a struct split.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
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
pub struct SimState {
    /// All nodes in the network — tags, relays, gateway — uniform NodeData.
    pub nodes: Vec<NodeData>,
    /// node_id → NodeKind mapping (inventory; presentation-only).
    pub inventory: HashMap<u8, NodeKind>,
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

    /// Inventory kind lookup by node_id.
    pub fn kind_for_id(&self, node_id: u8) -> NodeKind {
        self.inventory
            .get(&node_id)
            .copied()
            .unwrap_or(NodeKind::Tag)
    }

    fn with_infra(mut tags: Vec<NodeData>) -> Self {
        let relay = default_relay_node();
        let gateway = default_gateway_node();
        let mut inventory = HashMap::new();
        for t in &tags {
            inventory.insert(t.node_id, NodeKind::Tag);
        }
        inventory.insert(relay.node_id, NodeKind::Relay);
        inventory.insert(gateway.node_id, NodeKind::Gateway);
        tags.push(relay);
        tags.push(gateway);
        Self {
            nodes: tags,
            inventory,
        }
    }

    fn normal() -> Self {
        Self::with_infra(vec![make_tag(
            1,
            "tag-1",
            [0.55, 0.38],
            NodeState::Normal,
            8.0,
            true,
            false,
            false,
        )])
    }

    fn sos() -> Self {
        Self::with_infra(vec![make_tag(
            2,
            "tag-2",
            [0.60, 0.44],
            NodeState::Sos,
            42.0,
            true,
            true,
            false,
        )])
    }

    fn stale() -> Self {
        Self::with_infra(vec![
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
        ])
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
        Self::with_infra(vec![tag])
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
        Self::with_infra(vec![
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
        ])
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
        Self::with_infra(vec![tag])
    }
}

/// Relay node — same NodeData shape as a tag, populated as a relay broadcast.
/// 840 s = 14 min, well within relay's ~1800 s POSITION cadence per ADR-006.
fn default_relay_node() -> NodeData {
    NodeData {
        node_id: 101,
        label: "relay-1".into(),
        pos: [0.55, 0.60],
        state: NodeState::Normal,
        last_seen_secs: 840.0,
        gps_valid: true,
        sos: false,
        battery_low: false,
        track: Vec::new(),
        sightings: Vec::new(),
        last_valid_fix_pos: None,
        last_valid_fix_age_secs: None,
    }
}

/// Gateway node — local; `last_seen_secs = 0.0` sentinel (gateway is the
/// receiver and doesn't receive its own frames). UI presentation may render
/// this as `— (local)` rather than `0 s` per KIOSK-004 / KIOSK-003.
fn default_gateway_node() -> NodeData {
    NodeData {
        node_id: 200,
        label: "gw-0".into(),
        pos: [0.30, 0.65],
        state: NodeState::Normal,
        last_seen_secs: 0.0,
        gps_valid: true,
        sos: false,
        battery_low: false,
        track: Vec::new(),
        sightings: Vec::new(),
        last_valid_fix_pos: None,
        last_valid_fix_age_secs: None,
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
) -> NodeData {
    let track = if gps_valid {
        make_track(pos)
    } else {
        Vec::new()
    };
    let cadence = if sos { 50.0_f32 } else { 330.0 };
    NodeData {
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
        // Find the SOS tag (post-collapse: nodes vec includes infra too)
        let tag = s
            .nodes
            .iter()
            .find(|n| s.kind_for_id(n.node_id) == NodeKind::Tag && n.sos)
            .expect("SosNoFix scenario must contain an SOS tag");
        assert!(tag.sos);
        assert!(!tag.gps_valid);
        assert!(tag.last_valid_fix_pos.is_some());
        assert!(tag.last_valid_fix_age_secs.is_some());
    }

    // Post-collapse invariant: every node in SimState.nodes has an inventory entry.
    #[test]
    fn every_node_has_inventory_entry() {
        for scenario in ScenarioKind::all() {
            let s = SimState::from_scenario(*scenario);
            for node in &s.nodes {
                assert!(
                    s.inventory.contains_key(&node.node_id),
                    "scenario {:?}: node_id {} missing from inventory",
                    scenario,
                    node.node_id
                );
            }
        }
    }

    // Post-collapse invariant: each scenario has exactly one relay + one gateway.
    #[test]
    fn each_scenario_has_one_relay_and_one_gateway() {
        for scenario in ScenarioKind::all() {
            let s = SimState::from_scenario(*scenario);
            let n_relays = s
                .nodes
                .iter()
                .filter(|n| s.kind_for_id(n.node_id) == NodeKind::Relay)
                .count();
            let n_gateways = s
                .nodes
                .iter()
                .filter(|n| s.kind_for_id(n.node_id) == NodeKind::Gateway)
                .count();
            assert_eq!(n_relays, 1, "scenario {:?}: expected 1 relay", scenario);
            assert_eq!(n_gateways, 1, "scenario {:?}: expected 1 gateway", scenario);
        }
    }
}
