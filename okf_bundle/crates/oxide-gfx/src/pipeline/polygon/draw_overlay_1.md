---
okf_version: "0.2"
type: Function
title: draw_overlay
description: Draw all uploaded overlay polygon geometry. Callers composite this in
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/draw_overlay_1
language: rust
---

# draw_overlay

Draw all uploaded overlay polygon geometry. Callers composite this in

## Signature

```rust
pub fn draw_overlay(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
    )
```

## Visibility

- `pub`

## Docstring

Draw all uploaded overlay polygon geometry. Callers composite this in
a pass strictly after every base bucket, so overlay content always
renders on top — see `crate::scene_shader::ScenePrimitive::draw`.

## Source
Lines 347–359 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
