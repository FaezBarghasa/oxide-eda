---
okf_version: "0.2"
type: Class
title: AtoDGateway
description: Analog-to-Digital (AtoD) Gateway Boundary Bridge.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/AtoDGateway
language: rust
---

# AtoDGateway

Analog-to-Digital (AtoD) Gateway Boundary Bridge.

## Signature

```rust
pub struct AtoDGateway
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Analog-to-Digital (AtoD) Gateway Boundary Bridge.
Monitors continuous node voltages and detects threshold crossings using Hermite interpolation.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `signal_id`
- `v_il`
- `v_ih`
- `current_state`
- `last_voltage`
- `last_time_s`

## Source
Lines 144–151 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
