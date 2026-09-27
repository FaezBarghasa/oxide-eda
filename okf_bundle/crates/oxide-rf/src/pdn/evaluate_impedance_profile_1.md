---
okf_version: "0.2"
type: Function
title: evaluate_impedance_profile
description: Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/evaluate_impedance_profile_1
language: rust
---

# evaluate_impedance_profile

Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.

## Signature

```rust
pub fn evaluate_impedance_profile(
        plane: &PowerPlaneCavity,
        decaps: &[DecapModel],
        frequencies_hz: &[f64],
    ) -> Vec<f64>
```

## Visibility

- `pub`

## Docstring

Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.

## Source
Lines 96–134 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
