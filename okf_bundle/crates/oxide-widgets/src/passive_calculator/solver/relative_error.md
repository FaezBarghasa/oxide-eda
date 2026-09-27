---
okf_version: "0.2"
type: Function
title: relative_error
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/relative_error
language: rust
---

# relative_error

## Signature

```rust
fn relative_error(value: f64, target: f64) -> f64
```

## Source
Lines 539–547 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| called_by | [compare_networks](/crates/oxide-widgets/src/passive_calculator/solver/compare_networks.md) |
| called_by | [is_effectively_exact](/crates/oxide-widgets/src/passive_calculator/solver/is_effectively_exact.md) |
