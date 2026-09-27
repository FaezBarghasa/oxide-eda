---
okf_version: "0.2"
type: Module
title: expr_eval
description: Integration tests for the expression evaluator
resource: crates/oxide-sketch/tests/expr_eval.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_eval
language: rust
---

# expr_eval

Integration tests for the expression evaluator

## Docstring

Integration tests for the expression evaluator
(`crates/oxide-sketch/src/expr/eval.rs`).

Covers Task 4.4 of `docs/internal/SKETCH_MODE_v0.13_PLAN.md`.

ASTs are built by hand because the recursive-descent parser
(Task 4.3) is being implemented in a sibling agent and may not
have landed yet. Once it does, these tests can be rewritten with
`parse(...)`; the evaluator semantics they assert are independent.

## Relationships

| Type | Target |
|------|--------|
| related | [lit_mm](/crates/oxide-sketch/tests/expr_eval/lit_mm.md) |
| related | [lit](/crates/oxide-sketch/tests/expr_eval/lit.md) |
| related | [lit_count](/crates/oxide-sketch/tests/expr_eval/lit_count.md) |
| related | [r#ref](/crates/oxide-sketch/tests/expr_eval/r_ref.md) |
| related | [bin](/crates/oxide-sketch/tests/expr_eval/bin.md) |
| related | [una](/crates/oxide-sketch/tests/expr_eval/una.md) |
| related | [ternary](/crates/oxide-sketch/tests/expr_eval/ternary.md) |
| related | [ctx_with](/crates/oxide-sketch/tests/expr_eval/ctx_with.md) |
| related | [eval_literal_mm](/crates/oxide-sketch/tests/expr_eval/eval_literal_mm.md) |
| related | [eval_addition_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_addition_unit_conversion.md) |
| related | [eval_param_ref](/crates/oxide-sketch/tests/expr_eval/eval_param_ref.md) |
| related | [eval_ternary_takes_then](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_then.md) |
| related | [eval_ternary_takes_else](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_else.md) |
| related | [eval_lookup_match](/crates/oxide-sketch/tests/expr_eval/eval_lookup_match.md) |
| related | [eval_lookup_no_match_errors](/crates/oxide-sketch/tests/expr_eval/eval_lookup_no_match_errors.md) |
| related | [eval_unary_neg](/crates/oxide-sketch/tests/expr_eval/eval_unary_neg.md) |
| related | [eval_unit_mismatch_errors](/crates/oxide-sketch/tests/expr_eval/eval_unit_mismatch_errors.md) |
| related | [eval_unknown_param_errors](/crates/oxide-sketch/tests/expr_eval/eval_unknown_param_errors.md) |
| related | [eval_array_index_in_context](/crates/oxide-sketch/tests/expr_eval/eval_array_index_in_context.md) |
| related | [eval_array_index_outside_errors](/crates/oxide-sketch/tests/expr_eval/eval_array_index_outside_errors.md) |
| related | [eval_compare_returns_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_compare_returns_dimensionless.md) |
| related | [eval_logical_and_or](/crates/oxide-sketch/tests/expr_eval/eval_logical_and_or.md) |
| related | [eval_pow_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_pow_dimensionless.md) |
| related | [eval_mul_length_times_count_keeps_length_unit](/crates/oxide-sketch/tests/expr_eval/eval_mul_length_times_count_keeps_length_unit.md) |
| related | [eval_mul_length_times_length_errors](/crates/oxide-sketch/tests/expr_eval/eval_mul_length_times_length_errors.md) |
| related | [eval_div_length_by_length_returns_count](/crates/oxide-sketch/tests/expr_eval/eval_div_length_by_length_returns_count.md) |
| related | [eval_div_length_by_length_with_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_div_length_by_length_with_unit_conversion.md) |
| related | [eval_mod_length_by_length](/crates/oxide-sketch/tests/expr_eval/eval_mod_length_by_length.md) |
| related | [eval_unary_not_on_zero_returns_one](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_zero_returns_one.md) |
| related | [eval_unary_not_on_nonzero_returns_zero](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_nonzero_returns_zero.md) |
| related | [eval_unary_not_on_unit_errors](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_unit_errors.md) |
| related | [eval_lookup_shape_mismatch](/crates/oxide-sketch/tests/expr_eval/eval_lookup_shape_mismatch.md) |
| related | [eval_lookup_with_unit_conversion_in_keys](/crates/oxide-sketch/tests/expr_eval/eval_lookup_with_unit_conversion_in_keys.md) |
| related | [eval_ref_chains_recursively](/crates/oxide-sketch/tests/expr_eval/eval_ref_chains_recursively.md) |
| related | [eval_compare_cross_family_errors](/crates/oxide-sketch/tests/expr_eval/eval_compare_cross_family_errors.md) |
| related | [eval_compare_eq_with_tolerance](/crates/oxide-sketch/tests/expr_eval/eval_compare_eq_with_tolerance.md) |
| related | [eval_pow_length_to_one_is_identity](/crates/oxide-sketch/tests/expr_eval/eval_pow_length_to_one_is_identity.md) |
| related | [eval_pow_length_to_two_errors](/crates/oxide-sketch/tests/expr_eval/eval_pow_length_to_two_errors.md) |
| related | [eval_pow_with_unit_exponent_errors](/crates/oxide-sketch/tests/expr_eval/eval_pow_with_unit_exponent_errors.md) |
| related | [eval_array_index_j](/crates/oxide-sketch/tests/expr_eval/eval_array_index_j.md) |
