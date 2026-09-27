# chain

## Classs

- [ChainError](ChainError.md) — Why [`chain_into_closed_contour`] couldn't produce a single closed
- [ChainSegment](ChainSegment.md) — One input stroke to be chained. Mirrors the geometry-bearing fields
- [EndpointClusters](EndpointClusters.md) — Endpoint-adjacency result of clustering every segment's start/end

## Functions

- [average_point](average_point.md)
- [build_endpoint_clusters](build_endpoint_clusters.md) — Cluster the `2n` segment endpoints (start + end of every segment)
- [chain_into_closed_contour](chain_into_closed_contour.md) — Chain `segments` end-to-end via shared endpoints (within
- [dist_sq](dist_sq.md)
- [finalize_ring](finalize_ring.md) — Collapse consecutive (and wrap-around) duplicate points, reject
- [is_collinear](is_collinear.md) — `true` when every vertex in `ring` lies within `eps` mm of the
- [point_at_deg](point_at_deg.md) — Point at angle `deg` (degrees) on the circle — matches
- [polyline_length](polyline_length.md) — Total length of a tessellated polyline — the sum of consecutive
- [ref_index](ref_index.md) — Index into the flat `refs` array (and the `parent` union-find) for a
- [reject_sub_epsilon_segments](reject_sub_epsilon_segments.md) — Reject any segment whose total tessellated *length* is shorter than
- [signed_area_x2](signed_area_x2.md) — Twice the signed polygon area (shoelace, standard math orientation —
- [tessellate_segment](tessellate_segment.md) — Expand one input segment into its ordered point list, endpoints
- [uf_find](uf_find.md)
- [uf_union](uf_union.md)
- [validate_finite](validate_finite.md) — Reject any segment carrying a `NaN`/`inf` coordinate or radius up
- [validate_topology](validate_topology.md) — Confirm `clusters` describes exactly one simple cycle spanning all
- [walk_cycle](walk_cycle.md) — Walk the single simple cycle [`validate_topology`] already confirmed,
