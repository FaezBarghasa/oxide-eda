---
okf_version: "0.2"
type: Function
title: draw_selection_overlay
resource: crates/oxide-app/src/schematic_runtime/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/selection/draw_selection_overlay
language: rust
---

# draw_selection_overlay

## Signature

```rust
pub fn draw_selection_overlay(
    frame: &mut canvas::Frame,
    snapshot: &SchematicRenderSnapshot,
    selected: &[SelectedItem],
    transform: &ScreenTransform,
)
```

## Visibility

- `pub`

## Source
Lines 3–98 in `crates/oxide-app/src/schematic_runtime/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/schematic_runtime/selection.md) |
| calls | [item_aabb](/crates/oxide-app/src/schematic_runtime/mod/item_aabb.md) |
| calls | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [draw_selection](/crates/oxide-app/src/canvas/draw/scene/draw_selection.md) |
