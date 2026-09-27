---
okf_version: "0.2"
type: Function
title: solve_microstrip_width
description: Invert the microstrip formula to solve for the required trace width (in micrometers)
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/solve_microstrip_width_1
language: rust
---

# solve_microstrip_width

Invert the microstrip formula to solve for the required trace width (in micrometers)

## Signature

```rust
pub fn solve_microstrip_width(
        target_z0: f64,
        dielectric_height: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> Option<Microns>
```

## Visibility

- `pub`

## Docstring

Invert the microstrip formula to solve for the required trace width (in micrometers)
to achieve a target single-ended characteristic impedance $Z_0$.

## Source
Lines 161–193 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
