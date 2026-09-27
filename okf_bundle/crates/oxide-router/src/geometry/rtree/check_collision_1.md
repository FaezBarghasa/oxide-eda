---
okf_version: "0.2"
type: Function
title: check_collision
description: "Check collision with bounding box, optionally excluding specified net IDs."
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/check_collision_1
language: rust
---

# check_collision

Check collision with bounding box, optionally excluding specified net IDs.

## Signature

```rust
pub fn check_collision(
        &self,
        bbox: &BoundingBox,
        exclude_nets: &[NetId],
    ) -> Vec<&SpatialObject>
```

## Visibility

- `pub`

## Docstring

Check collision with bounding box, optionally excluding specified net IDs.

## Source
Lines 153–171 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
