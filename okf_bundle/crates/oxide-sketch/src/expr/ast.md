---
okf_version: "0.2"
type: Module
title: ast
description: Expression AST for the parametric sketch parameter table.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast
language: rust
---

# ast

Expression AST for the parametric sketch parameter table.

## Docstring

Expression AST for the parametric sketch parameter table.

The AST is constructed by the parser (`crate::expr::parse`) from the
source `String` stored in `ParameterTable`. The AST itself is *not*
serialised; the persisted form is the source string, and the AST is
rebuilt by re-parsing on load. This keeps the on-disk format simple
and human-editable.

# Design

Standard pure-functional expression tree:

- [`ExprNode::Literal`] — a [`Quantity`] (value + unit)
- [`ExprNode::Ref`] — named parameter reference
- [`ExprNode::ArrayIndex`] — `i` / `j` inside an array's per-instance
expressions or depopulation mask
- [`ExprNode::Binary`] — infix binary operators (arithmetic,
comparison, logical)
- [`ExprNode::Unary`] — prefix unary operators (negate, logical not)
- [`ExprNode::Ternary`] — `cond ? then : else`
- [`ExprNode::Lookup`] — table lookup, used for parametric pad-shape
selection by package family

## Relationships

| Type | Target |
|------|--------|
| related | [ExprNode](/crates/oxide-sketch/src/expr/ast/ExprNode.md) |
| related | [BinOp](/crates/oxide-sketch/src/expr/ast/BinOp.md) |
| related | [UnaryOp](/crates/oxide-sketch/src/expr/ast/UnaryOp.md) |
| related | [ArrayIndex](/crates/oxide-sketch/src/expr/ast/ArrayIndex.md) |
| related | [literal_mm](/crates/oxide-sketch/src/expr/ast/literal_mm.md) |
| related | [literal_count](/crates/oxide-sketch/src/expr/ast/literal_count.md) |
| related | [literal_mm](/crates/oxide-sketch/src/expr/ast/literal_mm.md) |
| related | [literal_count](/crates/oxide-sketch/src/expr/ast/literal_count.md) |
| related | [binop_eq_compare](/crates/oxide-sketch/src/expr/ast/binop_eq_compare.md) |
| related | [unary_eq_compare](/crates/oxide-sketch/src/expr/ast/unary_eq_compare.md) |
| related | [expr_node_clone_roundtrip](/crates/oxide-sketch/src/expr/ast/expr_node_clone_roundtrip.md) |
| related | [tree_shape_assertions](/crates/oxide-sketch/src/expr/ast/tree_shape_assertions.md) |
