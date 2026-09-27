---
okf_version: "0.2"
type: Function
title: eval_unaryop
description: "Apply `Neg` or `Not` to a single quantity."
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_unaryop
language: rust
---

# eval_unaryop

Apply `Neg` or `Not` to a single quantity.

## Signature

```rust
fn eval_unaryop(op: UnaryOp, v: Quantity) -> Result<Quantity, ExprError>
```

## Docstring

Apply `Neg` or `Not` to a single quantity.

## Source
Lines 125–136 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
