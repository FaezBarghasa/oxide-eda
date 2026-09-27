---
okf_version: "0.2"
type: Function
title: generate_thermal_spokes
description: Calculate thermal relief spokes around a pad center.
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod/generate_thermal_spokes
language: rust
---

# generate_thermal_spokes

Calculate thermal relief spokes around a pad center.

## Signature

```rust
impl CopperPourEngine { pub fn generate_thermal_spokes(
        center: Point2D,
        pad_radius: Microns,
        config: &CopperZoneConfig,
    ) -> Vec<ThermalSpoke> }
```

## Visibility

- `pub`

## Docstring

Calculate thermal relief spokes around a pad center.

## Source
Lines 64–108 in `crates/oxide-router/src/copper_pour/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copper_pour](/crates/oxide-router/src/copper_pour/mod.md) |
