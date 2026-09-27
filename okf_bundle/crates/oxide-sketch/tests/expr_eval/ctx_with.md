---
okf_version: "0.2"
type: Function
title: ctx_with
resource: crates/oxide-sketch/tests/expr_eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_eval/ctx_with
language: rust
---

# ctx_with

## Signature

```rust
fn ctx_with(params: &[(&str, ExprNode)]) -> EvalContext
```

## Source
Lines 52–61 in `crates/oxide-sketch/tests/expr_eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expr_eval](/crates/oxide-sketch/tests/expr_eval.md) |
| called_by | [eval_lookup_match](/crates/oxide-sketch/tests/expr_eval/eval_lookup_match.md) |
| called_by | [eval_lookup_no_match_errors](/crates/oxide-sketch/tests/expr_eval/eval_lookup_no_match_errors.md) |
| called_by | [eval_param_ref](/crates/oxide-sketch/tests/expr_eval/eval_param_ref.md) |
| called_by | [eval_ref_chains_recursively](/crates/oxide-sketch/tests/expr_eval/eval_ref_chains_recursively.md) |
