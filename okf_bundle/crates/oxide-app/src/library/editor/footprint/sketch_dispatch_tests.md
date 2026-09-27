---
okf_version: "0.2"
type: Module
title: sketch_dispatch_tests
description: Phase 5.4 + 7.3 dispatcher tests.
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests
language: rust
---

# sketch_dispatch_tests

Phase 5.4 + 7.3 dispatcher tests.

## Docstring

Phase 5.4 + 7.3 dispatcher tests.

These run as inline tests under `#[cfg(test)]` so they exercise
the dispatcher without spinning up the iced runtime.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_footprint](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/empty_footprint.md) |
| related | [point_with_pad](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_with_pad.md) |
| related | [add_entity_triggers_solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_entity_triggers_solve_and_bake.md) |
| related | [add_constraint_solves_geometry](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_constraint_solves_geometry.md) |
| related | [a_failed_solve_clears_the_previous_solves_colours](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/a_failed_solve_clears_the_previous_solves_colours.md) |
| related | [set_mode_initialises_sketch_field_and_preserves_literal_pads](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_mode_initialises_sketch_field_and_preserves_literal_pads.md) |
| related | [line_tool_two_clicks_creates_line_with_snap](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/line_tool_two_clicks_creates_line_with_snap.md) |
| related | [warning_wrapper_captures_parse_error_into_solve_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/warning_wrapper_captures_parse_error_into_solve_warnings.md) |
| related | [set_role_pad_on_point_attaches_pad_attr_and_bakes](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_on_point_attaches_pad_attr_and_bakes.md) |
| related | [set_role_pad_on_line_is_silent_noop](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_on_line_is_silent_noop.md) |
| related | [set_role_silk_top_attaches_silk_attr_with_top_layer](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_silk_top_attaches_silk_attr_with_top_layer.md) |
| related | [set_role_unassigned_clears_every_attr](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_unassigned_clears_every_attr.md) |
| related | [set_role_replaces_existing_attr_atomically](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_replaces_existing_attr_atomically.md) |
| related | [set_role_courtyard_attaches_courtyard_attr](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_courtyard_attaches_courtyard_attr.md) |
| related | [set_role_pad_increments_designator_across_entities](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_increments_designator_across_entities.md) |
| related | [solver_runs_on_every_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_runs_on_every_edit.md) |
| related | [solver_errors_surface_in_solve_warnings_not_silently_swallowed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_errors_surface_in_solve_warnings_not_silently_swallowed.md) |
| related | [break_track_editor](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_editor.md) |
| related | [sketch_lines](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/sketch_lines.md) |
| related | [point_xy](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_xy.md) |
| related | [break_track_split_at_mid_span_replaces_line_with_two_halves](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_split_at_mid_span_replaces_line_with_two_halves.md) |
| related | [break_track_reselects_line_a_not_line_b](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_reselects_line_a_not_line_b.md) |
| related | [break_track_miss_warns_and_leaves_line_intact](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_miss_warns_and_leaves_line_intact.md) |
| related | [break_track_click_near_endpoint_warns_no_split](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_click_near_endpoint_warns_no_split.md) |
| related | [edge_arc_editor](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_editor.md) |
| related | [sketch_arcs](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/sketch_arcs.md) |
| related | [click](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/click.md) |
| related | [edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks.md) |
| related | [edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on.md) |
| related | [edge_arc_rejects_a_collinear_third_pick_and_stays_armed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_rejects_a_collinear_third_pick_and_stays_armed.md) |
