---
okf_version: "0.2"
type: Function
title: calculate_microstrip
description: Calculate characteristic impedance (Z0 in Ohms) for a surface microstrip trace.
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_microstrip
language: rust
---

# calculate_microstrip

Calculate characteristic impedance (Z0 in Ohms) for a surface microstrip trace.

## Signature

```rust
impl ImpedanceCalculator { pub fn calculate_microstrip(
        trace_width: Microns,
        dielectric_height: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> f64 }
```

## Visibility

- `pub`

## Docstring

Calculate characteristic impedance (Z0 in Ohms) for a surface microstrip trace.

Implements standard IPC-2141 equation:
$$Z_0 = \frac{87}{\sqrt{\varepsilon_r + 1.41}} \ln\left( \frac{5.98 \cdot h}{0.8 \cdot w + t} \right)$$

# Arguments
* `trace_width` ($w$) - Width of the copper trace in micrometers.
* `dielectric_height` ($h$) - Height of the dielectric to the reference plane in micrometers.
* `copper_thickness` ($t$) - Thickness of the copper trace in micrometers (e.g. 35 µm for 1 oz).
* `er` ($\varepsilon_r$) - Relative dielectric constant of the substrate (e.g. 4.4 for FR-4).

## Source
Lines 21–49 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
