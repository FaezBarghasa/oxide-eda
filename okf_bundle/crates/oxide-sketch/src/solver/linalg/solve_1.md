---
okf_version: "0.2"
type: Function
title: solve
description: "Solve `A x = b` against the cached factorisation."
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/solve_1
language: rust
---

# solve

Solve `A x = b` against the cached factorisation.

## Signature

```rust
impl LuDecomposition { pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, LinAlgError> }
```

## Visibility

- `pub`

## Docstring

Solve `A x = b` against the cached factorisation.

## Source
Lines 152–154 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
| calls | [lu_solve](/crates/oxide-sketch/src/solver/linalg/lu_solve.md) |
