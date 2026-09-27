---
okf_version: "0.2"
type: Function
title: entity_radius
description: Resolve the radius of a Circle (from state vector) or an Arc
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius
language: rust
---

# entity_radius

Resolve the radius of a Circle (from state vector) or an Arc

## Signature

```rust
fn entity_radius(
    id: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<f64, SketchError>
```

## Docstring

Resolve the radius of a Circle (from state vector) or an Arc
(computed as `|start − center|`). Returns `EntityNotFound` if the
entity is neither.

## Source
Lines 43–65 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [find_entity](/crates/oxide-sketch/src/solver/state/find_entity.md) |
| calls | [circle_radius](/crates/oxide-sketch/src/solver/state/circle_radius.md) |
| calls | [arc_refs](/crates/oxide-sketch/src/solver/state/arc_refs.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| called_by | [equal_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_radius.md) |
| called_by | [tangent_arc_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_arc_arc.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
