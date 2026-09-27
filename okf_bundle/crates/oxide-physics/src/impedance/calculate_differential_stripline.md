---
okf_version: "0.2"
type: Function
title: calculate_differential_stripline
description: "Calculate edge-coupled differential stripline impedance ($Z_{\\text{diff}}$ in Ohms)."
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_differential_stripline
language: rust
---

# calculate_differential_stripline

Calculate edge-coupled differential stripline impedance ($Z_{\text{diff}}$ in Ohms).

## Signature

```rust
impl ImpedanceCalculator { pub fn calculate_differential_stripline(
        single_ended_z0: f64,
        trace_gap: Microns,
        total_plane_separation: Microns,
    ) -> f64 }
```

## Visibility

- `pub`

## Docstring

Calculate edge-coupled differential stripline impedance ($Z_{\text{diff}}$ in Ohms).

$$Z_{\text{diff}} \approx 2 \cdot Z_0 \cdot \left(1 - 0.347 \cdot e^{-2.9 \cdot \frac{s}{b}}\right)$$

## Source
Lines 143–157 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
