---
okf_version: "0.2"
type: Function
title: max_temperature_celsius
description: Evaluates maximum temperature across the board volume in Celsius.
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/max_temperature_celsius
language: rust
---

# max_temperature_celsius

Evaluates maximum temperature across the board volume in Celsius.

## Signature

```rust
impl ThermalGrid3D { pub fn max_temperature_celsius(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Evaluates maximum temperature across the board volume in Celsius.

## Source
Lines 150–153 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
