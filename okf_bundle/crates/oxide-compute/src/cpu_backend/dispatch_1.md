---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-compute/src/cpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/cpu_backend/dispatch_1
language: rust
---

# dispatch

## Signature

```rust
fn dispatch(
        &mut self,
        pipeline_id: PipelineId,
        _workgroups: [u32; 3],
        _bindings: &[BufferId],
    ) -> Result<(), ComputeError>
```

## Source
Lines 91–102 in `crates/oxide-compute/src/cpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu_backend](/crates/oxide-compute/src/cpu_backend.md) |
