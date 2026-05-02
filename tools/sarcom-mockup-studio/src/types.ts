export type SarcomState =
  | 'normal'
  | 'sos'
  | 'stale'
  | 'very_stale'
  | 'no_fix'
  | 'low_battery'

// Vocabulary matches nodes.toml ui_kind values from ARCHITECTURE.md §11
export type SarcomUiKind =
  | 'hiker'          // was "tag" — operator language from nodes.toml
  | 'relay'          // static solar relay
  | 'drone-relay'    // mobile relay (v1b drone-pod)
  | 'gateway'        // the Pi base station
  | 'info-card'      // selected entity detail card (right panel)
  | 'sos-banner'     // full-width SOS alert strip
  | 'clock-warning-banner'  // RTC not set — relative times unavailable
  | 'side-panel'     // entity list (data from rows_text)
  | 'no-fix-list'    // no-GPS-fix entries (data from rows_text)
  | 'scale-bar'
  | 'north-arrow'

export interface SarcomShapeProps {
  w: number
  h: number
  ui_kind: SarcomUiKind
  label: string
  // node_id: plain integer string matching u8 wire range 0–254 (e.g. "1", "101", "200")
  node_id: string
  state: SarcomState
  last_seen_text: string
  coords_text: string
  battery_text: string
  // Data-driven row content for side-panel and no-fix-list.
  // Newline-separated plain text rows; ignored by other shapes.
  rows_text: string
}

export const STATE_COLORS: Record<SarcomState, string> = {
  normal:      '#22c55e',
  sos:         '#ef4444',
  stale:       '#f97316',
  very_stale:  '#b45309',
  no_fix:      '#6b7280',
  low_battery: '#eab308',
}

export const STATE_LABELS: Record<SarcomState, string> = {
  normal:      'Normal',
  sos:         'SOS',
  stale:       'Stale (>2 min)',
  very_stale:  'Very stale (>10 min)',
  no_fix:      'No GPS fix',
  low_battery: 'Low battery',
}

export const UI_KIND_ICONS: Record<SarcomUiKind, string> = {
  'hiker':                 'H',
  'relay':                 'R',
  'drone-relay':           'D',
  'gateway':               'G',
  'info-card':             'I',
  'sos-banner':            '!',
  'clock-warning-banner':  'T',
  'side-panel':            'P',
  'no-fix-list':           'X',
  'scale-bar':             '—',
  'north-arrow':           'N',
}

export interface UiKindDefault {
  w: number
  h: number
  label: string
  node_id: string
  state: SarcomState
  last_seen_text: string
  coords_text: string
  battery_text: string
  rows_text: string
}

export const UI_KIND_DEFAULTS: Record<SarcomUiKind, UiKindDefault> = {
  'hiker': {
    w: 72, h: 88,
    label: 'TAG-01', node_id: '1', state: 'normal',
    last_seen_text: '2m 14s ago',
    coords_text: '46.9521°N 7.4386°E',
    battery_text: '87%',
    rows_text: '',
  },
  'relay': {
    w: 72, h: 88,
    label: 'RELAY-01', node_id: '101', state: 'normal',
    last_seen_text: '5m 08s ago',
    coords_text: '46.9530°N 7.4400°E',
    battery_text: '72%',
    rows_text: '',
  },
  'drone-relay': {
    w: 80, h: 88,
    label: 'DRONE-01', node_id: '201', state: 'normal',
    last_seen_text: '1m 44s ago',
    coords_text: '46.9535°N 7.4395°E',
    battery_text: '61%',
    rows_text: '',
  },
  'gateway': {
    w: 80, h: 96,
    label: 'GATEWAY', node_id: '200', state: 'normal',
    last_seen_text: '',
    coords_text: '46.9510°N 7.4365°E',
    battery_text: '',
    rows_text: '',
  },
  'info-card': {
    w: 220, h: 140,
    label: 'TAG-01', node_id: '1', state: 'normal',
    last_seen_text: '2m 14s ago',
    coords_text: '46.9521°N 7.4386°E',
    battery_text: '87%',
    rows_text: '',
  },
  'sos-banner': {
    w: 760, h: 56,
    label: 'SOS ALERT', node_id: '2', state: 'sos',
    last_seen_text: '3s ago',
    coords_text: '46.9519°N 7.4401°E',
    battery_text: '',
    rows_text: '',
  },
  'clock-warning-banner': {
    w: 760, h: 48,
    label: 'Clock not set — relative times unavailable', node_id: '', state: 'no_fix',
    last_seen_text: '',
    coords_text: '',
    battery_text: '',
    rows_text: '',
  },
  'side-panel': {
    w: 200, h: 460,
    label: 'Active tags', node_id: '', state: 'normal',
    last_seen_text: '',
    coords_text: '',
    battery_text: '',
    rows_text: '1  TAG-01  2m 14s ago  normal\n2  TAG-02  4m 52s ago  normal\n3  TAG-03  6m 20s ago  stale\n4  TAG-04  30s ago  no_fix',
  },
  'no-fix-list': {
    w: 200, h: 130,
    label: 'No GPS fix', node_id: '', state: 'no_fix',
    last_seen_text: '',
    coords_text: '',
    battery_text: '',
    rows_text: '4  TAG-04\n7  TAG-07',
  },
  'scale-bar': {
    w: 200, h: 40,
    label: '100m', node_id: '', state: 'normal',
    last_seen_text: '',
    coords_text: '',
    battery_text: '',
    rows_text: '',
  },
  'north-arrow': {
    w: 56, h: 72,
    label: 'N', node_id: '', state: 'normal',
    last_seen_text: '',
    coords_text: '',
    battery_text: '',
    rows_text: '',
  },
}

export interface SarcomMockupFile {
  version: 1
  name: string
  description?: string
  shapes: SarcomMockupShape[]
}

export interface SarcomMockupShape {
  type: 'sarcom'
  x: number
  y: number
  rotation?: number
  props: SarcomShapeProps
}
