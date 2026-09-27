---
okf_version: "0.2"
type: Module
title: tests
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests
language: rust
---

# tests

## Relationships

| Type | Target |
|------|--------|
| related | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| related | [empty_pads_mint_nothing](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/empty_pads_mint_nothing.md) |
| related | [three_pads_mint_three_points_with_pad_attrs](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/three_pads_mint_three_points_with_pad_attrs.md) |
| related | [skip_when_sketch_already_has_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/skip_when_sketch_already_has_entities.md) |
| related | [skip_when_sketch_only_has_construction_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/skip_when_sketch_only_has_construction_entities.md) |
| related | [mirror_add_pad_links_to_new_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_add_pad_links_to_new_sketch_entity.md) |
| related | [mirror_add_pad_with_existing_link_is_noop](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_add_pad_with_existing_link_is_noop.md) |
| related | [mirror_move_pad_updates_sketch_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_pad_updates_sketch_point.md) |
| related | [mirror_delete_pad_drops_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_sketch_entity.md) |
| related | [footprint_with_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/footprint_with_profile_pad.md) |
| related | [mirror_move_profile_pad_translates_profile_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_translates_profile_geometry.md) |
| related | [point_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of.md) |
| related | [sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sidecar.md) |
| related | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| related | [mirror_move_oval_translates_anchor_sidecars](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_oval_translates_anchor_sidecars.md) |
| related | [mirror_delete_drops_constraints_on_owned_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_constraints_on_owned_corners.md) |
| related | [mirror_delete_drops_per_corner_unlink_parameter](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_per_corner_unlink_parameter.md) |
| related | [format_f64_trims_trailing_zeros](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/format_f64_trims_trailing_zeros.md) |
| related | [shape_change_preserves_corner_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/shape_change_preserves_corner_positions.md) |
| related | [minted_pad_attr_carries_the_rotation](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_pad_attr_carries_the_rotation.md) |
| related | [rotation_survives_a_bake_round_trip](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotation_survives_a_bake_round_trip.md) |
| related | [sync_preserves_an_authored_rotation_expression](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_an_authored_rotation_expression.md) |
| related | [minted_points_in_pad_frame](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_points_in_pad_frame.md) |
| related | [assert_minted_geometry_stays_inside_the_turned_copper](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_minted_geometry_stays_inside_the_turned_copper.md) |
| related | [rotated_round_rect_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_round_rect_mints_geometry_that_closes.md) |
| related | [rotated_oval_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_oval_mints_geometry_that_closes.md) |
| related | [rotated_chamfered_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_chamfered_mints_geometry_that_closes.md) |
| related | [sync_preserves_a_bare_parameter_binding_with_no_eq_prefix](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_a_bare_parameter_binding_with_no_eq_prefix.md) |
| related | [sync_overwrites_every_bare_literal_form](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_overwrites_every_bare_literal_form.md) |
| related | [mirror_delete_pad_drops_constraints_on_the_whole_entity_set](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_constraints_on_the_whole_entity_set.md) |
| related | [owned_set_excludes_ids_with_no_live_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_set_excludes_ids_with_no_live_entity.md) |
| related | [in_place_remint_records_the_ledger_against_the_real_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_records_the_ledger_against_the_real_sketch.md) |
| related | [in_place_remint_matches_a_fresh_mint_for_every_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_matches_a_fresh_mint_for_every_shape.md) |
| related | [owned_point_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_point_positions.md) |
| related | [roundrect_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/roundrect_arc_centres.md) |
| related | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
| related | [mirror_move_profile_pad_owning_its_loop_applies_delta_once](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_owning_its_loop_applies_delta_once.md) |
