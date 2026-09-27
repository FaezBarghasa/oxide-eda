---
okf_version: "0.2"
type: Function
title: inject_joule_heat
description: "Injects localized heat source $Q$ (in Watts) from SPICE Joule losses into voxel $(x, y, z)$."
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/inject_joule_heat_1
language: rust
---

# inject_joule_heat

Injects localized heat source $Q$ (in Watts) from SPICE Joule losses into voxel $(x, y, z)$.

## Signature

```rust
pub fn inject_joule_heat(&mut self, x: usize, y: usize, z: usize, power_w: f64)
```

## Visibility

- `pub`

## Docstring

Injects localized heat source $Q$ (in Watts) from SPICE Joule losses into voxel $(x, y, z)$.

## Source
Lines 93–98 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
