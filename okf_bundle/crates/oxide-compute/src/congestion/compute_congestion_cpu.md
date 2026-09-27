---
okf_version: "0.2"
type: Function
title: compute_congestion_cpu
resource: crates/oxide-compute/src/congestion.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/congestion/compute_congestion_cpu
language: rust
---

# compute_congestion_cpu

## Signature

```rust
impl CongestionMapGenerator { pub fn compute_congestion_cpu(&self, tracks: &[GpuTrack]) -> CongestionResult }
```

## Visibility

- `pub`

## Source
Lines 108–169 in `crates/oxide-compute/src/congestion.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [congestion](/crates/oxide-compute/src/congestion.md) |
