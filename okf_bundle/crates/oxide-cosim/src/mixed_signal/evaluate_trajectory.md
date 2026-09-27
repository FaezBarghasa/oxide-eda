---
okf_version: "0.2"
type: Function
title: evaluate_trajectory
description: "Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded."
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/evaluate_trajectory
language: rust
---

# evaluate_trajectory

Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded.

## Signature

```rust
impl AtoDGateway { pub fn evaluate_trajectory(
        &mut self,
        t0: f64,
        v0: f64,
        t1: f64,
        v1: f64,
    ) -> Option<LogicEvent> }
```

## Visibility

- `pub`

## Docstring

Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded.

## Source
Lines 166–217 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
