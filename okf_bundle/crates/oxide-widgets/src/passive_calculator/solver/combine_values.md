---
okf_version: "0.2"
type: Function
title: combine_values
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/combine_values
language: rust
---

# combine_values

## Signature

```rust
fn combine_values(kind: ComponentKind, connection: Connection, left: f64, right: f64) -> f64
```

## Source
Lines 472–478 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [is_additive](/crates/oxide-widgets/src/passive_calculator/network/is_additive.md) |
| calls | [parallel_value](/crates/oxide-widgets/src/passive_calculator/network/parallel_value.md) |
| called_by | [find_balanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_balanced_level_four_brackets.md) |
| called_by | [find_level_three_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_level_three_brackets.md) |
| called_by | [find_unbalanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_unbalanced_level_four_brackets.md) |
| called_by | [generate_level_two_candidates](/crates/oxide-widgets/src/passive_calculator/solver/generate_level_two_candidates.md) |
