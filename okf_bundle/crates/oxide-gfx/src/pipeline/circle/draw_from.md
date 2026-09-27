---
okf_version: "0.2"
type: Function
title: draw_from
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/draw_from
language: rust
---

# draw_from

## Signature

```rust
impl CirclePipeline { fn draw_from(
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
        render_pipeline: &wgpu::RenderPipeline,
        instance_buffer: &wgpu::Buffer,
        instance_count: u32,
    ) }
```

## Source
Lines 221–236 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
