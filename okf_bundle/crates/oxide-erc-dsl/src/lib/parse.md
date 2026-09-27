---
okf_version: "0.2"
type: Function
title: parse
description: "Parse DSL source text into `RuleAst` items."
resource: crates/oxide-erc-dsl/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc-dsl/src/lib/parse
language: rust
---

# parse

Parse DSL source text into `RuleAst` items.

## Signature

```rust
pub fn parse(src: &str) -> Result<Vec<RuleAst>, Vec<DslError>>
```

## Visibility

- `pub`

## Docstring

Parse DSL source text into `RuleAst` items.

This API converts parser diagnostics into `DslError::Parse` values.

## Source
Lines 16–24 in `crates/oxide-erc-dsl/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc-dsl/src/lib.md) |
| called_by | [parse_validate_compile](/crates/oxide-erc-dsl/src/lib/parse_validate_compile.md) |
