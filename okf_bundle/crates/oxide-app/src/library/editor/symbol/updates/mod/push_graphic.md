---
okf_version: "0.2"
type: Function
title: push_graphic
description: "Push a graphic onto the symbol, recording an undo snapshot first."
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic
language: rust
---

# push_graphic

Push a graphic onto the symbol, recording an undo snapshot first.

## Signature

```rust
fn push_graphic(editor: &mut SymEditor, kind: oxide_library::SymbolGraphicKind, stroke_width: f64)
```

## Docstring

Push a graphic onto the symbol, recording an undo snapshot first.

## Source
Lines 121–138 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| calls | [mark_dirty](/crates/oxide-app/src/library/editor/symbol/updates/mod/mark_dirty.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
