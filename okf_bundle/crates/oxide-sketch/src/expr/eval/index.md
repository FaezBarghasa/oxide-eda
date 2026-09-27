# eval

## Classs

- [CompareOp](CompareOp.md) — Comparison operators classified for [`eval_compare`].
- [EvalContext](EvalContext.md) — Per-evaluation context — the parameter table (raw AST values, so

## Functions

- [add_mixed_lengths_uses_lhs_unit](add_mixed_lengths_uses_lhs_unit.md) — [test]
- [add_same_unit](add_same_unit.md) — [test]
- [canonical_pair](canonical_pair.md) — Convert two same-family quantities to canonical units (mm / rad /
- [check_nonzero](check_nonzero.md) — Reject a zero divisor before we feed it to `/` or `%`. Pure-zero
- [convert_to](convert_to.md) — Convert `q` into the unit `target_unit`. Caller has already
- [ctx](ctx.md)
- [div_by_zero_angle_returns_domain_error](div_by_zero_angle_returns_domain_error.md) — [test]
- [div_by_zero_count_returns_domain_error](div_by_zero_count_returns_domain_error.md) — [test]
- [div_by_zero_length_returns_domain_error](div_by_zero_length_returns_domain_error.md) — [test]
- [eval](eval.md) — Walk `node` and produce a [`Quantity`].
- [eval_add_sub](eval_add_sub.md) — Add / subtract — operands must share a family. RHS is converted to
- [eval_binop](eval_binop.md) — Apply a binary operator to two already-evaluated quantities.
- [eval_compare](eval_compare.md) — Comparison — operands must share a family; canonicalise both sides
- [eval_div_mod](eval_div_mod.md) — Division and modulus — same family combinations on both sides.
- [eval_logical](eval_logical.md) — Logical `&&` / `||` — both operands must be `Dimensionless`; non-zero
- [eval_lookup](eval_lookup.md) — `Lookup` — find the index `i` such that `key ≈ keys[i]` and return
- [eval_mul](eval_mul.md) — Multiplication — only the family combinations modelled in v0.13.
- [eval_pow](eval_pow.md) — Power — only `Dimensionless ^ Dimensionless` is supported in v0.13.
- [eval_unaryop](eval_unaryop.md) — Apply `Neg` or `Not` to a single quantity.
- [literal_passes_through](literal_passes_through.md) — [test]
- [mod_by_zero_length_returns_domain_error](mod_by_zero_length_returns_domain_error.md) — [test]
- [mod_by_zero_returns_domain_error](mod_by_zero_returns_domain_error.md) — [test]
- [nonzero_divisor_still_works](nonzero_divisor_still_works.md) — [test]
- [same_canonical](same_canonical.md) — Two quantities count as equal for `Lookup` matching iff they share
- [unit_mismatch_errors](unit_mismatch_errors.md) — [test]
