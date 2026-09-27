---
okf_version: "0.2"
type: Function
title: step_gmin
description: "Steps Gmin logarithmically from `gmin_start` down toward `gmin_target`."
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade/step_gmin
language: rust
---

# step_gmin

Steps Gmin logarithmically from `gmin_start` down toward `gmin_target`.

## Signature

```rust
impl ConvergenceCascade { pub fn step_gmin(&mut self, step_index: usize) -> f64 }
```

## Visibility

- `pub`

## Docstring

Steps Gmin logarithmically from `gmin_start` down toward `gmin_target`.

## Source
Lines 99–105 in `crates/oxide-sim/src/engine/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-sim/src/engine/cascade.md) |
