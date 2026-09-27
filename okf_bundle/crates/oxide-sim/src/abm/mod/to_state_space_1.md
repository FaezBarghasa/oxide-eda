---
okf_version: "0.2"
type: Function
title: to_state_space
description: "Transforms Laplace transfer function into controllable canonical state-space matrices (A, B, C, D)."
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod/to_state_space_1
language: rust
---

# to_state_space

Transforms Laplace transfer function into controllable canonical state-space matrices (A, B, C, D).

## Signature

```rust
pub fn to_state_space(&self) -> Option<(Vec<Vec<f64>>, Vec<f64>, Vec<f64>, f64)>
```

## Visibility

- `pub`

## Docstring

Transforms Laplace transfer function into controllable canonical state-space matrices (A, B, C, D).

## Source
Lines 69–112 in `crates/oxide-sim/src/abm/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abm](/crates/oxide-sim/src/abm/mod.md) |
