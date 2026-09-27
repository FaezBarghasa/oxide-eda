---
okf_version: "0.2"
type: Function
title: bridge_hole_into_outer
description: "Bridge a hole into the outer ring. Pick the hole's rightmost"
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/bridge_hole_into_outer
language: rust
---

# bridge_hole_into_outer

Bridge a hole into the outer ring. Pick the hole's rightmost

## Signature

```rust
fn bridge_hole_into_outer(outer: &[Point2], hole: &[Point2]) -> Vec<Point2>
```

## Docstring

Bridge a hole into the outer ring. Pick the hole's rightmost
vertex, find the outer vertex closest to it (left-of, on a
horizontal ray), and cut a "bridge" edge that visits both
rings. The bridge is two coincident edges so the merged ring
is still simple.

## Source
Lines 249–282 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| called_by | [ear_clip_with_holes](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes.md) |
