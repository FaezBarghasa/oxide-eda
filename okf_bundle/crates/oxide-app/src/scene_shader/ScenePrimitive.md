---
okf_version: "0.2"
type: Class
title: ScenePrimitive
description: "One frame's worth of scene geometry handed to the GPU. Cheap to build each"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/ScenePrimitive
language: rust
---

# ScenePrimitive

One frame's worth of scene geometry handed to the GPU. Cheap to build each

## Signature

```rust
pub struct ScenePrimitive
```

## Type Parameters

- `S: SceneSurface`

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

One frame's worth of scene geometry handed to the GPU. Cheap to build each
frame — it is the same instance data the CPU path already produces.

`S` selects the pipeline slot; see [`SceneSurface`].
[derive(Debug)]

## Methods

- `scene`
- `generation`
- `offset_px`
- `scale_px_per_mm`
- `surface`

## Source
Lines 229–247 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
