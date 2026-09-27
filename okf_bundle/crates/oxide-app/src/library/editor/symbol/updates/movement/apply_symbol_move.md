---
okf_version: "0.2"
type: Function
title: apply_symbol_move
resource: crates/oxide-app/src/library/editor/symbol/updates/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move
language: rust
---

# apply_symbol_move

## Signature

```rust
pub(super) fn apply_symbol_move(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–51 in `crates/oxide-app/src/library/editor/symbol/updates/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/updates/movement.md) |
| calls | [begin_drag_if_needed](/crates/oxide-app/src/library/editor/symbol/updates/mod/begin_drag_if_needed.md) |
| calls | [move_selected](/crates/oxide-app/src/library/editor/symbol/state/movement/move_selected.md) |
| calls | [move_multiple](/crates/oxide-app/src/library/editor/symbol/state/movement/move_multiple.md) |
| calls | [move_all](/crates/oxide-app/src/library/editor/symbol/state/movement/move_all.md) |
| calls | [graphic_handle_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/graphic_handle_msg_to_state.md) |
| calls | [move_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/move_graphic_handle.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
