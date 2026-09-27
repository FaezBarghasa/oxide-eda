---
okf_version: "0.2"
type: Function
title: render_schematic
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/render_schematic
language: rust
---

# render_schematic

## Signature

```rust
pub fn render_schematic(
    frame: &mut canvas::Frame,
    snapshot: &SchematicRenderSnapshot,
    transform: &ScreenTransform,
    colors: &CanvasColors,
    bounds: Rectangle,
    focus_set: Option<&HashSet<uuid::Uuid>>,
    wire_color_overrides: Option<&HashMap<uuid::Uuid, ThemeColor>>,
)
```

## Visibility

- `pub`

## Source
Lines 237–255 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [render_schematic_with_renderer](/crates/oxide-app/src/schematic_runtime/mod/render_schematic_with_renderer.md) |
| called_by | [draw_content](/crates/oxide-app/src/canvas/draw/scene/draw_content.md) |
