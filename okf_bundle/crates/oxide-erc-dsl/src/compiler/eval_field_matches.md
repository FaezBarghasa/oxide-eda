---
okf_version: "0.2"
type: Function
title: eval_field_matches
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/eval_field_matches
language: rust
---

# eval_field_matches

## Signature

```rust
fn eval_field_matches(field: &FieldExprAst, regex: &Regex, subject: Subject<'_>) -> bool
```

## Source
Lines 348–356 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [resolve_field](/crates/oxide-erc-dsl/src/compiler/resolve_field.md) |
| called_by | [eval_expr](/crates/oxide-erc-dsl/src/compiler/eval_expr.md) |
