---
okf_version: "0.2"
type: Function
title: polygon_from_f64
description: Convert a slice of f64 points to fixed-point.
resource: crates/oxide-sketch/src/geom/fixed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/fixed/polygon_from_f64
language: rust
---

# polygon_from_f64

Convert a slice of f64 points to fixed-point.

## Signature

```rust
pub fn polygon_from_f64(polygon: &[Point2]) -> Vec<FixPoint2>
```

## Visibility

- `pub`

## Docstring

Convert a slice of f64 points to fixed-point.

## Source
Lines 83–85 in `crates/oxide-sketch/src/geom/fixed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fixed](/crates/oxide-sketch/src/geom/fixed.md) |
| called_by | [round_trip_polygon](/crates/oxide-sketch/src/geom/fixed/round_trip_polygon.md) |
