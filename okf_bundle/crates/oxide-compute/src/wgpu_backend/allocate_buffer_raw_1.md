---
okf_version: "0.2"
type: Function
title: allocate_buffer_raw
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/allocate_buffer_raw_1
language: rust
---

# allocate_buffer_raw

## Signature

```rust
fn allocate_buffer_raw(
        &mut self,
        size_bytes: usize,
        data: Option<&[u8]>,
    ) -> Result<BufferId, ComputeError>
```

## Source
Lines 96–133 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
| calls | [BufferId](/crates/oxide-compute/src/backend/BufferId.md) |
