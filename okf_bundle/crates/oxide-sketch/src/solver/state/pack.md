---
okf_version: "0.2"
type: Function
title: pack
description: Build a state vector from a sketch. Fixed-constrained Points are
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/pack
language: rust
---

# pack

Build a state vector from a sketch. Fixed-constrained Points are

## Signature

```rust
pub fn pack(sketch: &SketchData) -> PackedState
```

## Visibility

- `pub`

## Docstring

Build a state vector from a sketch. Fixed-constrained Points are
excluded from the state vector; their coordinates are read directly
from the [`Entity`] at residual time so the solver cannot move them.

## Source
Lines 28–66 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
| called_by | [output_for](/crates/oxide-app/src/app/runtime/footprint_summaries/output_for.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [coincident_residual_nonzero_when_apart](/crates/oxide-sketch/tests/constraints_basics/coincident_residual_nonzero_when_apart.md) |
| called_by | [coincident_residual_zero_when_coincident](/crates/oxide-sketch/tests/constraints_basics/coincident_residual_zero_when_coincident.md) |
| called_by | [distance_pt_pt_expr_full_arithmetic](/crates/oxide-sketch/tests/constraints_basics/distance_pt_pt_expr_full_arithmetic.md) |
| called_by | [distance_pt_pt_expr_param_arithmetic](/crates/oxide-sketch/tests/constraints_basics/distance_pt_pt_expr_param_arithmetic.md) |
| called_by | [distance_pt_pt_expr_resolves_via_params](/crates/oxide-sketch/tests/constraints_basics/distance_pt_pt_expr_resolves_via_params.md) |
| called_by | [distance_pt_pt_literal_nonzero_when_off](/crates/oxide-sketch/tests/constraints_basics/distance_pt_pt_literal_nonzero_when_off.md) |
| called_by | [distance_pt_pt_literal_zero_on_3_4_5_triangle](/crates/oxide-sketch/tests/constraints_basics/distance_pt_pt_literal_zero_on_3_4_5_triangle.md) |
| called_by | [fixed_residual_is_empty](/crates/oxide-sketch/tests/constraints_basics/fixed_residual_is_empty.md) |
| called_by | [horizontal_residual_nonzero_when_diagonal](/crates/oxide-sketch/tests/constraints_basics/horizontal_residual_nonzero_when_diagonal.md) |
| called_by | [horizontal_residual_zero_when_horizontal](/crates/oxide-sketch/tests/constraints_basics/horizontal_residual_zero_when_horizontal.md) |
| called_by | [residual_count_matches_returned_vector_length](/crates/oxide-sketch/tests/constraints_basics/residual_count_matches_returned_vector_length.md) |
| called_by | [total_residual_concatenates_per_constraint](/crates/oxide-sketch/tests/constraints_basics/total_residual_concatenates_per_constraint.md) |
| called_by | [total_residual_length_matches_constraint_kind_count_sum](/crates/oxide-sketch/tests/constraints_basics/total_residual_length_matches_constraint_kind_count_sum.md) |
| called_by | [vertical_residual_nonzero_when_diagonal](/crates/oxide-sketch/tests/constraints_basics/vertical_residual_nonzero_when_diagonal.md) |
| called_by | [vertical_residual_zero_when_vertical](/crates/oxide-sketch/tests/constraints_basics/vertical_residual_zero_when_vertical.md) |
| called_by | [equal_length_residual_nonzero_when_mismatched](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_length_residual_nonzero_when_mismatched.md) |
| called_by | [equal_length_residual_zero_on_equal_lines](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_length_residual_zero_on_equal_lines.md) |
| called_by | [equal_radius_nonzero_on_mismatched_radii](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_radius_nonzero_on_mismatched_radii.md) |
| called_by | [equal_radius_zero_on_circle_and_arc_combo](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_radius_zero_on_circle_and_arc_combo.md) |
| called_by | [equal_radius_zero_on_two_equal_arcs](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_radius_zero_on_two_equal_arcs.md) |
| called_by | [equal_radius_zero_on_two_equal_circles](/crates/oxide-sketch/tests/constraints_equal_tangent/equal_radius_zero_on_two_equal_circles.md) |
| called_by | [residual_count_is_one_for_all_task_2_6_kinds](/crates/oxide-sketch/tests/constraints_equal_tangent/residual_count_is_one_for_all_task_2_6_kinds.md) |
| called_by | [tangent_arc_arc_external_nonzero_on_mismatched_setup](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_arc_arc_external_nonzero_on_mismatched_setup.md) |
| called_by | [tangent_arc_arc_external_zero_when_distance_equals_sum](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_arc_arc_external_zero_when_distance_equals_sum.md) |
| called_by | [tangent_arc_arc_internal_nonzero_on_mismatched_setup](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_arc_arc_internal_nonzero_on_mismatched_setup.md) |
| called_by | [tangent_arc_arc_internal_zero_when_distance_equals_diff](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_arc_arc_internal_zero_when_distance_equals_diff.md) |
| called_by | [tangent_line_arc_nonzero_on_non_tangent_line](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_line_arc_nonzero_on_non_tangent_line.md) |
| called_by | [tangent_line_arc_zero_on_horizontal_line_above_centre](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_line_arc_zero_on_horizontal_line_above_centre.md) |
| called_by | [tangent_line_arc_zero_on_horizontal_line_below_centre](/crates/oxide-sketch/tests/constraints_equal_tangent/tangent_line_arc_zero_on_horizontal_line_below_centre.md) |
| called_by | [angle_residual_handles_target_pi_over_2](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/angle_residual_handles_target_pi_over_2.md) |
| called_by | [angle_residual_signed_negative_for_clockwise_rotation](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/angle_residual_signed_negative_for_clockwise_rotation.md) |
| called_by | [angle_residual_wraps_across_pi_branch_cut](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/angle_residual_wraps_across_pi_branch_cut.md) |
| called_by | [angle_residual_zero_when_target_matches_geometry](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/angle_residual_zero_when_target_matches_geometry.md) |
| called_by | [angle_residual_zero_with_target_zero_matches_parallel_case](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/angle_residual_zero_with_target_zero_matches_parallel_case.md) |
| called_by | [parallel_residual_nonzero_on_perpendicular_lines](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/parallel_residual_nonzero_on_perpendicular_lines.md) |
| called_by | [parallel_residual_zero_on_antiparallel_lines](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/parallel_residual_zero_on_antiparallel_lines.md) |
| called_by | [parallel_residual_zero_on_two_horizontal_lines](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/parallel_residual_zero_on_two_horizontal_lines.md) |
| called_by | [parallel_residual_zero_on_two_lines_at_30_degrees](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/parallel_residual_zero_on_two_lines_at_30_degrees.md) |
| called_by | [perpendicular_residual_nonzero_on_parallel_lines](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/perpendicular_residual_nonzero_on_parallel_lines.md) |
| called_by | [perpendicular_residual_zero_on_45_and_135_degree_lines](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/perpendicular_residual_zero_on_45_and_135_degree_lines.md) |
| called_by | [perpendicular_residual_zero_on_horizontal_and_vertical](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/perpendicular_residual_zero_on_horizontal_and_vertical.md) |
| called_by | [residual_count_matches_returned_vector_length](/crates/oxide-sketch/tests/constraints_parallel_perp_angle/residual_count_matches_returned_vector_length.md) |
| called_by | [distance_pt_circle_residual_negative_inside](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_circle_residual_negative_inside.md) |
| called_by | [distance_pt_circle_residual_positive_outside](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_circle_residual_positive_outside.md) |
| called_by | [distance_pt_circle_residual_zero_when_on_circle](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_circle_residual_zero_when_on_circle.md) |
| called_by | [distance_pt_circle_works_on_arc](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_circle_works_on_arc.md) |
| called_by | [distance_pt_line_left_side_positive_target_disagrees_in_sign](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_line_left_side_positive_target_disagrees_in_sign.md) |
| called_by | [distance_pt_line_negative_target_satisfies_left_side](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_line_negative_target_satisfies_left_side.md) |
| called_by | [distance_pt_line_target_zero_reduces_to_point_on_line](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_line_target_zero_reduces_to_point_on_line.md) |
| called_by | [distance_pt_line_zero_when_target_matches](/crates/oxide-sketch/tests/constraints_point_on/distance_pt_line_zero_when_target_matches.md) |
| called_by | [point_on_arc_residual_negative_when_inside_circle](/crates/oxide-sketch/tests/constraints_point_on/point_on_arc_residual_negative_when_inside_circle.md) |
| called_by | [point_on_arc_residual_positive_when_outside_circle](/crates/oxide-sketch/tests/constraints_point_on/point_on_arc_residual_positive_when_outside_circle.md) |
| called_by | [point_on_arc_residual_uses_start_for_radius_not_end](/crates/oxide-sketch/tests/constraints_point_on/point_on_arc_residual_uses_start_for_radius_not_end.md) |
| called_by | [point_on_arc_residual_zero_at_arc_start](/crates/oxide-sketch/tests/constraints_point_on/point_on_arc_residual_zero_at_arc_start.md) |
| called_by | [point_on_arc_residual_zero_on_3_4_5_triangle](/crates/oxide-sketch/tests/constraints_point_on/point_on_arc_residual_zero_on_3_4_5_triangle.md) |
| called_by | [point_on_line_degenerate_zero_length_line_errors](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_degenerate_zero_length_line_errors.md) |
| called_by | [point_on_line_diagonal_normalises_by_length](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_diagonal_normalises_by_length.md) |
| called_by | [point_on_line_residual_signed_above_line](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_residual_signed_above_line.md) |
| called_by | [point_on_line_residual_signed_below_line](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_residual_signed_below_line.md) |
| called_by | [point_on_line_residual_zero_at_endpoint](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_residual_zero_at_endpoint.md) |
| called_by | [point_on_line_residual_zero_when_on_line](/crates/oxide-sketch/tests/constraints_point_on/point_on_line_residual_zero_when_on_line.md) |
| called_by | [point_on_residual_count_is_one_per_kind](/crates/oxide-sketch/tests/constraints_point_on/point_on_residual_count_is_one_per_kind.md) |
| called_by | [midpoint_nonzero_when_point_at_endpoint](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/midpoint_nonzero_when_point_at_endpoint.md) |
| called_by | [midpoint_zero_when_point_at_line_midpoint](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/midpoint_zero_when_point_at_line_midpoint.md) |
| called_by | [residual_count_matches_returned_vector_length](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/residual_count_matches_returned_vector_length.md) |
| called_by | [symmetric_about_line_nonzero_when_midpoint_off_line](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_line_nonzero_when_midpoint_off_line.md) |
| called_by | [symmetric_about_line_nonzero_when_segment_not_perpendicular](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_line_nonzero_when_segment_not_perpendicular.md) |
| called_by | [symmetric_about_line_zero_for_45_degree_line](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_line_zero_for_45_degree_line.md) |
| called_by | [symmetric_about_line_zero_mirrored_across_x_axis](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_line_zero_mirrored_across_x_axis.md) |
| called_by | [symmetric_about_point_nonzero_when_centre_off](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_point_nonzero_when_centre_off.md) |
| called_by | [symmetric_about_point_zero_when_centre_is_midpoint](/crates/oxide-sketch/tests/constraints_symmetric_midpoint/symmetric_about_point_zero_when_centre_is_midpoint.md) |
| called_by | [dof_fully_constrained_marks_black](/crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
| called_by | [dof_under_constrained_marks_blue](/crates/oxide-sketch/tests/dof/dof_under_constrained_marks_blue.md) |
| called_by | [jacobian_coincident_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_coincident_matches_analytical.md) |
| called_by | [jacobian_distance_pt_pt_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_distance_pt_pt_matches_analytical.md) |
| called_by | [jacobian_empty_sketch_is_zero_rows](/crates/oxide-sketch/tests/solver_basics/jacobian_empty_sketch_is_zero_rows.md) |
| called_by | [jacobian_horizontal_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_horizontal_matches_analytical.md) |
| called_by | [pack_circle_radius_is_a_free_var](/crates/oxide-sketch/tests/solver_basics/pack_circle_radius_is_a_free_var.md) |
| called_by | [pack_excludes_fixed_points](/crates/oxide-sketch/tests/solver_basics/pack_excludes_fixed_points.md) |
| called_by | [pack_two_points_lays_out_xy_in_order](/crates/oxide-sketch/tests/solver_basics/pack_two_points_lays_out_xy_in_order.md) |
| called_by | [build_ico](/tools/build_icons/build_ico.md) |
