---
okf_version: "0.2"
type: Function
title: cross
description: "2D scalar cross product `a × b = a.x·b.y − a.y·b.x`. Returns the"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/cross
language: rust
---

# cross

2D scalar cross product `a × b = a.x·b.y − a.y·b.x`. Returns the

## Signature

```rust
pub fn cross(a: Vec2, b: Vec2) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

2D scalar cross product `a × b = a.x·b.y − a.y·b.x`. Returns the
signed magnitude of the 3D cross's z-component, useful as a
"side of line" test (positive = `b` is left of `a`).
[inline]

## Source
Lines 65–67 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
| called_by | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
