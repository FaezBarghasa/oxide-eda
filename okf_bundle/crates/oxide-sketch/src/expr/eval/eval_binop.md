---
okf_version: "0.2"
type: Function
title: eval_binop
description: Apply a binary operator to two already-evaluated quantities.
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_binop
language: rust
---

# eval_binop

Apply a binary operator to two already-evaluated quantities.

## Signature

```rust
fn eval_binop(op: BinOp, lhs: Quantity, rhs: Quantity) -> Result<Quantity, ExprError>
```

## Docstring

Apply a binary operator to two already-evaluated quantities.

## Source
Lines 105–122 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [eval_add_sub](/crates/oxide-sketch/src/expr/eval/eval_add_sub.md) |
| calls | [eval_mul](/crates/oxide-sketch/src/expr/eval/eval_mul.md) |
| calls | [eval_div_mod](/crates/oxide-sketch/src/expr/eval/eval_div_mod.md) |
| calls | [eval_pow](/crates/oxide-sketch/src/expr/eval/eval_pow.md) |
| calls | [eval_compare](/crates/oxide-sketch/src/expr/eval/eval_compare.md) |
| calls | [eval_logical](/crates/oxide-sketch/src/expr/eval/eval_logical.md) |
| called_by | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
