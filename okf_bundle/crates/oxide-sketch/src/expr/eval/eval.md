---
okf_version: "0.2"
type: Function
title: eval
description: "Walk `node` and produce a [`Quantity`]."
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval
language: rust
---

# eval

Walk `node` and produce a [`Quantity`].

## Signature

```rust
pub fn eval(node: &ExprNode, ctx: &EvalContext) -> Result<Quantity, ExprError>
```

## Visibility

- `pub`

## Docstring

Walk `node` and produce a [`Quantity`].

Recursive interpreter; each binary op is type-checked by family
before reduction. Errors are bubbled up as [`ExprError`].

## Source
Lines 65–102 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
| calls | [eval_unaryop](/crates/oxide-sketch/src/expr/eval/eval_unaryop.md) |
| calls | [eval_lookup](/crates/oxide-sketch/src/expr/eval/eval_lookup.md) |
| called_by | [add_mixed_lengths_uses_lhs_unit](/crates/oxide-sketch/src/expr/eval/add_mixed_lengths_uses_lhs_unit.md) |
| called_by | [add_same_unit](/crates/oxide-sketch/src/expr/eval/add_same_unit.md) |
| called_by | [eval_lookup](/crates/oxide-sketch/src/expr/eval/eval_lookup.md) |
| called_by | [literal_passes_through](/crates/oxide-sketch/src/expr/eval/literal_passes_through.md) |
| called_by | [nonzero_divisor_still_works](/crates/oxide-sketch/src/expr/eval/nonzero_divisor_still_works.md) |
| called_by | [resolve](/crates/oxide-sketch/src/parameter/resolve.md) |
| called_by | [eval_addition_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_addition_unit_conversion.md) |
| called_by | [eval_array_index_in_context](/crates/oxide-sketch/tests/expr_eval/eval_array_index_in_context.md) |
| called_by | [eval_array_index_j](/crates/oxide-sketch/tests/expr_eval/eval_array_index_j.md) |
| called_by | [eval_array_index_outside_errors](/crates/oxide-sketch/tests/expr_eval/eval_array_index_outside_errors.md) |
| called_by | [eval_compare_eq_with_tolerance](/crates/oxide-sketch/tests/expr_eval/eval_compare_eq_with_tolerance.md) |
| called_by | [eval_compare_returns_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_compare_returns_dimensionless.md) |
| called_by | [eval_div_length_by_length_returns_count](/crates/oxide-sketch/tests/expr_eval/eval_div_length_by_length_returns_count.md) |
| called_by | [eval_div_length_by_length_with_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_div_length_by_length_with_unit_conversion.md) |
| called_by | [eval_literal_mm](/crates/oxide-sketch/tests/expr_eval/eval_literal_mm.md) |
| called_by | [eval_logical_and_or](/crates/oxide-sketch/tests/expr_eval/eval_logical_and_or.md) |
| called_by | [eval_lookup_match](/crates/oxide-sketch/tests/expr_eval/eval_lookup_match.md) |
| called_by | [eval_lookup_with_unit_conversion_in_keys](/crates/oxide-sketch/tests/expr_eval/eval_lookup_with_unit_conversion_in_keys.md) |
| called_by | [eval_mod_length_by_length](/crates/oxide-sketch/tests/expr_eval/eval_mod_length_by_length.md) |
| called_by | [eval_mul_length_times_count_keeps_length_unit](/crates/oxide-sketch/tests/expr_eval/eval_mul_length_times_count_keeps_length_unit.md) |
| called_by | [eval_param_ref](/crates/oxide-sketch/tests/expr_eval/eval_param_ref.md) |
| called_by | [eval_pow_dimensionless](/crates/oxide-sketch/tests/expr_eval/eval_pow_dimensionless.md) |
| called_by | [eval_pow_length_to_one_is_identity](/crates/oxide-sketch/tests/expr_eval/eval_pow_length_to_one_is_identity.md) |
| called_by | [eval_ref_chains_recursively](/crates/oxide-sketch/tests/expr_eval/eval_ref_chains_recursively.md) |
| called_by | [eval_ternary_takes_else](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_else.md) |
| called_by | [eval_ternary_takes_then](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_then.md) |
| called_by | [eval_unary_neg](/crates/oxide-sketch/tests/expr_eval/eval_unary_neg.md) |
| called_by | [eval_unary_not_on_nonzero_returns_zero](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_nonzero_returns_zero.md) |
| called_by | [eval_unary_not_on_unit_errors](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_unit_errors.md) |
| called_by | [eval_unary_not_on_zero_returns_one](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_zero_returns_one.md) |
| called_by | [eval_unit_mismatch_errors](/crates/oxide-sketch/tests/expr_eval/eval_unit_mismatch_errors.md) |
| called_by | [eval_unknown_param_errors](/crates/oxide-sketch/tests/expr_eval/eval_unknown_param_errors.md) |
