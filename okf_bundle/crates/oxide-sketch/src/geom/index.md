# geom

## Subdirectories

- [aabb_index](aabb_index/index.md)
- [boolean](boolean/index.md)
- [boolean_general](boolean_general/index.md)
- [curves](curves/index.md)
- [fixed](fixed/index.md)
- [halfedge](halfedge/index.md)
- [hull](hull/index.md)
- [mod](mod/index.md)
- [offset](offset/index.md)
- [offset_arc](offset_arc/index.md)
- [point](point/index.md)
- [polylabel](polylabel/index.md)
- [predicates](predicates/index.md)
- [segment](segment/index.md)
- [simplify](simplify/index.md)
- [triangulate](triangulate/index.md)

## Modules

- [aabb_index](aabb_index.md) — Bounding-box spatial index for 2D primitives.
- [boolean](boolean.md) — Polygon boolean operations.
- [boolean_general](boolean_general.md) — General polygon boolean operations — union, intersection,
- [curves](curves.md) — Curve × curve intersections — Circle × Circle, Arc × Circle,
- [fixed](fixed.md) — Fixed-point arithmetic for deterministic geometry.
- [geom](mod.md) — 2D computational-geometry primitives for the sketch crate.
- [halfedge](halfedge.md) — Half-edge mesh data structure for 2D planar polygons.
- [hull](hull.md) — Convex hull via the monotone-chain method, O(n log n).
- [offset](offset.md) — Polygon offset (Minkowski-style outward / inward expansion).
- [offset_arc](offset_arc.md) — Arc-aware polygon offset. Accepts a closed polyline whose
- [point](point.md) — Point / segment / polygon predicates shared across the editor surfaces.
- [polylabel](polylabel.md) — Pole of inaccessibility — finds the point inside a polygon
- [predicates](predicates.md) — Geometric predicates with epsilon-aware sign returns.
- [segment](segment.md) — Segment, circle, and arc intersection helpers.
- [simplify](simplify.md) — Polygon simplification — removes duplicate vertices, merges
- [triangulate](triangulate.md) — Polygon triangulation via ear-clipping.
