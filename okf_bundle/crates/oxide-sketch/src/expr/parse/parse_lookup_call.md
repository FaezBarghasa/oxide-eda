---
okf_version: "0.2"
type: Function
title: parse_lookup_call
description: "`lookup` has already been consumed and `(` is the look-ahead."
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse_lookup_call
language: rust
---

# parse_lookup_call

`lookup` has already been consumed and `(` is the look-ahead.

## Signature

```rust
impl Parser<'a> { fn parse_lookup_call(&mut self, call_pos: usize) -> Result<ExprNode, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

`lookup` has already been consumed and `(` is the look-ahead.

## Source
Lines 562–591 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
