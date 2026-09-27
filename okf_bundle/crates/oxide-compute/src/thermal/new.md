---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-compute/src/thermal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/thermal/new
language: rust
---

# new

## Signature

```rust
impl ThermalSimulator { pub fn new(mut backend: Box<dyn ComputeBackend>, grid_width: u32, grid_height: u32) -> Self }
```

## Visibility

- `pub`

## Source
Lines 40–55 in `crates/oxide-compute/src/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/oxide-compute/src/thermal.md) |
| calls | [PipelineId](/crates/oxide-compute/src/backend/PipelineId.md) |
