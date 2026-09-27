---
okf_version: "0.2"
type: Function
title: validate_expr
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
concept_id: crates/oxide-erc-dsl/src/validator/validate_expr
language: rust
---

# validate_expr

---------------------------------------------------------------------------

## Signature

```rust
fn validate_expr(rule_id: &str, target: TargetKind, expr: &ExprAst, out: &mut Vec<DslError>)
```

## Docstring

---------------------------------------------------------------------------
Expression traversal
---------------------------------------------------------------------------

## Source
Lines 86–100 in `crates/oxide-erc-dsl/src/validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [validator](/crates/oxide-erc-dsl/src/validator.md) |
| calls | [validate_helper](/crates/oxide-erc-dsl/src/validator/validate_helper.md) |
| calls | [validate_field](/crates/oxide-erc-dsl/src/validator/validate_field.md) |
| called_by | [validate](/crates/oxide-erc-dsl/src/validator/validate.md) |
