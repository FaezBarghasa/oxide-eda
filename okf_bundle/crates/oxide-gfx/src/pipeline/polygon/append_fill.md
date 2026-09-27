---
okf_version: "0.2"
type: Function
title: append_fill
description: "Ear-clip fill, concave-safe. Assumes `polygon.vertices.len() >= 3`."
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/append_fill
language: rust
---

# append_fill

Ear-clip fill, concave-safe. Assumes `polygon.vertices.len() >= 3`.

## Signature

```rust
fn append_fill(vertices: &mut Vec<PolygonVertex>, polygon: &GpuPolygon)
```

## Docstring

Ear-clip fill, concave-safe. Assumes `polygon.vertices.len() >= 3`.

A triangle fan from `points[0]` is exact only for convex contours — a
concave copper pour/rule-area (an arbitrary user outline, frequently
non-convex) would bridge triangles across the notch and paint copper where
the pour has none. `oxide_sketch::ear_clip` is the authoritative
triangulator for exactly this contract (already driving the sketch
overlay's filled-loop renderer), reused here instead of re-derived so the
GPU fill partitions the same polygon area the CPU `frame.fill` (lyon)
tessellates. Falls back to the fan only for the degenerate cases
`ear_clip` refuses (self-intersecting / all-collinear contours) — the CPU
path has no defined behaviour for those either, so a fan is a reasonable
best-effort rather than drawing nothing.

## Source
Lines 60–81 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
| calls | [append_fan_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fan_fill.md) |
| called_by | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
