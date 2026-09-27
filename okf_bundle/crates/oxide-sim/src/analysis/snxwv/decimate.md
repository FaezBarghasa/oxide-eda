---
okf_version: "0.2"
type: Function
title: decimate
description: "Decimates raw time and value slices into $N_{\\text{pixels}}$ min/max envelope buckets."
resource: crates/oxide-sim/src/analysis/snxwv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:36Z"
concept_id: crates/oxide-sim/src/analysis/snxwv/decimate
language: rust
---

# decimate

Decimates raw time and value slices into $N_{\text{pixels}}$ min/max envelope buckets.

## Signature

```rust
impl WaveformDecimator { pub fn decimate(
        time: &[f64],
        values: &[f64],
        pixel_count: usize,
        t_view_start: f64,
        t_view_end: f64,
    ) -> Vec<DecimationBucket> }
```

## Visibility

- `pub`

## Docstring

Decimates raw time and value slices into $N_{\text{pixels}}$ min/max envelope buckets.

## Source
Lines 49–122 in `crates/oxide-sim/src/analysis/snxwv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snxwv](/crates/oxide-sim/src/analysis/snxwv.md) |
