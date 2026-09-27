---
okf_version: "0.2"
type: Function
title: lit
resource: crates/oxide-sketch/tests/expr_eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_eval/lit
language: rust
---

# lit

## Signature

```rust
fn lit(v: f64, u: Unit) -> ExprNode
```

## Source
Lines 28–30 in `crates/oxide-sketch/tests/expr_eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr_eval](/crates/oxide-sketch/tests/expr_eval.md) |
| called_by | [eval_addition_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_addition_unit_conversion.md) |
| called_by | [eval_compare_cross_family_errors](/crates/oxide-sketch/tests/expr_eval/eval_compare_cross_family_errors.md) |
| called_by | [eval_compare_eq_with_tolerance](/crates/oxide-sketch/tests/expr_eval/eval_compare_eq_with_tolerance.md) |
| called_by | [eval_div_length_by_length_with_unit_conversion](/crates/oxide-sketch/tests/expr_eval/eval_div_length_by_length_with_unit_conversion.md) |
| called_by | [eval_pow_with_unit_exponent_errors](/crates/oxide-sketch/tests/expr_eval/eval_pow_with_unit_exponent_errors.md) |
| called_by | [eval_unit_mismatch_errors](/crates/oxide-sketch/tests/expr_eval/eval_unit_mismatch_errors.md) |
