---
okf_version: "0.2"
type: Function
title: calculate_trace_impedance_on_stackup
description: "Calculate single-ended impedance of a trace on a specific layer in a `LayerStackup`."
resource: crates/oxide-physics/src/impedance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/impedance/calculate_trace_impedance_on_stackup
language: rust
---

# calculate_trace_impedance_on_stackup

Calculate single-ended impedance of a trace on a specific layer in a `LayerStackup`.

## Signature

```rust
impl ImpedanceCalculator { pub fn calculate_trace_impedance_on_stackup(
        stackup: &LayerStackup,
        signal_layer_idx: usize,
        trace_width: Microns,
    ) -> Option<f64> }
```

## Visibility

- `pub`

## Docstring

Calculate single-ended impedance of a trace on a specific layer in a `LayerStackup`.

## Source
Lines 196–229 in `crates/oxide-physics/src/impedance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impedance](/crates/oxide-physics/src/impedance.md) |
