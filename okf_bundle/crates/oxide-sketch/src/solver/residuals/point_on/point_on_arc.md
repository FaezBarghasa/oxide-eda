---
okf_version: "0.2"
type: Function
title: point_on_arc
description: "PointOnArc: distance from `point` to the arc's centre equals the"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/point_on_arc
language: rust
---

# point_on_arc

PointOnArc: distance from `point` to the arc's centre equals the

## Signature

```rust
pub fn point_on_arc(
    point: SketchEntityId,
    arc: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

PointOnArc: distance from `point` to the arc's centre equals the
arc's underlying radius. The radius is implied by the arc's start
Point — `radius = |start − center|`.

This residual constrains `point` to lie on the FULL circle
through the arc's start; the start/end-sweep envelope is enforced
by the bake layer when rasterising.

## Source
Lines 89–102 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [arc_refs](/crates/oxide-sketch/src/solver/state/arc_refs.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
