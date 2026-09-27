---
okf_version: "0.2"
type: Function
title: eval_div_mod
description: Division and modulus — same family combinations on both sides.
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_div_mod
language: rust
---

# eval_div_mod

Division and modulus — same family combinations on both sides.

## Signature

```rust
fn eval_div_mod(
    lhs: Quantity,
    rhs: Quantity,
    f: fn(f64, f64) -> f64,
) -> Result<Quantity, ExprError>
```

## Docstring

Division and modulus — same family combinations on both sides.
Length / Length collapses to Dimensionless; the others mirror
[`eval_mul`]. A zero divisor on any branch surfaces
[`ExprError::Domain`] rather than producing inf / NaN — `1mm / 0`
and `0 % 0` would otherwise flow into the LM solver as a poisoned
state and silently break convergence.

## Source
Lines 200–244 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [check_nonzero](/crates/oxide-sketch/src/expr/eval/check_nonzero.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
