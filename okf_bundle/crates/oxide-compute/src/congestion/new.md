---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-compute/src/congestion.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/congestion/new
language: rust
---

# new

## Signature

```rust
impl CongestionMapGenerator { pub fn new(
        mut backend: Box<dyn ComputeBackend>,
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 46–63 in `crates/oxide-compute/src/congestion.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [congestion](/crates/oxide-compute/src/congestion.md) |
| calls | [PipelineId](/crates/oxide-compute/src/backend/PipelineId.md) |
