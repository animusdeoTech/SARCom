import { Editor, exportToBlob } from 'tldraw'

export async function exportPng(editor: Editor): Promise<void> {
  const ids = [...editor.getCurrentPageShapeIds()]
  if (ids.length === 0) {
    alert('Nothing on the canvas to export.')
    return
  }

  const blob = await exportToBlob({
    editor,
    ids,
    format: 'png',
    opts: {
      background: true,
      scale: 2,
      padding: 20,
    },
  })

  const filename = `sarcom-mockup_${new Date().toISOString().slice(0, 16).replace('T', '_').replace(':', '-')}.png`
  const url = URL.createObjectURL(blob)
  const a   = document.createElement('a')
  a.href     = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}
