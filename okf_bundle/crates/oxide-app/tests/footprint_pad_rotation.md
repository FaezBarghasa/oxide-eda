---
okf_version: "0.2"
type: Module
title: footprint_pad_rotation
description: "Pad rotation is a geometry input, and the active-bar transforms act"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation
language: rust
---

# footprint_pad_rotation

Pad rotation is a geometry input, and the active-bar transforms act

## Docstring

Pad rotation is a geometry input, and the active-bar transforms act
on the whole selection.

The pre-existing coverage in `regression.rs`
(`v026g_rotate_selection_increments_rotation_by_90_degrees` and
siblings) only ever asserted that the `rotation_deg` FIELD changed
on a single-pad selection. That is exactly why three defects
survived: nothing checked that rotation reached the geometry, and
nothing checked a multi-pad selection. These tests assert GEOMETRY
and MULTI-pad.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture](/crates/oxide-app/tests/footprint_pad_rotation/fixture.md) |
| related | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
| related | [rotated_pad_hit_tests_against_the_turned_copper](/crates/oxide-app/tests/footprint_pad_rotation/rotated_pad_hit_tests_against_the_turned_copper.md) |
| related | [courtyard_encloses_the_rotated_pad](/crates/oxide-app/tests/footprint_pad_rotation/courtyard_encloses_the_rotated_pad.md) |
| related | [rotate_turns_every_pad_in_the_selection](/crates/oxide-app/tests/footprint_pad_rotation/rotate_turns_every_pad_in_the_selection.md) |
| related | [flip_moves_every_selected_pad_to_the_back_side](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_every_selected_pad_to_the_back_side.md) |
| related | [one_undo_reverses_the_whole_multi_pad_rotate](/crates/oxide-app/tests/footprint_pad_rotation/one_undo_reverses_the_whole_multi_pad_rotate.md) |
| related | [touching_line_scores_the_rotated_pad_not_the_unrotated_box](/crates/oxide-app/tests/footprint_pad_rotation/touching_line_scores_the_rotated_pad_not_the_unrotated_box.md) |
| related | [flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad](/crates/oxide-app/tests/footprint_pad_rotation/flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad.md) |
| related | [rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper](/crates/oxide-app/tests/footprint_pad_rotation/rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper.md) |
| related | [flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper.md) |
| related | [properties_panel_rotation_moves_the_sketch_outline_corners](/crates/oxide-app/tests/footprint_pad_rotation/properties_panel_rotation_moves_the_sketch_outline_corners.md) |
| related | [sketched_pad_fixture](/crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture.md) |
| related | [assert_corners_match_pad](/crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad.md) |
| related | [sketch_edge_drag_resizes_a_rotated_pad](/crates/oxide-app/tests/footprint_pad_rotation/sketch_edge_drag_resizes_a_rotated_pad.md) |
