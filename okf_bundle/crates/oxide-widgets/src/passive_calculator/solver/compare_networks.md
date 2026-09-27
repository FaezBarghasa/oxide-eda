---
okf_version: "0.2"
type: Function
title: compare_networks
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/compare_networks
language: rust
---

# compare_networks

## Signature

```rust
fn compare_networks(left: &Network, right: &Network, kind: ComponentKind, target: f64) -> Ordering
```

## Source
Lines 518–533 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [relative_error](/crates/oxide-widgets/src/passive_calculator/solver/relative_error.md) |
| called_by | [sort_networks](/crates/oxide-widgets/src/passive_calculator/solver/sort_networks.md) |
