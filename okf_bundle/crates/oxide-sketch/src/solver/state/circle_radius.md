---
okf_version: "0.2"
type: Function
title: circle_radius
description: "Look up the radius of a [`EntityKind::Circle`] from the state"
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/circle_radius
language: rust
---

# circle_radius

Look up the radius of a [`EntityKind::Circle`] from the state

## Signature

```rust
pub fn circle_radius(id: SketchEntityId, state: &[f64], index: &EntityIndex) -> Option<f64>
```

## Visibility

- `pub`

## Docstring

Look up the radius of a [`EntityKind::Circle`] from the state
vector. Returns `None` for non-circle entities.

## Source
Lines 89–92 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
| called_by | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| called_by | [distance_pt_circle](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_circle.md) |
| called_by | [lm_solves_offset_circle_via_distance_pt_circle](/crates/oxide-sketch/tests/lm_basic/lm_solves_offset_circle_via_distance_pt_circle.md) |
| called_by | [pack_circle_radius_is_a_free_var](/crates/oxide-sketch/tests/solver_basics/pack_circle_radius_is_a_free_var.md) |
