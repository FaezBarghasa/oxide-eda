---
okf_version: "0.2"
type: Function
title: volumetric_heat_capacity
description: "Volumetric heat capacity $C_v = \\rho \\cdot c_p$ in $\\text{J}/(\\text{m}^3\\cdot\\text{K})$."
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/volumetric_heat_capacity
language: rust
---

# volumetric_heat_capacity

Volumetric heat capacity $C_v = \rho \cdot c_p$ in $\text{J}/(\text{m}^3\cdot\text{K})$.

## Signature

```rust
impl GridThermalMaterial { pub fn volumetric_heat_capacity(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Volumetric heat capacity $C_v = \rho \cdot c_p$ in $\text{J}/(\text{m}^3\cdot\text{K})$.

## Source
Lines 37–46 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
