---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-compute/src/signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:22Z"
concept_id: crates/oxide-compute/src/signal/new
language: rust
---

# new

## Signature

```rust
impl FdtdSimulator { pub fn new(
        mut backend: Box<dyn ComputeBackend>,
        grid_x: u32,
        grid_y: u32,
        grid_z: u32,
        dx: f32,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 44–72 in `crates/oxide-compute/src/signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [signal](/crates/oxide-compute/src/signal.md) |
| calls | [PipelineId](/crates/oxide-compute/src/backend/PipelineId.md) |
