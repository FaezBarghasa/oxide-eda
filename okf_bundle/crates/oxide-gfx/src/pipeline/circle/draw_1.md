---
okf_version: "0.2"
type: Function
title: draw
description: Draw all uploaded circle instances.
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/draw_1
language: rust
---

# draw

Draw all uploaded circle instances.

## Signature

```rust
pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
    )
```

## Visibility

- `pub`

## Docstring

Draw all uploaded circle instances.

## Source
Lines 190–202 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
