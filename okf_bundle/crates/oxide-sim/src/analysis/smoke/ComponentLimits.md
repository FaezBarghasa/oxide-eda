---
okf_version: "0.2"
type: Class
title: ComponentLimits
description: Maximum ratings / Safe Operating Area limits for a component.
resource: crates/oxide-sim/src/analysis/smoke.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:29:34Z"
concept_id: crates/oxide-sim/src/analysis/smoke/ComponentLimits
language: rust
---

# ComponentLimits

Maximum ratings / Safe Operating Area limits for a component.

## Signature

```rust
pub struct ComponentLimits
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Maximum ratings / Safe Operating Area limits for a component.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `max_voltage_v`
- `max_current_a`
- `max_power_w`
- `max_junction_temp_c`
- `derating_factor`

## Source
Lines 11–17 in `crates/oxide-sim/src/analysis/smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smoke](/crates/oxide-sim/src/analysis/smoke.md) |
