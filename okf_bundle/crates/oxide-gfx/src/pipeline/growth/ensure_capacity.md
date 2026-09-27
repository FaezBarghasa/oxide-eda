---
okf_version: "0.2"
type: Function
title: ensure_capacity
resource: crates/oxide-gfx/src/pipeline/growth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/pipeline/growth/ensure_capacity
language: rust
---

# ensure_capacity

## Signature

```rust
pub(crate) fn ensure_capacity(
    device: &wgpu::Device,
    buffer: &mut wgpu::Buffer,
    capacity: &mut usize,
    required: usize,
    params: &GrowthParams,
) -> usize
```

## Visibility

- `pub(crate)`

## Source
Lines 36–69 in `crates/oxide-gfx/src/pipeline/growth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [growth](/crates/oxide-gfx/src/pipeline/growth.md) |
| calls | [warn_clamp_once](/crates/oxide-gfx/src/pipeline/growth/warn_clamp_once.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/arc/upload.md) |
| called_by | [upload_into](/crates/oxide-gfx/src/pipeline/circle/upload_into.md) |
| called_by | [upload_into](/crates/oxide-gfx/src/pipeline/line/upload_into.md) |
| called_by | [upload_into](/crates/oxide-gfx/src/pipeline/polygon/upload_into.md) |
