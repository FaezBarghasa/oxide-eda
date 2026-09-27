---
okf_version: "0.2"
type: Function
title: distance_pt_circle
description: "v0.23 — DistancePtCircle: signed offset from `point` to the"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_circle
language: rust
---

# distance_pt_circle

v0.23 — DistancePtCircle: signed offset from `point` to the

## Signature

```rust
pub fn distance_pt_circle(
    point: SketchEntityId,
    circle: SketchEntityId,
    target_mm: f64,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

v0.23 — DistancePtCircle: signed offset from `point` to the
boundary of `circle`. Residual is `|p - centre| - radius - target`.
`target = 0` reduces to "point on the circle". Positive target
offsets outward (further from centre); negative offsets inward.

Works on both `EntityKind::Circle { center, radius }` and
`EntityKind::Arc` — for arcs, the radius is derived from the
`start` point's distance to `center` (matching `point_on_arc`
semantics).

## Source
Lines 132–164 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [circle_radius](/crates/oxide-sketch/src/solver/state/circle_radius.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
