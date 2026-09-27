---
okf_version: "0.2"
type: Module
title: linalg
description: Integration tests for the dense LU linear solver.
resource: crates/oxide-sketch/tests/linalg.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/linalg
language: rust
---

# linalg

Integration tests for the dense LU linear solver.

## Docstring

Integration tests for the dense LU linear solver.

Reference for hand-worked solutions: standard 3×3 systems
(Numerical Recipes §2.3 worked examples). All "expected" values
were verified by hand or by direct substitution.

## Relationships

| Type | Target |
|------|--------|
| related | [assert_vec_close](/crates/oxide-sketch/tests/linalg/assert_vec_close.md) |
| related | [mat_vec](/crates/oxide-sketch/tests/linalg/mat_vec.md) |
| related | [solve_2x2_trivial](/crates/oxide-sketch/tests/linalg/solve_2x2_trivial.md) |
| related | [solve_3x3_hand_computed](/crates/oxide-sketch/tests/linalg/solve_3x3_hand_computed.md) |
| related | [solve_2x2_zero_diagonal_forces_pivot](/crates/oxide-sketch/tests/linalg/solve_2x2_zero_diagonal_forces_pivot.md) |
| related | [solve_3x3_pivoting_required](/crates/oxide-sketch/tests/linalg/solve_3x3_pivoting_required.md) |
| related | [solve_identity_returns_b](/crates/oxide-sketch/tests/linalg/solve_identity_returns_b.md) |
| related | [solve_rank_deficient_2x2_is_singular](/crates/oxide-sketch/tests/linalg/solve_rank_deficient_2x2_is_singular.md) |
| related | [solve_zero_matrix_is_singular](/crates/oxide-sketch/tests/linalg/solve_zero_matrix_is_singular.md) |
| related | [solve_non_square_matrix_errors](/crates/oxide-sketch/tests/linalg/solve_non_square_matrix_errors.md) |
| related | [solve_b_length_mismatch_errors](/crates/oxide-sketch/tests/linalg/solve_b_length_mismatch_errors.md) |
| related | [solve_ragged_matrix_errors](/crates/oxide-sketch/tests/linalg/solve_ragged_matrix_errors.md) |
| related | [round_trip_3x3_case_1](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_1.md) |
| related | [round_trip_3x3_case_2_negative_entries](/crates/oxide-sketch/tests/linalg/round_trip_3x3_case_2_negative_entries.md) |
| related | [round_trip_5x5_diagonally_dominant](/crates/oxide-sketch/tests/linalg/round_trip_5x5_diagonally_dominant.md) |
| related | [lu_decompose_and_solve_separately](/crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately.md) |
| related | [lu_decompose_singular_matrix_errors](/crates/oxide-sketch/tests/linalg/lu_decompose_singular_matrix_errors.md) |
| related | [lu_solve_dimension_mismatch_errors](/crates/oxide-sketch/tests/linalg/lu_solve_dimension_mismatch_errors.md) |
| related | [solve_1x1_scalar](/crates/oxide-sketch/tests/linalg/solve_1x1_scalar.md) |
| related | [solve_1x1_zero_is_singular](/crates/oxide-sketch/tests/linalg/solve_1x1_zero_is_singular.md) |
| related | [qr_rank_full_3x3](/crates/oxide-sketch/tests/linalg/qr_rank_full_3x3.md) |
| related | [qr_rank_full_4x2](/crates/oxide-sketch/tests/linalg/qr_rank_full_4x2.md) |
| related | [qr_rank_deficient](/crates/oxide-sketch/tests/linalg/qr_rank_deficient.md) |
| related | [qr_rank_zero_matrix](/crates/oxide-sketch/tests/linalg/qr_rank_zero_matrix.md) |
