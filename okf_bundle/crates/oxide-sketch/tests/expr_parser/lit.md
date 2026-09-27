---
okf_version: "0.2"
type: Function
title: lit
description: "Pull the inner [`Quantity`] out of an [`ExprNode::Literal`], or"
resource: crates/oxide-sketch/tests/expr_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_parser/lit
language: rust
---

# lit

Pull the inner [`Quantity`] out of an [`ExprNode::Literal`], or

## Signature

```rust
fn lit(node: &ExprNode) -> Quantity
```

## Docstring

Pull the inner [`Quantity`] out of an [`ExprNode::Literal`], or
panic with a descriptive message.

## Source
Lines 16–21 in `crates/oxide-sketch/tests/expr_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr_parser](/crates/oxide-sketch/tests/expr_parser.md) |
| called_by | [parse_literal_dimensionless](/crates/oxide-sketch/tests/expr_parser/parse_literal_dimensionless.md) |
| called_by | [parse_literal_mm](/crates/oxide-sketch/tests/expr_parser/parse_literal_mm.md) |
| called_by | [parse_unary_neg](/crates/oxide-sketch/tests/expr_parser/parse_unary_neg.md) |
