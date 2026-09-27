---
okf_version: "0.2"
type: Function
title: validate_helper
resource: crates/oxide-erc-dsl/src/validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/validator/validate_helper
language: rust
---

# validate_helper

## Signature

```rust
fn validate_helper(
    rule_id: &str,
    target: TargetKind,
    name: &str,
    arg_count: usize,
    out: &mut Vec<DslError>,
)
```

## Source
Lines 102–138 in `crates/oxide-erc-dsl/src/validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [validator](/crates/oxide-erc-dsl/src/validator.md) |
| called_by | [validate_expr](/crates/oxide-erc-dsl/src/validator/validate_expr.md) |
