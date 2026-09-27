---
okf_version: "0.2"
type: Class
title: SceneShaderProgram
description: "The `shader::Program` mounted when a view routes its `Scene` through the"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/SceneShaderProgram
language: rust
---

# SceneShaderProgram

The `shader::Program` mounted when a view routes its `Scene` through the

## Signature

```rust
pub struct SceneShaderProgram
```

## Type Parameters

- `S: SceneSurface`

## Visibility

- `pub`

## Docstring

The `shader::Program` mounted when a view routes its `Scene` through the
GPU. Holds the built `Scene` and the active pan/zoom; pointer handling
stays on the CPU `canvas` layer stacked beneath this shader, so `update`
is the default no-op and never captures — events fall through to the
canvas below.

## Methods

- `scene`
- `generation`
- `offset_px`
- `scale_px_per_mm`
- `surface`

## Source
Lines 422–428 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
