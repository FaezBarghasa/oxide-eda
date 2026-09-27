---
okf_version: "0.2"
type: Function
title: compute_synchronized_timestep
description: "Determines the next synchronized continuous timestep $h_{\\text{sync}}$, truncating if a discrete event precedes $t + h$."
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/compute_synchronized_timestep_1
language: rust
---

# compute_synchronized_timestep

Determines the next synchronized continuous timestep $h_{\text{sync}}$, truncating if a discrete event precedes $t + h$.

## Signature

```rust
pub fn compute_synchronized_timestep(&self, proposed_dt: f64) -> f64
```

## Visibility

- `pub`

## Docstring

Determines the next synchronized continuous timestep $h_{\text{sync}}$, truncating if a discrete event precedes $t + h$.

## Source
Lines 294–302 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
