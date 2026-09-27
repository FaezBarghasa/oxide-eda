---
okf_version: "0.2"
type: Function
title: temperature_celsius
description: "Returns temperature in Celsius at voxel $(x, y, z)$."
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/temperature_celsius_1
language: rust
---

# temperature_celsius

Returns temperature in Celsius at voxel $(x, y, z)$.

## Signature

```rust
pub fn temperature_celsius(&self, x: usize, y: usize, z: usize) -> f64
```

## Visibility

- `pub`

## Docstring

Returns temperature in Celsius at voxel $(x, y, z)$.

## Source
Lines 156–162 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
