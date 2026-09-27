---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-gfx/src/pipeline/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/grid/draw_1
language: rust
---

# draw

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

## Source
Lines 99–107 in `crates/oxide-gfx/src/pipeline/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-gfx/src/pipeline/grid.md) |
