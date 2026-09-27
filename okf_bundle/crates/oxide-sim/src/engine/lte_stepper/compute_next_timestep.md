---
okf_version: "0.2"
type: Function
title: compute_next_timestep
description: "Evaluates Milne predictor-corrector error and determines next adaptive timestep $h_{\\text{next}}$."
resource: crates/oxide-sim/src/engine/lte_stepper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:51Z"
concept_id: crates/oxide-sim/src/engine/lte_stepper/compute_next_timestep
language: rust
---

# compute_next_timestep

Evaluates Milne predictor-corrector error and determines next adaptive timestep $h_{\text{next}}$.

## Signature

```rust
impl LteController { pub fn compute_next_timestep(
        &mut self,
        v_current: &[f64],
        v_predicted: &[f64],
        dt_current: f64,
    ) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Evaluates Milne predictor-corrector error and determines next adaptive timestep $h_{\text{next}}$.

## Source
Lines 57–96 in `crates/oxide-sim/src/engine/lte_stepper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lte_stepper](/crates/oxide-sim/src/engine/lte_stepper.md) |
