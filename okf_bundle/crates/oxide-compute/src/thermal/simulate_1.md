---
okf_version: "0.2"
type: Function
title: simulate
resource: crates/oxide-compute/src/thermal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/thermal/simulate_1
language: rust
---

# simulate

## Signature

```rust
pub fn simulate(
        &mut self,
        power_map: &[f32],
        material_map: &[u32],
        iterations: u32,
    ) -> Result<ThermalResult, crate::backend::ComputeError>
```

## Visibility

- `pub`

## Source
Lines 64–138 in `crates/oxide-compute/src/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/oxide-compute/src/thermal.md) |
