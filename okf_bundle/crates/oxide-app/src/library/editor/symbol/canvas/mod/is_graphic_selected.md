---
okf_version: "0.2"
type: Function
title: is_graphic_selected
description: "Returns `true` when the graphic at `idx` should be drawn in the"
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/is_graphic_selected
language: rust
---

# is_graphic_selected

Returns `true` when the graphic at `idx` should be drawn in the

## Signature

```rust
fn is_graphic_selected(sel: &Option<SymbolSelection>, idx: usize) -> bool
```

## Docstring

Returns `true` when the graphic at `idx` should be drawn in the
selection colour. Handles single-graphic, Multiple, and All
selections. Thin wrapper over `state::graphic_is_selected` — kept
here too since `hit_test_graphic_handle` (state-side) needs the
exact same check to scope `PolygonVertex` handle hit-testing to
the selected polygon only.

## Source
Lines 352–354 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
| calls | [graphic_is_selected](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_is_selected.md) |
| called_by | [draw_resize_handles](/crates/oxide-app/src/library/editor/symbol/canvas/draw/scene/draw_resize_handles.md) |
| called_by | [build_symbol_renderer_snapshot](/crates/oxide-app/src/library/editor/symbol/canvas/mod/build_symbol_renderer_snapshot.md) |
