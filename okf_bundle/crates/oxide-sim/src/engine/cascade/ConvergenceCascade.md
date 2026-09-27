---
okf_version: "0.2"
type: Class
title: ConvergenceCascade
description: Configuration and state for the 4-stage convergence recovery cascade.
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade/ConvergenceCascade
language: rust
---

# ConvergenceCascade

Configuration and state for the 4-stage convergence recovery cascade.

## Signature

```rust
pub struct ConvergenceCascade
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration and state for the 4-stage convergence recovery cascade.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `current_stage`
- `line_search_alpha`
- `gmin_start`
- `gmin_target`
- `gmin_current`
- `gmin_step_count`
- `source_alpha`
- `source_step_size`
- `ptc_tau`
- `ptc_pseudo_dt`

## Source
Lines 15–26 in `crates/oxide-sim/src/engine/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-sim/src/engine/cascade.md) |
