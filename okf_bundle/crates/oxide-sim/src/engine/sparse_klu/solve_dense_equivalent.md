---
okf_version: "0.2"
type: Function
title: solve_dense_equivalent
description: "Markowitz LU decomposition with threshold partial pivoting: A = P * L * U * Q."
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/solve_dense_equivalent
language: rust
---

# solve_dense_equivalent

Markowitz LU decomposition with threshold partial pivoting: A = P * L * U * Q.

## Signature

```rust
impl SparseMatrixCsc { pub fn solve_dense_equivalent(&self, rhs: &[f64]) -> Option<Vec<f64>> }
```

## Visibility

- `pub`

## Docstring

Markowitz LU decomposition with threshold partial pivoting: A = P * L * U * Q.
Returns (L_dense, U_dense, perm_p, perm_q) for the sparse block.

## Source
Lines 115–177 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
