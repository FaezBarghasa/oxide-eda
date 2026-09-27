---
okf_version: "0.2"
type: Function
title: lower_bound_networks
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/lower_bound_networks
language: rust
---

# lower_bound_networks

## Signature

```rust
fn lower_bound_networks(
    networks: &[Network],
    sorted_indices: &[usize],
    start: usize,
    target: f64,
    kind: ComponentKind,
) -> usize
```

## Source
Lines 439–457 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| called_by | [generate_target_brackets](/crates/oxide-widgets/src/passive_calculator/solver/generate_target_brackets.md) |
