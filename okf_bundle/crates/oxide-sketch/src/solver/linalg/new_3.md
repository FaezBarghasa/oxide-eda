---
okf_version: "0.2"
type: Function
title: new
description: "Factor an `m × n` matrix `A` using Householder reflections."
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/new_3
language: rust
---

# new

Factor an `m × n` matrix `A` using Householder reflections.

## Signature

```rust
pub fn new(a: &[Vec<f64>]) -> Result<Self, LinAlgError>
```

## Visibility

- `pub`

## Docstring

Factor an `m × n` matrix `A` using Householder reflections.
Works for any shape (`m < n`, `m == n`, `m > n`).

Algorithm (NR §2.10):

1. Copy `a` into a working buffer (the caller's input is
untouched).
2. For each column `k` in `0..min(m, n)`:
- Extract the sub-vector `x = a[k..m, k]`.
- Compute `α = -sign(x[0]) · |x|` so the reflector points
away from `x[0]` and avoids cancellation.
- Form `v = x − α · e_0` (i.e. `v[0] = x[0] − α`, the rest
of `v` is `x[1..]`), then normalise `v` to unit length.
- The reflector `H = I − 2vvᵀ` zeroes everything below row
`k` in column `k`. Apply it to the trailing submatrix
`a[k..m, k..n]` by, for each column `j` in `k..n`,
computing `β = 2 · v · a[k..m, j]` and updating
`a[k..m, j] -= β · v`.
- Overwrite the diagonal entry `a[k][k] = α` (the
Householder formula gives this exactly; we store it
explicitly to avoid roundoff drift).
3. The resulting upper triangle of `a` is `R`.

Edge cases:
- If `m == 0` or `n == 0`, the result is a degenerate
zero-rank factorisation (no reflections to apply).
- If at column `k` the sub-vector `x` has Euclidean norm below
`QR_ZERO_EPS`, the column is genuinely zero in its lower
tail and we skip the reflection. `R[k][k]` will be zero (or
already-zero) and counted as a rank-deficient diagonal by
[`Self::rank`].

## Source
Lines 277–358 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
