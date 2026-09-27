---
okf_version: "0.2"
type: Class
title: SimulationConfig
description: Simulation execution settings and options.
resource: crates/oxide-types/src/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:10:20Z"
concept_id: crates/oxide-types/src/sim/SimulationConfig
language: rust
---

# SimulationConfig

Simulation execution settings and options.

## Signature

```rust
pub struct SimulationConfig
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Simulation execution settings and options.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `analysis`
- `probes`
- `temp_c`
- `reltol`
- `vntol`
- `abstol`
- `custom_options`

## Source
Lines 106–120 in `crates/oxide-types/src/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-types/src/sim.md) |
