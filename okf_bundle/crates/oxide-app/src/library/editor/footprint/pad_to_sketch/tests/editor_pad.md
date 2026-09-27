---
okf_version: "0.2"
type: Function
title: editor_pad
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad
language: rust
---

# editor_pad

## Signature

```rust
fn editor_pad(number: &str, x: f64, y: f64) -> EditorPad
```

## Source
Lines 9–13 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| called_by | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
| called_by | [assert_minted_geometry_stays_inside_the_turned_copper](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_minted_geometry_stays_inside_the_turned_copper.md) |
| called_by | [footprint_with_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/footprint_with_profile_pad.md) |
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
