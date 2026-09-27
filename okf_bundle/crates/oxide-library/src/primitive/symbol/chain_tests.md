---
okf_version: "0.2"
type: Module
title: chain_tests
description: "Tests for `chain::chain_into_closed_contour` and friends."
resource: crates/oxide-library/src/primitive/symbol/chain_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain_tests
language: rust
---

# chain_tests

Tests for `chain::chain_into_closed_contour` and friends.

## Docstring

Tests for `chain::chain_into_closed_contour` and friends.

Split out of `chain.rs` into this sibling file (per the
`crates/oxide-app/src/keymap/editor_tests.rs` pattern:
`#[cfg(test)] mod chain_tests;` declared alongside `mod chain;` in
the parent `primitive/symbol/mod.rs`) to keep `chain.rs` under the
house 800-line file cap.

Because this file is a *sibling* of `chain`, not a child module of
it, it only has access to `chain`'s public surface — exactly the
same constraint a real caller has. The `dist_sq`/`signed_area_x2`
assertion helpers below are independent re-implementations of the
tiny math `chain` also happens to use internally, not a reach into
its private internals: a bug in `chain`'s own copy can't quietly
cancel out against the same bug here.

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-library/src/primitive/symbol/chain_tests/line.md) |
| related | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain_tests/dist_sq.md) |
| related | [signed_area_x2](/crates/oxide-library/src/primitive/symbol/chain_tests/signed_area_x2.md) |
| related | [assert_approx_eq](/crates/oxide-library/src/primitive/symbol/chain_tests/assert_approx_eq.md) |
| related | [assert_point_approx_eq](/crates/oxide-library/src/primitive/symbol/chain_tests/assert_point_approx_eq.md) |
| related | [square_from_four_lines_shuffled_and_reversed_closes_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/square_from_four_lines_shuffled_and_reversed_closes_ccw.md) |
| related | [cw_input_square_is_renormalised_to_ccw](/crates/oxide-library/src/primitive/symbol/chain_tests/cw_input_square_is_renormalised_to_ccw.md) |
| related | [triangle_with_one_arc_side_closes_with_tessellated_arc_points](/crates/oxide-library/src/primitive/symbol/chain_tests/triangle_with_one_arc_side_closes_with_tessellated_arc_points.md) |
| related | [wraparound_arc_through_zero_degrees_samples_lie_on_circle](/crates/oxide-library/src/primitive/symbol/chain_tests/wraparound_arc_through_zero_degrees_samples_lie_on_circle.md) |
| related | [near_full_sweep_arc_closes_via_its_own_tiny_chord_gap](/crates/oxide-library/src/primitive/symbol/chain_tests/near_full_sweep_arc_closes_via_its_own_tiny_chord_gap.md) |
| related | [open_chain_three_sides_of_square_reports_gap](/crates/oxide-library/src/primitive/symbol/chain_tests/open_chain_three_sides_of_square_reports_gap.md) |
| related | [t_junction_of_three_lines_reports_branching](/crates/oxide-library/src/primitive/symbol/chain_tests/t_junction_of_three_lines_reports_branching.md) |
| related | [two_separate_triangles_report_disjoint](/crates/oxide-library/src/primitive/symbol/chain_tests/two_separate_triangles_report_disjoint.md) |
| related | [line_plus_its_own_reverse_is_degenerate](/crates/oxide-library/src/primitive/symbol/chain_tests/line_plus_its_own_reverse_is_degenerate.md) |
| related | [sub_epsilon_stub_segment_is_rejected_with_its_index](/crates/oxide-library/src/primitive/symbol/chain_tests/sub_epsilon_stub_segment_is_rejected_with_its_index.md) |
| related | [endpoints_within_epsilon_still_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_within_epsilon_still_chain.md) |
| related | [endpoints_beyond_epsilon_report_open_chain](/crates/oxide-library/src/primitive/symbol/chain_tests/endpoints_beyond_epsilon_report_open_chain.md) |
| related | [zero_sweep_arc_alone_is_rejected_as_degenerate_segment](/crates/oxide-library/src/primitive/symbol/chain_tests/zero_sweep_arc_alone_is_rejected_as_degenerate_segment.md) |
| related | [non_finite_line_coordinate_reports_invalid_input](/crates/oxide-library/src/primitive/symbol/chain_tests/non_finite_line_coordinate_reports_invalid_input.md) |
| related | [non_finite_arc_radius_reports_invalid_input](/crates/oxide-library/src/primitive/symbol/chain_tests/non_finite_arc_radius_reports_invalid_input.md) |
| related | [empty_input_reports_empty](/crates/oxide-library/src/primitive/symbol/chain_tests/empty_input_reports_empty.md) |
| related | [self_intersecting_bowtie_with_net_zero_area_commits](/crates/oxide-library/src/primitive/symbol/chain_tests/self_intersecting_bowtie_with_net_zero_area_commits.md) |
| related | [three_collinear_points_still_report_degenerate](/crates/oxide-library/src/primitive/symbol/chain_tests/three_collinear_points_still_report_degenerate.md) |
