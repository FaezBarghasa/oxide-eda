---
okf_version: "0.2"
type: Function
title: assert_vec_close
description: "Check that two `f64` vectors agree element-wise within `TOL`."
resource: crates/oxide-sketch/tests/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/linalg/assert_vec_close
language: rust
---

# assert_vec_close

Check that two `f64` vectors agree element-wise within `TOL`.

## Signature

```rust
fn assert_vec_close(actual: &[f64], expected: &[f64])
```

## Docstring

Check that two `f64` vectors agree element-wise within `TOL`.

## Source
Lines 14–33 in `crates/oxide-sketch/tests/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/tests/linalg.md) |
| called_by | [lu_decompose_and_solve_separately](/crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately.md) |
| called_by | [round_trip_3x3_case_1](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_1.md) |
| called_by | [round_trip_3x3_case_2_negative_entries](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_2_negative_entries.md) |
| called_by | [round_trip_5x5_diagonally_dominant](/crates/oxide-sketch/tests/linalg/round_trip_5x5_diagonally_dominant.md) |
| called_by | [solve_1x1_scalar](/crates/oxide-sketch/tests/linalg/solve_1x1_scalar.md) |
| called_by | [solve_2x2_trivial](/crates/oxide-sketch/tests/linalg/solve_2x2_trivial.md) |
| called_by | [solve_2x2_zero_diagonal_forces_pivot](/crates/oxide-sketch/tests/linalg/solve_2x2_zero_diagonal_forces_pivot.md) |
| called_by | [solve_3x3_hand_computed](/crates/oxide-sketch/tests/linalg/solve_3x3_hand_computed.md) |
| called_by | [solve_3x3_pivoting_required](/crates/oxide-sketch/tests/linalg/solve_3x3_pivoting_required.md) |
| called_by | [solve_identity_returns_b](/crates/oxide-sketch/tests/linalg/solve_identity_returns_b.md) |
