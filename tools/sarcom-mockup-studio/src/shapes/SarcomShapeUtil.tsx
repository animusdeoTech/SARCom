import type { CSSProperties } from 'react'
import {
  BaseBoxShapeUtil,
  HTMLContainer,
  RecordProps,
  T,
  TLBaseShape,
} from 'tldraw'
import { SarcomShapeProps, SarcomState, STATE_COLORS } from '../types'

export type SarcomShape = TLBaseShape<'sarcom', SarcomShapeProps>

const STATE_VALS = [
  'normal', 'sos', 'stale', 'very_stale', 'no_fix', 'low_battery',
] as const

const KIND_VALS = [
  'hiker', 'relay', 'drone-relay', 'gateway',
  'info-card', 'sos-banner', 'clock-warning-banner',
  'side-panel', 'no-fix-list', 'scale-bar', 'north-arrow',
] as const

export class SarcomShapeUtil extends BaseBoxShapeUtil<SarcomShape> {
  static override type = 'sarcom' as const

  static override props: RecordProps<SarcomShape> = {
    w:              T.positiveNumber,
    h:              T.positiveNumber,
    ui_kind:        T.literalEnum(...KIND_VALS),
    label:          T.string,
    node_id:        T.string,
    state:          T.literalEnum(...STATE_VALS),
    last_seen_text: T.string,
    coords_text:    T.string,
    battery_text:   T.string,
    rows_text:      T.string,
  }

  getDefaultProps(): SarcomShape['props'] {
    return {
      w:              72,
      h:              88,
      ui_kind:        'hiker',
      label:          'TAG-01',
      node_id:        '1',
      state:          'normal',
      last_seen_text: '2m 14s ago',
      coords_text:    '46.9521°N 7.4386°E',
      battery_text:   '87%',
      rows_text:      '',
    }
  }

  component(shape: SarcomShape) {
    const { w, h } = shape.props
    return (
      <HTMLContainer
        id={shape.id}
        style={{ width: w, height: h, overflow: 'hidden', pointerEvents: 'none', userSelect: 'none' }}
      >
        <Renderer {...shape.props} />
      </HTMLContainer>
    )
  }

  indicator(shape: SarcomShape) {
    return <rect width={shape.props.w} height={shape.props.h} />
  }
}

// ── Dispatcher ───────────────────────────────────────────────────────────────

function Renderer(props: SarcomShapeProps) {
  switch (props.ui_kind) {
    case 'hiker':                 return <HikerRenderer {...props} />
    case 'relay':                 return <RelayRenderer {...props} />
    case 'drone-relay':           return <DroneRelayRenderer {...props} />
    case 'gateway':               return <GatewayRenderer {...props} />
    case 'info-card':             return <InfoCardRenderer {...props} />
    case 'sos-banner':            return <SosBannerRenderer {...props} />
    case 'clock-warning-banner':  return <ClockWarningBannerRenderer {...props} />
    case 'side-panel':            return <SidePanelRenderer {...props} />
    case 'no-fix-list':           return <NoFixListRenderer {...props} />
    case 'scale-bar':             return <ScaleBarRenderer {...props} />
    case 'north-arrow':           return <NorthArrowRenderer {...props} />
    default:                      return <UnknownRenderer />
  }
}

// ── Map markers ──────────────────────────────────────────────────────────────

function HikerRenderer({ w, h, label, state, battery_text }: SarcomShapeProps) {
  const color = STATE_COLORS[state]
  const iconSize = Math.min(w, h) * 0.52

  return (
    <div style={centeredCol(w, h, { paddingTop: 6, gap: 3 })}>
      <svg width={iconSize} height={iconSize} viewBox="0 0 40 40" style={{ overflow: 'visible', flexShrink: 0 }}>
        {state === 'sos' && (
          <circle cx="20" cy="20" r="21" fill="none" stroke="#ef4444" strokeWidth="2.5" opacity="0.5" />
        )}
        <circle cx="20" cy="20" r="17" fill={color} stroke="white" strokeWidth="2.5" />
        <text x="20" y="25" textAnchor="middle" fill="white" fontSize="15" fontWeight="bold" fontFamily="monospace">H</text>
        {state === 'low_battery' && (
          <rect x="13" y="33" width="14" height="5" rx="1" fill="#eab308" stroke="white" strokeWidth="1" />
        )}
      </svg>
      <span style={markerLabel()}>{label}</span>
      {battery_text && <span style={{ fontSize: 9, color: '#475569' }}>{battery_text}</span>}
    </div>
  )
}

function RelayRenderer({ w, h, label, state }: SarcomShapeProps) {
  const color = STATE_COLORS[state]
  const iconSize = Math.min(w, h) * 0.52

  return (
    <div style={centeredCol(w, h, { paddingTop: 6, gap: 3 })}>
      <svg width={iconSize} height={iconSize} viewBox="0 0 40 40" style={{ overflow: 'visible', flexShrink: 0 }}>
        {/* antenna pole */}
        <line x1="20" y1="4" x2="20" y2="14" stroke={color} strokeWidth="2" strokeLinecap="round" />
        <line x1="14" y1="9" x2="20" y2="4"  stroke={color} strokeWidth="1.5" strokeLinecap="round" />
        <line x1="26" y1="9" x2="20" y2="4"  stroke={color} strokeWidth="1.5" strokeLinecap="round" />
        {/* diamond body */}
        <polygon points="20,14 34,27 20,40 6,27" fill={color} stroke="white" strokeWidth="2" strokeLinejoin="round" />
        <text x="20" y="32" textAnchor="middle" fill="white" fontSize="13" fontWeight="bold" fontFamily="monospace">R</text>
      </svg>
      <span style={markerLabel()}>{label}</span>
    </div>
  )
}

function DroneRelayRenderer({ w, h, label, state }: SarcomShapeProps) {
  const color = STATE_COLORS[state]
  const iconSize = Math.min(w, h) * 0.52

  return (
    <div style={centeredCol(w, h, { paddingTop: 6, gap: 3 })}>
      <svg width={iconSize} height={iconSize} viewBox="0 0 40 40" style={{ overflow: 'visible', flexShrink: 0 }}>
        {/* X-frame drone */}
        <line x1="10" y1="10" x2="30" y2="30" stroke={color} strokeWidth="2.5" strokeLinecap="round" />
        <line x1="30" y1="10" x2="10" y2="30" stroke={color} strokeWidth="2.5" strokeLinecap="round" />
        <circle cx="10" cy="10" r="6" fill={color} stroke="white" strokeWidth="1.5" />
        <circle cx="30" cy="10" r="6" fill={color} stroke="white" strokeWidth="1.5" />
        <circle cx="10" cy="30" r="6" fill={color} stroke="white" strokeWidth="1.5" />
        <circle cx="30" cy="30" r="6" fill={color} stroke="white" strokeWidth="1.5" />
        <circle cx="20" cy="20" r="5" fill={color} stroke="white" strokeWidth="1.5" />
        <text x="20" y="24" textAnchor="middle" fill="white" fontSize="8" fontWeight="bold" fontFamily="monospace">D</text>
      </svg>
      <span style={markerLabel()}>{label}</span>
    </div>
  )
}

function GatewayRenderer({ w, h, label, state }: SarcomShapeProps) {
  const color = STATE_COLORS[state]
  const iconSize = Math.min(w, h) * 0.55

  return (
    <div style={centeredCol(w, h, { paddingTop: 4, gap: 3 })}>
      <svg width={iconSize} height={iconSize} viewBox="0 0 40 46" style={{ overflow: 'visible', flexShrink: 0 }}>
        <line x1="20" y1="16" x2="20" y2="5"  stroke={color} strokeWidth="2.5" strokeLinecap="round" />
        <line x1="15" y1="10" x2="20" y2="5"  stroke={color} strokeWidth="1.5" strokeLinecap="round" />
        <line x1="25" y1="10" x2="20" y2="5"  stroke={color} strokeWidth="1.5" strokeLinecap="round" />
        <rect x="5" y="16" width="30" height="22" rx="3" fill={color} stroke="white" strokeWidth="2" />
        <text x="20" y="31" textAnchor="middle" fill="white" fontSize="13" fontWeight="bold" fontFamily="monospace">G</text>
      </svg>
      <span style={markerLabel()}>{label}</span>
    </div>
  )
}

// ── Overlay shapes ────────────────────────────────────────────────────────────

function InfoCardRenderer({ w, h, label, node_id, state, last_seen_text, coords_text, battery_text }: SarcomShapeProps) {
  const accent = STATE_COLORS[state]

  return (
    <div style={{
      width: w, height: h,
      background: 'white',
      border: `1px solid #e2e8f0`,
      borderLeftWidth: 4,
      borderLeftColor: accent,
      borderRadius: 6,
      boxShadow: '0 1px 8px rgba(0,0,0,0.18)',
      display: 'flex', flexDirection: 'column', overflow: 'hidden',
    }}>
      <div style={{ background: accent, padding: '4px 8px', display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexShrink: 0 }}>
        <span style={{ fontWeight: 700, color: 'white', fontSize: 12 }}>{label}</span>
        <span style={{ color: 'rgba(255,255,255,0.85)', fontSize: 9, fontFamily: 'monospace' }}>#{node_id}</span>
      </div>
      <div style={{ padding: '6px 8px', display: 'flex', flexDirection: 'column', gap: 4, flex: 1 }}>
        <Row label="Last heard" value={last_seen_text} />
        <Row label="Coords"     value={coords_text} mono />
        <Row label="Battery"    value={battery_text} />
      </div>
    </div>
  )
}

function SosBannerRenderer({ w, h, node_id, label, last_seen_text, coords_text }: SarcomShapeProps) {
  return (
    <div style={{
      width: w, height: h,
      background: 'linear-gradient(135deg, #b91c1c 0%, #ef4444 100%)',
      borderRadius: 4,
      boxShadow: '0 2px 16px rgba(185,28,28,0.55)',
      border: '2px solid #fca5a5',
      display: 'flex', alignItems: 'center', justifyContent: 'space-between',
      padding: '0 16px',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 14 }}>
        <span style={{ fontSize: 22, fontWeight: 900, color: 'white', letterSpacing: '0.18em', textShadow: '0 0 20px rgba(255,255,255,0.4)' }}>SOS</span>
        <div style={{ width: 1, height: 28, background: 'rgba(255,255,255,0.3)' }} />
        <span style={{ color: 'white', fontWeight: 700, fontSize: 13, fontFamily: 'monospace' }}>#{node_id} {label}</span>
      </div>
      <div style={{ color: 'rgba(255,255,255,0.92)', fontSize: 11, textAlign: 'right', lineHeight: 1.6 }}>
        <div style={{ fontFamily: 'monospace', fontSize: 10 }}>{coords_text}</div>
        <div>{last_seen_text}</div>
      </div>
    </div>
  )
}

function ClockWarningBannerRenderer({ w, h, label }: SarcomShapeProps) {
  return (
    <div style={{
      width: w, height: h,
      background: '#78350f',
      border: '2px solid #f59e0b',
      borderRadius: 4,
      display: 'flex', alignItems: 'center',
      padding: '0 14px', gap: 12,
    }}>
      <span style={{ fontSize: 14, fontWeight: 900, color: '#fbbf24', letterSpacing: '0.1em', flexShrink: 0 }}>RTC</span>
      <div style={{ width: 1, height: 24, background: 'rgba(251,191,36,0.3)', flexShrink: 0 }} />
      <span style={{ color: '#fde68a', fontSize: 11, fontWeight: 600 }}>
        {label || 'Clock not set — relative times unavailable'}
      </span>
    </div>
  )
}

function SidePanelRenderer({ w, h, label, rows_text }: SarcomShapeProps) {
  const rows = rows_text.split('\n').filter(r => r.trim())

  return (
    <div style={{ width: w, height: h, background: '#0f172a', borderRadius: 6, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
      <div style={{ padding: '7px 12px', background: '#1e293b', fontSize: 10, fontWeight: 700, color: '#94a3b8', letterSpacing: '0.1em', textTransform: 'uppercase', flexShrink: 0 }}>
        {label}
      </div>
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        {rows.length > 0
          ? rows.map((row, i) => (
            <div key={i} style={{
              padding: '5px 12px',
              borderBottom: '1px solid #1e293b',
              fontSize: 10,
              color: '#e2e8f0',
              fontFamily: 'monospace',
              lineHeight: 1.4,
              overflow: 'hidden',
              whiteSpace: 'nowrap',
              textOverflow: 'ellipsis',
            }}>{row}</div>
          ))
          : <div style={{ padding: '8px 12px', fontSize: 10, color: '#334155' }}>No rows — edit rows_text</div>
        }
      </div>
    </div>
  )
}

function NoFixListRenderer({ w, h, label, rows_text }: SarcomShapeProps) {
  const rows = rows_text.split('\n').filter(r => r.trim())

  return (
    <div style={{ width: w, height: h, background: 'white', border: '1px solid #e2e8f0', borderRadius: 6, overflow: 'hidden' }}>
      <div style={{ padding: '5px 10px', background: '#78716c', color: 'white', fontSize: 10, fontWeight: 700, letterSpacing: '0.05em' }}>
        {label}
      </div>
      <div style={{ padding: '5px 10px', display: 'flex', flexDirection: 'column', gap: 4 }}>
        {rows.length > 0
          ? rows.map((row, i) => (
            <div key={i} style={{ display: 'flex', alignItems: 'center', gap: 7, fontSize: 10, color: '#57534e' }}>
              <div style={{ width: 6, height: 6, borderRadius: '50%', background: '#9ca3af', flexShrink: 0 }} />
              <span style={{ fontFamily: 'monospace' }}>{row}</span>
            </div>
          ))
          : <div style={{ fontSize: 10, color: '#9ca3af' }}>No entries</div>
        }
      </div>
    </div>
  )
}

function ScaleBarRenderer({ w, h, label }: SarcomShapeProps) {
  const seg = (w - 20) / 2
  const dist = parseInt(label) || 100

  return (
    <div style={{ width: w, height: h, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <svg width={w - 8} height={32} viewBox={`0 0 ${w - 8} 32`}>
        <rect x="0" y="3" width={w - 8} height="26" fill="rgba(255,255,255,0.82)" rx="2" />
        <rect x="4" y="10" width={seg} height="8" fill="#1e293b" />
        <rect x={4 + seg} y="10" width={seg} height="8" fill="white" stroke="#1e293b" strokeWidth="1" />
        <rect x="4" y="10" width={seg * 2} height="8" fill="none" stroke="#1e293b" strokeWidth="1" />
        <text x="4"        y="28" fontSize="8" fill="#1e293b" fontFamily="sans-serif">0</text>
        <text x={4 + seg}  y="28" fontSize="8" fill="#1e293b" fontFamily="sans-serif" textAnchor="middle">{Math.floor(dist / 2)}m</text>
        <text x={4 + seg * 2} y="28" fontSize="8" fill="#1e293b" fontFamily="sans-serif" textAnchor="end">{dist}m</text>
      </svg>
    </div>
  )
}

function NorthArrowRenderer({ w, h, label }: SarcomShapeProps) {
  const sz = Math.min(w, h - 20) * 0.8

  return (
    <div style={centeredCol(w, h, { gap: 2 })}>
      <svg width={sz} height={sz * 1.15} viewBox="0 0 40 50" style={{ flexShrink: 0 }}>
        <polygon points="20,3 30,40 20,33 10,40" fill="#1e293b" stroke="white" strokeWidth="1.5" strokeLinejoin="round" />
        <polygon points="20,3 10,40 20,33"        fill="white"  stroke="#1e293b" strokeWidth="1"   strokeLinejoin="round" />
        <circle cx="20" cy="33" r="4" fill="#1e293b" stroke="white" strokeWidth="1.5" />
      </svg>
      <span style={{ fontSize: 13, fontWeight: 800, color: '#1e293b', textShadow: '0 0 5px white,0 0 5px white', letterSpacing: '0.12em' }}>
        {label}
      </span>
    </div>
  )
}

function UnknownRenderer() {
  return <div style={{ padding: 8, fontSize: 10, color: '#ef4444' }}>Unknown ui_kind</div>
}

// ── Helpers ───────────────────────────────────────────────────────────────────

function centeredCol(w: number, h: number, extra?: CSSProperties): CSSProperties {
  return { width: w, height: h, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', ...extra }
}

function markerLabel(): CSSProperties {
  return { fontSize: 11, fontWeight: 700, color: '#1e293b', textShadow: '0 0 5px white,0 0 5px white', textAlign: 'center', lineHeight: 1.1 }
}

function Row({ label, value, mono }: { label: string; value: string; mono?: boolean }) {
  return (
    <div style={{ fontSize: 10, color: '#475569', display: 'flex', gap: 4, alignItems: 'baseline' }}>
      <span style={{ minWidth: 62, color: '#94a3b8', fontSize: 9, flexShrink: 0 }}>{label}:</span>
      <span style={{ fontFamily: mono ? 'monospace' : 'inherit', fontSize: mono ? 9 : 10, wordBreak: 'break-all' }}>{value}</span>
    </div>
  )
}
