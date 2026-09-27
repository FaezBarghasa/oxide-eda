---
okf_version: "0.2"
type: Function
title: compute_congestion
resource: crates/oxide-compute/src/congestion.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/congestion/compute_congestion_1
language: rust
---

# compute_congestion

## Signature

```rust
pub fn compute_congestion(
        &mut self,
        tracks: &[GpuTrack],
    ) -> Result<CongestionResult, crate::backend::ComputeError>
```

## Visibility

- `pub`

## Source
Lines 65–106 in `crates/oxide-compute/src/congestion.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [congestion](/crates/oxide-compute/src/congestion.md) |
