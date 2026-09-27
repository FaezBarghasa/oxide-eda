---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-gfx/src/pipeline/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/grid/new
language: rust
---

# new

## Signature

```rust
impl GridPipeline { pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 46–97 in `crates/oxide-gfx/src/pipeline/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-gfx/src/pipeline/grid.md) |
