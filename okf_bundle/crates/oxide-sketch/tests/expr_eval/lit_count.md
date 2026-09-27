---
okf_version: "0.2"
type: Function
title: lit_count
resource: crates/oxide-sketch/tests/expr_eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_eval/lit_count
language: rust
---

# lit_count

## Signature

```rust
fn lit_count(v: f64) -> ExprNode
```

## Source
Lines 32–34 in `crates/oxide-sketch/tests/expr_eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr_eval](/crates/oxide-sketch/tests/expr_eval.md) |
| called_by | [eval_compare_returns_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_compare_returns_dimensionless.md) |
| called_by | [eval_logical_and_or](/crates/oxide-sketch/tests/expr_eval/eval_logical_and_or.md) |
| called_by | [eval_lookup_match](/crates/oxide-sketch/tests/expr_eval/eval_lookup_match.md) |
| called_by | [eval_lookup_no_match_errors](/crates/oxide-sketch/tests/expr_eval/eval_lookup_no_match_errors.md) |
| called_by | [eval_lookup_shape_mismatch](/crates/oxide-sketch/tests/expr_eval/eval_lookup_shape_mismatch.md) |
| called_by | [eval_mul_length_times_count_keeps_length_unit](/crates/oxide-sketch/tests/expr_eval/eval_mul_length_times_count_keeps_length_unit.md) |
| called_by | [eval_param_ref](/crates/oxide-sketch/tests/expr_eval/eval_param_ref.md) |
| called_by | [eval_pow_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_pow_dimensionless.md) |
| called_by | [eval_pow_length_to_one_is_identity](/crates/oxide-sketch/tests/expr_eval/eval_pow_length_to_one_is_identity.md) |
| called_by | [eval_pow_length_to_two_errors](/crates/oxide-sketch/tests/expr_eval/eval_pow_length_to_two_errors.md) |
| called_by | [eval_pow_with_unit_exponent_errors](/crates/oxide-sketch/tests/expr_eval/eval_pow_with_unit_exponent_errors.md) |
| called_by | [eval_ref_chains_recursively](/crates/oxide-sketch/tests/expr_eval/eval_ref_chains_recursively.md) |
| called_by | [eval_ternary_takes_else](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_else.md) |
| called_by | [eval_ternary_takes_then](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_then.md) |
| called_by | [eval_unary_not_on_nonzero_returns_zero](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_nonzero_returns_zero.md) |
| called_by | [eval_unary_not_on_zero_returns_one](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_zero_returns_one.md) |
