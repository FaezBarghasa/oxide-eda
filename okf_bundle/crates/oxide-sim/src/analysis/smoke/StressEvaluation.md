---
okf_version: "0.2"
type: Class
title: StressEvaluation
description: Component stress ratio evaluation results.
resource: crates/oxide-sim/src/analysis/smoke.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:29:34Z"
concept_id: crates/oxide-sim/src/analysis/smoke/StressEvaluation
language: rust
---

# StressEvaluation

Component stress ratio evaluation results.

## Signature

```rust
pub struct StressEvaluation
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

Component stress ratio evaluation results.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `designator`
- `voltage_ratio`
- `current_ratio`
- `power_ratio`
- `temp_ratio`
- `max_stress_ratio`
- `is_overstressed`
- `primary_violation`

## Source
Lines 42–51 in `crates/oxide-sim/src/analysis/smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smoke](/crates/oxide-sim/src/analysis/smoke.md) |
