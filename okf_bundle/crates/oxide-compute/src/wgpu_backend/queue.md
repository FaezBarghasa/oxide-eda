---
okf_version: "0.2"
type: Function
title: queue
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/queue
language: rust
---

# queue

## Signature

```rust
impl WgpuBackend { pub fn queue(&self) -> &wgpu::Queue }
```

## Visibility

- `pub`

## Source
Lines 74–76 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
