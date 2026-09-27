---
okf_version: "0.2"
type: Function
title: world_origin_matches_the_screen_transform_mapping
description: "The mapping this mirrors is `ScreenTransform::world_to_screen`"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/world_origin_matches_the_screen_transform_mapping
language: rust
---

# world_origin_matches_the_screen_transform_mapping

The mapping this mirrors is `ScreenTransform::world_to_screen`

## Signature

```rust
fn world_origin_matches_the_screen_transform_mapping()
```

## Decorators

- `test`

## Docstring

The mapping this mirrors is `ScreenTransform::world_to_screen`
(`screen_px = world_mm * scale + offset_px`), so a surface feeding this
program from a `ScreenTransform` passes `offset_x` / `offset_y` as
`offset_px` and `scale` as `scale_px_per_mm`. Ported from
`schematic_shader`'s round-trip test (#625) — that file held the only
check tying the two together.
[test]

## Source
Lines 542–554 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| calls | [world_origin_mm](/crates/oxide-app/src/scene_shader/world_origin_mm.md) |
