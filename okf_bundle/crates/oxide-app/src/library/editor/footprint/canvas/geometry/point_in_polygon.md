---
okf_version: "0.2"
type: Function
title: point_in_polygon
description: Even-odd point-in-polygon test (implicitly-closed vertex ring) — a thin
resource: crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_in_polygon
language: rust
---

# point_in_polygon

Even-odd point-in-polygon test (implicitly-closed vertex ring) — a thin

## Signature

```rust
pub(super) fn point_in_polygon(px: f64, py: f64, vertices: &[[f64; 2]]) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Even-odd point-in-polygon test (implicitly-closed vertex ring) — a thin
adapter over [`oxide_sketch::geom::point_in_polygon`].

## Source
Lines 36–39 in `crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/canvas/geometry.md) |
