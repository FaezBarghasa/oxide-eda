# profile

## Classs

- [ProfileEdge](ProfileEdge.md) — One edge of a traced profile, oriented the way the walk crossed it.
- [ProfileEntities](ProfileEntities.md) — Entity-level result of a profile trace — the loop's *topology*.
- [TraceError](TraceError.md) — Trace failure modes — the bake site decides whether to warn or

## Functions

- [build_adjacency](build_adjacency.md)
- [collect_edges](collect_edges.md) — Collect non-construction edge entities (Lines + Arcs). Circles are
- [edge_endpoints](edge_endpoints.md) — Endpoint Points of an edge — Line `(start, end)` or Arc
- [profile_points](profile_points.md) — Deduplicated Points a traced profile owns, in first-seen order.
- [push_arc_interior_if_arc](push_arc_interior_if_arc.md) — If `entity` is an [`EntityKind::Arc`], append [`ARC_SAMPLES`]
- [rectangle_sketch](rectangle_sketch.md) — Build a sketch with one rectangle (4 Points + 4 Lines), solve,
- [solve](solve.md)
- [trace_arc_seed_walks_back_through_line](trace_arc_seed_walks_back_through_line.md) — [test]
- [trace_branching_topology_errors](trace_branching_topology_errors.md) — [test]
- [trace_closed_profile](trace_closed_profile.md) — Trace a closed boundary starting at `start` and emit its boundary
- [trace_closed_profile_entities](trace_closed_profile_entities.md) — Trace a closed boundary starting at `start`, returning its
- [trace_construction_lines_skipped](trace_construction_lines_skipped.md) — [test]
- [trace_d_shape_cw_arc_closes_lower_half](trace_d_shape_cw_arc_closes_lower_half.md) — [test]
- [trace_d_shape_line_plus_arc_closes](trace_d_shape_line_plus_arc_closes.md) — [test]
- [trace_open_chain_returns_open_error](trace_open_chain_returns_open_error.md) — [test]
- [trace_rectangle_closes](trace_rectangle_closes.md) — [test]
