---
okf_version: "0.2"
type: Function
title: convert_to
description: "Convert `q` into the unit `target_unit`. Caller has already"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/convert_to
language: rust
---

# convert_to

Convert `q` into the unit `target_unit`. Caller has already

## Signature

```rust
fn convert_to(q: Quantity, target_unit: Unit) -> Result<f64, ExprError>
```

## Docstring

Convert `q` into the unit `target_unit`. Caller has already
verified that the families match.

## Source
Lines 356–381 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_add_sub](/crates/oxide-sketch/src/expr/eval/eval_add_sub.md) |
