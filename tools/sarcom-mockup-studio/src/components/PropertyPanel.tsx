import type { CSSProperties, ReactNode } from 'react'
import { useEditor, useValue } from 'tldraw'
import { SarcomShape } from '../shapes/SarcomShapeUtil'
import {
  SarcomState, SarcomUiKind,
  STATE_LABELS, UI_KIND_ICONS,
} from '../types'

const STATES: SarcomState[] = ['normal', 'sos', 'stale', 'very_stale', 'no_fix', 'low_battery']
const KINDS: SarcomUiKind[] = [
  'tag', 'relay', 'gateway', 'info-card', 'sos-banner',
  'side-panel', 'no-fix-list', 'scale-bar', 'north-arrow',
]

export function PropertyPanel() {
  const editor = useEditor()

  const shape = useValue<SarcomShape | null>(
    'selected sarcom',
    () => {
      const shapes = editor.getSelectedShapes()
      if (shapes.length === 1 && shapes[0].type === 'sarcom') {
        return shapes[0] as SarcomShape
      }
      return null
    },
    [editor],
  )

  if (!shape) {
    return (
      <div
        className="sarcom-panel"
        style={{ position: 'absolute', right: 8, top: 52, bottom: 8, width: 220, pointerEvents: 'all' }}
      >
        <div className="sarcom-panel-header">Properties</div>
        <div style={{ padding: 12, fontSize: 11, color: '#475569', lineHeight: 1.7 }}>
          Select a SARCOM shape to edit its properties.
        </div>
      </div>
    )
  }

  const p = shape.props

  function set<K extends keyof SarcomShape['props']>(key: K, val: SarcomShape['props'][K]) {
    editor.updateShape<SarcomShape>({ id: shape!.id, type: 'sarcom', props: { [key]: val } as Partial<SarcomShape['props']> })
  }

  return (
    <div
      className="sarcom-panel"
      style={{ position: 'absolute', right: 8, top: 52, bottom: 8, width: 220, pointerEvents: 'all', overflowY: 'auto' }}
    >
      <div className="sarcom-panel-header">Properties</div>
      <div style={{ padding: '8px 10px', display: 'flex', flexDirection: 'column', gap: 10 }}>

        <Field label="ui_kind">
          <select
            className="sarcom-prop-select"
            value={p.ui_kind}
            onChange={e => set('ui_kind', e.target.value as SarcomUiKind)}
          >
            {KINDS.map(k => (
              <option key={k} value={k}>{UI_KIND_ICONS[k]}  {k}</option>
            ))}
          </select>
        </Field>

        <Field label="label">
          <input
            className="sarcom-prop-input"
            value={p.label}
            onChange={e => set('label', e.target.value)}
          />
        </Field>

        <Field label="node_id">
          <input
            className="sarcom-prop-input"
            value={p.node_id}
            onChange={e => set('node_id', e.target.value)}
          />
        </Field>

        <Field label="state">
          <select
            className="sarcom-prop-select"
            value={p.state}
            onChange={e => set('state', e.target.value as SarcomState)}
          >
            {STATES.map(s => (
              <option key={s} value={s}>{STATE_LABELS[s]}</option>
            ))}
          </select>
        </Field>

        <Field label="last_seen_text">
          <input
            className="sarcom-prop-input"
            value={p.last_seen_text}
            onChange={e => set('last_seen_text', e.target.value)}
            placeholder="e.g. 5s ago"
          />
        </Field>

        <Field label="coords_text">
          <input
            className="sarcom-prop-input"
            value={p.coords_text}
            onChange={e => set('coords_text', e.target.value)}
            placeholder="e.g. 46.9521°N 7.4386°E"
          />
        </Field>

        <Field label="battery_text">
          <input
            className="sarcom-prop-input"
            value={p.battery_text}
            onChange={e => set('battery_text', e.target.value)}
            placeholder="e.g. 87%"
          />
        </Field>

        <div style={{ borderTop: '1px solid #1e293b', paddingTop: 8 }}>
          <div className="sarcom-panel-header" style={{ marginBottom: 6, padding: 0 }}>Size</div>
          <div style={{ display: 'flex', gap: 6 }}>
            <Field label="w" style={{ flex: 1 }}>
              <input
                className="sarcom-prop-input"
                type="number"
                min={20}
                value={p.w}
                onChange={e => set('w', Math.max(20, Number(e.target.value)))}
              />
            </Field>
            <Field label="h" style={{ flex: 1 }}>
              <input
                className="sarcom-prop-input"
                type="number"
                min={20}
                value={p.h}
                onChange={e => set('h', Math.max(20, Number(e.target.value)))}
              />
            </Field>
          </div>
        </div>

      </div>
    </div>
  )
}

function Field({
  label, children, style,
}: {
  label: string
  children: ReactNode
  style?: CSSProperties
}) {
  return (
    <div style={style}>
      <div className="sarcom-prop-label">{label}</div>
      {children}
    </div>
  )
}
