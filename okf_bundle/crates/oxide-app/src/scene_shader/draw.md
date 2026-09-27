---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/draw
language: rust
---

# draw

## Signature

```rust
impl ScenePrimitive<S> { fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool }
```

## Type Parameters

- `S: SceneSurface`

## Source
Lines 358–414 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| calls | [log_text_error_once](/crates/oxide-app/src/scene_shader/log_text_error_once.md) |
