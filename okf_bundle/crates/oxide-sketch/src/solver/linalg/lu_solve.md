---
okf_version: "0.2"
type: Function
title: lu_solve
description: Forward + back substitution given an LU-decomposed matrix and a
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/lu_solve
language: rust
---

# lu_solve

Forward + back substitution given an LU-decomposed matrix and a

## Signature

```rust
pub fn lu_solve(lu: &[Vec<f64>], perm: &[usize], b: &[f64]) -> Result<Vec<f64>, LinAlgError>
```

## Visibility

- `pub`

## Docstring

Forward + back substitution given an LU-decomposed matrix and a
pivot permutation produced by [`lu_decompose`].

Steps:
1. Apply the recorded row pivots to `b` in the same order they
were applied during decomposition.
2. Forward-substitute through L (unit diagonal) to obtain `y`
such that `L y = P b`.
3. Back-substitute through U to obtain `x` such that `U x = y`.

## Source
Lines 166–213 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
| called_by | [main](/crates/oxide-sketch/examples/bench_linalg/main.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/linalg/solve.md) |
| called_by | [lu_decompose_and_solve_separately](/crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately.md) |
| called_by | [lu_solve_dimension_mismatch_errors](/crates/oxide-sketch/tests/linalg/lu_solve_dimension_mismatch_errors.md) |
