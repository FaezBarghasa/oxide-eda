---
okf_version: "0.2"
type: Function
title: component
resource: crates/oxide-widgets/tests/passive_calculator/network_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/tests/passive_calculator/network_tests/component
language: rust
---

# component

## Signature

```rust
fn component(significand: u16, decade: i8, tolerance: Tolerance) -> Network
```

## Source
Lines 5–11 in `crates/oxide-widgets/tests/passive_calculator/network_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_tests](/crates/oxide-widgets/tests/passive_calculator/network_tests.md) |
| called_by | [capacitance_inverts_series_and_parallel_equations](/crates/oxide-widgets/tests/passive_calculator/network_tests/capacitance_inverts_series_and_parallel_equations.md) |
| called_by | [expressions_use_unicode_subscripts_and_plain_text_fallback](/crates/oxide-widgets/tests/passive_calculator/network_tests/expressions_use_unicode_subscripts_and_plain_text_fallback.md) |
| called_by | [individual_leaf_tolerance_can_be_changed](/crates/oxide-widgets/tests/passive_calculator/network_tests/individual_leaf_tolerance_can_be_changed.md) |
| called_by | [mixed_tolerances_produce_exact_monotone_bounds](/crates/oxide-widgets/tests/passive_calculator/network_tests/mixed_tolerances_produce_exact_monotone_bounds.md) |
| called_by | [resistance_and_uncoupled_inductance_use_the_same_connection_equations](/crates/oxide-widgets/tests/passive_calculator/network_tests/resistance_and_uncoupled_inductance_use_the_same_connection_equations.md) |
