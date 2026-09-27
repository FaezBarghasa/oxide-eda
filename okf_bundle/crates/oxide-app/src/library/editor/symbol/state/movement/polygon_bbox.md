---
okf_version: "0.2"
type: Function
title: polygon_bbox
description: "Axis-aligned bounding box over a polygon's vertices. `None` for an"
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/polygon_bbox
language: rust
---

# polygon_bbox

Axis-aligned bounding box over a polygon's vertices. `None` for an

## Signature

```rust
fn polygon_bbox(vertices: &[[f64; 2]]) -> Option<(f64, f64, f64, f64)>
```

## Docstring

Axis-aligned bounding box over a polygon's vertices. `None` for an
empty vertex list (degenerate — box-select treats it as a miss).

## Source
Lines 343–352 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| called_by | [graphic_fully_inside_box](/crates/oxide-app/src/library/editor/symbol/state/movement/graphic_fully_inside_box.md) |
| called_by | [graphic_intersects_box](/crates/oxide-app/src/library/editor/symbol/state/movement/graphic_intersects_box.md) |
