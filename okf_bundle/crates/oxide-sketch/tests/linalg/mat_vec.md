---
okf_version: "0.2"
type: Function
title: mat_vec
description: Multiply a square matrix by a vector — used to round-trip test
resource: crates/oxide-sketch/tests/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/linalg/mat_vec
language: rust
---

# mat_vec

Multiply a square matrix by a vector — used to round-trip test

## Signature

```rust
fn mat_vec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64>
```

## Docstring

Multiply a square matrix by a vector — used to round-trip test
`solve(A, b)` by checking `A x ≈ b`.

## Source
Lines 37–48 in `crates/oxide-sketch/tests/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/tests/linalg.md) |
| called_by | [lu_decompose_and_solve_separately](/crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately.md) |
| called_by | [round_trip_3x3_case_1](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_1.md) |
| called_by | [round_trip_3x3_case_2_negative_entries](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_2_negative_entries.md) |
| called_by | [round_trip_5x5_diagonally_dominant](/crates/oxide-sketch/tests/linalg/round_trip_5x5_diagonally_dominant.md) |
