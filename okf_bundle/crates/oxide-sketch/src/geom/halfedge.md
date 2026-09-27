---
okf_version: "0.2"
type: Module
title: halfedge
description: Half-edge mesh data structure for 2D planar polygons.
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge
language: rust
---

# halfedge

Half-edge mesh data structure for 2D planar polygons.

## Docstring

Half-edge mesh data structure for 2D planar polygons.

Each undirected polygon edge becomes two oriented half-edges
(twin pair). Each half-edge belongs to exactly one face and
points to its successor around that face's boundary. Vertices
cache one outgoing half-edge so vertex-incidence walks don't
need a search.

For a single closed polygon with N vertices the mesh has
`2N` half-edges and `2` faces — the inside and the unbounded
outer face. More complex inputs (multiple connected rings,
holes, planar subdivisions) extend the same structure with
additional faces.

Use cases for this crate:
- Closed-loop walker for `oxide-bake::profile` (uniform face
traversal regardless of the ad-hoc adjacency map).
- Polygon boolean cleanup (Phase 2 follow-up).
- Multi-region pad-stack composition.

## Relationships

| Type | Target |
|------|--------|
| related | [VertexId](/crates/oxide-sketch/src/geom/halfedge/VertexId.md) |
| related | [HalfEdgeId](/crates/oxide-sketch/src/geom/halfedge/HalfEdgeId.md) |
| related | [FaceId](/crates/oxide-sketch/src/geom/halfedge/FaceId.md) |
| related | [Vertex](/crates/oxide-sketch/src/geom/halfedge/Vertex.md) |
| related | [HalfEdge](/crates/oxide-sketch/src/geom/halfedge/HalfEdge.md) |
| related | [Face](/crates/oxide-sketch/src/geom/halfedge/Face.md) |
| related | [Mesh](/crates/oxide-sketch/src/geom/halfedge/Mesh.md) |
| related | [new](/crates/oxide-sketch/src/geom/halfedge/new.md) |
| related | [from_polygon](/crates/oxide-sketch/src/geom/halfedge/from_polygon.md) |
| related | [vertex](/crates/oxide-sketch/src/geom/halfedge/vertex.md) |
| related | [half_edge](/crates/oxide-sketch/src/geom/halfedge/half_edge.md) |
| related | [face](/crates/oxide-sketch/src/geom/halfedge/face.md) |
| related | [face_halfedges](/crates/oxide-sketch/src/geom/halfedge/face_halfedges.md) |
| related | [face_vertices](/crates/oxide-sketch/src/geom/halfedge/face_vertices.md) |
| related | [vertex_outgoing](/crates/oxide-sketch/src/geom/halfedge/vertex_outgoing.md) |
| related | [face_polygon](/crates/oxide-sketch/src/geom/halfedge/face_polygon.md) |
| related | [new](/crates/oxide-sketch/src/geom/halfedge/new.md) |
| related | [from_polygon](/crates/oxide-sketch/src/geom/halfedge/from_polygon.md) |
| related | [vertex](/crates/oxide-sketch/src/geom/halfedge/vertex.md) |
| related | [half_edge](/crates/oxide-sketch/src/geom/halfedge/half_edge.md) |
| related | [face](/crates/oxide-sketch/src/geom/halfedge/face.md) |
| related | [face_halfedges](/crates/oxide-sketch/src/geom/halfedge/face_halfedges.md) |
| related | [face_vertices](/crates/oxide-sketch/src/geom/halfedge/face_vertices.md) |
| related | [vertex_outgoing](/crates/oxide-sketch/src/geom/halfedge/vertex_outgoing.md) |
| related | [face_polygon](/crates/oxide-sketch/src/geom/halfedge/face_polygon.md) |
| related | [p](/crates/oxide-sketch/src/geom/halfedge/p.md) |
| related | [empty_polygon_returns_none](/crates/oxide-sketch/src/geom/halfedge/empty_polygon_returns_none.md) |
| related | [degenerate_zero_area_returns_none](/crates/oxide-sketch/src/geom/halfedge/degenerate_zero_area_returns_none.md) |
| related | [unit_square_has_two_faces_eight_halfedges](/crates/oxide-sketch/src/geom/halfedge/unit_square_has_two_faces_eight_halfedges.md) |
| related | [face_polygon_round_trip](/crates/oxide-sketch/src/geom/halfedge/face_polygon_round_trip.md) |
| related | [cw_input_normalises_to_ccw_inner](/crates/oxide-sketch/src/geom/halfedge/cw_input_normalises_to_ccw_inner.md) |
| related | [face_halfedge_ring_is_closed](/crates/oxide-sketch/src/geom/halfedge/face_halfedge_ring_is_closed.md) |
| related | [vertex_outgoing_yields_inner_and_outer](/crates/oxide-sketch/src/geom/halfedge/vertex_outgoing_yields_inner_and_outer.md) |
