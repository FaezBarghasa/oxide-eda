---
okf_version: "0.2"
type: Function
title: close_pickers
description: Close any open colour picker (graphic-fill / local-colours). Call
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers
language: rust
---

# close_pickers

Close any open colour picker (graphic-fill / local-colours). Call

## Signature

```rust
pub(super) fn close_pickers(editor: &mut SymEditor)
```

## Visibility

- `pub(super)`

## Docstring

Close any open colour picker (graphic-fill / local-colours). Call
whenever the selection is dropped or the graphics vector is
structurally mutated (delete / undo / redo / part switch) so a
picker keyed by a now-stale graphic index can't silently reopen on
an unrelated shape that happens to reuse that index.

## Source
Lines 115–118 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| called_by | [apply_symbol_history](/crates/oxide-app/src/library/editor/symbol/updates/history/apply_symbol_history.md) |
| called_by | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| called_by | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
