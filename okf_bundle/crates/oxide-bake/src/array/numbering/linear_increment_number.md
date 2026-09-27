---
okf_version: "0.2"
type: Function
title: linear_increment_number
description: "`LinearIncrement` — `start + i * step`, both rounded to integer"
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/linear_increment_number
language: rust
---

# linear_increment_number

`LinearIncrement` — `start + i * step`, both rounded to integer

## Signature

```rust
pub(super) fn linear_increment_number(
    start_expr: &str,
    step_expr: &str,
    i: usize,
    params_ast: &BTreeMap<String, ExprNode>,
) -> Result<String, String>
```

## Visibility

- `pub(super)`

## Docstring

`LinearIncrement` — `start + i * step`, both rounded to integer
after canonical evaluation.

GH #599 — on an expression error this returns the offending
expression and the reason instead of `None`. The caller still
falls back to a default number rather than aborting the whole
array bake, but it can now say which expression it gave up on:
pad and instance numbers are what the netlist binds to, so a
typo'd designator expression that silently renumbers the array
`0, 1, 2, …` is not a cosmetic failure.

## Source
Lines 67–81 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| calls | [eval_numbering_expr](/crates/oxide-bake/src/array/numbering/eval_numbering_expr.md) |
| called_by | [derive_pad_number](/crates/oxide-bake/src/array/numbering/derive_pad_number.md) |
| called_by | [derive_pad_number_2d](/crates/oxide-bake/src/array/numbering/derive_pad_number_2d.md) |
