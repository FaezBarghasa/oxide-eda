---
okf_version: "0.2"
type: Function
title: string_lit_parser
description: "Double-quoted string literal, returns the content without quotes."
resource: crates/oxide-erc-dsl/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/parser/string_lit_parser
language: rust
---

# string_lit_parser

Double-quoted string literal, returns the content without quotes.

## Signature

```rust
fn string_lit_parser() -> impl Parser<'src, &'src str, String, Err<'src>> + Clone
```

## Type Parameters

- `'src`

## Docstring

Double-quoted string literal, returns the content without quotes.

## Source
Lines 268–273 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| called_by | [literal_parser](/crates/oxide-erc-dsl/src/parser/literal_parser.md) |
| called_by | [primary_parser](/crates/oxide-erc-dsl/src/parser/primary_parser.md) |
| called_by | [rule_parser](/crates/oxide-erc-dsl/src/parser/rule_parser.md) |
