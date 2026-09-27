---
okf_version: "0.2"
type: Class
title: ThermalGrid3D
description: 3D Electro-Thermal Grid Solver.
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/ThermalGrid3D
language: rust
---

# ThermalGrid3D

3D Electro-Thermal Grid Solver.

## Signature

```rust
pub struct ThermalGrid3D
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

3D Electro-Thermal Grid Solver.
[derive(Debug, Clone)]

## Methods

- `dim_x`
- `dim_y`
- `dim_z`
- `voxel_size_m`
- `ambient_temp_k`
- `materials`
- `temperature_k`
- `power_dissipation_w`

## Source
Lines 51–60 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
