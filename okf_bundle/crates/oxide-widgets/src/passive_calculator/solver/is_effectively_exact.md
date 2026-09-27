---
okf_version: "0.2"
type: Function
title: is_effectively_exact
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/is_effectively_exact
language: rust
---

# is_effectively_exact

## Signature

```rust
fn is_effectively_exact(network: &Network, kind: ComponentKind, target: f64) -> bool
```

## Source
Lines 535–537 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [relative_error](/crates/oxide-widgets/src/passive_calculator/solver/relative_error.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
