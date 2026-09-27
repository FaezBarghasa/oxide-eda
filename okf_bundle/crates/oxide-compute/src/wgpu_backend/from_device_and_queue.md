---
okf_version: "0.2"
type: Function
title: from_device_and_queue
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/from_device_and_queue
language: rust
---

# from_device_and_queue

## Signature

```rust
impl WgpuBackend { pub fn from_device_and_queue(
        device: wgpu::Device,
        queue: wgpu::Queue,
        adapter_info: wgpu::AdapterInfo,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 55–68 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
