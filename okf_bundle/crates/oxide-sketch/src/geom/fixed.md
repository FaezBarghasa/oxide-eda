---
okf_version: "0.2"
type: Module
title: fixed
description: Fixed-point arithmetic for deterministic geometry.
resource: crates/oxide-sketch/src/geom/fixed.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/fixed
language: rust
---

# fixed

Fixed-point arithmetic for deterministic geometry.

## Docstring

Fixed-point arithmetic for deterministic geometry.

`f64` results aren't bit-stable across machines because the
order of operations and the FPU rounding mode can shift the
last few bits. For tests / persistence / cross-machine
reproducibility we want geometry that returns IDENTICAL
integer coordinates from identical inputs.

`FixPoint2` carries `(i64, i64)` coordinates that map to mm
through a power-of-two `SCALE`. Default `SCALE = 1024` gives
≈ 0.001 mm precision (≈ 1 µm) over a ±1 km world — way more
headroom than any practical PCB.

The `to_f64` / `from_f64` helpers round-trip with deterministic
truncation. Geometry on `FixPoint2` is integer arithmetic
end-to-end, so two runs with the same inputs produce
byte-identical outputs.

## Relationships

| Type | Target |
|------|--------|
| related | [FixPoint2](/crates/oxide-sketch/src/geom/fixed/FixPoint2.md) |
| related | [new](/crates/oxide-sketch/src/geom/fixed/new.md) |
| related | [from_f64](/crates/oxide-sketch/src/geom/fixed/from_f64.md) |
| related | [to_f64](/crates/oxide-sketch/src/geom/fixed/to_f64.md) |
| related | [new](/crates/oxide-sketch/src/geom/fixed/new.md) |
| related | [from_f64](/crates/oxide-sketch/src/geom/fixed/from_f64.md) |
| related | [to_f64](/crates/oxide-sketch/src/geom/fixed/to_f64.md) |
| related | [signed_area_2x](/crates/oxide-sketch/src/geom/fixed/signed_area_2x.md) |
| related | [orient2d](/crates/oxide-sketch/src/geom/fixed/orient2d.md) |
| related | [polygon_from_f64](/crates/oxide-sketch/src/geom/fixed/polygon_from_f64.md) |
| related | [polygon_to_f64](/crates/oxide-sketch/src/geom/fixed/polygon_to_f64.md) |
| related | [p](/crates/oxide-sketch/src/geom/fixed/p.md) |
| related | [round_trip_preserves_value_at_scale_resolution](/crates/oxide-sketch/src/geom/fixed/round_trip_preserves_value_at_scale_resolution.md) |
| related | [signed_area_unit_square_is_2_times_one](/crates/oxide-sketch/src/geom/fixed/signed_area_unit_square_is_2_times_one.md) |
| related | [orient2d_ccw_triangle_positive](/crates/oxide-sketch/src/geom/fixed/orient2d_ccw_triangle_positive.md) |
| related | [orient2d_colinear_zero](/crates/oxide-sketch/src/geom/fixed/orient2d_colinear_zero.md) |
| related | [round_trip_polygon](/crates/oxide-sketch/src/geom/fixed/round_trip_polygon.md) |
