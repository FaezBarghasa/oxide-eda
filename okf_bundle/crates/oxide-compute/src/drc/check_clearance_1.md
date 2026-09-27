---
okf_version: "0.2"
type: Function
title: check_clearance
resource: crates/oxide-compute/src/drc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/drc/check_clearance_1
language: rust
---

# check_clearance

## Signature

```rust
pub fn check_clearance(
        &mut self,
        objects: &[GpuBBox],
        min_clearance: f32,
    ) -> Result<Vec<GpuViolation>, crate::backend::ComputeError>
```

## Visibility

- `pub`

## Source
Lines 65–116 in `crates/oxide-compute/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-compute/src/drc.md) |
