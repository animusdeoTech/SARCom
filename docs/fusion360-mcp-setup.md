# Fusion 360 MCP setup

Claude Code CLI ↔ Fusion 360 pipeline for SARCOM design work (gateway enclosure CAD, tag enclosure, relay mount adapter).

## Chosen MCP server

[`ndoo/fusion360-mcp-bridge`](https://github.com/ndoo/fusion360-mcp-bridge) — pinned at commit `6bd42f4` (2026-03-27).

Two tools exposed:

| Tool | Purpose |
|------|---------|
| `fusion_execute` | Run arbitrary Python with full `adsk.*` API access inside the live Fusion process |
| `fusion_screenshot` | Capture the active viewport as a PNG for visual verification |

## Why this one

Explicitly Claude-Code-targeted, open source (MIT), no Fusion subscription required, and the bridge is intentionally minimal — all Fusion-API knowledge lives in the bridge's own `CLAUDE.md` which Claude reads at session start. Alternative path if this breaks: Autodesk's [official Fusion MCP](https://www.autodesk.com/products/fusion-360/blog/introducing-the-fusion-mcp-opening-fusion-to-ai-powered-workflows/).

## Prereqs confirmed on this workstation

| Component | Version | Source |
|-----------|---------|--------|
| Fusion 360 | `2702.1.58` | `adsk.core.Application.get().version` |
| Python | `3.13.13` | `python --version` |
| git | `2.54.0.windows.1` | `git --version` |
| Claude Code CLI | (whatever ships when `claude mcp list` works) | `claude mcp list` |

Bridge Python deps (from `mcp-server/requirements.txt`):

```
mcp>=1.0.0
httpx>=0.27.0
```

## Install layout

| What | Where |
|------|-------|
| Bridge clone | `C:\Users\Pieter\tools\fusion360-mcp-bridge\` |
| Fusion add-in | `%APPDATA%\Autodesk\Autodesk Fusion 360\API\AddIns\FusionMCPBridge\` |
| Shared secret | `C:\Users\Pieter\.fusion-mcp-secret` (Bearer token, read by both add-in and MCP server) |
| MCP server entry point | `C:\Users\Pieter\tools\fusion360-mcp-bridge\mcp-server\server.py` |

The bridge clone lives **outside** the SARCOM repo on purpose — it is workstation-wide tooling, not a project dependency. Do not vendor or submodule it.

## Install steps that worked

These match the README's manual-install path (Windows section), adapted for PowerShell. Run from any directory.

```powershell
# 1. Clone outside the SARCOM repo
git clone https://github.com/ndoo/fusion360-mcp-bridge.git C:\Users\Pieter\tools\fusion360-mcp-bridge

# 2. Install Python deps (user install — no venv needed on Windows)
pip install -r C:\Users\Pieter\tools\fusion360-mcp-bridge\mcp-server\requirements.txt --user

# 3. Generate shared secret
python -c "import secrets; print(secrets.token_hex(32))" > C:\Users\Pieter\.fusion-mcp-secret

# 4. Copy the add-in folder into Fusion's AddIns directory
xcopy /E /I C:\Users\Pieter\tools\fusion360-mcp-bridge\fusion-addin\FusionMCPBridge `
  "$env:APPDATA\Autodesk\Autodesk Fusion 360\API\AddIns\FusionMCPBridge"

# 5. Register the MCP server with Claude Code CLI (user scope)
claude mcp add fusion360 -s user python C:\Users\Pieter\tools\fusion360-mcp-bridge\mcp-server\server.py
```

Then in Fusion 360: **Tools → Add-Ins** (`Shift+S`) → select `FusionMCPBridge` under *My Add-Ins* → **Run**, with **Run on Startup** checked.

Finally restart Claude Code CLI so the new MCP entry is picked up.

## MCP config entry

Stored in Claude Code's user-scope config (verbatim from `claude mcp get fusion360`):

```
fusion360:
  Scope: User config (available in all your projects)
  Type: stdio
  Command: python
  Args: C:\Users\Pieter\tools\fusion360-mcp-bridge\mcp-server\server.py
  Environment: (none)
```

No env vars required. The bridge reads `C:\Users\Pieter\.fusion-mcp-secret` directly. Default localhost port is `7654`; override with `FUSION_MCP_PORT` (set before launching Fusion so the add-in picks it up).

## Boot / shutdown sequence

**To bring the pipeline up:**

1. Launch Fusion 360. The `FusionMCPBridge` add-in starts on launch (because `runOnStartup: true` in its manifest + the **Run on Startup** checkbox).
2. Open or create a design.
3. Launch Claude Code CLI. It spawns the MCP server (`python …\server.py`) on first tool use.

**To take it down:**

- Stop the add-in: Fusion → **Tools → Add-Ins** → `FusionMCPBridge` → **Stop**. Or quit Fusion entirely.
- Stop the MCP server: end the Claude Code session. The server is `stdio`-attached, so it dies with the CLI.

**To remove:**

```powershell
claude mcp remove fusion360 -s user
```

(Add-in folder + bridge clone can be deleted manually.)

## Verification

Two test prompts from the project setup brief:

**Test 1 — READ.** *"List the active design's components and sketches."*

Expected: the MCP responds with a non-error structured listing. Confirmed working in this session against `gateway-v1` — returned 3 components (`reference-sketches`, `front-shell`, `gateway-v1`) and 4 sketches.

**Test 2 — WRITE.** *"Create a sketch on the XY plane with a 50×30 mm rectangle centered on the origin."*

Expected: a new sketch appears in the Fusion timeline; a 50 × 30 mm rectangle is visible centered at origin in the viewport. Confirmed working — verified in setup against a programmatic throwaway `Untitled` design (created via `app.documents.add(...)`, sketch `verify-rectangle` on XY plane, `addTwoPointRectangle` with corners at (-2.5, -1.5, 0) and (+2.5, +1.5, 0) cm, bounding box measured at 50 × 30 mm centered on origin, then closed without saving).

A quick health-check from PowerShell (requires the add-in to be running):

```powershell
$token = Get-Content C:\Users\Pieter\.fusion-mcp-secret -Raw
Invoke-RestMethod -Uri http://localhost:7654/health -Headers @{ Authorization = "Bearer $($token.Trim())" }
```

## Known limitations / rough edges

- **Active document required.** The tools error with "No active document" if Fusion is open but no design is loaded.
- **Main-thread marshalling.** All API calls are funnelled onto Fusion's main thread via `CustomEvent` + a blocking `threading.Event`. Long-running scripts block the Fusion UI for their duration.
- **Single design at a time.** `design` in the execute context is `adsk.core.Application.get().activeProduct` — if you switch documents mid-session, subsequent calls operate on the new active doc.
- **Port collision.** Default 7654; if something else binds it, set `FUSION_MCP_PORT` *before* launching Fusion.
- **Auth token quirk.** If you regenerate `.fusion-mcp-secret` while Fusion is running, the add-in keeps the old token in memory — restart Fusion to pick up the new one.
- **README is macOS-first.** Quick-start script `scripts/quickstart-mac.sh` is mac-only. Windows install is the manual path (above).
- **`fusion_screenshot` broken against Fusion 2702.1.58.** Errors with `Viewport.saveAsImageFileWithOptions() takes 2 positional arguments but 5 were given` — Autodesk changed the signature and the bridge add-in still calls it the old way. Workaround: rely on `fusion_execute` to query geometry (bounding boxes, body volumes, etc.) for verification instead of visual screenshots. Fix lives in `fusion-addin/FusionMCPBridge/FusionMCPBridge.py`.

## Fallback if this breaks

Autodesk shipped a [native Fusion MCP](https://www.autodesk.com/products/fusion-360/blog/introducing-the-fusion-mcp-opening-fusion-to-ai-powered-workflows/) — switch to that if `ndoo/fusion360-mcp-bridge` becomes unmaintained or incompatible.
