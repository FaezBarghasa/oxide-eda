---
okf_version: "0.2"
type: Class
title: Token
description: One lexical token.
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/Token
language: rust
---

# Token

One lexical token.

## Signature

```rust
enum Token
```

## Decorators

- `derive(Clone, Debug, PartialEq)`

## Docstring

One lexical token.

`Quantity` carries the raw source slice (digits + unit suffix) so
the parser can hand it to [`crate::unit::parse_quantity`] without
re-lexing.
[derive(Clone, Debug, PartialEq)]

## Source
Lines 57–83 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
