---
okf_version: "0.2"
type: Class
title: ArrayIndex
description: Array index variable used inside per-instance expressions.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast/ArrayIndex
language: rust
---

# ArrayIndex

Array index variable used inside per-instance expressions.

## Signature

```rust
pub enum ArrayIndex
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Array index variable used inside per-instance expressions.

`I` is the row index, `J` is the column index of a 2D array. For
1D arrays only `I` is meaningful.
[derive(Clone, Copy, Debug, PartialEq, Eq)]

## Source
Lines 98–101 in `crates/oxide-sketch/src/expr/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/oxide-sketch/src/expr/ast.md) |
| called_by | [parse_primary](/crates/oxide-sketch/src/expr/parse/parse_primary.md) |
| called_by | [eval_array_index_in_context](/crates/oxide-sketch/tests/expr_eval/eval_array_index_in_context.md) |
| called_by | [eval_array_index_j](/crates/oxide-sketch/tests/expr_eval/eval_array_index_j.md) |
| called_by | [eval_array_index_outside_errors](/crates/oxide-sketch/tests/expr_eval/eval_array_index_outside_errors.md) |
| called_by | [eval_ternary_takes_else](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_else.md) |
| called_by | [eval_ternary_takes_then](/crates/oxide-sketch/tests/expr_eval/eval_ternary_takes_then.md) |
