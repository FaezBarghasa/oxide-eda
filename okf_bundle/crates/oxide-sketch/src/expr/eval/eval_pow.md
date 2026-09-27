---
okf_version: "0.2"
type: Function
title: eval_pow
description: "Power — only `Dimensionless ^ Dimensionless` is supported in v0.13."
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_pow
language: rust
---

# eval_pow

Power — only `Dimensionless ^ Dimensionless` is supported in v0.13.

## Signature

```rust
fn eval_pow(base: Quantity, exp: Quantity) -> Result<Quantity, ExprError>
```

## Docstring

Power — only `Dimensionless ^ Dimensionless` is supported in v0.13.
`Length ^ 2` would need an Area unit; non-integer powers of Length
are similarly meaningless without a richer unit model.

## Source
Lines 260–283 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
