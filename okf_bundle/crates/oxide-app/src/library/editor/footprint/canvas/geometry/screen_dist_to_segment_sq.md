---
okf_version: "0.2"
type: Function
title: screen_dist_to_segment_sq
description: v0.18.25 — squared distance from a screen-space point to a line
resource: crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/geometry/screen_dist_to_segment_sq
language: rust
---

# screen_dist_to_segment_sq

v0.18.25 — squared distance from a screen-space point to a line

## Signature

```rust
pub(super) fn screen_dist_to_segment_sq(p: Point, a: Point, b: Point) -> f32
```

## Visibility

- `pub(super)`

## Docstring

v0.18.25 — squared distance from a screen-space point to a line
segment. Standard projection-onto-segment with clamped t ∈ [0, 1].
Used by `sketch_hit_other` to score nearest-Line candidates.

## Source
Lines 10–26 in `crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/canvas/geometry.md) |
| called_by | [sketch_hit_other](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_hit_other.md) |
