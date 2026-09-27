---
okf_version: "0.2"
type: Function
title: signed_perp_distance
description: "Signed perpendicular distance from point `p` to the infinite line"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance
language: rust
---

# signed_perp_distance

Signed perpendicular distance from point `p` to the infinite line

## Signature

```rust
fn signed_perp_distance(p: Vec2, a: Vec2, b: Vec2) -> Option<f64>
```

## Docstring

Signed perpendicular distance from point `p` to the infinite line
through `a, b`. `None` if `|b − a| < DEGENERATE_LEN_EPS` (line is
degenerate; perpendicular has no defined direction).

## Source
Lines 37–44 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| calls | [norm](/crates/oxide-sketch/src/solver/math/norm.md) |
| calls | [cross](/crates/oxide-sketch/src/solver/math/cross.md) |
| called_by | [distance_pt_line](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_line.md) |
| called_by | [point_on_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_line.md) |
