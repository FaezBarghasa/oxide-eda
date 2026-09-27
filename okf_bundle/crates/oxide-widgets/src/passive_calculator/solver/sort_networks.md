---
okf_version: "0.2"
type: Function
title: sort_networks
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/sort_networks
language: rust
---

# sort_networks

## Signature

```rust
fn sort_networks(networks: &mut [Network], kind: ComponentKind, target: f64)
```

## Source
Lines 514–516 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [compare_networks](/crates/oxide-widgets/src/passive_calculator/solver/compare_networks.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| called_by | [generate_leaves](/crates/oxide-widgets/src/passive_calculator/solver/generate_leaves.md) |
| called_by | [retain_frontier](/crates/oxide-widgets/src/passive_calculator/solver/retain_frontier.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
