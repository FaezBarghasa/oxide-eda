---
okf_version: "0.2"
type: Class
title: InProcessMnaSolver
description: "Built-in in-process MNA Solver implementing analytical Newton-Raphson iterations,"
resource: crates/oxide-sim/src/engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:33:35Z"
concept_id: crates/oxide-sim/src/engine/InProcessMnaSolver
language: rust
---

# InProcessMnaSolver

Built-in in-process MNA Solver implementing analytical Newton-Raphson iterations,

## Signature

```rust
pub struct InProcessMnaSolver
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Built-in in-process MNA Solver implementing analytical Newton-Raphson iterations,
Trapezoidal/BDF integration, and 4-stage convergence recovery cascades.
[derive(Debug, Clone)]

## Methods

- `node_count`
- `branch_count`
- `total_dim`
- `g_matrix`
- `c_matrix`
- `rhs_vector`
- `state_vector`
- `prev_state_vector`
- `prev_deriv_vector`
- `gmin`
- `source_factor`
- `pseudo_tau`
- `reltol`
- `abstol`
- `max_iterations`
- `initialized`

## Source
Lines 75–92 in `crates/oxide-sim/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-sim/src/engine.md) |
