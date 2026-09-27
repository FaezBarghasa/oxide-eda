---
okf_version: "0.2"
type: Function
title: cascade_step
resource: crates/oxide-router/src/interactive/conflict.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:06:43Z"
concept_id: crates/oxide-router/src/interactive/conflict/cascade_step
language: rust
---

# cascade_step

## Signature

```rust
impl PushAndShoveEngine { fn cascade_step(
        &self,
        spatial_index: &SpatialIndex,
        results: &mut Vec<PushResult>,
        visited: &mut HashSet<ObjectId>,
        pushed_positions: &mut HashMap<ObjectId, Point2D>,
        source_obs: &SpatialObject,
        required_clearance: Microns,
        depth: usize,
    ) }
```

## Source
Lines 125–179 in `crates/oxide-router/src/interactive/conflict.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [conflict](/crates/oxide-router/src/interactive/conflict.md) |
