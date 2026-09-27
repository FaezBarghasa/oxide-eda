---
okf_version: "0.2"
type: Function
title: eval_numbering_expr
description: "Parse + evaluate one numbering expression, naming both the field"
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/eval_numbering_expr
language: rust
---

# eval_numbering_expr

Parse + evaluate one numbering expression, naming both the field

## Signature

```rust
fn eval_numbering_expr(field: &str, src: &str, ctx: &EvalContext) -> Result<f64, String>
```

## Docstring

Parse + evaluate one numbering expression, naming both the field
(`start` / `step`) and the authored source text in the error.

## Source
Lines 85–92 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| calls | [strip_eq_prefix](/crates/oxide-bake/src/array/numbering/strip_eq_prefix.md) |
| called_by | [linear_increment_number](/crates/oxide-bake/src/array/numbering/linear_increment_number.md) |
