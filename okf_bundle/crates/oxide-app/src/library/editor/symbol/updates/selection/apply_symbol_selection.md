---
okf_version: "0.2"
type: Function
title: apply_symbol_selection
resource: crates/oxide-app/src/library/editor/symbol/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/selection/apply_symbol_selection
language: rust
---

# apply_symbol_selection

## Signature

```rust
pub(super) fn apply_symbol_selection(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–32 in `crates/oxide-app/src/library/editor/symbol/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/symbol/updates/selection.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
