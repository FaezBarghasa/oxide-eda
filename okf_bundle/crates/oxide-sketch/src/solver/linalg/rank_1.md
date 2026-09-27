---
okf_version: "0.2"
type: Function
title: rank
description: "Numerical rank: count of diagonal entries `|R[i][i]| > tol`."
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/rank_1
language: rust
---

# rank

Numerical rank: count of diagonal entries `|R[i][i]| > tol`.

## Signature

```rust
pub fn rank(&self, tol: f64) -> usize
```

## Visibility

- `pub`

## Docstring

Numerical rank: count of diagonal entries `|R[i][i]| > tol`.
`tol` is interpreted as an absolute threshold; for matrices
of LM normal equations a value matching the LM tolerance
(e.g. `1e-9`) classifies a singular value as "active".

For an `m × n` matrix the diagonal runs from `(0,0)` to
`(min(m,n)-1, min(m,n)-1)`. Empty matrices have rank 0.

## Source
Lines 367–381 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
