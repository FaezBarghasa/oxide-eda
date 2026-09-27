---
okf_version: "0.2"
type: Function
title: mirror_add_pad_to_sketch
description: "v0.15 — when a pad is added in Pads mode, mirror the new pad into"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch
language: rust
---

# mirror_add_pad_to_sketch

v0.15 — when a pad is added in Pads mode, mirror the new pad into

## Signature

```rust
pub fn mirror_add_pad_to_sketch(pad: &mut EditorPad, footprint: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

v0.15 — when a pad is added in Pads mode, mirror the new pad into
the sketch as a `Point` + `PadAttr`. Stores the minted sketch
entity ID back on the editor pad so later moves / deletes can
mirror through.

v0.24 Track A — branches on `pad.shape` so each shape mints its
own parametric geometry: Round → Circle + diameter param;
RoundRect → 4 anchors + 4 inset corners + 4 Lines + 4 Arcs
sharing `corner_r`; Oval → stadium with shared `width`/`height`;
Chamfered → outline with shared `chamfer_len`. Other shapes get
the v0.16 4-Line bbox outline.

## Source
Lines 133–138 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
| called_by | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
| called_by | [assert_minted_geometry_stays_inside_the_turned_copper](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_minted_geometry_stays_inside_the_turned_copper.md) |
| called_by | [in_place_remint_records_the_ledger_against_the_real_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_records_the_ledger_against_the_real_sketch.md) |
| called_by | [minted_pad_attr_carries_the_rotation](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_pad_attr_carries_the_rotation.md) |
| called_by | [mirror_add_pad_links_to_new_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_add_pad_links_to_new_sketch_entity.md) |
| called_by | [mirror_add_pad_with_existing_link_is_noop](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_add_pad_with_existing_link_is_noop.md) |
| called_by | [mirror_delete_drops_constraints_on_owned_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_constraints_on_owned_corners.md) |
| called_by | [mirror_delete_drops_per_corner_unlink_parameter](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_per_corner_unlink_parameter.md) |
| called_by | [mirror_delete_pad_drops_constraints_on_the_whole_entity_set](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_constraints_on_the_whole_entity_set.md) |
| called_by | [mirror_delete_pad_drops_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_sketch_entity.md) |
| called_by | [mirror_move_oval_translates_anchor_sidecars](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_oval_translates_anchor_sidecars.md) |
| called_by | [mirror_move_pad_updates_sketch_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_pad_updates_sketch_point.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [owned_set_excludes_ids_with_no_live_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_set_excludes_ids_with_no_live_entity.md) |
| called_by | [rotation_survives_a_bake_round_trip](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotation_survives_a_bake_round_trip.md) |
| called_by | [shape_change_preserves_corner_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/shape_change_preserves_corner_positions.md) |
| called_by | [sync_overwrites_every_bare_literal_form](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_overwrites_every_bare_literal_form.md) |
| called_by | [sync_preserves_a_bare_parameter_binding_with_no_eq_prefix](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_a_bare_parameter_binding_with_no_eq_prefix.md) |
| called_by | [sync_preserves_an_authored_rotation_expression](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_an_authored_rotation_expression.md) |
| called_by | [add_hole](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_hole.md) |
| called_by | [add_pad](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_pad.md) |
| called_by | [add_via](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_via.md) |
| called_by | [apply_footprint_clipboard_op](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op.md) |
| called_by | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| called_by | [rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle](/crates/oxide-app/tests/footprint_pad_remint/rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle.md) |
| called_by | [sketch_edge_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor.md) |
| called_by | [sketch_edge_drag_resizes_a_rotated_pad](/crates/oxide-app/tests/footprint_pad_rotation/sketch_edge_drag_resizes_a_rotated_pad.md) |
| called_by | [sketched_pad_fixture](/crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture.md) |
| called_by | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| called_by | [footprint_with_two_pads_sharing_a_number](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number.md) |
| called_by | [issue142_move_repairs_drifted_bbox_corners](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_move_repairs_drifted_bbox_corners.md) |
| called_by | [v026e_paste_does_not_alias_template_shape_params](/crates/oxide-app/tests/footprint_pad_sketch_mirror/v026e_paste_does_not_alias_template_shape_params.md) |
| called_by | [editing_corner_r_via_properties_updates_all_4_arcs](/crates/oxide-app/tests/regression/library_cross_track/editing_corner_r_via_properties_updates_all_4_arcs.md) |
| called_by | [oval_width_edit_propagates_to_arc_centre_via_solve](/crates/oxide-app/tests/regression/library_cross_track/oval_width_edit_propagates_to_arc_centre_via_solve.md) |
| called_by | [place_round_rect_then_select_arc_unlink_then_undo_restores_link](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_select_arc_unlink_then_undo_restores_link.md) |
| called_by | [unlink_one_corner_only_that_arc_reads_per_corner_param](/crates/oxide-app/tests/regression/library_cross_track/unlink_one_corner_only_that_arc_reads_per_corner_param.md) |
| called_by | [editing_chamfer_len_propagates_through_solve](/crates/oxide-app/tests/regression/library_pad_geometry/editing_chamfer_len_propagates_through_solve.md) |
| called_by | [editing_corner_radius_updates_all_4_arcs](/crates/oxide-app/tests/regression/library_pad_geometry/editing_corner_radius_updates_all_4_arcs.md) |
| called_by | [editing_oval_width_param_propagates_through_solve](/crates/oxide-app/tests/regression/library_pad_geometry/editing_oval_width_param_propagates_through_solve.md) |
| called_by | [mirror_add_chamfered_pad_mints_anchors_per_enabled_corner](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_chamfered_pad_mints_anchors_per_enabled_corner.md) |
| called_by | [mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params.md) |
| called_by | [mirror_add_round_pad_mints_circle_with_diameter_param](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_round_pad_mints_circle_with_diameter_param.md) |
| called_by | [mirror_add_round_rect_pad_mints_4_arcs_linked_to_corner_r](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_round_rect_pad_mints_4_arcs_linked_to_corner_r.md) |
| called_by | [properties_panel_shows_corner_radius_for_round_rect_pad](/crates/oxide-app/tests/regression/library_pad_geometry/properties_panel_shows_corner_radius_for_round_rect_pad.md) |
| called_by | [reverse_mirror_updates_pad_stack_corner_radius_pct](/crates/oxide-app/tests/regression/library_pad_geometry/reverse_mirror_updates_pad_stack_corner_radius_pct.md) |
| called_by | [unlink_corner_radius_mints_per_corner_param](/crates/oxide-app/tests/regression/library_pad_geometry/unlink_corner_radius_mints_per_corner_param.md) |
| called_by | [v025_oval_width_edit_mirrors_back_to_pad_size_mm](/crates/oxide-app/tests/regression/library_pad_geometry/v025_oval_width_edit_mirrors_back_to_pad_size_mm.md) |
| called_by | [v027_sketch_line_drag_resizes_rect_pad_bbox](/crates/oxide-app/tests/regression/library_placement/v027_sketch_line_drag_resizes_rect_pad_bbox.md) |
