---
okf_version: "0.2"
type: Function
title: walk_cycle
description: "Walk the single simple cycle [`validate_topology`] already confirmed,"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/walk_cycle
language: rust
---

# walk_cycle

Walk the single simple cycle [`validate_topology`] already confirmed,

## Signature

```rust
fn walk_cycle(polylines: &[Vec<[f64; 2]>], clusters: &EndpointClusters) -> Vec<[f64; 2]>
```

## Docstring

Walk the single simple cycle [`validate_topology`] already confirmed,
reversing each segment's tessellated polyline as needed, and return
the raw (not yet deduplicated / wound) point sequence.

## Source
Lines 371–401 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
