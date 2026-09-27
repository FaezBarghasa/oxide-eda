---
okf_version: "0.2"
type: Function
title: parse
description: "Parse a source string into an [`ExprNode`] tree."
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse
language: rust
---

# parse

Parse a source string into an [`ExprNode`] tree.

## Signature

```rust
pub fn parse(src: &str) -> Result<ExprNode, ExprError>
```

## Visibility

- `pub`

## Docstring

Parse a source string into an [`ExprNode`] tree.

On success the returned tree has been fully consumed — any trailing
junk produces [`ExprError::Parse`].

## Source
Lines 615–625 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
| called_by | [smoke_addition](/crates/oxide-sketch/src/expr/parse/smoke_addition.md) |
| called_by | [smoke_literal](/crates/oxide-sketch/src/expr/parse/smoke_literal.md) |
| called_by | [resolve](/crates/oxide-sketch/src/parameter/resolve.md) |
| called_by | [parse_quantity](/crates/oxide-sketch/src/unit/parse_quantity.md) |
| called_by | [parse_addition_with_units](/crates/oxide-sketch/tests/expr_parser/parse_addition_with_units.md) |
| called_by | [parse_array_index_i](/crates/oxide-sketch/tests/expr_parser/parse_array_index_i.md) |
| called_by | [parse_array_index_j](/crates/oxide-sketch/tests/expr_parser/parse_array_index_j.md) |
| called_by | [parse_compare_and_logic](/crates/oxide-sketch/tests/expr_parser/parse_compare_and_logic.md) |
| called_by | [parse_left_assoc](/crates/oxide-sketch/tests/expr_parser/parse_left_assoc.md) |
| called_by | [parse_literal_dimensionless](/crates/oxide-sketch/tests/expr_parser/parse_literal_dimensionless.md) |
| called_by | [parse_literal_mm](/crates/oxide-sketch/tests/expr_parser/parse_literal_mm.md) |
| called_by | [parse_long_ident_is_ref_not_index](/crates/oxide-sketch/tests/expr_parser/parse_long_ident_is_ref_not_index.md) |
| called_by | [parse_lookup](/crates/oxide-sketch/tests/expr_parser/parse_lookup.md) |
| called_by | [parse_parens](/crates/oxide-sketch/tests/expr_parser/parse_parens.md) |
| called_by | [parse_precedence](/crates/oxide-sketch/tests/expr_parser/parse_precedence.md) |
| called_by | [parse_ref](/crates/oxide-sketch/tests/expr_parser/parse_ref.md) |
| called_by | [parse_right_assoc_pow](/crates/oxide-sketch/tests/expr_parser/parse_right_assoc_pow.md) |
| called_by | [parse_ternary](/crates/oxide-sketch/tests/expr_parser/parse_ternary.md) |
| called_by | [parse_unary_neg](/crates/oxide-sketch/tests/expr_parser/parse_unary_neg.md) |
| called_by | [parse_unary_not](/crates/oxide-sketch/tests/expr_parser/parse_unary_not.md) |
