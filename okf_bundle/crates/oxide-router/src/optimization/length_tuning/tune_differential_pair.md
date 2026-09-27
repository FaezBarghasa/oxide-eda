---
okf_version: "0.2"
type: Function
title: tune_differential_pair
description: Match lengths between positive and negative traces of a differential pair.
resource: crates/oxide-router/src/optimization/length_tuning.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:57:46Z"
concept_id: crates/oxide-router/src/optimization/length_tuning/tune_differential_pair
language: rust
---

# tune_differential_pair

Match lengths between positive and negative traces of a differential pair.

## Signature

```rust
impl LengthTuningOptimizer { pub fn tune_differential_pair(
        &self,
        pos_path: &mut RoutingPath,
        neg_path: &mut RoutingPath,
    ) -> TuningResult }
```

## Visibility

- `pub`

## Docstring

Match lengths between positive and negative traces of a differential pair.

## Source
Lines 28–49 in `crates/oxide-router/src/optimization/length_tuning.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [length_tuning](/crates/oxide-router/src/optimization/length_tuning.md) |
