---
okf_version: "0.2"
type: Function
title: line
resource: crates/oxide-library/src/primitive/symbol/chain_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain_tests/line
language: rust
---

# line

## Signature

```rust
fn line(from: [f64; 2], to: [f64; 2]) -> ChainSegment
```

## Source
Lines 19–21 in `crates/oxide-library/src/primitive/symbol/chain_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain_tests](/crates/oxide-library/src/primitive/symbol/chain_tests.md) |
| called_by | [cw_input_square_is_renormalised_to_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/cw_input_square_is_renormalised_to_ccw.md) |
| called_by | [endpoints_beyond_epsilon_report_open_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_beyond_epsilon_report_open_chain.md) |
| called_by | [endpoints_within_epsilon_still_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_within_epsilon_still_chain.md) |
| called_by | [line_plus_its_own_reverse_is_degenerate](/crates/oxide-library/src/primitive/symbol/chain_tests/line_plus_its_own_reverse_is_degenerate.md) |
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
