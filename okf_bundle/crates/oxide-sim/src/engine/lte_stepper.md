---
okf_version: "0.2"
type: Module
title: lte_stepper
description: "Variable-Order Integration (Gear BDF 1-6 / Trapezoidal) & Milne Local Truncation Error (LTE) Controller."
resource: crates/oxide-sim/src/engine/lte_stepper.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:51Z"
concept_id: crates/oxide-sim/src/engine/lte_stepper
language: rust
---

# lte_stepper

Variable-Order Integration (Gear BDF 1-6 / Trapezoidal) & Milne Local Truncation Error (LTE) Controller.

## Docstring

Variable-Order Integration (Gear BDF 1-6 / Trapezoidal) & Milne Local Truncation Error (LTE) Controller.

Conforms to Master Technical Directive Horizon I (§2, Task 1.2):
- Gear BDF Orders 1 through 6 and variable Trapezoidal integration.
- Milne predictor-corrector error estimation across dynamic storage nodes.
- Adaptive dynamic timestep bounds based on user-defined RELTOL and ABSTOL.

## Relationships

| Type | Target |
|------|--------|
| related | [IntegrationMethod](/crates/oxide-sim/src/engine/lte_stepper/IntegrationMethod.md) |
| related | [LteController](/crates/oxide-sim/src/engine/lte_stepper/LteController.md) |
| related | [default](/crates/oxide-sim/src/engine/lte_stepper/default.md) |
| related | [default](/crates/oxide-sim/src/engine/lte_stepper/default.md) |
| related | [new](/crates/oxide-sim/src/engine/lte_stepper/new.md) |
| related | [compute_next_timestep](/crates/oxide-sim/src/engine/lte_stepper/compute_next_timestep.md) |
| related | [integration_coefficients](/crates/oxide-sim/src/engine/lte_stepper/integration_coefficients.md) |
| related | [new](/crates/oxide-sim/src/engine/lte_stepper/new.md) |
| related | [compute_next_timestep](/crates/oxide-sim/src/engine/lte_stepper/compute_next_timestep.md) |
| related | [integration_coefficients](/crates/oxide-sim/src/engine/lte_stepper/integration_coefficients.md) |
| related | [test_lte_timestep_adaptation](/crates/oxide-sim/src/engine/lte_stepper/test_lte_timestep_adaptation.md) |
| related | [test_lte_timestep_reduction_on_large_error](/crates/oxide-sim/src/engine/lte_stepper/test_lte_timestep_reduction_on_large_error.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
