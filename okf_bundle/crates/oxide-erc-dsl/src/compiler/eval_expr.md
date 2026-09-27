---
okf_version: "0.2"
type: Function
title: eval_expr
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/eval_expr
language: rust
---

# eval_expr

## Signature

```rust
fn eval_expr(expr: &CompiledExpr, subject: Subject<'_>) -> bool
```

## Source
Lines 262–273 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [subject_ref](/crates/oxide-erc-dsl/src/compiler/subject_ref.md) |
| calls | [eval_helper](/crates/oxide-erc-dsl/src/compiler/eval_helper.md) |
| calls | [eval_field_cmp](/crates/oxide-erc-dsl/src/compiler/eval_field_cmp.md) |
| calls | [eval_field_matches](/crates/oxide-erc-dsl/src/compiler/eval_field_matches.md) |
| called_by | [evaluate_rule](/crates/oxide-erc-dsl/src/compiler/evaluate_rule.md) |
| called_by | [resolve_dim](/crates/oxide-sketch/src/solver/residual/resolve_dim.md) |
