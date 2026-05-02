import { Tldraw, Editor, createShapeId } from 'tldraw'
import { SarcomShapeUtil } from './shapes/SarcomShapeUtil'
import { SarcomUI } from './components/SarcomUI'

const shapeUtils = [SarcomShapeUtil]

function setupCanvas(editor: Editor) {
  // Create a locked geo rectangle as the 800×480 kiosk screen reference boundary
  const outlineId = createShapeId('kiosk-outline')
  editor.createShape({
    id: outlineId,
    type: 'geo',
    x: 0,
    y: 0,
    props: {
      w:    800,
      h:    480,
      geo:  'rectangle',
      color: 'blue',
      fill:  'none',
      dash:  'dashed',
      size:  's',
    },
  })
  editor.updateShape({ id: outlineId, isLocked: true })

  requestAnimationFrame(() => {
    editor.zoomToFit({ animation: { duration: 300 } })
  })
}

export default function App() {
  return (
    <div style={{ position: 'fixed', inset: 0 }}>
      <Tldraw
        shapeUtils={shapeUtils}
        components={{
          Toolbar:            null,
          StylePanel:         null,
          PageMenu:           null,
          NavigationPanel:    null,
          InFrontOfTheCanvas: SarcomUI,
        }}
        onMount={setupCanvas}
      />
    </div>
  )
}
