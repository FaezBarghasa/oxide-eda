---
okf_version: "0.2"
type: Function
title: eval_at_expression
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/eval_at_expression
language: rust
---

# eval_at_expression

## Signature

```rust
fn eval_at_expression(expr: &str, ctx: &ExpressionEvalContext<'_>) -> Option<String>
```

## Source
Lines 542–554 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [lookup_ci](/crates/oxide-types/src/markup/lookup_ci.md) |
| called_by | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
