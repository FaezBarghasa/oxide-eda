---
okf_version: "0.2"
type: Function
title: prepare
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/prepare_1
language: rust
---

# prepare

## Signature

```rust
fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    )
```

## Source
Lines 252–356 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| calls | [world_origin_mm](/crates/oxide-app/src/scene_shader/world_origin_mm.md) |
| calls | [log_text_error_once](/crates/oxide-app/src/scene_shader/log_text_error_once.md) |
