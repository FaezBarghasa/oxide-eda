---
okf_version: "0.2"
type: Function
title: rule_parser
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
concept_id: crates/oxide-erc-dsl/src/parser/rule_parser
language: rust
---

# rule_parser

---------------------------------------------------------------------------

## Signature

```rust
fn rule_parser() -> impl Parser<'src, &'src str, RuleAst, Err<'src>>
```

## Type Parameters

- `'src`

## Docstring

---------------------------------------------------------------------------
Rule declaration parser
---------------------------------------------------------------------------

## Source
Lines 84–141 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [string_lit_parser](/crates/oxide-erc-dsl/src/parser/string_lit_parser.md) |
| calls | [expr_parser](/crates/oxide-erc-dsl/src/parser/expr_parser.md) |
| called_by | [program_parser](/crates/oxide-erc-dsl/src/parser/program_parser.md) |
