---
okf_version: "0.2"
type: Module
title: profile
description: "Closed-profile walker — given a starting Line entity, trace a"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile
language: rust
---

# profile

Closed-profile walker — given a starting Line entity, trace a

## Docstring

Closed-profile walker — given a starting Line entity, trace a
connected loop of edge entities through shared endpoint Points and
emit the boundary as a Polygon (Vec of `[x_mm, y_mm]` vertices).

Used by the v0.14 silk / courtyard / mask / pour / keepout /
cutout / 3D-extrude bake modules to convert sketch profiles into
baked library polygons.

v0.14.1 scope:
- Lines and Arcs both participate. Arc segments are tessellated
into [`ARC_SAMPLES`] interior vertices using
`(center, start, end, sweep_ccw)` from the solved state.
- Circles are still rejected up front; the bake module is
expected to handle a Circle entity directly (a Circle is an
already-closed primitive without start / end endpoints, so the
walker has nothing to walk).
- Construction entities are skipped silently — they're solver
scaffolding and never participate in the baked geometry.
- Branching topology (a vertex with 3+ incident edges) returns
[`TraceError::Branching`]; the bake skips with a warning.

Cleanroom: traversal is a textbook depth-first walk over the
endpoint-incidence graph; arc tessellation is a textbook polar
sample (Hearn & Baker §3.13 "Drawing Circular Arcs"). No third-
party CAD-tooling source consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [TraceError](/crates/oxide-bake/src/profile/TraceError.md) |
| related | [ProfileEdge](/crates/oxide-bake/src/profile/ProfileEdge.md) |
| related | [ProfileEntities](/crates/oxide-bake/src/profile/ProfileEntities.md) |
| related | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
| related | [profile_points](/crates/oxide-bake/src/profile/profile_points.md) |
| related | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| related | [push_arc_interior_if_arc](/crates/oxide-bake/src/profile/push_arc_interior_if_arc.md) |
| related | [collect_edges](/crates/oxide-bake/src/profile/collect_edges.md) |
| related | [build_adjacency](/crates/oxide-bake/src/profile/build_adjacency.md) |
| related | [edge_endpoints](/crates/oxide-bake/src/profile/edge_endpoints.md) |
| related | [rectangle_sketch](/crates/oxide-bake/src/profile/rectangle_sketch.md) |
| related | [solve](/crates/oxide-bake/src/profile/solve.md) |
| related | [trace_rectangle_closes](/crates/oxide-bake/src/profile/trace_rectangle_closes.md) |
| related | [trace_open_chain_returns_open_error](/crates/oxide-bake/src/profile/trace_open_chain_returns_open_error.md) |
| related | [trace_d_shape_line_plus_arc_closes](/crates/oxide-bake/src/profile/trace_d_shape_line_plus_arc_closes.md) |
| related | [trace_d_shape_cw_arc_closes_lower_half](/crates/oxide-bake/src/profile/trace_d_shape_cw_arc_closes_lower_half.md) |
| related | [trace_arc_seed_walks_back_through_line](/crates/oxide-bake/src/profile/trace_arc_seed_walks_back_through_line.md) |
| related | [trace_branching_topology_errors](/crates/oxide-bake/src/profile/trace_branching_topology_errors.md) |
| related | [trace_construction_lines_skipped](/crates/oxide-bake/src/profile/trace_construction_lines_skipped.md) |
