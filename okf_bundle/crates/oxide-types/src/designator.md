---
okf_version: "0.2"
type: Module
title: designator
description: Natural ordering for reference designators and pin numbers.
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator
language: rust
---

# designator

Natural ordering for reference designators and pin numbers.

## Docstring

Natural ordering for reference designators and pin numbers.

Designators mix a letter prefix with a decimal index (`R1`, `R10`, `VR2`)
and may carry further sections (`U1_2`, `J3-10`). Ordering them with
[`str::cmp`] compares bytes, so `R10` lands between `R1` and `R2` and every
BOM, netlist and drift list reads wrong. Every such list routes through
[`compare_references`] instead.

## Relationships

| Type | Target |
|------|--------|
| related | [take_run](/crates/oxide-types/src/designator/take_run.md) |
| related | [compare_digit_runs](/crates/oxide-types/src/designator/compare_digit_runs.md) |
| related | [strip_leading_zeros](/crates/oxide-types/src/designator/strip_leading_zeros.md) |
| related | [compare_text_runs](/crates/oxide-types/src/designator/compare_text_runs.md) |
| related | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
| related | [sorted](/crates/oxide-types/src/designator/sorted.md) |
| related | [orders_index_numerically_not_lexicographically](/crates/oxide-types/src/designator/orders_index_numerically_not_lexicographically.md) |
| related | [orders_every_section_of_a_multi_section_designator](/crates/oxide-types/src/designator/orders_every_section_of_a_multi_section_designator.md) |
| related | [orders_prefixes_case_insensitively](/crates/oxide-types/src/designator/orders_prefixes_case_insensitively.md) |
| related | [orders_numeric_tails_too_long_to_parse](/crates/oxide-types/src/designator/orders_numeric_tails_too_long_to_parse.md) |
| related | [treats_leading_zeros_as_padding_but_keeps_a_total_order](/crates/oxide-types/src/designator/treats_leading_zeros_as_padding_but_keeps_a_total_order.md) |
| related | [is_a_strict_total_order_over_a_mixed_set](/crates/oxide-types/src/designator/is_a_strict_total_order_over_a_mixed_set.md) |
