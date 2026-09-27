---
okf_version: "0.2"
type: Function
title: expect
description: "Consume the current token if it equals `expected`, otherwise"
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/expect
language: rust
---

# expect

Consume the current token if it equals `expected`, otherwise

## Signature

```rust
impl Parser<'a> { fn expect(&mut self, expected: &Token, what: &str) -> Result<(), ExprError> }
```

## Type Parameters

- `'a`

## Docstring

Consume the current token if it equals `expected`, otherwise
produce a parse error pointing at the look-ahead's position.

## Source
Lines 359–369 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
