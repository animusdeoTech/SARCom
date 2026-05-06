# Claude Code MCP workflow

Reference for setting up Claude Code MCP servers on a new machine for this project.

## How the config files work

| File | Tracked | Purpose |
|------|---------|---------|
| `.mcp.json` | **No** (gitignored) | Your local MCP config — create from the example below |
| `.mcp.example.json` | **Yes** | Reproducible template — copy this to `.mcp.json` and add your key |

`.mcp.json` is intentionally gitignored. It contains an env-var reference (`${BRAVE_API_KEY}`) rather than a literal key, but local paths or personal tokens could end up in it. The example file is the canonical template.

## First-time setup

```powershell
# 1. Copy the template
Copy-Item .mcp.example.json .mcp.json

# 2. Set the API key as a persistent user env var (survives restarts)
[System.Environment]::SetEnvironmentVariable("BRAVE_API_KEY", "your-key-here", "User")

# 3. Restart Claude Code so it inherits the new env var
```

Verify the key is visible to the shell before starting Claude Code:

```powershell
$env:BRAVE_API_KEY   # should print the key, not blank
```

## Verifying the servers are running

Type `/mcp` in the Claude Code prompt. All configured servers should show `connected`. If any show `error`:

1. Confirm `BRAVE_API_KEY` is set and you restarted Claude Code after setting it.
2. Run the server manually to see startup errors:
   ```powershell
   npx -y @brave/brave-search-mcp-server --transport stdio
   ```
3. Confirm Node.js / npx is on the PATH: `node --version`.

## When to use each server

### `brave-search` — external web research

Use for anything that lives on the public web:

- Hardware datasheets (SX1262, UC6580, DS3231, Dragino HAT)
- Vendor product pages (Semtech, Heltec, Mouser, DigiKey)
- crates.io release notes / changelogs not in context7
- Regulatory documents (ETSI EN 300 220, LoRa Alliance specs)

**Do not use for Rust API docs** — context7 is more accurate for that.

### `context7` — library documentation

Use for Rust ecosystem library APIs:

- Embassy crates (`embassy-executor`, `embassy-time`, `embassy-hal-*`)
- `lora-phy`, `egui`, `walkers`, `tokio`, `sqlx`, `rusqlite`, `rpi-pal` (the maintained fork of the archived `rppal`)
- Anything you would normally look up on docs.rs

### Prefer local sources over both when possible

- **rust-analyzer / inline rustdoc** — hover types and go-to-def are faster than any MCP call when the crate is already in `Cargo.toml`
- **Repo source files** — `ARCHITECTURE.md`, ADRs, and the Rust source are the ground truth for this project; MCP servers have no visibility into them

### `sequential-thinking`

Structured multi-step reasoning for complex design or debugging problems. Use explicitly when you want to think through a decision step by step.

## Rules

- **Never commit API keys.** `.mcp.example.json` uses `"${BRAVE_API_KEY}"` — the shell substitutes this at runtime. The literal key must never appear in any tracked file.
- **Brave is for external web research.** Context7, rustdoc, and rust-analyzer are the correct tools for Rust API documentation.
- **Do not add new MCP servers without discussion.** Each server is an additional dependency and attack surface.
