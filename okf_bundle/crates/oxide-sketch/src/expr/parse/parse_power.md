---
okf_version: "0.2"
type: Function
title: parse_power
description: "Right-associative: `2^3^2` = `2^(3^2)` = 512."
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse_power
language: rust
---

# parse_power

Right-associative: `2^3^2` = `2^(3^2)` = 512.

## Signature

```rust
impl Parser<'a> { fn parse_power(&mut self) -> Result<ExprNode, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

Right-associative: `2^3^2` = `2^(3^2)` = 512.

## Source
Lines 486–498 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
