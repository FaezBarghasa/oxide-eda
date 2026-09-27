---
okf_version: "0.2"
type: Function
title: check_nonzero
description: "Reject a zero divisor before we feed it to `/` or `%`. Pure-zero"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/check_nonzero
language: rust
---

# check_nonzero

Reject a zero divisor before we feed it to `/` or `%`. Pure-zero

## Signature

```rust
fn check_nonzero(divisor: f64) -> Result<(), ExprError>
```

## Docstring

Reject a zero divisor before we feed it to `/` or `%`. Pure-zero
equality is intentional; sub-ULP non-zero values still divide
cleanly under f64 (returning a large finite ratio rather than inf).

## Source
Lines 249–255 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_div_mod](/crates/oxide-sketch/src/expr/eval/eval_div_mod.md) |
