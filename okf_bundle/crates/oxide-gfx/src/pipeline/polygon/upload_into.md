---
okf_version: "0.2"
type: Function
title: upload_into
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/upload_into
language: rust
---

# upload_into

## Signature

```rust
impl PolygonPipeline { fn upload_into(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[PolygonVertex],
        buffer: &mut wgpu::Buffer,
        capacity: &mut usize,
        count: &mut u32,
        label: &'static str,
    ) }
```

## Source
Lines 299–328 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [ensure_capacity](/crates/oxide-gfx/src/pipeline/growth/ensure_capacity.md) |
