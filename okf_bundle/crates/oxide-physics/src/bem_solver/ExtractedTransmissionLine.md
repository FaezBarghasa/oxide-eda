---
okf_version: "0.2"
type: Class
title: ExtractedTransmissionLine
description: Extracted 2D Transmission Line Parameters.
resource: crates/oxide-physics/src/bem_solver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:35:08Z"
concept_id: crates/oxide-physics/src/bem_solver/ExtractedTransmissionLine
language: rust
---

# ExtractedTransmissionLine

Extracted 2D Transmission Line Parameters.

## Signature

```rust
pub struct ExtractedTransmissionLine
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Extracted 2D Transmission Line Parameters.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `characteristic_impedance_z0`
- `differential_impedance_zdiff`
- `capacitance_per_meter_f`
- `inductance_per_meter_h`
- `propagation_delay_ps_per_mm`
- `effective_er`

## Source
Lines 46–53 in `crates/oxide-physics/src/bem_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bem_solver](/crates/oxide-physics/src/bem_solver.md) |
