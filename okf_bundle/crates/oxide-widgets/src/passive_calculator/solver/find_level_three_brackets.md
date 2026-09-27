---
okf_version: "0.2"
type: Function
title: find_level_three_brackets
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/find_level_three_brackets
language: rust
---

# find_level_three_brackets

## Signature

```rust
fn find_level_three_brackets(
    leaves: &[Network],
    level_two: &[LevelTwoCandidate],
    kind: ComponentKind,
    target: f64,
) -> ValueBrackets<LevelThreeCandidate>
```

## Source
Lines 258–291 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [desired_right_value](/crates/oxide-widgets/src/passive_calculator/solver/desired_right_value.md) |
| calls | [lower_bound_level_two](/crates/oxide-widgets/src/passive_calculator/solver/lower_bound_level_two.md) |
| calls | [neighboring_indices](/crates/oxide-widgets/src/passive_calculator/solver/neighboring_indices.md) |
| calls | [combine_values](/crates/oxide-widgets/src/passive_calculator/solver/combine_values.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| called_by | [find_unbalanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_unbalanced_level_four_brackets.md) |
