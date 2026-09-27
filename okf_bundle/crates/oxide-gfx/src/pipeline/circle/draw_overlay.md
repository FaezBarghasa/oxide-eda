---
okf_version: "0.2"
type: Function
title: draw_overlay
description: Draw all uploaded overlay circle instances. Callers composite this in
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/draw_overlay
language: rust
---

# draw_overlay

Draw all uploaded overlay circle instances. Callers composite this in

## Signature

```rust
impl CirclePipeline { pub fn draw_overlay(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
        camera_bind_group: &wgpu::BindGroup,
    ) }
```

## Visibility

- `pub`

## Docstring

Draw all uploaded overlay circle instances. Callers composite this in
a pass strictly after every base bucket, so overlay content always
renders on top.

## Source
Lines 207–219 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
