---
okf_version: "0.2"
type: Class
title: ExprError
description: Errors produced by the expression layer (parse + evaluate +
resource: crates/oxide-sketch/src/expr/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/mod/ExprError
language: rust
---

# ExprError

Errors produced by the expression layer (parse + evaluate +

## Signature

```rust
pub enum ExprError
```

## Decorators

- `derive(Clone, Debug, PartialEq, Error)`

## Visibility

- `pub`

## Docstring

Errors produced by the expression layer (parse + evaluate +
parameter resolution).
[derive(Clone, Debug, PartialEq, Error)]

## Methods

- `pos`
- `msg`
- `lhs`
- `rhs`
- `expected`
- `got`

## Source
Lines 24–70 in `crates/oxide-sketch/src/expr/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr](/crates/oxide-sketch/src/expr/mod.md) |
