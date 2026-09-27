---
okf_version: "0.2"
type: Function
title: chain_into_closed_contour
description: "Chain `segments` end-to-end via shared endpoints (within"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour
language: rust
---

# chain_into_closed_contour

Chain `segments` end-to-end via shared endpoints (within

## Signature

```rust
pub fn chain_into_closed_contour(segments: &[ChainSegment]) -> Result<Vec<[f64; 2]>, ChainError>
```

## Visibility

- `pub`

## Docstring

Chain `segments` end-to-end via shared endpoints (within
[`CHAIN_ENDPOINT_EPSILON_MM`]) into one closed contour.

Segments may be given in any order and any direction — a segment is
reversed as needed while walking the chain. [`ChainSegment::Arc`]
legs are tessellated into [`CHAIN_ARC_SAMPLES`] straight segments
(endpoint-exact: the first/last tessellated points are the arc's true
endpoints, so chaining tolerance behaves identically to a `Line`).

Returns the closed ring as an ordered vertex list **without**
repeating the first vertex at the end (implicit close); consecutive
duplicate points (within epsilon, including the wrap-around
last-to-first pair) are collapsed. The winding is normalised to
counter-clockwise (positive shoelace area).

Adjacency is resolved by direct pairwise (O(n²)) epsilon comparison
rather than hashing rounded coordinates or bucketing into a spatial
grid: symbol "join into polygon" selections are a handful of
segments, so the quadratic cost is irrelevant, and avoiding both
float-hashing and bucket-boundary edge cases keeps the endpoint match
exact and easy to audit. Clustering is **transitive**: if endpoint
`A` is within epsilon of `B`, and `B` is within epsilon of `C`, then
`A` and `C` join the same node even when `A` and `C` alone are
farther apart than epsilon — a chain of several sub-epsilon joints
can span a cumulative distance larger than one epsilon.

## Source
Lines 166–179 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [validate_finite](/crates/oxide-library/src/primitive/symbol/chain/validate_finite.md) |
| calls | [reject_sub_epsilon_segments](/crates/oxide-library/src/primitive/symbol/chain/reject_sub_epsilon_segments.md) |
| calls | [build_endpoint_clusters](/crates/oxide-library/src/primitive/symbol/chain/build_endpoint_clusters.md) |
| calls | [validate_topology](/crates/oxide-library/src/primitive/symbol/chain/validate_topology.md) |
| calls | [finalize_ring](/crates/oxide-library/src/primitive/symbol/chain/finalize_ring.md) |
| calls | [walk_cycle](/crates/oxide-library/src/primitive/symbol/chain/walk_cycle.md) |
| called_by | [resolve_ring_with_auto_close](/crates/oxide-app/src/library/editor/symbol/updates/join/resolve_ring_with_auto_close.md) |
| called_by | [cw_input_square_is_renormalised_to_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/cw_input_square_is_renormalised_to_ccw.md) |
| called_by | [endpoints_beyond_epsilon_report_open_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_beyond_epsilon_report_open_chain.md) |
| called_by | [endpoints_within_epsilon_still_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_within_epsilon_still_chain.md) |
| called_by | [line_plus_its_own_reverse_is_degenerate](/crates/oxide-library/src/primitive/symbol/chain_tests/line_plus_its_own_reverse_is_degenerate.md) |
| called_by | [near_full_sweep_arc_closes_via_its_own_tiny_chord_gap](/crates/oxide-library/src/primitive/symbol/chain_tests/near_full_sweep_arc_closes_via_its_own_tiny_chord_gap.md) |
| called_by | [non_finite_arc_radius_reports_invalid_input](/crates/oxide-library/src/primitive/symbol/chain_tests/non_finite_arc_radius_reports_invalid_input.md) |
| called_by | [non_finite_line_coordinate_reports_invalid_input](/crates/oxide-library/src/primitive/symbol/chain_tests/non_finite_line_coordinate_reports_invalid_input.md) |
| called_by | [open_chain_three_sides_of_square_reports_gap](/crates/oxide-library/src/primitive/symbol/chain_tests/open_chain_three_sides_of_square_reports_gap.md) |
| called_by | [self_intersecting_bowtie_with_net_zero_area_commits](/crates/oxide-library/src/primitive/symbol/chain_tests/self_intersecting_bowtie_with_net_zero_area_commits.md) |
| called_by | [square_from_four_lines_shuffled_and_reversed_closes_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/square_from_four_lines_shuffled_and_reversed_closes_ccw.md) |
| called_by | [sub_epsilon_stub_segment_is_rejected_with_its_index](/crates/oxide-library/src/primitive/symbol/chain_tests/sub_epsilon_stub_segment_is_rejected_with_its_index.md) |
| called_by | [t_junction_of_three_lines_reports_branching](/crates/oxide-library/src/primitive/symbol/chain_tests/t_junction_of_three_lines_reports_branching.md) |
| called_by | [three_collinear_points_still_report_degenerate](/crates/oxide-library/src/primitive/symbol/chain_tests/three_collinear_points_still_report_degenerate.md) |
| called_by | [triangle_with_one_arc_side_closes_with_tessellated_arc_points](/crates/oxide-library/src/primitive/symbol/chain_tests/triangle_with_one_arc_side_closes_with_tessellated_arc_points.md) |
| called_by | [two_separate_triangles_report_disjoint](/crates/oxide-library/src/primitive/symbol/chain_tests/two_separate_triangles_report_disjoint.md) |
| called_by | [wraparound_arc_through_zero_degrees_samples_lie_on_circle](/crates/oxide-library/src/primitive/symbol/chain_tests/wraparound_arc_through_zero_degrees_samples_lie_on_circle.md) |
| called_by | [zero_sweep_arc_alone_is_rejected_as_degenerate_segment](/crates/oxide-library/src/primitive/symbol/chain_tests/zero_sweep_arc_alone_is_rejected_as_degenerate_segment.md) |
