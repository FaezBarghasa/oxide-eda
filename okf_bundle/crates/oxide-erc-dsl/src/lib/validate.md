---
okf_version: "0.2"
type: Function
title: validate
description: Validate parsed rules against helper and field compatibility constraints.
resource: crates/oxide-erc-dsl/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc-dsl/src/lib/validate
language: rust
---

# validate

Validate parsed rules against helper and field compatibility constraints.

## Signature

```rust
pub fn validate(rules: &[RuleAst]) -> Vec<DslError>
```

## Visibility

- `pub`

## Docstring

Validate parsed rules against helper and field compatibility constraints.

## Source
Lines 27–29 in `crates/oxide-erc-dsl/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc-dsl/src/lib.md) |
| called_by | [parse_validate_compile](/crates/oxide-erc-dsl/src/lib/parse_validate_compile.md) |
