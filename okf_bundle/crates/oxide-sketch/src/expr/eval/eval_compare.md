---
okf_version: "0.2"
type: Function
title: eval_compare
description: Comparison — operands must share a family; canonicalise both sides
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_compare
language: rust
---

# eval_compare

Comparison — operands must share a family; canonicalise both sides

## Signature

```rust
fn eval_compare(lhs: Quantity, rhs: Quantity, op: CompareOp) -> Result<Quantity, ExprError>
```

## Docstring

Comparison — operands must share a family; canonicalise both sides
(mm, rad, or raw) and compare. Returns a `Dimensionless` `0.0` or
`1.0`.

## Source
Lines 299–318 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [canonical_pair](/crates/oxide-sketch/src/expr/eval/canonical_pair.md) |
| called_by | [eval_binop](/crates/oxide-sketch/src/expr/eval/eval_binop.md) |
