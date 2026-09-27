---
okf_version: "0.2"
type: Function
title: validate_field
resource: crates/oxide-erc-dsl/src/validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/validator/validate_field
language: rust
---

# validate_field

## Signature

```rust
fn validate_field(
    rule_id: &str,
    target: TargetKind,
    field: &FieldExprAst,
    out: &mut Vec<DslError>,
)
```

## Source
Lines 140–188 in `crates/oxide-erc-dsl/src/validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [validator](/crates/oxide-erc-dsl/src/validator.md) |
| called_by | [validate_expr](/crates/oxide-erc-dsl/src/validator/validate_expr.md) |
