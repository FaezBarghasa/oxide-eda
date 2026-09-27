---
okf_version: "0.2"
type: Function
title: norm
description: "Euclidean norm `|v| = sqrt(v.x² + v.y²)`. Uses [`f64::hypot`]"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/norm
language: rust
---

# norm

Euclidean norm `|v| = sqrt(v.x² + v.y²)`. Uses [`f64::hypot`]

## Signature

```rust
pub fn norm(v: Vec2) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Euclidean norm `|v| = sqrt(v.x² + v.y²)`. Uses [`f64::hypot`]
for numerical stability on extreme-magnitude inputs.
[inline]

## Source
Lines 72–74 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [distance](/crates/oxide-sketch/src/solver/math/distance.md) |
| called_by | [line_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
| called_by | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
