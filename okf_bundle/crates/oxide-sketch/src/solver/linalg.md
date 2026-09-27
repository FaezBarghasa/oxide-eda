---
okf_version: "0.2"
type: Module
title: linalg
description: Dense LU linear solver with partial pivoting for small systems
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg
language: rust
---

# linalg

Dense LU linear solver with partial pivoting for small systems

## Docstring

Dense LU linear solver with partial pivoting for small systems
(n < 200). Designed for the LM step's `(J^T J + λI) Δx = b`.

Reference: *Numerical Recipes* (Press et al., 3rd ed.) §2.3
("LU Decomposition and Its Applications"). The algorithm is
standard textbook 1965-era numerical analysis: Gaussian
elimination with row pivots, packed into a single matrix per
the conventional LAPACK convention where L's unit diagonal is
implicit and U occupies the upper triangle.

Cleanroom: no third-party numerical library source consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [LinAlgError](/crates/oxide-sketch/src/solver/linalg/LinAlgError.md) |
| related | [solve](/crates/oxide-sketch/src/solver/linalg/solve.md) |
| related | [lu_decompose](/crates/oxide-sketch/src/solver/linalg/lu_decompose.md) |
| related | [LuDecomposition](/crates/oxide-sketch/src/solver/linalg/LuDecomposition.md) |
| related | [new](/crates/oxide-sketch/src/solver/linalg/new.md) |
| related | [solve](/crates/oxide-sketch/src/solver/linalg/solve.md) |
| related | [new](/crates/oxide-sketch/src/solver/linalg/new.md) |
| related | [solve](/crates/oxide-sketch/src/solver/linalg/solve.md) |
| related | [lu_solve](/crates/oxide-sketch/src/solver/linalg/lu_solve.md) |
| related | [QrDecomposition](/crates/oxide-sketch/src/solver/linalg/QrDecomposition.md) |
| related | [new](/crates/oxide-sketch/src/solver/linalg/new.md) |
| related | [rank](/crates/oxide-sketch/src/solver/linalg/rank.md) |
| related | [new](/crates/oxide-sketch/src/solver/linalg/new.md) |
| related | [rank](/crates/oxide-sketch/src/solver/linalg/rank.md) |
