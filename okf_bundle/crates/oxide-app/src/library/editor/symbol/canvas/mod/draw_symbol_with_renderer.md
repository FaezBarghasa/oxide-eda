---
okf_version: "0.2"
type: Function
title: draw_symbol_with_renderer
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/draw_symbol_with_renderer
language: rust
---

# draw_symbol_with_renderer

## Signature

```rust
impl SymbolCanvas<'a> { fn draw_symbol_with_renderer(
        &self,
        frame: &mut canvas::Frame,
        selected: &Option<SymbolSelection>,
        scale: f32,
    ) }
```

## Type Parameters

- `'a`

## Source
Lines 471–515 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| calls | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
