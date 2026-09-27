---
okf_version: "0.2"
type: Function
title: pole_of_inaccessibility
description: Find the pole of inaccessibility — the point inside the
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel/pole_of_inaccessibility
language: rust
---

# pole_of_inaccessibility

Find the pole of inaccessibility — the point inside the

## Signature

```rust
pub fn pole_of_inaccessibility(polygon: &[Point2], precision: f64) -> Option<Point2>
```

## Visibility

- `pub`

## Docstring

Find the pole of inaccessibility — the point inside the
polygon with the maximum minimum-distance-to-edge. Returns
`None` for fewer than three vertices.

`precision` controls when subdivision stops. Smaller =
better label position but more work; `0.5 mm` is plenty for
PCB designator placement on typical pads.

## Source
Lines 87–159 in `crates/oxide-sketch/src/geom/polylabel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polylabel](/crates/oxide-sketch/src/geom/polylabel.md) |
| calls | [polygon_centroid](/crates/oxide-sketch/src/geom/polylabel/polygon_centroid.md) |
| called_by | [l_shape_pole_in_thicker_arm](/crates/oxide-sketch/src/geom/polylabel/l_shape_pole_in_thicker_arm.md) |
| called_by | [rectangle_pole_at_centre](/crates/oxide-sketch/src/geom/polylabel/rectangle_pole_at_centre.md) |
| called_by | [unit_square_pole_at_centre](/crates/oxide-sketch/src/geom/polylabel/unit_square_pole_at_centre.md) |
