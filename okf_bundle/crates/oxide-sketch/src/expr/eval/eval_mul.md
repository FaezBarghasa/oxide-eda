---
okf_version: "0.2"
type: Function
title: eval_mul
description: Multiplication — only the family combinations modelled in v0.13.
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_mul
language: rust
---

# eval_mul

Multiplication — only the family combinations modelled in v0.13.

## Signature

```rust
fn eval_mul(lhs: Quantity, rhs: Quantity) -> Result<Quantity, ExprError>
```

## Docstring

Multiplication — only the family combinations modelled in v0.13.

## Source
Lines 163–192 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
