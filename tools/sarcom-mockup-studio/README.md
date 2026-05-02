# SARCOM Mockup Studio

Visual drag-and-drop mockup tool for the SARCOM 7" kiosk UI.
Runs in the browser on Windows — no backend, no cloud, no Electron.

**This is NOT the production kiosk.**
The production kiosk is native Rust/egui/walkers/PMTiles (see `ARCHITECTURE.md`).
This tool exists only for fast UX iteration on Windows desktop.

---

## Quick start (PowerShell)

```powershell
cd tools\sarcom-mockup-studio
npm install
npm run dev
```

Then open `http://localhost:5173` in Chrome or Edge.

---

## Layout

```
┌─────────────────────────────── TopBar ────────────────────────────────────┐
│ SARCOM Mockup Studio  │ Sample ▼  │  Load JSON │ Save JSON │ PNG │ TOML  │
├─── Toolbar ──┬─────────── tldraw canvas ─────────────┬─── Properties ────┤
│ Add Tag      │                                        │  (selected shape) │
│ Add Relay    │   dashed 800×480 kiosk outline         │  label            │
│ Add Gateway  │                                        │  node_id          │
│ Add Card     │   drag shapes here                     │  ui_kind          │
│ Add SOS ...  │                                        │  state            │
│ Add Panel    │                                        │  last_seen_text   │
│ Add No-Fix   │                                        │  coords_text      │
│ Add Scale    │                                        │  battery_text     │
│ Add North    │                                        │  w / h            │
└──────────────┴────────────────────────────────────────┴───────────────────┘
```

---

## Usage

### Adding shapes
Click any button in the left toolbar. The shape appears near the canvas centre.
Drag it into position. Resize with the corner/edge handles.

### Editing properties
Click a shape to select it. The right panel shows its properties.
All fields are live-editable — changes render immediately.

### Kiosk boundary
The dashed blue rectangle is the 800×480 kiosk screen boundary.
It is locked (cannot be moved or deleted accidentally).
Place shapes inside it to see what the real kiosk would show.

### Canvas controls
| Action           | How                         |
|------------------|-----------------------------|
| Pan              | Space + drag, or middle drag|
| Zoom             | Scroll wheel                |
| Select           | Click                       |
| Multi-select     | Shift+click or drag-select  |
| Delete           | Delete / Backspace          |
| Undo / Redo      | Ctrl+Z / Ctrl+Y             |
| Fit to screen    | Shift+1                     |

---

## Save / Load

### Save a mockup
Click **Save JSON** in the top bar.
A file named `mockup-YYYY-MM-DD_HH-MM.json` downloads to your browser's default download folder.
Move it to `tools/sarcom-mockup-studio/mockups/` to keep it with the project.

### Load a mockup
Click **Load JSON** and pick a `.json` file.
Loading replaces all SARCOM shapes on the canvas (the locked kiosk outline stays).

### Sample mockups (built-in)
Use the **Sample** dropdown to load pre-built scenarios:
- **Normal multi-tag** — three active tags, relay, gateway, all healthy
- **SOS state** — one tag in SOS, banner + info card shown
- **Stale / no-fix** — mixed health: stale, very stale, no-fix, low battery

The source JSON files live in `src/mockups/`.

---

## Exports

### PNG export
Click **Export PNG** — downloads a 2× retina PNG of everything on the canvas.
Rendered by tldraw's built-in `exportToBlob` (requires Chrome or Edge).

### TOML scenario export
Click **Export TOML** — downloads a `scenario-*.toml` file.
**This is a mockup design file, not the production schema.**
It lists nodes and overlays with their labels, states, and canvas positions.
Useful for communicating a UX scenario to others without opening the browser tool.

Place exported files in `tools/sarcom-mockup-studio/exports/` by convention.

---

## File locations

| What               | Where                                          |
|--------------------|------------------------------------------------|
| Saved mockups      | `tools/sarcom-mockup-studio/mockups/`          |
| Exported PNGs      | Your browser download folder                   |
| Exported TOMLs     | `tools/sarcom-mockup-studio/exports/` (manual) |
| Sample mockup JSON | `src/mockups/`                                 |
| Shape definitions  | `src/shapes/SarcomShapeUtil.tsx`               |

---

## Adding custom shapes

All shapes live in a single `SarcomShape` type with a `ui_kind` discriminator.
To add a new kind:

1. Add the kind string to `SarcomUiKind` in `src/types.ts`
2. Add defaults to `UI_KIND_DEFAULTS` in `src/types.ts`
3. Add a `case` in `Renderer()` in `src/shapes/SarcomShapeUtil.tsx`
4. Add a toolbar button in `src/components/Toolbar.tsx`

---

## Known limitations

- **PNG export** uses tldraw's built-in renderer, which requires Chrome or Edge.
  Firefox may fail on the `exportToBlob` call — check the browser console.
- **Background map image**: not implemented in v0.1. Workaround: add a tldraw `image`
  shape manually (drag an image file onto the canvas) and lock it.
- **Kiosk frame** is a visual guide only — shapes placed outside it are not clipped.
- Sample mockups use placeholder coordinates and times; they are not real GPS data.
- The TOML export is a design-only format, not parsed by any production code.
