---
okf_version: "0.2"
type: Function
title: find_closest_network
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/find_closest_network
language: rust
---

# find_closest_network

## Signature

```rust
fn find_closest_network(
    leaves: &[Network],
    kind: ComponentKind,
    target: f64,
    max_parts: usize,
) -> Network
```

## Source
Lines 147–230 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [sort_networks](/crates/oxide-widgets/src/passive_calculator/solver/sort_networks.md) |
| calls | [is_effectively_exact](/crates/oxide-widgets/src/passive_calculator/solver/is_effectively_exact.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [generate_target_brackets](/crates/oxide-widgets/src/passive_calculator/solver/generate_target_brackets.md) |
| calls | [generate_level_two_candidates](/crates/oxide-widgets/src/passive_calculator/solver/generate_level_two_candidates.md) |
| calls | [lower_bound_level_two](/crates/oxide-widgets/src/passive_calculator/solver/lower_bound_level_two.md) |
| calls | [neighboring_indices](/crates/oxide-widgets/src/passive_calculator/solver/neighboring_indices.md) |
| calls | [materialize_level_two](/crates/oxide-widgets/src/passive_calculator/solver/materialize_level_two.md) |
| calls | [find_level_three_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_level_three_brackets.md) |
| calls | [materialize_level_three](/crates/oxide-widgets/src/passive_calculator/solver/materialize_level_three.md) |
| calls | [find_balanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_balanced_level_four_brackets.md) |
| calls | [find_unbalanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_unbalanced_level_four_brackets.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
