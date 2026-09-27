---
okf_version: "0.2"
type: Function
title: parse_primary
description: "-- primary -------------------------------------------------------"
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse_primary
language: rust
---

# parse_primary

-- primary -------------------------------------------------------

## Signature

```rust
impl Parser<'a> { fn parse_primary(&mut self) -> Result<ExprNode, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

-- primary -------------------------------------------------------

## Source
Lines 520–559 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
| calls | [parse_quantity](/crates/oxide-sketch/src/unit/parse_quantity.md) |
| calls | [ArrayIndex](/crates/oxide-sketch/src/expr/ast/ArrayIndex.md) |
