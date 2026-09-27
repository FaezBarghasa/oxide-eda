---
okf_version: "0.2"
type: Function
title: calculate_stripline
description: Calculate characteristic impedance (Z0 in Ohms) for a symmetrical stripline trace.
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_stripline_1
language: rust
---

# calculate_stripline

Calculate characteristic impedance (Z0 in Ohms) for a symmetrical stripline trace.

## Signature

```rust
pub fn calculate_stripline(
        trace_width: Microns,
        total_plane_separation: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> f64
```

## Visibility

- `pub`

## Docstring

Calculate characteristic impedance (Z0 in Ohms) for a symmetrical stripline trace.

Implements standard IPC-2141 equation:
$$Z_0 = \frac{60}{\sqrt{\varepsilon_r}} \ln\left( \frac{1.9 \cdot b}{0.8 \cdot w + t} \right)$$

# Arguments
* `trace_width` ($w$) - Width of the copper trace in micrometers.
* `total_plane_separation` ($b$) - Distance between ground/power reference planes in micrometers.
* `copper_thickness` ($t$) - Thickness of the copper trace in micrometers.
* `er` ($\varepsilon_r$) - Relative dielectric constant of the core/prepreg substrate.

## Source
Lines 89–114 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
