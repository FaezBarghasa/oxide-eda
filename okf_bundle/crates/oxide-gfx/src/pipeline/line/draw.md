---
okf_version: "0.2"
type: Function
title: draw
description: Draw all uploaded line instances.
resource: crates/oxide-gfx/src/pipeline/line.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/line/draw
language: rust
---

# draw

Draw all uploaded line instances.

## Signature

```rust
impl LinePipeline { pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
    ) }
```

## Visibility

- `pub`

## Docstring

Draw all uploaded line instances.

## Source
Lines 200–212 in `crates/oxide-gfx/src/pipeline/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/pipeline/line.md) |
