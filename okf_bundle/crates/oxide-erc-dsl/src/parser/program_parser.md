---
okf_version: "0.2"
type: Function
title: program_parser
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
concept_id: crates/oxide-erc-dsl/src/parser/program_parser
language: rust
---

# program_parser

---------------------------------------------------------------------------

## Signature

```rust
fn program_parser() -> impl Parser<'src, &'src str, Vec<RuleAst>, Err<'src>>
```

## Type Parameters

- `'src`

## Docstring

---------------------------------------------------------------------------
Program parser
---------------------------------------------------------------------------

## Source
Lines 72–78 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [rule_parser](/crates/oxide-erc-dsl/src/parser/rule_parser.md) |
| called_by | [parse](/crates/oxide-erc-dsl/src/parser/parse.md) |
