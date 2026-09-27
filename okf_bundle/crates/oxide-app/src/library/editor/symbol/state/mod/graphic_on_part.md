---
okf_version: "0.2"
type: Function
title: graphic_on_part
description: "A graphic is visible/editable on `active_part` when it is shared"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part
language: rust
---

# graphic_on_part

A graphic is visible/editable on `active_part` when it is shared

## Signature

```rust
pub fn graphic_on_part(g: &oxide_library::SymbolGraphic, active_part: u8) -> bool
```

## Visibility

- `pub`

## Docstring

A graphic is visible/editable on `active_part` when it is shared
(part 0) or scoped to that exact unit — mirrors pin part visibility.

## Source
Lines 333–335 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [draw_resize_handles](/crates/oxide-app/src/library/editor/symbol/canvas/draw/scene/draw_resize_handles.md) |
| called_by | [build_symbol_renderer_snapshot](/crates/oxide-app/src/library/editor/symbol/canvas/mod/build_symbol_renderer_snapshot.md) |
| called_by | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test.md) |
| called_by | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
| called_by | [join_source_indices](/crates/oxide-app/src/library/editor/symbol/state/mod/join_source_indices.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
