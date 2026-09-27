---
okf_version: "0.2"
type: Function
title: query_bbox
description: "Query objects whose bounding box intersects `bbox`."
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/query_bbox_1
language: rust
---

# query_bbox

Query objects whose bounding box intersects `bbox`.

## Signature

```rust
pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<&SpatialObject>
```

## Visibility

- `pub`

## Docstring

Query objects whose bounding box intersects `bbox`.

## Source
Lines 127–132 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
