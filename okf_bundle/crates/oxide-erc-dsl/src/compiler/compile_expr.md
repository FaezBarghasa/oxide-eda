---
okf_version: "0.2"
type: Function
title: compile_expr
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/compile_expr
language: rust
---

# compile_expr

## Signature

```rust
fn compile_expr(rule_id: &str, expr: &ExprAst) -> Result<CompiledExpr, DslError>
```

## Source
Lines 123–174 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| called_by | [compile_rule](/crates/oxide-erc-dsl/src/compiler/compile_rule.md) |
