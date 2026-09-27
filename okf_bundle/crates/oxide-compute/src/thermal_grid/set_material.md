---
okf_version: "0.2"
type: Function
title: set_material
description: "Sets material type at 3D voxel coordinate $(x, y, z)$."
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/set_material
language: rust
---

# set_material

Sets material type at 3D voxel coordinate $(x, y, z)$.

## Signature

```rust
impl ThermalGrid3D { pub fn set_material(&mut self, x: usize, y: usize, z: usize, material: GridThermalMaterial) }
```

## Visibility

- `pub`

## Docstring

Sets material type at 3D voxel coordinate $(x, y, z)$.

## Source
Lines 85–90 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
