# footprint_pad_rotation

## Functions

- [assert_corners_match_pad](assert_corners_match_pad.md) — Every outline-corner `Point` must sit exactly where the pad's real
- [courtyard_encloses_the_rotated_pad](courtyard_encloses_the_rotated_pad.md) — The auto-fit courtyard is built from the pad extents. Reading the
- [dispatch](dispatch.md)
- [fixture](fixture.md) — Fresh app + a footprint editor holding `count` default pads, with
- [flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad](flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad.md) — Flip mirrors the pad's copper to the other side. `oxide_bake::pad`
- [flip_moves_every_selected_pad_to_the_back_side](flip_moves_every_selected_pad_to_the_back_side.md) — The fab-error case: a partial flip leaves some pads on F.* and some
- [flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper](flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper.md) — Same defect on the Flip arm — it negates the angle, so
- [one_undo_reverses_the_whole_multi_pad_rotate](one_undo_reverses_the_whole_multi_pad_rotate.md) — `apply_footprint_primitive_edit` does NOT push a snapshot for Rotate —
- [properties_panel_rotation_moves_the_sketch_outline_corners](properties_panel_rotation_moves_the_sketch_outline_corners.md) — The Properties-panel rotation field is the third sibling: it also
- [rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper](rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper.md) — The rotate arm mutated `rotation_deg` and called only
- [rotate_turns_every_pad_in_the_selection](rotate_turns_every_pad_in_the_selection.md) — Rotate acted on `selected_pad` alone; pads 1 and 2 never turned.
- [rotated_pad_hit_tests_against_the_turned_copper](rotated_pad_hit_tests_against_the_turned_copper.md) — A 2×1 mm pad turned 90° occupies ±0.5 mm in X and ±1.0 mm in Y.
- [sketch_edge_drag_resizes_a_rotated_pad](sketch_edge_drag_resizes_a_rotated_pad.md) — The v0.27 sketch-line-edge-drag → pad-resize propagation
- [sketched_pad_fixture](sketched_pad_fixture.md) — A footprint editor holding one selected `Rect` pad at the origin
- [touching_line_scores_the_rotated_pad_not_the_unrotated_box](touching_line_scores_the_rotated_pad_not_the_unrotated_box.md) — Touching Line is a sibling of rubber-band select and scored pads
