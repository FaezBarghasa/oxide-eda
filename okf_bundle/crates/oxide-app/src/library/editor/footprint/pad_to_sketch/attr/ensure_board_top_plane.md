---
okf_version: "0.2"
type: Function
title: ensure_board_top_plane
description: "Look up (or create) the footprint's `BoardTop` plane and return"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/ensure_board_top_plane
language: rust
---

# ensure_board_top_plane

Look up (or create) the footprint's `BoardTop` plane and return

## Signature

```rust
pub(super) fn ensure_board_top_plane(footprint: &mut Footprint) -> PlaneId
```

## Visibility

- `pub(super)`

## Docstring

Look up (or create) the footprint's `BoardTop` plane and return
its ID. The pad-mirror code assumes every minted entity lives on
this single plane.

## Source
Lines 115–131 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| called_by | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
