---
okf_version: "0.2"
type: Function
title: upload_overlay
description: "Upload overlay polygon geometry into the dedicated overlay buffer,"
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/upload_overlay_1
language: rust
---

# upload_overlay

Upload overlay polygon geometry into the dedicated overlay buffer,

## Signature

```rust
pub fn upload_overlay(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        polygons: &[GpuPolygon],
    )
```

## Visibility

- `pub`

## Docstring

Upload overlay polygon geometry into the dedicated overlay buffer,
drawn by [`Self::draw_overlay`] in a separate later pass.

## Source
Lines 281–297 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
