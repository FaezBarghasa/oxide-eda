---
okf_version: "0.2"
type: Function
title: primary_parser
description: "---------------------------------------------------------------------------"
resource: crates/oxide-erc-dsl/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/parser/primary_parser
language: rust
---

# primary_parser

---------------------------------------------------------------------------

## Signature

```rust
fn primary_parser(
    expr: Boxed<'src, 'src, &'src str, ExprAst, Err<'src>>,
) -> impl Parser<'src, &'src str, ExprAst, Err<'src>>
```

## Type Parameters

- `'src`

## Docstring

---------------------------------------------------------------------------
Primary expression parser
---------------------------------------------------------------------------

## Source
Lines 184–261 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [string_lit_parser](/crates/oxide-erc-dsl/src/parser/string_lit_parser.md) |
| calls | [literal_parser](/crates/oxide-erc-dsl/src/parser/literal_parser.md) |
| called_by | [expr_parser](/crates/oxide-erc-dsl/src/parser/expr_parser.md) |
