---
okf_version: "0.2"
type: Module
title: expr_parser
description: Integration tests for the recursive-descent expression parser
resource: crates/oxide-sketch/tests/expr_parser.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/expr_parser
language: rust
---

# expr_parser

Integration tests for the recursive-descent expression parser

## Docstring

Integration tests for the recursive-descent expression parser
(`crates/oxide-sketch/src/expr/parse.rs`).

Covers Task 4.3 of `docs/internal/SKETCH_MODE_v0.13_PLAN.md`.

## Relationships

| Type | Target |
|------|--------|
| related | [lit](/crates/oxide-sketch/tests/expr_parser/lit.md) |
| related | [parse_literal_mm](/crates/oxide-sketch/tests/expr_parser/parse_literal_mm.md) |
| related | [parse_literal_dimensionless](/crates/oxide-sketch/tests/expr_parser/parse_literal_dimensionless.md) |
| related | [parse_addition_with_units](/crates/oxide-sketch/tests/expr_parser/parse_addition_with_units.md) |
| related | [parse_left_assoc](/crates/oxide-sketch/tests/expr_parser/parse_left_assoc.md) |
| related | [parse_right_assoc_pow](/crates/oxide-sketch/tests/expr_parser/parse_right_assoc_pow.md) |
| related | [parse_precedence](/crates/oxide-sketch/tests/expr_parser/parse_precedence.md) |
| related | [parse_parens](/crates/oxide-sketch/tests/expr_parser/parse_parens.md) |
| related | [parse_unary_neg](/crates/oxide-sketch/tests/expr_parser/parse_unary_neg.md) |
| related | [parse_unary_not](/crates/oxide-sketch/tests/expr_parser/parse_unary_not.md) |
| related | [parse_ref](/crates/oxide-sketch/tests/expr_parser/parse_ref.md) |
| related | [parse_array_index_i](/crates/oxide-sketch/tests/expr_parser/parse_array_index_i.md) |
| related | [parse_array_index_j](/crates/oxide-sketch/tests/expr_parser/parse_array_index_j.md) |
| related | [parse_long_ident_is_ref_not_index](/crates/oxide-sketch/tests/expr_parser/parse_long_ident_is_ref_not_index.md) |
| related | [parse_ternary](/crates/oxide-sketch/tests/expr_parser/parse_ternary.md) |
| related | [parse_lookup](/crates/oxide-sketch/tests/expr_parser/parse_lookup.md) |
| related | [parse_compare_and_logic](/crates/oxide-sketch/tests/expr_parser/parse_compare_and_logic.md) |
| related | [parse_invalid_unit_fails](/crates/oxide-sketch/tests/expr_parser/parse_invalid_unit_fails.md) |
| related | [parse_unbalanced_paren_fails](/crates/oxide-sketch/tests/expr_parser/parse_unbalanced_paren_fails.md) |
| related | [parse_lookup_length_mismatch_fails](/crates/oxide-sketch/tests/expr_parser/parse_lookup_length_mismatch_fails.md) |
| related | [parse_empty_fails](/crates/oxide-sketch/tests/expr_parser/parse_empty_fails.md) |
