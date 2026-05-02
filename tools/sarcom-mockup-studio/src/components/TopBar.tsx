import { useRef, useState } from 'react'
import { useEditor, createShapeId } from 'tldraw'
import { SarcomShape } from '../shapes/SarcomShapeUtil'
import { SarcomMockupFile, SarcomShapeProps } from '../types'
import { exportPng } from '../utils/exportUtils'
import { exportToml } from '../utils/tomlExport'
import normalMultiTag  from '../mockups/normal_multi_tag.json'
import sosState        from '../mockups/sos_state.json'
import staleNoFix      from '../mockups/stale_no_fix_state.json'

const SAMPLES: Record<string, SarcomMockupFile> = {
  'Normal multi-tag':  normalMultiTag  as SarcomMockupFile,
  'SOS state':         sosState         as SarcomMockupFile,
  'Stale / no-fix':    staleNoFix       as SarcomMockupFile,
}

export function TopBar() {
  const editor   = useEditor()
  const fileRef  = useRef<HTMLInputElement>(null)
  const [busy, setBusy] = useState(false)

  // ── Load a SarcomMockupFile into the editor ────────────────────────────
  function applyMockup(file: SarcomMockupFile) {
    // Remove existing sarcom shapes
    const existing = editor.getCurrentPageShapes()
      .filter(s => s.type === 'sarcom')
      .map(s => s.id)
    if (existing.length) editor.deleteShapes(existing)

    // Create shapes from file
    editor.createShapes(
      file.shapes.map(s => ({
        id:       createShapeId(),
        type:     'sarcom' as const,
        x:        s.x,
        y:        s.y,
        rotation: s.rotation ?? 0,
        props:    s.props as SarcomShapeProps,
      })),
    )
  }

  // ── Save ────────────────────────────────────────────────────────────────
  function saveMockup() {
    const shapes = editor.getCurrentPageShapes()
      .filter(s => s.type === 'sarcom') as SarcomShape[]

    const file: SarcomMockupFile = {
      version:     1,
      name:        'My Mockup',
      description: '',
      shapes:      shapes.map(s => ({
        type:     'sarcom',
        x:        s.x,
        y:        s.y,
        rotation: s.rotation,
        props:    s.props as SarcomShapeProps,
      })),
    }

    download(
      JSON.stringify(file, null, 2),
      `mockup-${timestamp()}.json`,
      'application/json',
    )
  }

  // ── Load from file ──────────────────────────────────────────────────────
  function loadFromFile(e: React.ChangeEvent<HTMLInputElement>) {
    const f = e.target.files?.[0]
    if (!f) return
    const reader = new FileReader()
    reader.onload = ev => {
      try {
        const file = JSON.parse(ev.target!.result as string) as SarcomMockupFile
        if (file.version !== 1) { alert('Unknown mockup version.'); return }
        applyMockup(file)
      } catch {
        alert('Could not parse JSON mockup file.')
      }
    }
    reader.readAsText(f)
    e.target.value = ''   // reset so same file can be re-loaded
  }

  // ── Export PNG ──────────────────────────────────────────────────────────
  async function handleExportPng() {
    setBusy(true)
    try {
      await exportPng(editor)
    } catch (err) {
      console.error(err)
      alert('PNG export failed. See console for details.')
    } finally {
      setBusy(false)
    }
  }

  // ── Export TOML ─────────────────────────────────────────────────────────
  function handleExportToml() {
    const shapes = editor.getCurrentPageShapes()
      .filter(s => s.type === 'sarcom') as SarcomShape[]
    const toml = exportToml(shapes)
    download(toml, `scenario-${timestamp()}.toml`, 'text/plain')
  }

  return (
    <div
      style={{
        position: 'absolute',
        top: 0, left: 0, right: 0,
        height: 44,
        background: '#0f172a',
        borderBottom: '1px solid #1e293b',
        display: 'flex',
        alignItems: 'center',
        padding: '0 10px',
        gap: 8,
        pointerEvents: 'all',
        zIndex: 10,
        flexShrink: 0,
      }}
    >
      {/* Title */}
      <span style={{ fontWeight: 700, fontSize: 12, color: '#e2e8f0', letterSpacing: '0.06em', marginRight: 4, whiteSpace: 'nowrap' }}>
        SARCOM Mockup Studio
      </span>

      <div style={{ width: 1, height: 24, background: '#1e293b' }} />

      {/* Sample selector */}
      <label style={{ fontSize: 10, color: '#64748b', whiteSpace: 'nowrap' }}>Sample:</label>
      <select
        style={{ background: '#1e293b', border: '1px solid #334155', borderRadius: 4, color: '#e2e8f0', fontSize: 11, padding: '3px 6px', cursor: 'pointer' }}
        defaultValue=""
        onChange={e => {
          const file = SAMPLES[e.target.value]
          if (file) { applyMockup(file); e.target.value = '' }
        }}
      >
        <option value="" disabled>Load sample…</option>
        {Object.keys(SAMPLES).map(k => <option key={k} value={k}>{k}</option>)}
      </select>

      <div style={{ flex: 1 }} />

      {/* Action buttons */}
      <TopBtn label="Load JSON" onClick={() => fileRef.current?.click()} />
      <input ref={fileRef} type="file" accept=".json" style={{ display: 'none' }} onChange={loadFromFile} />

      <TopBtn label="Save JSON" onClick={saveMockup} accent />
      <TopBtn label={busy ? 'Exporting…' : 'Export PNG'} onClick={handleExportPng} disabled={busy} accent />
      <TopBtn label="Export TOML" onClick={handleExportToml} />
    </div>
  )
}

// ── Sub-components ────────────────────────────────────────────────────────

function TopBtn({
  label, onClick, accent, disabled,
}: {
  label: string
  onClick: () => void
  accent?: boolean
  disabled?: boolean
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      style={{
        padding: '4px 10px',
        background: accent ? '#1d4ed8' : '#1e293b',
        border: `1px solid ${accent ? '#3b82f6' : '#334155'}`,
        borderRadius: 5,
        color: disabled ? '#475569' : '#e2e8f0',
        fontSize: 11,
        cursor: disabled ? 'default' : 'pointer',
        whiteSpace: 'nowrap',
      }}
    >
      {label}
    </button>
  )
}

// ── Utils ─────────────────────────────────────────────────────────────────

function download(content: string, filename: string, mimeType: string) {
  const blob = new Blob([content], { type: mimeType })
  const url  = URL.createObjectURL(blob)
  const a    = document.createElement('a')
  a.href     = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

function timestamp(): string {
  return new Date().toISOString().slice(0, 16).replace('T', '_').replace(':', '-')
}
