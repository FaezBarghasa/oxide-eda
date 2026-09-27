---
okf_version: "0.2"
type: Class
title: ScenePipeline
description: "The set of `oxide_gfx` pipelines plus the camera, created once by iced and"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/ScenePipeline
language: rust
---

# ScenePipeline

The set of `oxide_gfx` pipelines plus the camera, created once by iced and

## Signature

```rust
pub struct ScenePipeline
```

## Visibility

- `pub`

## Docstring

The set of `oxide_gfx` pipelines plus the camera, created once by iced and
reused across frames.

iced stores one of these per primitive type (see [`SceneSurface`]), so each
surface gets its own buffers, camera and warn-once flags.

## Methods

- `camera`
- `line`
- `circle`
- `arc`
- `polygon`
- `text`
- `uploaded_generation`
- `text_upload_warned`
- `text_draw_warned`

## Source
Lines 170–188 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
