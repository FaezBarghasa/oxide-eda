---
okf_version: "0.2"
type: Function
title: calculate_embedded_microstrip
description: Calculate characteristic impedance (Z0 in Ohms) for an embedded / coated microstrip.
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_embedded_microstrip
language: rust
---

# calculate_embedded_microstrip

Calculate characteristic impedance (Z0 in Ohms) for an embedded / coated microstrip.

## Signature

```rust
impl ImpedanceCalculator { pub fn calculate_embedded_microstrip(
        trace_width: Microns,
        dielectric_height: Microns,
        coating_height: Microns,
        copper_thickness: Microns,
        er: f64,
        coating_er: f64,
    ) -> f64 }
```

## Visibility

- `pub`

## Docstring

Calculate characteristic impedance (Z0 in Ohms) for an embedded / coated microstrip.

Extends surface microstrip with a protective solder mask or dielectric coating of height $h_1$:
$$\varepsilon_{r,\text{eff}} = \varepsilon_r \cdot \left(1 - e^{-1.55 \cdot h_1 / h}\right) + e^{-1.55 \cdot h_1 / h}$$

## Source
Lines 55–77 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
