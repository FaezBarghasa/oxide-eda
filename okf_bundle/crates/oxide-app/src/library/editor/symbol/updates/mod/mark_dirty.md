---
okf_version: "0.2"
type: Function
title: mark_dirty
description: Mark the symbol as dirty and invalidate the canvas cache.
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/mark_dirty
language: rust
---

# mark_dirty

Mark the symbol as dirty and invalidate the canvas cache.

## Signature

```rust
fn mark_dirty(editor: &mut SymEditor)
```

## Docstring

Mark the symbol as dirty and invalidate the canvas cache.

## Source
Lines 72–75 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [push_graphic](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic.md) |
