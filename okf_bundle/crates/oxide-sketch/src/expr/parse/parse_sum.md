---
okf_version: "0.2"
type: Function
title: parse_sum
description: "-- sum / product / power ----------------------------------------"
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse_sum
language: rust
---

# parse_sum

-- sum / product / power ----------------------------------------

## Signature

```rust
impl Parser<'a> { fn parse_sum(&mut self) -> Result<ExprNode, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

-- sum / product / power ----------------------------------------

## Source
Lines 454–467 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
