# aabb_index

## Classs

- [Aabb](Aabb.md) — Axis-aligned bounding box in plane-local mm.
- [AabbIndex](AabbIndex.md) — Flat-array AABB index. The user inserts `(item, bbox)` pairs;

## Functions

- [aabb_contains_inclusive](aabb_contains_inclusive.md) — [test]
- [aabb_from_points_empty](aabb_from_points_empty.md) — [test]
- [aabb_from_points_finds_extents](aabb_from_points_finds_extents.md) — [test]
- [aabb_overlaps_disjoint_is_false](aabb_overlaps_disjoint_is_false.md) — [test]
- [aabb_overlaps_touching_is_true](aabb_overlaps_touching_is_true.md) — [test]
- [clear](clear.md)
- [clear](clear_1.md)
- [contains](contains.md)
- [contains](contains_1.md)
- [expanded](expanded.md) — Expand the box by `pad` in every direction. Used to query
- [expanded](expanded_1.md) — Expand the box by `pad` in every direction. Used to query
- [expanded_grows_in_all_directions](expanded_grows_in_all_directions.md) — [test]
- [from_points](from_points.md) — Build an Aabb covering all `points`. Returns `None` for an
- [from_points](from_points_1.md) — Build an Aabb covering all `points`. Returns `None` for an
- [index_query_point_finds_overlapping](index_query_point_finds_overlapping.md) — [test]
- [index_query_region_iterator](index_query_region_iterator.md) — [test]
- [insert](insert.md)
- [insert](insert_1.md)
- [is_empty](is_empty.md)
- [is_empty](is_empty_1.md)
- [len](len.md)
- [len](len_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [overlaps](overlaps.md) — `true` when this box overlaps `other` — inclusive on the
- [overlaps](overlaps_1.md) — `true` when this box overlaps `other` — inclusive on the
- [p](p.md)
- [query_point](query_point.md) — Items whose bbox contains `p`. Returns clones so callers
- [query_point](query_point_1.md) — Items whose bbox contains `p`. Returns clones so callers
- [query_region](query_region.md) — Items whose bbox overlaps `region`. Iterator-style so
- [query_region](query_region_1.md) — Items whose bbox overlaps `region`. Iterator-style so
- [with_capacity](with_capacity.md)
- [with_capacity](with_capacity_1.md)
