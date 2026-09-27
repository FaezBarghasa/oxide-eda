---
okf_version: "0.2"
type: Function
title: point_and_line
description: "Resolve `(point_xy, line_endpoints[0], line_endpoints[1])` for a"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/point_and_line
language: rust
---

# point_and_line

Resolve `(point_xy, line_endpoints[0], line_endpoints[1])` for a

## Signature

```rust
fn point_and_line(
    point: SketchEntityId,
    line: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<(Vec2, Vec2, Vec2), SketchError>
```

## Docstring

Resolve `(point_xy, line_endpoints[0], line_endpoints[1])` for a
`point + line` constraint. Centralises the three `EntityNotFound`
error sites so [`point_on_line`] / [`distance_pt_line`] stay
short.

## Source
Lines 50–62 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| called_by | [distance_pt_line](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_line.md) |
| called_by | [point_on_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_line.md) |
