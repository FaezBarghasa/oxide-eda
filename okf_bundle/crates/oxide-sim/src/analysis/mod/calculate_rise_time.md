---
okf_version: "0.2"
type: Function
title: calculate_rise_time
description: "Find 10% to 90% rise time of a step waveform."
resource: crates/oxide-sim/src/analysis/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:57Z"
concept_id: crates/oxide-sim/src/analysis/mod/calculate_rise_time
language: rust
---

# calculate_rise_time

Find 10% to 90% rise time of a step waveform.

## Signature

```rust
pub fn calculate_rise_time(time_vals: &[f64], trace_vals: &[f64]) -> Option<f64>
```

## Visibility

- `pub`

## Docstring

Find 10% to 90% rise time of a step waveform.

## Source
Lines 60–87 in `crates/oxide-sim/src/analysis/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analysis](/crates/oxide-sim/src/analysis/mod.md) |
| called_by | [calculate_rise_time_step](/crates/oxide-sim/src/analysis/mod/calculate_rise_time_step.md) |
