---
okf_version: "0.2"
type: Function
title: calculate_ipc7351c_pad
description: Computes IPC-7351C mathematical land pattern parameters.
resource: crates/oxide-bake/src/ipc7351/calculator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:10:00Z"
concept_id: crates/oxide-bake/src/ipc7351/calculator/calculate_ipc7351c_pad
language: rust
---

# calculate_ipc7351c_pad

Computes IPC-7351C mathematical land pattern parameters.

## Signature

```rust
pub fn calculate_ipc7351c_pad(
    lead_span_min: f64,
    lead_span_max: f64,
    inner_span_min: f64,
    inner_span_max: f64,
    lead_width_min: f64,
    lead_width_max: f64,
    fillet: &FilletTargets,
) -> SolvedPadGeometry
```

## Visibility

- `pub`

## Docstring

Computes IPC-7351C mathematical land pattern parameters.

## Source
Lines 120–161 in `crates/oxide-bake/src/ipc7351/calculator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [calculator](/crates/oxide-bake/src/ipc7351/calculator.md) |
| called_by | [generate_footprint](/crates/oxide-bake/src/ipc7351/mod/generate_footprint.md) |
