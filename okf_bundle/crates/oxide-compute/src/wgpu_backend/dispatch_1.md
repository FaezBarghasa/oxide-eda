---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-compute/src/wgpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:51:16Z"
concept_id: crates/oxide-compute/src/wgpu_backend/dispatch_1
language: rust
---

# dispatch

## Signature

```rust
fn dispatch(
        &mut self,
        pipeline_id: PipelineId,
        workgroups: [u32; 3],
        bindings: &[BufferId],
    ) -> Result<(), ComputeError>
```

## Source
Lines 228–279 in `crates/oxide-compute/src/wgpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wgpu_backend](/crates/oxide-compute/src/wgpu_backend.md) |
