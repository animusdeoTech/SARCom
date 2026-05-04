# MCP Setup

MCP servers configured for this project in `.mcp.json`.

## Servers

| Name | Package | Purpose |
|------|---------|---------|
| `brave-search` | `@brave/brave-search-mcp-server` | External web research, datasheets, vendor pages |
| `context7` | `@upstash/context7-mcp` | Library documentation (Embassy, Tokio, egui, etc.) |
| `github` | `@modelcontextprotocol/server-github` | GitHub API (issues, PRs, file contents) |
| `sequential-thinking` | `@modelcontextprotocol/server-sequential-thinking` | Structured multi-step reasoning |

## Required environment variables

| Variable | Used by | Notes |
|----------|---------|-------|
| `BRAVE_API_KEY` | `brave-search` | Brave Search API key — never commit this |

No other servers require env vars. `context7` and `github` use public APIs (GitHub unauthenticated or via the Claude Code plugin separately).

## Setting BRAVE_API_KEY on Windows (PowerShell)

### Per-session (current shell only)

```powershell
$env:BRAVE_API_KEY = "your-key-here"
```

### Persistent (user-level, survives restarts)

```powershell
[System.Environment]::SetEnvironmentVariable("BRAVE_API_KEY", "your-key-here", "User")
```

Restart Claude Code after setting it so the new process inherits the variable.

### Verify it is set

```powershell
$env:BRAVE_API_KEY
```

Should print your key (not blank). If blank, Claude Code will launch `brave-search` but every search call will fail with an auth error.

## Verifying the MCP servers are running

Type `/mcp` in the Claude Code prompt. You should see all four servers listed with status `connected`. If any show `error` or `disconnected`:

1. Check the env var is set and Claude Code was restarted after setting it.
2. Run `npx -y @brave/brave-search-mcp-server --transport stdio` manually in a terminal to see startup errors.
3. Check that Node.js / npx is on the PATH (`node --version`).

## When to use each server

### Use `brave-search` for

- Hardware datasheets (SX1262, UC6580, DS3231, Dragino HAT)
- Vendor product pages (Semtech, Heltec, Mouser, DigiKey)
- crates.io release notes or changelogs not covered by context7
- Any information that lives on the public web and changes over time

### Use `context7` for

- Embassy crate docs (`embassy-executor`, `embassy-time`, `embassy-hal-*`)
- Tokio, Axum, SQLx, egui, walkers, and other Rust ecosystem library APIs
- Anything you would normally look up on docs.rs

**Do not use Brave Search for Rust API docs** — context7 fetches from the actual published crate documentation and is more accurate than a web search over docs.rs HTML.

### Prefer `rust-analyzer` / inline rustdoc over both when

- The crate is already in `Cargo.toml` — hover types and go-to-definition are faster
- You need to navigate trait impls or type signatures across the workspace

### Use `github` for

- Reading issues or PRs on external repos (e.g., `lora-rs/lora-rs`, `emilk/egui`)
- Fetching source files from a pinned commit

## Rules

- **Never commit API keys.** `.mcp.json` uses `"${BRAVE_API_KEY}"` — the shell variable is substituted at runtime. The literal key must never appear in any tracked file.
- **Brave is for external web research.** Context7, rustdoc, and rust-analyzer are the correct tools for Rust API documentation.
- **`.mcp.json` is project-scoped.** Changes to it affect everyone who clones the repo (minus their own env vars). Keep it minimal and deliberate.
