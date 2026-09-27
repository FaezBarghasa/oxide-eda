---
okf_version: "0.2"
type: Function
title: derive_pad_number
description: Resolve the pad number for the i-th instance of a linear array.
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/derive_pad_number
language: rust
---

# derive_pad_number

Resolve the pad number for the i-th instance of a linear array.

## Signature

```rust
pub(super) fn derive_pad_number(
    numbering: &NumberingScheme,
    i: usize,
    params_ast: &BTreeMap<String, ExprNode>,
    warnings: &mut Vec<String>,
    source: SketchEntityId,
) -> String
```

## Visibility

- `pub(super)`

## Docstring

Resolve the pad number for the i-th instance of a linear array.

## Source
Lines 14–55 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| calls | [linear_increment_number](/crates/oxide-bake/src/array/numbering/linear_increment_number.md) |
| calls | [record_numbering_warning](/crates/oxide-bake/src/array/numbering/record_numbering_warning.md) |
| called_by | [bake_linear](/crates/oxide-bake/src/array/linear/bake_linear.md) |
| called_by | [linear_increment_expression_error_is_reported_not_swallowed](/crates/oxide-bake/src/array/numbering/linear_increment_expression_error_is_reported_not_swallowed.md) |
| called_by | [linear_increment_expression_error_is_reported_once_per_array](/crates/oxide-bake/src/array/numbering/linear_increment_expression_error_is_reported_once_per_array.md) |
| called_by | [linear_increment_valid_expression_stays_silent](/crates/oxide-bake/src/array/numbering/linear_increment_valid_expression_stays_silent.md) |
| called_by | [bake_polar](/crates/oxide-bake/src/array/polar/bake_polar.md) |
