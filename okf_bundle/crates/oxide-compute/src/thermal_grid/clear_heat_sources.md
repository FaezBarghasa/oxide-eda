---
okf_version: "0.2"
type: Function
title: clear_heat_sources
description: Clears all dynamic power dissipation terms.
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/clear_heat_sources
language: rust
---

# clear_heat_sources

Clears all dynamic power dissipation terms.

## Signature

```rust
impl ThermalGrid3D { pub fn clear_heat_sources(&mut self) }
```

## Visibility

- `pub`

## Docstring

Clears all dynamic power dissipation terms.

## Source
Lines 101–103 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
