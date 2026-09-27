---
okf_version: "0.2"
type: Class
title: LayerVisibility
description: "Per-layer visibility map — a 7-entry struct we index by `FpLayer`"
resource: crates/oxide-app/src/library/editor/footprint/layers.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/layers/LayerVisibility
language: rust
---

# LayerVisibility

Per-layer visibility map — a 7-entry struct we index by `FpLayer`

## Signature

```rust
pub struct LayerVisibility
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Per-layer visibility map — a 7-entry struct we index by `FpLayer`
rather than a `HashMap` so the footprint state is `Clone +
PartialEq` without a derive dance.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `f_cu`
- `b_cu`
- `f_silks`
- `b_silks`
- `f_fab`
- `b_fab`
- `edge_cuts`

## Source
Lines 106–114 in `crates/oxide-app/src/library/editor/footprint/layers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layers](/crates/oxide-app/src/library/editor/footprint/layers.md) |
