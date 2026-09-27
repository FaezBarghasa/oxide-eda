---
okf_version: "0.2"
type: Function
title: draw_from
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/draw_from_1
language: rust
---

# draw_from

## Signature

```rust
fn draw_from(
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
        render_pipeline: &wgpu::RenderPipeline,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
    )
```

## Source
Lines 361–376 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
