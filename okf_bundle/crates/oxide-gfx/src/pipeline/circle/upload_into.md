---
okf_version: "0.2"
type: Function
title: upload_into
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/upload_into
language: rust
---

# upload_into

## Signature

```rust
impl CirclePipeline { fn upload_into(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        circles: &[Circle],
        buffer: &mut wgpu::Buffer,
        capacity: &mut usize,
        count: &mut u32,
        label: &'static str,
    ) }
```

## Source
Lines 158–187 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
| calls | [ensure_capacity](/crates/oxide-gfx/src/pipeline/growth/ensure_capacity.md) |
