---
okf_version: "0.2"
type: Function
title: peek_byte_at
description: Look at the byte one past the current cursor.
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/peek_byte_at
language: rust
---

# peek_byte_at

Look at the byte one past the current cursor.

## Signature

```rust
impl Lexer<'a> { fn peek_byte_at(&self, offset: usize) -> Option<u8> }
```

## Type Parameters

- `'a`

## Docstring

Look at the byte one past the current cursor.

## Source
Lines 127–129 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
