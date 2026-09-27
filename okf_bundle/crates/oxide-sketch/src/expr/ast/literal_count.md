---
okf_version: "0.2"
type: Function
title: literal_count
description: Construct a dimensionless count literal.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast/literal_count
language: rust
---

# literal_count

Construct a dimensionless count literal.

## Signature

```rust
impl ExprNode { pub fn literal_count(value: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct a dimensionless count literal.

## Source
Lines 110–112 in `crates/oxide-sketch/src/expr/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/oxide-sketch/src/expr/ast.md) |
