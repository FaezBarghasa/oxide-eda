---
okf_version: "0.2"
type: Class
title: CoSimStats
description: Statistics and timeline metrics for the active co-simulation.
resource: crates/oxide-cosim/src/session.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:06Z"
concept_id: crates/oxide-cosim/src/session/CoSimStats
language: rust
---

# CoSimStats

Statistics and timeline metrics for the active co-simulation.

## Signature

```rust
pub struct CoSimStats
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Statistics and timeline metrics for the active co-simulation.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `sim_time_s`
- `wall_clock_elapsed_s`
- `speed_ratio`
- `total_steps`
- `mcu_cycles`
- `packets_transferred`

## Source
Lines 17–24 in `crates/oxide-cosim/src/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-cosim/src/session.md) |
