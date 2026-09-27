---
okf_version: "0.2"
type: Function
title: ctx
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/ctx
language: rust
---

# ctx

## Signature

```rust
fn ctx() -> EvalContext
```

## Source
Lines 409–411 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [add_mixed_lengths_uses_lhs_unit](/crates/oxide-sketch/src/expr/eval/add_mixed_lengths_uses_lhs_unit.md) |
| called_by | [add_same_unit](/crates/oxide-sketch/src/expr/eval/add_same_unit.md) |
| called_by | [literal_passes_through](/crates/oxide-sketch/src/expr/eval/literal_passes_through.md) |
| called_by | [nonzero_divisor_still_works](/crates/oxide-sketch/src/expr/eval/nonzero_divisor_still_works.md) |
