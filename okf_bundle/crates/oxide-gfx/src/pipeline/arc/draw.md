---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-gfx/src/pipeline/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/arc/draw
language: rust
---

# draw

## Signature

```rust
impl ArcPipeline { pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
    ) }
```

## Visibility

- `pub`

## Source
Lines 147–160 in `crates/oxide-gfx/src/pipeline/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/pipeline/arc.md) |
