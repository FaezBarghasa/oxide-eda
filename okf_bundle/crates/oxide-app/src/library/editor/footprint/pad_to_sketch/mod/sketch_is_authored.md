---
okf_version: "0.2"
type: Function
title: sketch_is_authored
description: "Whether the footprint's sketch already holds authored (non-"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sketch_is_authored
language: rust
---

# sketch_is_authored

Whether the footprint's sketch already holds authored (non-

## Signature

```rust
pub fn sketch_is_authored(footprint: &Footprint) -> bool
```

## Visibility

- `pub`

## Docstring

Whether the footprint's sketch already holds authored (non-
construction) content.

This is exactly the condition [`auto_mint_for_literal_pads`]
early-returns on, so a caller that mints a single pad (paste) can
use it to tell the two cases apart: authored → auto-mint will never
pick this pad up, mint it now; not authored → auto-mint still
covers it, and minting early is what would break that.

## Source
Lines 68–73 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| called_by | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| called_by | [apply_footprint_clipboard_op](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op.md) |
