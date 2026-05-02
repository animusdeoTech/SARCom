import { stopEventPropagation } from 'tldraw'
import { Toolbar } from './Toolbar'
import { PropertyPanel } from './PropertyPanel'
import { TopBar } from './TopBar'

/**
 * Rendered by tldraw's InFrontOfTheCanvas.
 * All children have access to useEditor() since they are inside the TldrawEditor context.
 *
 * Outer wrapper has pointer-events:none so canvas clicks pass through to tldraw.
 * Events that bubble UP from the panels (which have pointer-events:all) are caught
 * here and stopped so tldraw does not react to UI interactions.
 */
export function SarcomUI() {
  return (
    <div
      style={{ position: 'absolute', inset: 0, pointerEvents: 'none', overflow: 'hidden' }}
      onPointerDown={stopEventPropagation}
      onPointerMove={stopEventPropagation}
      onPointerUp={stopEventPropagation}
      onPointerEnter={stopEventPropagation}
      onPointerLeave={stopEventPropagation}
      onWheel={e => e.stopPropagation()}
    >
      <TopBar />
      <Toolbar />
      <PropertyPanel />
    </div>
  )
}
