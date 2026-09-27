---
okf_version: "0.2"
type: Function
title: append_fan_fill
description: Triangle-fan fill from the first vertex — exact for convex contours only.
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/append_fan_fill
language: rust
---

# append_fan_fill

Triangle-fan fill from the first vertex — exact for convex contours only.

## Signature

```rust
fn append_fan_fill(vertices: &mut Vec<PolygonVertex>, points: &[[f32; 2]], fill_color: [f32; 4])
```

## Docstring

Triangle-fan fill from the first vertex — exact for convex contours only.
Used as the [`append_fill`] fallback when `ear_clip` can't triangulate the
contour at all.

## Source
Lines 86–102 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| called_by | [append_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fill.md) |
