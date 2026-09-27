---
okf_version: "0.2"
type: Module
title: round_trip
resource: crates/oxide-sketch/tests/round_trip.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/round_trip
language: rust
---

# round_trip

## Relationships

| Type | Target |
|------|--------|
| related | [entity_id_round_trip](/crates/oxide-sketch/tests/round_trip/entity_id_round_trip.md) |
| related | [constraint_id_round_trip](/crates/oxide-sketch/tests/round_trip/constraint_id_round_trip.md) |
| related | [plane_board_top_round_trip](/crates/oxide-sketch/tests/round_trip/plane_board_top_round_trip.md) |
| related | [plane_body_top_round_trip](/crates/oxide-sketch/tests/round_trip/plane_body_top_round_trip.md) |
| related | [point_entity_round_trip](/crates/oxide-sketch/tests/round_trip/point_entity_round_trip.md) |
| related | [line_entity_round_trip](/crates/oxide-sketch/tests/round_trip/line_entity_round_trip.md) |
| related | [arc_entity_round_trip](/crates/oxide-sketch/tests/round_trip/arc_entity_round_trip.md) |
| related | [circle_entity_round_trip](/crates/oxide-sketch/tests/round_trip/circle_entity_round_trip.md) |
| related | [smd_rect_pad](/crates/oxide-sketch/tests/round_trip/smd_rect_pad.md) |
| related | [pad_attr_round_trip_rect](/crates/oxide-sketch/tests/round_trip/pad_attr_round_trip_rect.md) |
| related | [pad_attr_round_rect_corner_radius_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_round_rect_corner_radius_round_trip.md) |
| related | [pad_attr_chamfered_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_chamfered_round_trip.md) |
| related | [pad_attr_custom_static_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_custom_static_round_trip.md) |
| related | [pad_attr_with_rotation_and_offset_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_with_rotation_and_offset_round_trip.md) |
| related | [pad_attr_tht_with_drill_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_tht_with_drill_round_trip.md) |
| related | [pad_attr_npt_mounting_hole_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_npt_mounting_hole_round_trip.md) |
| related | [pad_attr_with_mask_paste_overrides_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_with_mask_paste_overrides_round_trip.md) |
| related | [pad_attr_thermal_grid_round_trip](/crates/oxide-sketch/tests/round_trip/pad_attr_thermal_grid_round_trip.md) |
| related | [standalone_mask_opening_round_trip](/crates/oxide-sketch/tests/round_trip/standalone_mask_opening_round_trip.md) |
| related | [standalone_paste_aperture_round_trip](/crates/oxide-sketch/tests/round_trip/standalone_paste_aperture_round_trip.md) |
| related | [fiducial_pad_round_trip](/crates/oxide-sketch/tests/round_trip/fiducial_pad_round_trip.md) |
| related | [pour_attr_round_trip_default](/crates/oxide-sketch/tests/round_trip/pour_attr_round_trip_default.md) |
| related | [pour_attr_hatched_with_overrides_round_trip](/crates/oxide-sketch/tests/round_trip/pour_attr_hatched_with_overrides_round_trip.md) |
| related | [keepout_attr_no_copper_round_trip](/crates/oxide-sketch/tests/round_trip/keepout_attr_no_copper_round_trip.md) |
| related | [keepout_attr_antenna_preset_round_trip](/crates/oxide-sketch/tests/round_trip/keepout_attr_antenna_preset_round_trip.md) |
| related | [keepout_attr_routing_only_round_trip](/crates/oxide-sketch/tests/round_trip/keepout_attr_routing_only_round_trip.md) |
| related | [board_cutout_through_round_trip](/crates/oxide-sketch/tests/round_trip/board_cutout_through_round_trip.md) |
| related | [board_cutout_sharp_round_trip](/crates/oxide-sketch/tests/round_trip/board_cutout_sharp_round_trip.md) |
| related | [castellated_pad_round_trip](/crates/oxide-sketch/tests/round_trip/castellated_pad_round_trip.md) |
| related | [v_score_hint_default_round_trip](/crates/oxide-sketch/tests/round_trip/v_score_hint_default_round_trip.md) |
| related | [v_score_hint_with_overrides_round_trip](/crates/oxide-sketch/tests/round_trip/v_score_hint_with_overrides_round_trip.md) |
| related | [linear_array_round_trip](/crates/oxide-sketch/tests/round_trip/linear_array_round_trip.md) |
| related | [grid_array_with_bga_numbering_round_trip](/crates/oxide-sketch/tests/round_trip/grid_array_with_bga_numbering_round_trip.md) |
| related | [polar_array_round_trip](/crates/oxide-sketch/tests/round_trip/polar_array_round_trip.md) |
| related | [polar_array_with_depopulation_round_trip](/crates/oxide-sketch/tests/round_trip/polar_array_with_depopulation_round_trip.md) |
| related | [grid_array_with_corner_depopulation_round_trip](/crates/oxide-sketch/tests/round_trip/grid_array_with_corner_depopulation_round_trip.md) |
| related | [grid_array_with_suppressed_instances_round_trip](/crates/oxide-sketch/tests/round_trip/grid_array_with_suppressed_instances_round_trip.md) |
| related | [polar_array_with_suppressed_instances_round_trip](/crates/oxide-sketch/tests/round_trip/polar_array_with_suppressed_instances_round_trip.md) |
| related | [grid_depopulation_default_suppressed_instances_back_compat](/crates/oxide-sketch/tests/round_trip/grid_depopulation_default_suppressed_instances_back_compat.md) |
| related | [Wrapper](/crates/oxide-sketch/tests/round_trip/Wrapper.md) |
| related | [explicit_numbering_round_trip](/crates/oxide-sketch/tests/round_trip/explicit_numbering_round_trip.md) |
| related | [bga_letters_basic](/crates/oxide-sketch/tests/round_trip/bga_letters_basic.md) |
| related | [bga_letters_no_skip](/crates/oxide-sketch/tests/round_trip/bga_letters_no_skip.md) |
| related | [empty_sketch_round_trip](/crates/oxide-sketch/tests/round_trip/empty_sketch_round_trip.md) |
| related | [populated_sketch_round_trip](/crates/oxide-sketch/tests/round_trip/populated_sketch_round_trip.md) |
| related | [distance_pt_circle_constraint_round_trip](/crates/oxide-sketch/tests/round_trip/distance_pt_circle_constraint_round_trip.md) |
