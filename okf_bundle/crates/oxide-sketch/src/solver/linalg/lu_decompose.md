---
okf_version: "0.2"
type: Function
title: lu_decompose
description: Compute the LU decomposition in place with partial (row) pivoting.
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/lu_decompose
language: rust
---

# lu_decompose

Compute the LU decomposition in place with partial (row) pivoting.

## Signature

```rust
pub fn lu_decompose(a: &mut [Vec<f64>]) -> Result<Vec<usize>, LinAlgError>
```

## Visibility

- `pub`

## Docstring

Compute the LU decomposition in place with partial (row) pivoting.

On success the input matrix is overwritten with the packed LU
form: the strict lower triangle holds L (with an implicit unit
diagonal), and the upper triangle including the diagonal holds
U. The returned `perm` vector records the row pivots; entry
`perm[k] = r` means "during column k, rows k and r were
swapped". The same permutation must be applied to the right-
hand side before forward substitution — see [`lu_solve`].

Algorithm (NR §2.3):
1. For each column `k` in `0..n`:
- **Pivot:** find row `r` in `[k, n)` with the largest
`|a[r][k]|`. If `|a[r][k]| < PIVOT_EPS`, return
[`LinAlgError::Singular`]. Swap rows `k` and `r` and
record `perm[k] = r`.
- **Eliminate:** for every row `i > k`, store L's i-th
multiplier in place with `a[i][k] /= a[k][k]`, then for
every `j > k` update U's submatrix with
`a[i][j] -= a[i][k] * a[k][j]`.

## Source
Lines 79–126 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
| called_by | [main](/crates/oxide-sketch/examples/bench_linalg/main.md) |
| called_by | [new](/crates/oxide-sketch/src/solver/linalg/new.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/linalg/solve.md) |
| called_by | [lu_decompose_and_solve_separately](/crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately.md) |
| called_by | [lu_decompose_singular_matrix_errors](/crates/oxide-sketch/tests/linalg/lu_decompose_singular_matrix_errors.md) |
