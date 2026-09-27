---
okf_version: "0.2"
type: Function
title: approx_eq
description: Compare two scalars with a mixed absolute/relative tolerance —
resource: crates/oxide-sketch/tests/solver_basics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/solver_basics/approx_eq
language: rust
---

# approx_eq

Compare two scalars with a mixed absolute/relative tolerance —

## Signature

```rust
fn approx_eq(actual: f64, expected: f64, rel_tol: f64)
```

## Docstring

Compare two scalars with a mixed absolute/relative tolerance —
avoids spurious failures near zero where pure relative tolerance
is meaningless.

## Source
Lines 122–129 in `crates/oxide-sketch/tests/solver_basics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_basics](/crates/oxide-sketch/tests/solver_basics.md) |
| called_by | [jacobian_coincident_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_coincident_matches_analytical.md) |
| called_by | [jacobian_distance_pt_pt_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_distance_pt_pt_matches_analytical.md) |
| called_by | [jacobian_horizontal_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_horizontal_matches_analytical.md) |
