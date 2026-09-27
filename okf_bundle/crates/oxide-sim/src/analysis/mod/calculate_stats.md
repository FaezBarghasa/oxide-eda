---
okf_version: "0.2"
type: Function
title: calculate_stats
description: Calculate basic statistics for a waveform trace.
resource: crates/oxide-sim/src/analysis/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:57Z"
concept_id: crates/oxide-sim/src/analysis/mod/calculate_stats
language: rust
---

# calculate_stats

Calculate basic statistics for a waveform trace.

## Signature

```rust
pub fn calculate_stats(trace: &WaveformTrace) -> Option<TraceStats>
```

## Visibility

- `pub`

## Docstring

Calculate basic statistics for a waveform trace.

## Source
Lines 24–57 in `crates/oxide-sim/src/analysis/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analysis](/crates/oxide-sim/src/analysis/mod.md) |
| called_by | [calculate_stats_basic](/crates/oxide-sim/src/analysis/mod/calculate_stats_basic.md) |
