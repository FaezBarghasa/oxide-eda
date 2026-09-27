---
okf_version: "0.2"
type: Function
title: entity_center_xy
description: "Resolve the centre `(x, y)` of an Arc or Circle."
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_center_xy
language: rust
---

# entity_center_xy

Resolve the centre `(x, y)` of an Arc or Circle.

## Signature

```rust
fn entity_center_xy(
    id: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec2, SketchError>
```

## Docstring

Resolve the centre `(x, y)` of an Arc or Circle.

## Source
Lines 68–81 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [find_entity](/crates/oxide-sketch/src/solver/state/find_entity.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| called_by | [tangent_arc_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_arc_arc.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
