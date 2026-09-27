---
okf_version: "0.2"
type: Function
title: bump
description: Consume the current look-ahead and refill from the lexer.
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/bump
language: rust
---

# bump

Consume the current look-ahead and refill from the lexer.

## Signature

```rust
impl Parser<'a> { fn bump(&mut self) -> Result<Spanned, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

Consume the current look-ahead and refill from the lexer.

## Source
Lines 352–355 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
