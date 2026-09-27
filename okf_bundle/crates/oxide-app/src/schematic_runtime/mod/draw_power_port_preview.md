---
okf_version: "0.2"
type: Function
title: draw_power_port_preview
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview
language: rust
---

# draw_power_port_preview

## Signature

```rust
pub fn draw_power_port_preview(
    frame: &mut canvas::Frame,
    symbol: &Symbol,
    transform: &ScreenTransform,
    color: Color,
)
```

## Visibility

- `pub`

## Source
Lines 168–235 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [screen_px_to_world_mm](/crates/oxide-app/src/schematic_runtime/mod/screen_px_to_world_mm.md) |
| calls | [to_rgba](/crates/oxide-app/src/schematic_runtime/mod/to_rgba.md) |
| calls | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [draw_ghost_symbol](/crates/oxide-app/src/canvas/draw/ghosts/draw_ghost_symbol.md) |
