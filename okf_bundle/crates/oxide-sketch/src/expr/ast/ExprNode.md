---
okf_version: "0.2"
type: Class
title: ExprNode
description: One node of an expression tree.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast/ExprNode
language: rust
---

# ExprNode

One node of an expression tree.

## Signature

```rust
pub enum ExprNode
```

## Decorators

- `derive(Clone, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

One node of an expression tree.

All variants carry their own children boxed where recursion is
needed, so an `ExprNode` is `Sized` and small enough to pass by
value without indirection at the top level.
[derive(Clone, Debug, PartialEq)]

## Methods

- `key`
- `keys`
- `values`

## Source
Lines 32–56 in `crates/oxide-sketch/src/expr/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/oxide-sketch/src/expr/ast.md) |
