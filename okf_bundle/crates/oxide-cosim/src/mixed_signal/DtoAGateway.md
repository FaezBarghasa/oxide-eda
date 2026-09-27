---
okf_version: "0.2"
type: Class
title: DtoAGateway
description: Digital-to-Analog (DtoA) Gateway Boundary Bridge.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/DtoAGateway
language: rust
---

# DtoAGateway

Digital-to-Analog (DtoA) Gateway Boundary Bridge.

## Signature

```rust
pub struct DtoAGateway
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Digital-to-Analog (DtoA) Gateway Boundary Bridge.
Converts discrete logic state transitions into continuous exponential voltage ramps.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `signal_id`
- `v_low`
- `v_high`
- `rise_time_s`
- `fall_time_s`
- `r_out_ohms`
- `target_voltage`
- `initial_voltage`
- `transition_start_time_s`

## Source
Lines 223–233 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
