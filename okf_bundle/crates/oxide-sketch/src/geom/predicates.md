---
okf_version: "0.2"
type: Module
title: predicates
description: Geometric predicates with epsilon-aware sign returns.
resource: crates/oxide-sketch/src/geom/predicates.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/predicates
language: rust
---

# predicates

Geometric predicates with epsilon-aware sign returns.

## Docstring

Geometric predicates with epsilon-aware sign returns.

These wrap the textbook formulae for orientation and signed area
with a tolerance-aware sign so callers don't have to repeat the
same `det.abs() < EPS` boilerplate everywhere. PCB sketch
tolerances live at the mm scale where IEEE-754 f64 has ~12
decimal digits of headroom; the relative + absolute bound check
below is enough without a multi-precision fallback.

## Relationships

| Type | Target |
|------|--------|
| related | [Sign](/crates/oxide-sketch/src/geom/predicates/Sign.md) |
| related | [from_signed](/crates/oxide-sketch/src/geom/predicates/from_signed.md) |
| related | [from_signed](/crates/oxide-sketch/src/geom/predicates/from_signed.md) |
| related | [orient2d](/crates/oxide-sketch/src/geom/predicates/orient2d.md) |
| related | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| related | [p](/crates/oxide-sketch/src/geom/predicates/p.md) |
| related | [ccw_triangle_is_positive](/crates/oxide-sketch/src/geom/predicates/ccw_triangle_is_positive.md) |
| related | [cw_triangle_is_negative](/crates/oxide-sketch/src/geom/predicates/cw_triangle_is_negative.md) |
| related | [colinear_points_are_zero](/crates/oxide-sketch/src/geom/predicates/colinear_points_are_zero.md) |
| related | [near_colinear_within_tolerance_zero](/crates/oxide-sketch/src/geom/predicates/near_colinear_within_tolerance_zero.md) |
| related | [signed_area_unit_square_ccw_is_one](/crates/oxide-sketch/src/geom/predicates/signed_area_unit_square_ccw_is_one.md) |
| related | [signed_area_unit_square_cw_is_negative_one](/crates/oxide-sketch/src/geom/predicates/signed_area_unit_square_cw_is_negative_one.md) |
| related | [signed_area_degenerate_returns_zero](/crates/oxide-sketch/src/geom/predicates/signed_area_degenerate_returns_zero.md) |
