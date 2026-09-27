---
okf_version: "0.2"
type: Function
title: draw_erc_markers
resource: crates/oxide-app/src/schematic_runtime/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/overlay/draw_erc_markers
language: rust
---

# draw_erc_markers

## Signature

```rust
pub fn draw_erc_markers(
    frame: &mut canvas::Frame,
    markers: &[ErcMarker],
    transform: &ScreenTransform,
)
```

## Visibility

- `pub`

## Source
Lines 17–103 in `crates/oxide-app/src/schematic_runtime/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/schematic_runtime/overlay.md) |
| calls | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [draw_selection](/crates/oxide-app/src/canvas/draw/scene/draw_selection.md) |
