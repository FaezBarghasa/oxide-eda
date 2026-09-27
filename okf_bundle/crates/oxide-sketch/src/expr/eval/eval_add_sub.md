---
okf_version: "0.2"
type: Function
title: eval_add_sub
description: Add / subtract — operands must share a family. RHS is converted to
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_add_sub
language: rust
---

# eval_add_sub

Add / subtract — operands must share a family. RHS is converted to

## Signature

```rust
fn eval_add_sub(
    lhs: Quantity,
    rhs: Quantity,
    f: fn(f64, f64) -> f64,
) -> Result<Quantity, ExprError>
```

## Docstring

Add / subtract — operands must share a family. RHS is converted to
LHS's unit before applying `f`; the result keeps LHS's unit.

## Source
Lines 140–160 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [convert_to](/crates/oxide-sketch/src/expr/eval/convert_to.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
