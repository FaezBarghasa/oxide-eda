---
okf_version: "0.2"
type: Function
title: eval_logical
description: "Logical `&&` / `||` — both operands must be `Dimensionless`; non-zero"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_logical
language: rust
---

# eval_logical

Logical `&&` / `||` — both operands must be `Dimensionless`; non-zero

## Signature

```rust
fn eval_logical(
    lhs: Quantity,
    rhs: Quantity,
    f: fn(bool, bool) -> bool,
) -> Result<Quantity, ExprError>
```

## Docstring

Logical `&&` / `||` — both operands must be `Dimensionless`; non-zero
is true.

## Source
Lines 322–331 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
