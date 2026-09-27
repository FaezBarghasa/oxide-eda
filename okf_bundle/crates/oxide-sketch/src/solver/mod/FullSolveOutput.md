---
okf_version: "0.2"
type: Class
title: FullSolveOutput
description: "Bundled output of a full [`Solver::solve`] call: the LM result,"
resource: crates/oxide-sketch/src/solver/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/mod/FullSolveOutput
language: rust
---

# FullSolveOutput

Bundled output of a full [`Solver::solve`] call: the LM result,

## Signature

```rust
pub struct FullSolveOutput
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Bundled output of a full [`Solver::solve`] call: the LM result,
DOF colour per Point entity, the list of constraint IDs flagged
over-constrained, the Jacobian at the solved state (carried
for downstream UI/debugging — DOF rendering reuses it without
recomputing), and the resolved parameter map the solve ran with.
[derive(Debug, Clone)]

## Methods

- `result`
- `colours`
- `over_constraints`
- `jacobian`
- `params`

## Source
Lines 53–66 in `crates/oxide-sketch/src/solver/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-sketch/src/solver/mod.md) |
