---
okf_version: "0.2"
type: Module
title: eval
description: Expression evaluator for the parametric sketch parameter table.
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval
language: rust
---

# eval

Expression evaluator for the parametric sketch parameter table.

## Docstring

Expression evaluator for the parametric sketch parameter table.

Walks the [`ExprNode`] tree and produces a [`Quantity`] with
unit-family type-checking on every binary op. Cleanroom; no
third-party expression-evaluator source consulted.

# Semantics summary

- `Add` / `Sub` — operands must share a unit family. Length+Length
converts RHS to LHS unit before adding; the result keeps LHS's
unit. Same for Angle+Angle and Dimensionless+Dimensionless.
- `Mul` — `Length × Dimensionless → Length`,
`Angle × Dimensionless → Angle`,
`Dimensionless × Dimensionless → Dimensionless`. `Length × Length`
would produce an Area, which we don't model in v0.13, so it errors.
- `Div` / `Mod` — `Length / Length → Dimensionless`,
`Length / Dimensionless → Length`, `Angle / Dimensionless → Angle`,
`Dimensionless / Dimensionless → Dimensionless`.
- `Pow` — `Dimensionless ^ Dimensionless → Dimensionless` is the
only supported case; `Length ^ n` for `n != 1` would produce an
Area or higher, which we don't model.
- Comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) — operands must
share a family; the result is a `Dimensionless` `0.0` or `1.0`.
`==` and `!=` use a `1e-12` tolerance against the canonical value
to avoid exact-equality footguns; the strict orderings use exact
`f64` comparison.
- Logical (`&&`, `||`) — operands must be `Dimensionless`; any
non-zero value is true.
- `Unary(Neg)` — negate the value, keep the unit.
- `Unary(Not)` — operand must be `Dimensionless`; result is the
logical inverse as `Dimensionless` `0.0` or `1.0`.
- `Ternary` — condition must be `Dimensionless`.
- `Lookup` — match key against each entry of `keys` (same family +
value within tolerance) and return `eval(values[i])`. Errors if
no match or shapes differ.

## Relationships

| Type | Target |
|------|--------|
| related | [EvalContext](/crates/oxide-sketch/src/expr/eval/EvalContext.md) |
| related | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
| related | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
| related | [eval_unaryop](/crates/oxide-sketch/src/expr/eval/eval_unaryop.md) |
| related | [eval_add_sub](/crates/oxide-sketch/src/expr/eval/eval_add_sub.md) |
| related | [eval_mul](/crates/oxide-sketch/src/expr/eval/eval_mul.md) |
| related | [eval_div_mod](/crates/oxide-sketch/src/expr/eval/eval_div_mod.md) |
| related | [check_nonzero](/crates/oxide-sketch/src/expr/eval/check_nonzero.md) |
| related | [eval_pow](/crates/oxide-sketch/src/expr/eval/eval_pow.md) |
| related | [CompareOp](/crates/oxide-sketch/src/expr/eval/CompareOp.md) |
| related | [eval_compare](/crates/oxide-sketch/src/expr/eval/eval_compare.md) |
| related | [eval_logical](/crates/oxide-sketch/src/expr/eval/eval_logical.md) |
| related | [eval_lookup](/crates/oxide-sketch/src/expr/eval/eval_lookup.md) |
| related | [convert_to](/crates/oxide-sketch/src/expr/eval/convert_to.md) |
| related | [canonical_pair](/crates/oxide-sketch/src/expr/eval/canonical_pair.md) |
| related | [same_canonical](/crates/oxide-sketch/src/expr/eval/same_canonical.md) |
| related | [ctx](/crates/oxide-sketch/src/expr/eval/ctx.md) |
| related | [literal_passes_through](/crates/oxide-sketch/src/expr/eval/literal_passes_through.md) |
| related | [add_same_unit](/crates/oxide-sketch/src/expr/eval/add_same_unit.md) |
| related | [add_mixed_lengths_uses_lhs_unit](/crates/oxide-sketch/src/expr/eval/add_mixed_lengths_uses_lhs_unit.md) |
| related | [unit_mismatch_errors](/crates/oxide-sketch/src/expr/eval/unit_mismatch_errors.md) |
| related | [div_by_zero_length_returns_domain_error](/crates/oxide-sketch/src/expr/eval/div_by_zero_length_returns_domain_error.md) |
| related | [div_by_zero_count_returns_domain_error](/crates/oxide-sketch/src/expr/eval/div_by_zero_count_returns_domain_error.md) |
| related | [mod_by_zero_returns_domain_error](/crates/oxide-sketch/src/expr/eval/mod_by_zero_returns_domain_error.md) |
| related | [mod_by_zero_length_returns_domain_error](/crates/oxide-sketch/src/expr/eval/mod_by_zero_length_returns_domain_error.md) |
| related | [div_by_zero_angle_returns_domain_error](/crates/oxide-sketch/src/expr/eval/div_by_zero_angle_returns_domain_error.md) |
| related | [nonzero_divisor_still_works](/crates/oxide-sketch/src/expr/eval/nonzero_divisor_still_works.md) |
