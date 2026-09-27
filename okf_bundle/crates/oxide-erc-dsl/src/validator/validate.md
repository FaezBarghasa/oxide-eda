---
okf_version: "0.2"
type: Function
title: validate
description: "---------------------------------------------------------------------------"
resource: crates/oxide-erc-dsl/src/validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/validator/validate
language: rust
---

# validate

---------------------------------------------------------------------------

## Signature

```rust
pub fn validate(rules: &[RuleAst]) -> Vec<DslError>
```

## Visibility

- `pub`

## Docstring

---------------------------------------------------------------------------
Public entry point
---------------------------------------------------------------------------

## Source
Lines 74–80 in `crates/oxide-erc-dsl/src/validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [validator](/crates/oxide-erc-dsl/src/validator.md) |
| calls | [validate_expr](/crates/oxide-erc-dsl/src/validator/validate_expr.md) |
