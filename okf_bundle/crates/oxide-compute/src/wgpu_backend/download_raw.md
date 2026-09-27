---
okf_version: "0.2"
type: Function
title: download_raw
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/download_raw
language: rust
---

# download_raw

## Signature

```rust
impl WgpuBackend { fn download_raw(
        &mut self,
        buffer_id: BufferId,
        out_bytes: &mut [u8],
    ) -> Result<(), ComputeError> }
```

## Source
Lines 144–196 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
