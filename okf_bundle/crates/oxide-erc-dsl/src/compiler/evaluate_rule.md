---
okf_version: "0.2"
type: Function
title: evaluate_rule
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/evaluate_rule
language: rust
---

# evaluate_rule

## Signature

```rust
fn evaluate_rule(
    ctx: &ErcContext,
    target: TargetKind,
    expr: &CompiledExpr,
    message: &str,
    severity: Severity,
    rule_id: &RuleId,
    rule_kind: RuleKind,
) -> Vec<Diagnostic>
```

## Source
Lines 183–260 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [eval_expr](/crates/oxide-erc-dsl/src/compiler/eval_expr.md) |
| calls | [Net](/crates/oxide-types/src/net/Net.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| called_by | [compile_rule](/crates/oxide-erc-dsl/src/compiler/compile_rule.md) |
