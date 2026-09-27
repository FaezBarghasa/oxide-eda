---
okf_version: "0.2"
type: Function
title: draw_resize_handles
description: Corner + edge-midpoint resize handles for selected graphics.
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/scene/draw_resize_handles
language: rust
---

# draw_resize_handles

Corner + edge-midpoint resize handles for selected graphics.

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn draw_resize_handles(
        &self,
        frame: &mut canvas::Frame,
    ) }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Corner + edge-midpoint resize handles for selected graphics.

## Source
Lines 12–54 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/library/editor/symbol/canvas/draw/scene.md) |
| calls | [is_graphic_selected](/crates/oxide-app/src/library/editor/symbol/canvas/mod/is_graphic_selected.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| calls | [graphic_handles](/crates/oxide-app/src/library/editor/symbol/state/hit_test/graphic_handles.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
