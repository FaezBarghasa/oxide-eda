---
okf_version: "0.2"
type: Function
title: polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry
description: "Three `PolygonClick`s then `PolygonCommit` push exactly one"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry
language: rust
---

# polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry

Three `PolygonClick`s then `PolygonCommit` push exactly one

## Signature

```rust
fn polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry()
```

## Decorators

- `test`

## Docstring

Three `PolygonClick`s then `PolygonCommit` push exactly one
graphic and exactly one undo snapshot — mirrors what every
close gesture (click-on-first-vertex / double-click / Enter)
collapses to from the dispatcher's point of view.
[test]

## Source
Lines 647–685 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
