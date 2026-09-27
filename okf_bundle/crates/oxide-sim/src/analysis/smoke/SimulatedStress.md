---
okf_version: "0.2"
type: Class
title: SimulatedStress
description: Simulated electrical and thermal stresses on a component.
resource: crates/oxide-sim/src/analysis/smoke.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:29:34Z"
concept_id: crates/oxide-sim/src/analysis/smoke/SimulatedStress
language: rust
---

# SimulatedStress

Simulated electrical and thermal stresses on a component.

## Signature

```rust
pub struct SimulatedStress
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Simulated electrical and thermal stresses on a component.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `peak_voltage_v`
- `rms_current_a`
- `avg_power_w`
- `junction_temp_c`

## Source
Lines 33–38 in `crates/oxide-sim/src/analysis/smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smoke](/crates/oxide-sim/src/analysis/smoke.md) |
