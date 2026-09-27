---
okf_version: "0.2"
type: Function
title: upload
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/upload
language: rust
---

# upload

## Signature

```rust
impl PolygonPipeline { pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, polygons: &[GpuPolygon]) }
```

## Visibility

- `pub`

## Source
Lines 266–277 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
