---
okf_version: "0.2"
type: Function
title: expr_parser
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
concept_id: crates/oxide-erc-dsl/src/parser/expr_parser
language: rust
---

# expr_parser

---------------------------------------------------------------------------

## Signature

```rust
fn expr_parser() -> impl Parser<'src, &'src str, ExprAst, Err<'src>>
```

## Type Parameters

- `'src`

## Docstring

---------------------------------------------------------------------------
Expression parser (precedence: or < and < not < primary)
---------------------------------------------------------------------------

## Source
Lines 147–178 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [primary_parser](/crates/oxide-erc-dsl/src/parser/primary_parser.md) |
| called_by | [rule_parser](/crates/oxide-erc-dsl/src/parser/rule_parser.md) |
