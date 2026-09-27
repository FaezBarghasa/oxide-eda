---
okf_version: "0.2"
type: Function
title: world_origin_mm
description: World coordinate (mm) at the render pass origin (top-left).
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/world_origin_mm
language: rust
---

# world_origin_mm

World coordinate (mm) at the render pass origin (top-left).

## Signature

```rust
pub fn world_origin_mm(offset_px: [f32; 2], scale_px_per_mm: f32) -> [f32; 2]
```

## Visibility

- `pub`

## Docstring

World coordinate (mm) at the render pass origin (top-left).

The screen mapping is `screen_px = world_mm * scale + offset_px` — the same
mapping [`crate::schematic_runtime::ScreenTransform::world_to_screen`]
applies — so the world point drawn at the top-left corner is
`-offset / scale`. Returns the origin unchanged when the scale is
degenerate.

## Source
Lines 135–144 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| called_by | [prepare](/crates/oxide-app/src/scene_shader/prepare.md) |
| called_by | [world_origin_is_negative_offset_over_scale](/crates/oxide-app/src/scene_shader/world_origin_is_negative_offset_over_scale.md) |
| called_by | [world_origin_matches_the_screen_transform_mapping](/crates/oxide-app/src/scene_shader/world_origin_matches_the_screen_transform_mapping.md) |
