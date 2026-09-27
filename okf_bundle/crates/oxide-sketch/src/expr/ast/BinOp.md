---
okf_version: "0.2"
type: Class
title: BinOp
description: Binary infix operator.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast/BinOp
language: rust
---

# BinOp

Binary infix operator.

## Signature

```rust
pub enum BinOp
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Binary infix operator.

Comparison and logical operators yield a dimensionless `0.0` or
`1.0` so they can be composed with arithmetic in the same
expression (e.g. `(a > b) * 5mm`).
[derive(Clone, Copy, Debug, PartialEq, Eq)]

## Source
Lines 64–82 in `crates/oxide-sketch/src/expr/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/oxide-sketch/src/expr/ast.md) |
