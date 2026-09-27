---
okf_version: "0.2"
type: Function
title: una
resource: crates/oxide-sketch/tests/expr_eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_eval/una
language: rust
---

# una

## Signature

```rust
fn una(op: UnaryOp, e: ExprNode) -> ExprNode
```

## Source
Lines 44–46 in `crates/oxide-sketch/tests/expr_eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr_eval](/crates/oxide-sketch/tests/expr_eval.md) |
| called_by | [eval_unary_neg](/crates/oxide-sketch/tests/expr_eval/eval_unary_neg.md) |
| called_by | [eval_unary_not_on_nonzero_returns_zero](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_nonzero_returns_zero.md) |
| called_by | [eval_unary_not_on_unit_errors](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_unit_errors.md) |
| called_by | [eval_unary_not_on_zero_returns_one](/crates/oxide-sketch/tests/expr_eval/eval_unary_not_on_zero_returns_one.md) |
