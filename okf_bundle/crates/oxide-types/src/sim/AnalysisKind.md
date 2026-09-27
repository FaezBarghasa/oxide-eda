---
okf_version: "0.2"
type: Class
title: AnalysisKind
description: Simulation analysis directive and parameters.
resource: crates/oxide-types/src/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:10:20Z"
concept_id: crates/oxide-types/src/sim/AnalysisKind
language: rust
---

# AnalysisKind

Simulation analysis directive and parameters.

## Signature

```rust
pub enum AnalysisKind
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Simulation analysis directive and parameters.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `start_time`
- `stop_time`
- `step_time`
- `max_step`
- `uic`
- `sweep_type`
- `points`
- `start_freq`
- `stop_freq`
- `source`
- `start`
- `stop`
- `step`
- `nested`
- `param_name`
- `sweep_values`
- `temps`

## Source
Lines 27–61 in `crates/oxide-types/src/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-types/src/sim.md) |
