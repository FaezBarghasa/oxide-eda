# boolean_general

## Classs

- [BoolOp](BoolOp.md) — Boolean operation type.
- [Vertex](Vertex.md) — [derive(Clone, Copy, Debug)]

## Functions

- [area](area.md)
- [build_ring](build_ring.md) — Build a closed ring from a slice of corner positions. The
- [classify_entries](classify_entries.md) — Walk the whole ring once and assign `entry` to each
- [corner](corner.md)
- [corner](corner_1.md)
- [disjoint_difference_is_subject](disjoint_difference_is_subject.md) — [test]
- [disjoint_intersection_is_empty](disjoint_intersection_is_empty.md) — [test]
- [disjoint_union_is_two_polygons](disjoint_union_is_two_polygons.md) — [test]
- [first_corner](first_corner.md) — Find the first non-intersection vertex starting from `start`.
- [fully_contained_intersection_returns_inner](fully_contained_intersection_returns_inner.md) — [test]
- [fully_contained_union_returns_outer](fully_contained_union_returns_outer.md) — [test]
- [insert_after_in_alpha_order](insert_after_in_alpha_order.md) — Insert intersection vertex at the right edge-sorted position
- [intersection](intersection.md)
- [intersection](intersection_1.md)
- [is_walk_start](is_walk_start.md) — True when intersection vertex `idx` is unvisited and matches
- [p](p.md)
- [point_in_ring](point_in_ring.md) — Even-odd point-in-polygon test against the linked ring rooted
- [polygon_op](polygon_op.md) — Compute `subject ∩ clip`, `subject ∪ clip`, or `subject − clip`
- [proper_segment_intersection](proper_segment_intersection.md) — Strict-interior segment×segment intersection. Returns the hit
- [quarter_overlap_intersection_unit_area](quarter_overlap_intersection_unit_area.md) — [test]
- [subject_start_pos_or_default](subject_start_pos_or_default.md)
- [walk_one_ring](walk_one_ring.md) — Walk one output ring starting at `start` (an intersection on
