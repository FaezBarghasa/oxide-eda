---
okf_version: "0.2"
type: Function
title: retain_frontier
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/retain_frontier
language: rust
---

# retain_frontier

## Signature

```rust
fn retain_frontier(candidates: Vec<Network>, kind: ComponentKind, target: f64) -> Vec<Network>
```

## Source
Lines 505–512 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [sort_networks](/crates/oxide-widgets/src/passive_calculator/solver/sort_networks.md) |
| calls | [truncate](/crates/oxide-output/examples/qa_harness/truncate.md) |
| called_by | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
