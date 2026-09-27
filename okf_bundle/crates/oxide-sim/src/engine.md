---
okf_version: "0.2"
type: Module
title: engine
description: In-process sparse Modified Nodal Analysis (MNA) linear/non-linear circuit solver.
resource: crates/oxide-sim/src/engine.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:33:35Z"
concept_id: crates/oxide-sim/src/engine
language: rust
---

# engine

In-process sparse Modified Nodal Analysis (MNA) linear/non-linear circuit solver.

## Docstring

In-process sparse Modified Nodal Analysis (MNA) linear/non-linear circuit solver.

Replaces external process spawning with a shared-memory sparse matrix MNA solver,
supporting analytical Jacobians, four-stage automated convergence cascades,
and adaptive local truncation error (LTE) step selection (Trapezoidal / Gear BDF).

## Relationships

| Type | Target |
|------|--------|
| related | [SimError](/crates/oxide-sim/src/engine/SimError.md) |
| related | [ConvergenceStage](/crates/oxide-sim/src/engine/ConvergenceStage.md) |
| related | [StepTelemetry](/crates/oxide-sim/src/engine/StepTelemetry.md) |
| related | [MnaSolver](/crates/oxide-sim/src/engine/MnaSolver.md) |
| related | [InProcessMnaSolver](/crates/oxide-sim/src/engine/InProcessMnaSolver.md) |
| related | [default](/crates/oxide-sim/src/engine/default.md) |
| related | [default](/crates/oxide-sim/src/engine/default.md) |
| related | [new](/crates/oxide-sim/src/engine/new.md) |
| related | [index](/crates/oxide-sim/src/engine/index.md) |
| related | [solve_linear](/crates/oxide-sim/src/engine/solve_linear.md) |
| related | [new](/crates/oxide-sim/src/engine/new.md) |
| related | [index](/crates/oxide-sim/src/engine/index.md) |
| related | [solve_linear](/crates/oxide-sim/src/engine/solve_linear.md) |
| related | [initialize](/crates/oxide-sim/src/engine/initialize.md) |
| related | [stamp_conductance](/crates/oxide-sim/src/engine/stamp_conductance.md) |
| related | [stamp_storage](/crates/oxide-sim/src/engine/stamp_storage.md) |
| related | [stamp_nonlinear_jacobian](/crates/oxide-sim/src/engine/stamp_nonlinear_jacobian.md) |
| related | [solve_step](/crates/oxide-sim/src/engine/solve_step.md) |
| related | [recover_convergence](/crates/oxide-sim/src/engine/recover_convergence.md) |
| related | [initialize](/crates/oxide-sim/src/engine/initialize.md) |
| related | [stamp_conductance](/crates/oxide-sim/src/engine/stamp_conductance.md) |
| related | [stamp_storage](/crates/oxide-sim/src/engine/stamp_storage.md) |
| related | [stamp_nonlinear_jacobian](/crates/oxide-sim/src/engine/stamp_nonlinear_jacobian.md) |
| related | [solve_step](/crates/oxide-sim/src/engine/solve_step.md) |
| related | [recover_convergence](/crates/oxide-sim/src/engine/recover_convergence.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
