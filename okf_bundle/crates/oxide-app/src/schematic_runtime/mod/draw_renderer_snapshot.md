---
okf_version: "0.2"
type: Function
title: draw_renderer_snapshot
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot
language: rust
---

# draw_renderer_snapshot

## Signature

```rust
fn draw_renderer_snapshot(
    frame: &mut canvas::Frame,
    snapshot: &RendererSnapshot,
    theme: &ResolvedTheme,
    dirty: DirtyFlags,
    transform: &ScreenTransform,
)
```

## Source
Lines 346–366 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
| called_by | [draw_label_preview](/crates/oxide-app/src/schematic_runtime/label/draw_label_preview.md) |
| called_by | [draw_power_port_preview](/crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview.md) |
| called_by | [render_schematic_with_renderer](/crates/oxide-app/src/schematic_runtime/mod/render_schematic_with_renderer.md) |
| called_by | [draw_erc_markers](/crates/oxide-app/src/schematic_runtime/overlay/draw_erc_markers.md) |
| called_by | [draw_selection_overlay](/crates/oxide-app/src/schematic_runtime/selection/draw_selection_overlay.md) |
| called_by | [draw_text_note_preview](/crates/oxide-app/src/schematic_runtime/text/draw_text_note_preview.md) |
