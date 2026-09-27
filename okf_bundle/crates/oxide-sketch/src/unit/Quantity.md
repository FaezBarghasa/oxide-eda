---
okf_version: "0.2"
type: Class
title: Quantity
description: "A scalar value paired with a [`Unit`]."
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/Quantity
language: rust
---

# Quantity

A scalar value paired with a [`Unit`].

## Signature

```rust
pub struct Quantity
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A scalar value paired with a [`Unit`].

`Quantity` is `Copy + Serialize + Deserialize` so it can sit
inside an `ExprNode::Literal` without further indirection.
[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `value`
- `unit`

## Source
Lines 62–65 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
| called_by | [lex_quantity](/crates/oxide-sketch/src/expr/parse/lex_quantity.md) |
