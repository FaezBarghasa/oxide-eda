---
okf_version: "0.2"
type: Function
title: integration_coefficients
description: "Companion dynamic coefficients $\\alpha_0, \\beta_0$ for BDF / Trapezoidal discretizations."
resource: crates/oxide-sim/src/engine/lte_stepper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:51Z"
concept_id: crates/oxide-sim/src/engine/lte_stepper/integration_coefficients_1
language: rust
---

# integration_coefficients

Companion dynamic coefficients $\alpha_0, \beta_0$ for BDF / Trapezoidal discretizations.

## Signature

```rust
pub fn integration_coefficients(&self, dt: f64) -> (f64, f64)
```

## Visibility

- `pub`

## Docstring

Companion dynamic coefficients $\alpha_0, \beta_0$ for BDF / Trapezoidal discretizations.

## Source
Lines 99–117 in `crates/oxide-sim/src/engine/lte_stepper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lte_stepper](/crates/oxide-sim/src/engine/lte_stepper.md) |
