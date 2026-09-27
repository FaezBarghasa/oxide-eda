---
okf_version: "0.2"
type: Function
title: next_token
description: "Produce the next token, advancing the cursor."
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/next_token
language: rust
---

# next_token

Produce the next token, advancing the cursor.

## Signature

```rust
impl Lexer<'a> { fn next_token(&mut self) -> Result<Spanned, ExprError> }
```

## Type Parameters

- `'a`

## Docstring

Produce the next token, advancing the cursor.

## Source
Lines 132–288 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
