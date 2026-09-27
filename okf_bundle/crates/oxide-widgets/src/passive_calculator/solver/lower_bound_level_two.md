---
okf_version: "0.2"
type: Function
title: lower_bound_level_two
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/lower_bound_level_two
language: rust
---

# lower_bound_level_two

## Signature

```rust
fn lower_bound_level_two(candidates: &[LevelTwoCandidate], start: usize, target: f64) -> usize
```

## Source
Lines 375–387 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| called_by | [find_balanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_balanced_level_four_brackets.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| called_by | [find_level_three_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_level_three_brackets.md) |
