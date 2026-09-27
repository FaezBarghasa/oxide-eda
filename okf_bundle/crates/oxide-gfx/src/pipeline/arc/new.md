---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-gfx/src/pipeline/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/arc/new
language: rust
---

# new

## Signature

```rust
impl ArcPipeline { pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 19–118 in `crates/oxide-gfx/src/pipeline/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/pipeline/arc.md) |
