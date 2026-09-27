---
okf_version: "0.2"
type: Module
title: library_cross_track
description: "Phase-5 tests that span two tracks (undo + placement + geometry) at once — the Phase-5 counterparts of the Phase-3 `library_pad_geometry` tests."
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track
language: rust
---

# library_cross_track

Phase-5 tests that span two tracks (undo + placement + geometry) at once — the Phase-5 counterparts of the Phase-3 `library_pad_geometry` tests.

## Docstring

Phase-5 tests that span two tracks (undo + placement + geometry) at once — the Phase-5 counterparts of the Phase-3 `library_pad_geometry` tests.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| related | [EditorStateProj](/crates/oxide-app/tests/regression/library_cross_track/EditorStateProj.md) |
| related | [editor_state_proj](/crates/oxide-app/tests/regression/library_cross_track/editor_state_proj.md) |
| related | [set_pad_defaults](/crates/oxide-app/tests/regression/library_cross_track/set_pad_defaults.md) |
| related | [place_round_rect_then_undo_restores_pre_place_state](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_undo_restores_pre_place_state.md) |
| related | [ctrl_z_during_tangent_arc_undoes_last_segment](/crates/oxide-app/tests/regression/library_cross_track/ctrl_z_during_tangent_arc_undoes_last_segment.md) |
| related | [placement_input_does_not_corrupt_history_on_undo](/crates/oxide-app/tests/regression/library_cross_track/placement_input_does_not_corrupt_history_on_undo.md) |
| related | [place_round_rect_then_select_arc_unlink_then_undo_restores_link](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_select_arc_unlink_then_undo_restores_link.md) |
| related | [editing_corner_r_via_properties_updates_all_4_arcs](/crates/oxide-app/tests/regression/library_cross_track/editing_corner_r_via_properties_updates_all_4_arcs.md) |
| related | [unlink_one_corner_only_that_arc_reads_per_corner_param](/crates/oxide-app/tests/regression/library_cross_track/unlink_one_corner_only_that_arc_reads_per_corner_param.md) |
| related | [oval_width_edit_propagates_to_arc_centre_via_solve](/crates/oxide-app/tests/regression/library_cross_track/oval_width_edit_propagates_to_arc_centre_via_solve.md) |
| related | [type_5_during_line_draw_commits_at_5mm](/crates/oxide-app/tests/regression/library_cross_track/type_5_during_line_draw_commits_at_5mm.md) |
| related | [tangent_arc_after_line_creates_tangent_constraint](/crates/oxide-app/tests/regression/library_cross_track/tangent_arc_after_line_creates_tangent_constraint.md) |
| related | [chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts](/crates/oxide-app/tests/regression/library_cross_track/chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts.md) |
