---
okf_version: "0.2"
type: Function
title: parse_validate_compile
description: "Parse, validate, and compile DSL source in a single call."
resource: crates/oxide-erc-dsl/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc-dsl/src/lib/parse_validate_compile
language: rust
---

# parse_validate_compile

Parse, validate, and compile DSL source in a single call.

## Signature

```rust
pub fn parse_validate_compile(src: &str) -> Result<Vec<CompiledRule>, Vec<DslError>>
```

## Visibility

- `pub`

## Docstring

Parse, validate, and compile DSL source in a single call.

## Source
Lines 37–44 in `crates/oxide-erc-dsl/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc-dsl/src/lib.md) |
| calls | [parse](/crates/oxide-erc-dsl/src/lib/parse.md) |
| calls | [validate](/crates/oxide-erc-dsl/src/lib/validate.md) |
| calls | [compile](/crates/oxide-erc-dsl/src/lib/compile.md) |
| called_by | [invalid_regex_returns_compile_error](/crates/oxide-erc-dsl/src/lib/invalid_regex_returns_compile_error.md) |
| called_by | [parse_validate_compile_to_eval_fns](/crates/oxide-erc-dsl/src/lib/parse_validate_compile_to_eval_fns.md) |
