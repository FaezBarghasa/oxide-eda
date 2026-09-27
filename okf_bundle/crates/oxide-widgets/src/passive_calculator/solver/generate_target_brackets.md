---
okf_version: "0.2"
type: Function
title: generate_target_brackets
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/generate_target_brackets
language: rust
---

# generate_target_brackets

## Signature

```rust
fn generate_target_brackets(
    left_networks: &[Network],
    right_networks: &[Network],
    same_networks: bool,
    kind: ComponentKind,
    target: f64,
) -> Vec<Network>
```

## Source
Lines 399–437 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [desired_right_value](/crates/oxide-widgets/src/passive_calculator/solver/desired_right_value.md) |
| calls | [lower_bound_networks](/crates/oxide-widgets/src/passive_calculator/solver/lower_bound_networks.md) |
| calls | [neighboring_indices](/crates/oxide-widgets/src/passive_calculator/solver/neighboring_indices.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
