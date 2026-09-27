---
okf_version: "0.2"
type: Function
title: generate_leaves
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/generate_leaves
language: rust
---

# generate_leaves

## Signature

```rust
fn generate_leaves(options: SolveOptions) -> Vec<Network>
```

## Source
Lines 480–503 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [sort_networks](/crates/oxide-widgets/src/passive_calculator/solver/sort_networks.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
