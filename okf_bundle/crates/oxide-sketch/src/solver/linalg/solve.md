---
okf_version: "0.2"
type: Function
title: solve
description: "Solve `A x = b` via partial-pivot LU."
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/solve
language: rust
---

# solve

Solve `A x = b` via partial-pivot LU.

## Signature

```rust
pub fn solve(a: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, LinAlgError>
```

## Visibility

- `pub`

## Docstring

Solve `A x = b` via partial-pivot LU.

`A` is borrowed and cloned internally (the caller's matrix is
untouched); `b` is borrowed and cloned. Returns `x` as a fresh
`Vec<f64>` of length `n = A.len()`.

Errors:
- [`LinAlgError::DimensionMismatch`] if `A` is non-square or
`b.len() != A.len()`.
- [`LinAlgError::Singular`] if a zero pivot is encountered.

## Source
Lines 43–57 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
| calls | [lu_decompose](/crates/oxide-sketch/src/solver/linalg/lu_decompose.md) |
| calls | [lu_solve](/crates/oxide-sketch/src/solver/linalg/lu_solve.md) |
