---
okf_version: "0.2"
type: Function
title: sub
description: "Component-wise subtraction `a − b`."
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/sub
language: rust
---

# sub

Component-wise subtraction `a − b`.

## Signature

```rust
pub fn sub(a: Vec2, b: Vec2) -> Vec2
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Component-wise subtraction `a − b`.
[inline]

## Source
Lines 39–41 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [distance](/crates/oxide-sketch/src/solver/math/distance.md) |
| called_by | [line_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
| called_by | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
| called_by | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
| called_by | [midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/midpoint.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
| called_by | [symmetric_about_point](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_point.md) |
| called_by | [retarget_relational](/crates/oxide-sketch/src/split/constraints/retarget_relational.md) |
