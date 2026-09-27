---
okf_version: "0.2"
type: Function
title: compute_mst
description: "Compute Minimum Spanning Tree across points using Prim's algorithm."
resource: crates/oxide-net/src/ratsnest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:14:41Z"
concept_id: crates/oxide-net/src/ratsnest/compute_mst_1
language: rust
---

# compute_mst

Compute Minimum Spanning Tree across points using Prim's algorithm.

## Signature

```rust
fn compute_mst(points: &[Point]) -> Vec<(Point, Point, f64)>
```

## Docstring

Compute Minimum Spanning Tree across points using Prim's algorithm.

## Source
Lines 68–103 in `crates/oxide-net/src/ratsnest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ratsnest](/crates/oxide-net/src/ratsnest.md) |
