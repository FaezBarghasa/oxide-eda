---
okf_version: "0.2"
type: Function
title: new
description: Creates a new 3D Thermal grid initialized to ambient temperature.
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/new
language: rust
---

# new

Creates a new 3D Thermal grid initialized to ambient temperature.

## Signature

```rust
impl ThermalGrid3D { pub fn new(dim_x: usize, dim_y: usize, dim_z: usize, voxel_size_m: f64, ambient_temp_c: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates a new 3D Thermal grid initialized to ambient temperature.

## Source
Lines 64–77 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
