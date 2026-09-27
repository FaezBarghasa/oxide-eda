---
okf_version: "0.2"
type: Function
title: polygon_op
description: "Compute `subject ∩ clip`, `subject ∪ clip`, or `subject − clip`"
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/polygon_op
language: rust
---

# polygon_op

Compute `subject ∩ clip`, `subject ∪ clip`, or `subject − clip`

## Signature

```rust
pub fn polygon_op(subject: &[Point2], clip: &[Point2], op: BoolOp) -> Vec<Vec<Point2>>
```

## Visibility

- `pub`

## Docstring

Compute `subject ∩ clip`, `subject ∪ clip`, or `subject − clip`
for general (concave) polygons. Returns a list of result rings;
boolean operations on connected inputs can yield multiple
disjoint output polygons.

Disjoint inputs short-circuit:
- Disjoint + `Intersection` → empty.
- Disjoint + `Union` → both polygons.
- Disjoint + `Difference` → subject only.

Subject fully inside clip:
- `Intersection` → subject.
- `Union` → clip.
- `Difference` → empty.

Clip fully inside subject:
- `Intersection` → clip.
- `Union` → subject.
- `Difference` → subject as outer + clip reversed as a hole
(callers needing hole support should request the result
rings separately — for now this path returns the subject
ring only and is documented as a limitation).

## Source
Lines 380–453 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| calls | [build_ring](/crates/oxide-sketch/src/geom/boolean_general/build_ring.md) |
| calls | [proper_segment_intersection](/crates/oxide-sketch/src/geom/boolean_general/proper_segment_intersection.md) |
| calls | [insert_after_in_alpha_order](/crates/oxide-sketch/src/geom/boolean_general/insert_after_in_alpha_order.md) |
| calls | [point_in_ring](/crates/oxide-sketch/src/geom/boolean_general/point_in_ring.md) |
| calls | [first_corner](/crates/oxide-sketch/src/geom/boolean_general/first_corner.md) |
| calls | [classify_entries](/crates/oxide-sketch/src/geom/boolean_general/classify_entries.md) |
| calls | [is_walk_start](/crates/oxide-sketch/src/geom/boolean_general/is_walk_start.md) |
| calls | [walk_one_ring](/crates/oxide-sketch/src/geom/boolean_general/walk_one_ring.md) |
| called_by | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard_outline.md) |
| called_by | [disjoint_difference_is_subject](/crates/oxide-sketch/src/geom/boolean_general/disjoint_difference_is_subject.md) |
| called_by | [disjoint_intersection_is_empty](/crates/oxide-sketch/src/geom/boolean_general/disjoint_intersection_is_empty.md) |
| called_by | [disjoint_union_is_two_polygons](/crates/oxide-sketch/src/geom/boolean_general/disjoint_union_is_two_polygons.md) |
| called_by | [fully_contained_intersection_returns_inner](/crates/oxide-sketch/src/geom/boolean_general/fully_contained_intersection_returns_inner.md) |
| called_by | [fully_contained_union_returns_outer](/crates/oxide-sketch/src/geom/boolean_general/fully_contained_union_returns_outer.md) |
| called_by | [quarter_overlap_intersection_unit_area](/crates/oxide-sketch/src/geom/boolean_general/quarter_overlap_intersection_unit_area.md) |
