---
okf_version: "0.2"
type: Module
title: cascade
description: Automated Four-Stage Convergence Recovery Cascade for Stiff Non-Linear Circuits.
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade
language: rust
---

# cascade

Automated Four-Stage Convergence Recovery Cascade for Stiff Non-Linear Circuits.

## Docstring

Automated Four-Stage Convergence Recovery Cascade for Stiff Non-Linear Circuits.

Conforms to Master Technical Directive Horizon I (§2, Task 1.3):
- Stage 0: Standard Newton-Raphson Iteration.
- Stage 1: Damped Line-Search Newton-Raphson.
- Stage 2: Adaptive Gmin Conductance Stepping ($10^{-2}\,\text{S} \to 10^{-12}\,\text{S}$).
- Stage 3: Dynamic Source Stepping Continuation ($\alpha \in [0.0 \to 1.0]$).
- Stage 4: Pseudo-Transient Continuation (PTC) with virtual node capacitances.

## Relationships

| Type | Target |
|------|--------|
| related | [ConvergenceCascade](/crates/oxide-sim/src/engine/cascade/ConvergenceCascade.md) |
| related | [default](/crates/oxide-sim/src/engine/cascade/default.md) |
| related | [default](/crates/oxide-sim/src/engine/cascade/default.md) |
| related | [new](/crates/oxide-sim/src/engine/cascade/new.md) |
| related | [reset](/crates/oxide-sim/src/engine/cascade/reset.md) |
| related | [advance_fallback_stage](/crates/oxide-sim/src/engine/cascade/advance_fallback_stage.md) |
| related | [compute_line_search_damping](/crates/oxide-sim/src/engine/cascade/compute_line_search_damping.md) |
| related | [step_gmin](/crates/oxide-sim/src/engine/cascade/step_gmin.md) |
| related | [step_source](/crates/oxide-sim/src/engine/cascade/step_source.md) |
| related | [new](/crates/oxide-sim/src/engine/cascade/new.md) |
| related | [reset](/crates/oxide-sim/src/engine/cascade/reset.md) |
| related | [advance_fallback_stage](/crates/oxide-sim/src/engine/cascade/advance_fallback_stage.md) |
| related | [compute_line_search_damping](/crates/oxide-sim/src/engine/cascade/compute_line_search_damping.md) |
| related | [step_gmin](/crates/oxide-sim/src/engine/cascade/step_gmin.md) |
| related | [step_source](/crates/oxide-sim/src/engine/cascade/step_source.md) |
| related | [test_cascade_stage_transitions](/crates/oxide-sim/src/engine/cascade/test_cascade_stage_transitions.md) |
| related | [test_gmin_and_source_stepping](/crates/oxide-sim/src/engine/cascade/test_gmin_and_source_stepping.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
