---
okf_version: "0.2"
type: Class
title: WaveformDataset
description: Complete dataset returned by a simulation run.
resource: crates/oxide-types/src/sim.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:10:20Z"
concept_id: crates/oxide-types/src/sim/WaveformDataset
language: rust
---

# WaveformDataset

Complete dataset returned by a simulation run.

## Signature

```rust
pub struct WaveformDataset
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Complete dataset returned by a simulation run.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `title`
- `analysis_name`
- `x_trace`
- `traces`
- `operating_point`
- `log`

## Source
Lines 187–200 in `crates/oxide-types/src/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-types/src/sim.md) |
