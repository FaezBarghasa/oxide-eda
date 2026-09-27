# segment

## Classs

- [Arc2](Arc2.md) — 2D circular arc. `start_rad` and `end_rad` are angles in radians
- [Circle2](Circle2.md) — 2D circle.
- [Segment2](Segment2.md) — 2D line segment from `a` to `b`. Endpoints stored as bare points
- [SegmentIntersection](SegmentIntersection.md) — Outcome of a segment ↔ segment intersection query.

## Functions

- [arc_angle_containment_within_quadrant](arc_angle_containment_within_quadrant.md) — [test]
- [arc_seam_crossing](arc_seam_crossing.md) — [test]
- [at](at.md) — Linearly interpolate along the segment. `t = 0` returns `a`,
- [at](at_1.md) — Linearly interpolate along the segment. `t = 0` returns `a`,
- [close](close.md)
- [colinear_overlap](colinear_overlap.md) — [test]
- [contains_angle](contains_angle.md) — `true` when the angle `theta` (any reference frame, will be
- [contains_angle](contains_angle_1.md) — `true` when the angle `theta` (any reference frame, will be
- [cross_intersection](cross_intersection.md) — [test]
- [dx](dx.md)
- [dx](dx_1.md)
- [dy](dy.md)
- [dy](dy_1.md)
- [left_turn_or_colinear](left_turn_or_colinear.md) — `true` when the three points `a`, `b`, `c` form a left turn (or
- [length_sq](length_sq.md)
- [length_sq](length_sq_1.md)
- [miss_outside_segment_range](miss_outside_segment_range.md) — [test]
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [p](p.md)
- [parallel_disjoint](parallel_disjoint.md) — [test]
- [segment_arc_filters_outside_sweep](segment_arc_filters_outside_sweep.md) — [test]
- [segment_arc_intersections](segment_arc_intersections.md) — Intersect a segment with an arc. Built on top of
- [segment_circle_intersections](segment_circle_intersections.md) — Intersect a segment with a circle. Returns 0, 1, or 2 hit points
- [segment_circle_miss](segment_circle_miss.md) — [test]
- [segment_circle_partial_inside_segment](segment_circle_partial_inside_segment.md) — [test]
- [segment_circle_tangent_one_hit](segment_circle_tangent_one_hit.md) — [test]
- [segment_circle_two_hits](segment_circle_two_hits.md) — [test]
- [segment_segment_intersection](segment_segment_intersection.md) — Intersect two line segments. Returns `None` for parallel-disjoint
- [t_intersection_at_endpoint](t_intersection_at_endpoint.md) — [test]
