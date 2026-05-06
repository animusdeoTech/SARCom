---
description: Run the lora-phy preflight before editing SARCOM radio code
---

You are about to touch SARCOM radio code that uses `lora-phy`. Run this preflight before drafting.

## Steps

1. **Read** `resources/docs/lora-phy-preflight.md` end to end.
2. **Determine current state.**
   - Does the workspace `Cargo.lock` contain a `lora-phy` entry? If yes, note the pinned version.
   - Does `target/doc/lora_phy/index.html` exist?
   - You can run `scripts/check-lora-phy-docs.ps1` to print both at once.
3. **Pick the source** per the lookup order in the preflight file:
   - If local rustdoc exists at `target/doc/lora_phy/index.html`, read it first.
   - Else if a vendored source exists at `vendor/lora-phy/`, read it.
   - Else use the `github` MCP against `lora-rs/lora-rs`. Read `lora-phy/src/` (especially `mod.rs` and the chip-specific module for SX1262 or SX1276) and the `examples/` directory.
   - Do **not** use `Context7` for `lora-phy` — it does not resolve to the correct upstream as of 2026-05-06.
   - If none of the above are reachable, **stop**. Tell the user. Do not draft radio code.
4. **Extract the exact API facts** needed for the current task: radio type, config type, init method, tx/rx/CAD methods, async timer requirement, error type. Quote method signatures verbatim from the source you consulted.
5. **Produce the preflight statement** (template in `resources/docs/lora-phy-preflight.md` under "Required preflight statement") and paste it into the conversation **before** writing or modifying code.
6. **After drafting**, use `cargo check` and `rust-analyzer` for validation and repair only — they are not a doc-lookup substitute.

## Hard rules

- Do not use `Context7` for `lora-phy`.
- Do not import from archived `embassy-rs/lora-phy`.
- Do not paraphrase remembered APIs.
- If the preflight cannot be completed because no authoritative source is reachable, say so explicitly and refuse to draft `lora-phy` code from memory.
