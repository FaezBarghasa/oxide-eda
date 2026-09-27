---
okf_version: "0.2"
type: Function
title: tessellate_segment
description: "Expand one input segment into its ordered point list, endpoints"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/tessellate_segment
language: rust
---

# tessellate_segment

Expand one input segment into its ordered point list, endpoints

## Signature

```rust
fn tessellate_segment(seg: &ChainSegment) -> Vec<[f64; 2]>
```

## Docstring

Expand one input segment into its ordered point list, endpoints
included. `Line` is trivially its two endpoints; `Arc` is sampled
into `CHAIN_ARC_SAMPLES + 1` points per the module-level convention.

## Source
Lines 464–488 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [point_at_deg](/crates/oxide-library/src/primitive/symbol/chain/point_at_deg.md) |
