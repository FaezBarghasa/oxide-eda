---
okf_version: "0.2"
type: Function
title: upload_raw
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/upload_raw_1
language: rust
---

# upload_raw

## Signature

```rust
fn upload_raw(&mut self, buffer_id: BufferId, data: &[u8]) -> Result<(), ComputeError>
```

## Source
Lines 135–142 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
