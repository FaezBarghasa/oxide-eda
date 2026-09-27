---
okf_version: "0.2"
type: Function
title: evaluate_stress
description: Evaluates component stresses against rated limits with safety derating.
resource: crates/oxide-sim/src/analysis/smoke.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:29:34Z"
concept_id: crates/oxide-sim/src/analysis/smoke/evaluate_stress
language: rust
---

# evaluate_stress

Evaluates component stresses against rated limits with safety derating.

## Signature

```rust
impl SmokeAnalyzer { pub fn evaluate_stress(
        designator: &str,
        limits: &ComponentLimits,
        stress: &SimulatedStress,
    ) -> StressEvaluation }
```

## Visibility

- `pub`

## Docstring

Evaluates component stresses against rated limits with safety derating.

## Source
Lines 59–101 in `crates/oxide-sim/src/analysis/smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smoke](/crates/oxide-sim/src/analysis/smoke.md) |
