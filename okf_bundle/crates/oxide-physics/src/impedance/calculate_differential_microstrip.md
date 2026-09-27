---
okf_version: "0.2"
type: Function
title: calculate_differential_microstrip
description: "Calculate edge-coupled differential microstrip impedance ($Z_{\\text{diff}}$ in Ohms)."
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_differential_microstrip
language: rust
---

# calculate_differential_microstrip

Calculate edge-coupled differential microstrip impedance ($Z_{\text{diff}}$ in Ohms).

## Signature

```rust
impl ImpedanceCalculator { pub fn calculate_differential_microstrip(
        single_ended_z0: f64,
        trace_gap: Microns,
        dielectric_height: Microns,
    ) -> f64 }
```

## Visibility

- `pub`

## Docstring

Calculate edge-coupled differential microstrip impedance ($Z_{\text{diff}}$ in Ohms).

$$Z_{\text{diff}} \approx 2 \cdot Z_0 \cdot \left(1 - 0.48 \cdot e^{-0.96 \cdot \frac{s}{h}}\right)$$

# Arguments
* `single_ended_z0` - Characteristic single-ended impedance of one trace in Ohms.
* `trace_gap` ($s$) - Edge-to-edge separation between the pair traces in micrometers.
* `dielectric_height` ($h$) - Height of the dielectric to the reference plane in micrometers.

## Source
Lines 124–138 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
