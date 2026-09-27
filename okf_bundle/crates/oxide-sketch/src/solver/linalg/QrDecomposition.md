---
okf_version: "0.2"
type: Class
title: QrDecomposition
description: "Result of a Householder QR factorisation: `R` is the upper-"
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/QrDecomposition
language: rust
---

# QrDecomposition

Result of a Householder QR factorisation: `R` is the upper-

## Signature

```rust
pub struct QrDecomposition
```

## Visibility

- `pub`

## Docstring

Result of a Householder QR factorisation: `R` is the upper-
triangular factor.

The DOF analysis only needs `rank(R)` — the rank of `A` equals the
rank of `R` because `Q` is orthogonal — so we deliberately do not
materialise `Q`. `R` is stored as a row-major `m × n` matrix; only
the upper triangle (`i ≤ j`) is meaningful, the lower triangle
holds whatever scratch the in-place factorisation left behind.

Reference: *Numerical Recipes* (Press et al., 3rd ed.) §2.10
("QR Decomposition"). Algorithm derived from first principles —
no third-party numerical-library source consulted.

## Methods

- `r`

## Source
Lines 241–243 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
