# halfedge

## Classs

- [Face](Face.md) — [derive(Debug, Clone)]
- [FaceId](FaceId.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
- [HalfEdge](HalfEdge.md) — [derive(Debug, Clone)]
- [HalfEdgeId](HalfEdgeId.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
- [Mesh](Mesh.md) — [derive(Debug, Clone, Default)]
- [Vertex](Vertex.md) — [derive(Debug, Clone)]
- [VertexId](VertexId.md) — Index types so signatures read clearly. `usize` underneath but

## Functions

- [cw_input_normalises_to_ccw_inner](cw_input_normalises_to_ccw_inner.md) — [test]
- [degenerate_zero_area_returns_none](degenerate_zero_area_returns_none.md) — [test]
- [empty_polygon_returns_none](empty_polygon_returns_none.md) — [test]
- [face](face.md)
- [face](face_1.md)
- [face_halfedge_ring_is_closed](face_halfedge_ring_is_closed.md) — [test]
- [face_halfedges](face_halfedges.md) — Iterate the half-edges along a face's boundary.
- [face_halfedges](face_halfedges_1.md) — Iterate the half-edges along a face's boundary.
- [face_polygon](face_polygon.md) — Convenience: positions of the boundary vertices of `f` in
- [face_polygon](face_polygon_1.md) — Convenience: positions of the boundary vertices of `f` in
- [face_polygon_round_trip](face_polygon_round_trip.md) — [test]
- [face_vertices](face_vertices.md) — Vertices around a face in boundary order.
- [face_vertices](face_vertices_1.md) — Vertices around a face in boundary order.
- [from_polygon](from_polygon.md) — Build a mesh from a single closed polygon. The input must
- [from_polygon](from_polygon_1.md) — Build a mesh from a single closed polygon. The input must
- [half_edge](half_edge.md)
- [half_edge](half_edge_1.md)
- [new](new.md)
- [new](new_1.md)
- [p](p.md)
- [unit_square_has_two_faces_eight_halfedges](unit_square_has_two_faces_eight_halfedges.md) — [test]
- [vertex](vertex.md)
- [vertex](vertex_1.md)
- [vertex_outgoing](vertex_outgoing.md) — Half-edges originating at a vertex. Walks via `twin.next`
- [vertex_outgoing](vertex_outgoing_1.md) — Half-edges originating at a vertex. Walks via `twin.next`
- [vertex_outgoing_yields_inner_and_outer](vertex_outgoing_yields_inner_and_outer.md) — [test]
