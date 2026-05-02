import { useEditor, createShapeId } from 'tldraw'
import { SarcomUiKind, UI_KIND_DEFAULTS, UI_KIND_ICONS } from '../types'

const BUTTONS: Array<{ kind: SarcomUiKind; label: string; color: string }> = [
  { kind: 'tag',         label: 'Add Tag',         color: '#22c55e' },
  { kind: 'relay',       label: 'Add Relay',        color: '#3b82f6' },
  { kind: 'gateway',     label: 'Add Gateway',      color: '#8b5cf6' },
  { kind: 'info-card',   label: 'Add Info Card',    color: '#0ea5e9' },
  { kind: 'sos-banner',  label: 'Add SOS Banner',   color: '#ef4444' },
  { kind: 'side-panel',  label: 'Add Side Panel',   color: '#475569' },
  { kind: 'no-fix-list', label: 'Add No-Fix List',  color: '#78716c' },
  { kind: 'scale-bar',   label: 'Add Scale Bar',    color: '#0f172a' },
  { kind: 'north-arrow', label: 'Add North Arrow',  color: '#0f172a' },
]

export function Toolbar() {
  const editor = useEditor()

  function addShape(kind: SarcomUiKind) {
    const d = UI_KIND_DEFAULTS[kind]
    // Place near centre of current viewport
    const vp = editor.getViewportPageBounds()
    const cx = vp.midX - d.w / 2 + (Math.random() - 0.5) * 60
    const cy = vp.midY - d.h / 2 + (Math.random() - 0.5) * 60

    editor.createShape({
      id: createShapeId(),
      type: 'sarcom',
      x: cx,
      y: cy,
      props: {
        w:              d.w,
        h:              d.h,
        ui_kind:        kind,
        label:          d.label,
        node_id:        d.node_id,
        state:          d.state,
        last_seen_text: d.last_seen_text,
        coords_text:    d.coords_text,
        battery_text:   d.battery_text,
      },
    })
  }

  return (
    <div
      className="sarcom-panel"
      style={{
        position: 'absolute',
        left: 8,
        top: 52,
        bottom: 8,
        width: 162,
        pointerEvents: 'all',
        gap: 0,
        overflowY: 'auto',
      }}
    >
      <div className="sarcom-panel-header">Shapes</div>
      <div style={{ padding: 6, display: 'flex', flexDirection: 'column', gap: 4 }}>
        {BUTTONS.map(({ kind, label, color }) => (
          <button
            key={kind}
            className="sarcom-btn"
            onClick={() => addShape(kind)}
          >
            <span style={{
              display: 'inline-flex', alignItems: 'center', justifyContent: 'center',
              width: 20, height: 20, borderRadius: 4,
              background: color, color: 'white',
              fontSize: 11, fontWeight: 800, fontFamily: 'monospace', flexShrink: 0,
            }}>
              {UI_KIND_ICONS[kind]}
            </span>
            {label}
          </button>
        ))}
      </div>

      <div className="sarcom-panel-header" style={{ marginTop: 4 }}>Tips</div>
      <div style={{ padding: '6px 10px', fontSize: 10, color: '#475569', lineHeight: 1.6 }}>
        <div>Click shape to select</div>
        <div>Drag to move</div>
        <div>Resize via handles</div>
        <div>Del to delete</div>
        <div>Ctrl+Z / Ctrl+Y</div>
        <div>Scroll to zoom</div>
        <div>Space+drag to pan</div>
      </div>
    </div>
  )
}
