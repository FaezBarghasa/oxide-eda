---
okf_version: "0.2"
type: Function
title: resolve_field
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/resolve_field
language: rust
---

# resolve_field

## Signature

```rust
fn resolve_field(field: &FieldExprAst, subject: Subject<'_>) -> Option<Value>
```

## Source
Lines 358–405 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [pin_type_name](/crates/oxide-erc-dsl/src/compiler/pin_type_name.md) |
| called_by | [eval_field_cmp](/crates/oxide-erc-dsl/src/compiler/eval_field_cmp.md) |
| called_by | [eval_field_matches](/crates/oxide-erc-dsl/src/compiler/eval_field_matches.md) |
