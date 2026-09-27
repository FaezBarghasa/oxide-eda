---
okf_version: "0.2"
type: Function
title: push_undo
description: Push a full snapshot onto the undo stack; clear the redo stack.
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo
language: rust
---

# push_undo

Push a full snapshot onto the undo stack; clear the redo stack.

## Signature

```rust
fn push_undo(editor: &mut SymEditor)
```

## Docstring

Push a full snapshot onto the undo stack; clear the redo stack.
Capped at 100 entries — oldest entry is evicted when the cap is hit.

## Source
Lines 40–43 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [push_undo_snapshot](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo_snapshot.md) |
| called_by | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [begin_drag_if_needed](/crates/oxide-app/src/library/editor/symbol/updates/mod/begin_drag_if_needed.md) |
| called_by | [push_graphic](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic.md) |
| called_by | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
