---
okf_version: "0.2"
type: Module
title: jacobian
description: Numerical Jacobian via central differences.
resource: crates/oxide-sketch/src/solver/jacobian.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/jacobian
language: rust
---

# jacobian

Numerical Jacobian via central differences.

## Docstring

Numerical Jacobian via central differences.

The Jacobian J is an (m × n) matrix where m = total residual count
and n = state vector length. Each entry J[i][j] is dr_i/dx_j, the
sensitivity of residual i to state coordinate j. Computed by
finite difference at a step `H = 1e-7`:

J[i][j] ≈ (r_i(x + h e_j) − r_i(x − h e_j)) / (2h)

Reference: *Numerical Recipes* (Press et al., 3rd ed.) §5.7
(numerical derivatives — central difference is second-order
accurate, h = 1e-7 chosen to balance truncation vs roundoff for
double-precision input on the unit interval).

## Relationships

| Type | Target |
|------|--------|
| related | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
