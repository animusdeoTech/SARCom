---
title: "lora-phy preflight"
type: preflight
crate: lora-phy
upstream: lora-rs/lora-rs
---

# lora-phy preflight

Local truth pointer for `lora-phy` before SARCOM radio code is touched. This file is **not** a tutorial. It exists so that skipping source lookup becomes visibly noncompliant with repo instructions.

## Status (verified 2026-05-06)

- **lora-phy version pinned in `Cargo.lock`?** No. The workspace `Cargo.lock` contains only the `protocol` crate.
- **Local rustdoc at `target/doc/lora_phy/index.html`?** No.
- **First lookup, until pinned:** `github` MCP against `lora-rs/lora-rs`.

When `lora-phy` is added to a workspace member's `Cargo.toml`:

1. Run `cargo doc -p lora-phy --locked` to populate `target/doc/lora_phy/`.
2. Update the **Status** block of this file (date + new pin).
3. The first lookup shifts to local rustdoc automatically per the order below.

## Upstream

- **Correct:** [`lora-rs/lora-rs`](https://github.com/lora-rs/lora-rs)
- **Rejected (archived):** `embassy-rs/lora-phy` — superseded and explicitly off-limits per [ADR-001](../../decisions/ADR-001-firmware-language.md).

If a generated snippet, doc page, or example imports from `embassy-rs/lora-phy`, references deprecated trait names, or shows removed config types, discard it.

## Required lookup order

1. **Local rustdoc** if `target/doc/lora_phy/index.html` exists for the pinned version.
2. **Local vendored source** if `vendor/lora-phy/` exists (not currently set up).
3. **`github` MCP** reading `lora-rs/lora-rs` source (`lora-phy/src/`) and `examples/`.
4. **Stop and report** if none of the above are available. Do not draft from memory.

`Context7` is **not** in this list. Context7 has no `lora-phy` entry as of 2026-05-06; do not use it for this crate until it resolves to `lora-rs/lora-rs`.

## Do not invent

- Do not write `lora-phy` init / send / receive / CAD code from memory.
- Do not infer APIs from old blog posts, old crate examples, or anything under archived `embassy-rs/lora-phy`.
- Do not use `Context7` for `lora-phy` until it resolves correctly.
- Do not paraphrase a remembered API shape into "looks right" code. The trait surface (`RadioKind`, config types, async timer) has changed between versions — guesses compile sometimes and run wrong every time.

## Required preflight statement

Before editing any file that initialises, transmits, receives, performs CAD on, or configures the LoRa radio via `lora-phy`, fill in and post this in the conversation (and/or commit message body):

```md
## lora-phy preflight statement

- Task:
- Crate/version:
- Source consulted:
- Exact files/docs read:
- API facts extracted:
  - radio type:
  - config type:
  - async delay/timer requirement:
  - init method:
  - tx method:
  - rx/cad method, if relevant:
- Rejected sources:
- Confidence:
```

If a field cannot be filled because the source did not cover it, leave it blank and say so under **Confidence**. Blank-and-honest beats invented-and-confident.

## Helpers

- `scripts/check-lora-phy-docs.ps1` — advisory PowerShell script, reports current pin/docs status and the recommended next action.
- `.claude/commands/rust_lora_phy_preflight.md` — slash command `/rust_lora_phy_preflight` that walks Claude through the lookup and statement.
