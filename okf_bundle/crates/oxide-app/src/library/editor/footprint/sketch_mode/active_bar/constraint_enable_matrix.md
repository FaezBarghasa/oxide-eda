---
okf_version: "0.2"
type: Function
title: constraint_enable_matrix
description: Compute the per-tag enable state from the current selection slots.
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/constraint_enable_matrix
language: rust
---

# constraint_enable_matrix

Compute the per-tag enable state from the current selection slots.

## Signature

```rust
fn constraint_enable_matrix(editor: &FootprintEditorState) -> [bool; 19]
```

## Docstring

Compute the per-tag enable state from the current selection slots.
Returns a fixed-length array indexed by [`tag_index`].

## Source
Lines 405–489 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [tag_index](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/tag_index.md) |
| called_by | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
