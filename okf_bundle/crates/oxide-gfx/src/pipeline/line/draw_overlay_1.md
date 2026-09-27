---
okf_version: "0.2"
type: Function
title: draw_overlay
description: Draw all uploaded overlay line instances. Callers composite this in a
resource: crates/oxide-gfx/src/pipeline/line.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/line/draw_overlay_1
language: rust
---

# draw_overlay

Draw all uploaded overlay line instances. Callers composite this in a

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

Draw all uploaded overlay line instances. Callers composite this in a
pass strictly after every base bucket, so overlay content always
renders on top — see `crate::scene_shader::ScenePrimitive::draw`.

## Source
Lines 217–229 in `crates/oxide-gfx/src/pipeline/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/pipeline/line.md) |
