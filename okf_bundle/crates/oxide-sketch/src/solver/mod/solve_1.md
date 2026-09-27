---
okf_version: "0.2"
type: Function
title: solve
description: "Run LM to convergence, then compute DOF analysis on the"
resource: crates/oxide-sketch/src/solver/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/mod/solve_1
language: rust
---

# solve

Run LM to convergence, then compute DOF analysis on the

## Signature

```rust
pub fn solve(
        &self,
        sketch: &SketchData,
        params: &ResolvedParams,
    ) -> Result<FullSolveOutput, SolveError>
```

## Visibility

- `pub`

## Docstring

Run LM to convergence, then compute DOF analysis on the
solved state. Errors propagate from [`solve_lm`]; on success
every output field is populated.

## Source
Lines 72–104 in `crates/oxide-sketch/src/solver/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-sketch/src/solver/mod.md) |
| calls | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
| calls | [over_constraint_ids](/crates/oxide-sketch/src/solver/dof/over_constraint_ids.md) |
