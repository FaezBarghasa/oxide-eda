---
okf_version: "0.2"
type: Function
title: compile_rule
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/compile_rule
language: rust
---

# compile_rule

## Signature

```rust
fn compile_rule(rule: &RuleAst) -> Result<CompiledRule, DslError>
```

## Source
Lines 84–121 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [compile_expr](/crates/oxide-erc-dsl/src/compiler/compile_expr.md) |
| calls | [map_target](/crates/oxide-erc-dsl/src/compiler/map_target.md) |
| calls | [map_scope](/crates/oxide-erc-dsl/src/compiler/map_scope.md) |
| calls | [map_applicability](/crates/oxide-erc-dsl/src/compiler/map_applicability.md) |
| calls | [map_severity](/crates/oxide-erc-dsl/src/compiler/map_severity.md) |
| calls | [fallback_kind](/crates/oxide-erc-dsl/src/compiler/fallback_kind.md) |
| calls | [evaluate_rule](/crates/oxide-erc-dsl/src/compiler/evaluate_rule.md) |
| called_by | [compile](/crates/oxide-erc-dsl/src/compiler/compile.md) |
