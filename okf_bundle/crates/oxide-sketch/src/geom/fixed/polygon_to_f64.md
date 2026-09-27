---
okf_version: "0.2"
type: Function
title: polygon_to_f64
description: Convert a slice of fixed-point points back to f64.
resource: crates/oxide-sketch/src/geom/fixed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/fixed/polygon_to_f64
language: rust
---

# polygon_to_f64

Convert a slice of fixed-point points back to f64.

## Signature

```rust
pub fn polygon_to_f64(polygon: &[FixPoint2]) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Convert a slice of fixed-point points back to f64.

## Source
Lines 88–90 in `crates/oxide-sketch/src/geom/fixed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fixed](/crates/oxide-sketch/src/geom/fixed.md) |
| called_by | [round_trip_polygon](/crates/oxide-sketch/src/geom/fixed/round_trip_polygon.md) |
