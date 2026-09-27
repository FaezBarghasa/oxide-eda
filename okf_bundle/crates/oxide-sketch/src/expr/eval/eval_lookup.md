---
okf_version: "0.2"
type: Function
title: eval_lookup
description: "`Lookup` — find the index `i` such that `key ≈ keys[i]` and return"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/eval_lookup
language: rust
---

# eval_lookup

`Lookup` — find the index `i` such that `key ≈ keys[i]` and return

## Signature

```rust
fn eval_lookup(
    key: &ExprNode,
    keys: &[ExprNode],
    values: &[ExprNode],
    ctx: &EvalContext,
) -> Result<Quantity, ExprError>
```

## Docstring

`Lookup` — find the index `i` such that `key ≈ keys[i]` and return
`eval(values[i])`.

## Source
Lines 335–352 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
| calls | [same_canonical](/crates/oxide-sketch/src/expr/eval/same_canonical.md) |
| called_by | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
