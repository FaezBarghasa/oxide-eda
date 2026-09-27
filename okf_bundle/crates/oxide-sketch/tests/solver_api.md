---
okf_version: "0.2"
type: Module
title: solver_api
description: Task 3.6 — Solver public API tests.
resource: crates/oxide-sketch/tests/solver_api.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/solver_api
language: rust
---

# solver_api

Task 3.6 — Solver public API tests.

## Docstring

Task 3.6 — Solver public API tests.

Exercises the high-level `Solver::solve` façade (LM + DOF in one
shot). The previous `AutoPauseState` hysteresis was removed in
v0.22 — footprint sketches stay small enough that every solve
completes well under the per-frame budget; pause mode added a
confusing UI state and complicated downstream agents reading
solver state.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_params](/crates/oxide-sketch/tests/solver_api/empty_params.md) |
| related | [solver_default_solves_anchored_horizontal_distance](/crates/oxide-sketch/tests/solver_api/solver_default_solves_anchored_horizontal_distance.md) |
| related | [solver_detects_over_constrained](/crates/oxide-sketch/tests/solver_api/solver_detects_over_constrained.md) |
| related | [solver_under_constrained_returns_under_dof](/crates/oxide-sketch/tests/solver_api/solver_under_constrained_returns_under_dof.md) |
| related | [solver_custom_timeout_and_iter_cap](/crates/oxide-sketch/tests/solver_api/solver_custom_timeout_and_iter_cap.md) |
| related | [solver_max_iters_field_is_honoured](/crates/oxide-sketch/tests/solver_api/solver_max_iters_field_is_honoured.md) |
| related | [solver_tolerance_field_is_honoured](/crates/oxide-sketch/tests/solver_api/solver_tolerance_field_is_honoured.md) |
