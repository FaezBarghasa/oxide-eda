---
okf_version: "0.2"
type: Function
title: lex_ident
description: "Eat an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`. The cursor is on"
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/lex_ident_1
language: rust
---

# lex_ident

Eat an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`. The cursor is on

## Signature

```rust
fn lex_ident(&mut self) -> Token
```

## Docstring

Eat an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`. The cursor is on
an alpha or underscore when this is called.

## Source
Lines 317–328 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
