---
okf_version: "0.2"
type: Function
title: equal_radius
description: "EqualRadius: `r2 − r1 = 0`. Each entity may be a Circle or an"
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_radius
language: rust
---

# equal_radius

EqualRadius: `r2 − r1 = 0`. Each entity may be a Circle or an

## Signature

```rust
pub fn equal_radius(
    e1: SketchEntityId,
    e2: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

EqualRadius: `r2 − r1 = 0`. Each entity may be a Circle or an
Arc; the dispatch is handled by [`entity_radius`].

## Source
Lines 98–108 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
