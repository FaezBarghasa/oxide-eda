---
okf_version: "0.2"
type: Function
title: literal_parser
description: "Literal value: `true` | `false` | `\"string\"` | ident."
resource: crates/oxide-erc-dsl/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/parser/literal_parser
language: rust
---

# literal_parser

Literal value: `true` | `false` | `"string"` | ident.

## Signature

```rust
fn literal_parser() -> impl Parser<'src, &'src str, LiteralAst, Err<'src>> + Clone
```

## Type Parameters

- `'src`

## Docstring

Literal value: `true` | `false` | `"string"` | ident.

## Source
Lines 276–286 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [string_lit_parser](/crates/oxide-erc-dsl/src/parser/string_lit_parser.md) |
| called_by | [primary_parser](/crates/oxide-erc-dsl/src/parser/primary_parser.md) |
