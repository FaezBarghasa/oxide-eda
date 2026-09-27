---
okf_version: "0.2"
type: Module
title: pad
description: Footprint-editor active-tab accessors and pad-property setters —
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad
language: rust
---

# pad

Footprint-editor active-tab accessors and pad-property setters —

## Docstring

Footprint-editor active-tab accessors and pad-property setters —
the helper methods behind the `FpEditor*` pad-defaults / selected-
pad / sketch-pad dock-panel messages. They mutate `next_pad_defaults`
or the selected pad on the active `.snxfpt` editor and re-bake as
needed; the dispatcher in `mod.rs` routes the panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [active_footprint_editor](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/active_footprint_editor.md) |
| related | [active_footprint_editor_mut](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/active_footprint_editor_mut.md) |
| related | [fp_editor_set_next_pad_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_designator.md) |
| related | [fp_editor_set_next_pad_size_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_size_x.md) |
| related | [fp_editor_set_next_pad_size_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_size_y.md) |
| related | [fp_editor_set_next_pad_side](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_side.md) |
| related | [fp_editor_set_next_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_rotation.md) |
| related | [fp_editor_set_selected_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_rotation.md) |
| related | [fp_editor_set_next_pad_shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_shape.md) |
| related | [fp_editor_set_next_pad_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_kind.md) |
| related | [fp_editor_set_next_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_diameter.md) |
| related | [fp_editor_set_next_pad_drill_slot_length](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_slot_length.md) |
| related | [fp_editor_set_next_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_corner_radius_pct.md) |
| related | [fp_editor_set_next_pad_template](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_template.md) |
| related | [fp_editor_set_next_pad_template_library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_template_library.md) |
| related | [fp_editor_set_next_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_top.md) |
| related | [fp_editor_set_next_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_bottom.md) |
| related | [fp_editor_toggle_next_pad_paste_enabled_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_paste_enabled_top.md) |
| related | [fp_editor_toggle_next_pad_paste_enabled_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_paste_enabled_bottom.md) |
| related | [fp_editor_set_next_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_top.md) |
| related | [fp_editor_set_next_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_bottom.md) |
| related | [fp_editor_toggle_next_pad_mask_tented_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_mask_tented_top.md) |
| related | [fp_editor_toggle_next_pad_mask_tented_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_mask_tented_bottom.md) |
| related | [fp_editor_toggle_next_pad_thermal_relief](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_thermal_relief.md) |
| related | [fp_editor_set_next_pad_feature_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_feature_top.md) |
| related | [fp_editor_set_next_pad_feature_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_feature_bottom.md) |
| related | [fp_editor_toggle_next_pad_testpoint_top_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_top_assembly.md) |
| related | [fp_editor_toggle_next_pad_testpoint_top_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_top_fab.md) |
| related | [fp_editor_toggle_next_pad_testpoint_bottom_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_bottom_assembly.md) |
| related | [fp_editor_toggle_next_pad_testpoint_bottom_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_bottom_fab.md) |
| related | [with_selected_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad.md) |
| related | [with_selected_sketch_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_sketch_pad.md) |
| related | [fp_editor_set_selected_pad_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_designator.md) |
| related | [fp_editor_set_selected_pad_side](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_side.md) |
| related | [fp_editor_set_selected_pad_shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_shape.md) |
| related | [fp_editor_set_selected_pad_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_kind.md) |
| related | [fp_editor_set_selected_pad_size_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_size_x.md) |
| related | [fp_editor_set_selected_pad_size_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_size_y.md) |
| related | [fp_editor_set_selected_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_diameter.md) |
| related | [fp_editor_set_selected_pad_drill_slot_length](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_slot_length.md) |
| related | [fp_editor_set_selected_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_corner_radius_pct.md) |
| related | [fp_editor_set_selected_pad_template](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_template.md) |
| related | [fp_editor_set_selected_pad_template_library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_template_library.md) |
| related | [fp_editor_set_selected_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_top.md) |
| related | [fp_editor_set_selected_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_bottom.md) |
| related | [fp_editor_toggle_selected_pad_paste_enabled_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_paste_enabled_top.md) |
| related | [fp_editor_toggle_selected_pad_paste_enabled_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_paste_enabled_bottom.md) |
| related | [fp_editor_set_selected_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_top.md) |
| related | [fp_editor_set_selected_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_bottom.md) |
| related | [fp_editor_toggle_selected_pad_mask_tented_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_mask_tented_top.md) |
| related | [fp_editor_toggle_selected_pad_mask_tented_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_mask_tented_bottom.md) |
| related | [fp_editor_toggle_selected_pad_thermal_relief](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_thermal_relief.md) |
| related | [fp_editor_set_selected_pad_feature_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_feature_top.md) |
| related | [fp_editor_set_selected_pad_feature_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_feature_bottom.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_top_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_top_assembly.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_top_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_top_fab.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_bottom_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_bottom_assembly.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_bottom_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_bottom_fab.md) |
| related | [handle_fp_editor_set_pad_stack_tab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_pad_stack_tab.md) |
| related | [handle_fp_editor_set_next_pad_electrical_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_electrical_type.md) |
| related | [handle_fp_editor_set_next_pad_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_net.md) |
| related | [handle_fp_editor_toggle_next_pad_locked](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_toggle_next_pad_locked.md) |
| related | [handle_fp_editor_set_next_pad_hole_tolerance_plus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_plus.md) |
| related | [handle_fp_editor_set_next_pad_hole_tolerance_minus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_minus.md) |
| related | [handle_fp_editor_set_next_pad_hole_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_rotation.md) |
| related | [handle_fp_editor_set_next_pad_copper_offset_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_x.md) |
| related | [handle_fp_editor_set_next_pad_copper_offset_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_y.md) |
| related | [handle_fp_editor_toggle_next_pad_plated](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_toggle_next_pad_plated.md) |
| related | [active_footprint_editor](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/active_footprint_editor.md) |
| related | [active_footprint_editor_mut](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/active_footprint_editor_mut.md) |
| related | [fp_editor_set_next_pad_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_designator.md) |
| related | [fp_editor_set_next_pad_size_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_size_x.md) |
| related | [fp_editor_set_next_pad_size_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_size_y.md) |
| related | [fp_editor_set_next_pad_side](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_side.md) |
| related | [fp_editor_set_next_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_rotation.md) |
| related | [fp_editor_set_selected_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_rotation.md) |
| related | [fp_editor_set_next_pad_shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_shape.md) |
| related | [fp_editor_set_next_pad_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_kind.md) |
| related | [fp_editor_set_next_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_diameter.md) |
| related | [fp_editor_set_next_pad_drill_slot_length](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_slot_length.md) |
| related | [fp_editor_set_next_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_corner_radius_pct.md) |
| related | [fp_editor_set_next_pad_template](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_template.md) |
| related | [fp_editor_set_next_pad_template_library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_template_library.md) |
| related | [fp_editor_set_next_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_top.md) |
| related | [fp_editor_set_next_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_bottom.md) |
| related | [fp_editor_toggle_next_pad_paste_enabled_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_paste_enabled_top.md) |
| related | [fp_editor_toggle_next_pad_paste_enabled_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_paste_enabled_bottom.md) |
| related | [fp_editor_set_next_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_top.md) |
| related | [fp_editor_set_next_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_bottom.md) |
| related | [fp_editor_toggle_next_pad_mask_tented_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_mask_tented_top.md) |
| related | [fp_editor_toggle_next_pad_mask_tented_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_mask_tented_bottom.md) |
| related | [fp_editor_toggle_next_pad_thermal_relief](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_thermal_relief.md) |
| related | [fp_editor_set_next_pad_feature_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_feature_top.md) |
| related | [fp_editor_set_next_pad_feature_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_feature_bottom.md) |
| related | [fp_editor_toggle_next_pad_testpoint_top_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_top_assembly.md) |
| related | [fp_editor_toggle_next_pad_testpoint_top_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_top_fab.md) |
| related | [fp_editor_toggle_next_pad_testpoint_bottom_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_bottom_assembly.md) |
| related | [fp_editor_toggle_next_pad_testpoint_bottom_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_next_pad_testpoint_bottom_fab.md) |
| related | [with_selected_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad.md) |
| related | [with_selected_sketch_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_sketch_pad.md) |
| related | [fp_editor_set_selected_pad_designator](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_designator.md) |
| related | [fp_editor_set_selected_pad_side](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_side.md) |
| related | [fp_editor_set_selected_pad_shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_shape.md) |
| related | [fp_editor_set_selected_pad_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_kind.md) |
| related | [fp_editor_set_selected_pad_size_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_size_x.md) |
| related | [fp_editor_set_selected_pad_size_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_size_y.md) |
| related | [fp_editor_set_selected_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_diameter.md) |
| related | [fp_editor_set_selected_pad_drill_slot_length](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_slot_length.md) |
| related | [fp_editor_set_selected_pad_corner_radius_pct](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_corner_radius_pct.md) |
| related | [fp_editor_set_selected_pad_template](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_template.md) |
| related | [fp_editor_set_selected_pad_template_library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_template_library.md) |
| related | [fp_editor_set_selected_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_top.md) |
| related | [fp_editor_set_selected_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_bottom.md) |
| related | [fp_editor_toggle_selected_pad_paste_enabled_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_paste_enabled_top.md) |
| related | [fp_editor_toggle_selected_pad_paste_enabled_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_paste_enabled_bottom.md) |
| related | [fp_editor_set_selected_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_top.md) |
| related | [fp_editor_set_selected_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_bottom.md) |
| related | [fp_editor_toggle_selected_pad_mask_tented_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_mask_tented_top.md) |
| related | [fp_editor_toggle_selected_pad_mask_tented_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_mask_tented_bottom.md) |
| related | [fp_editor_toggle_selected_pad_thermal_relief](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_thermal_relief.md) |
| related | [fp_editor_set_selected_pad_feature_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_feature_top.md) |
| related | [fp_editor_set_selected_pad_feature_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_feature_bottom.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_top_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_top_assembly.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_top_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_top_fab.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_bottom_assembly](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_bottom_assembly.md) |
| related | [fp_editor_toggle_selected_pad_testpoint_bottom_fab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_toggle_selected_pad_testpoint_bottom_fab.md) |
| related | [handle_fp_editor_set_pad_stack_tab](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_pad_stack_tab.md) |
| related | [handle_fp_editor_set_next_pad_electrical_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_electrical_type.md) |
| related | [handle_fp_editor_set_next_pad_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_net.md) |
| related | [handle_fp_editor_toggle_next_pad_locked](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_toggle_next_pad_locked.md) |
| related | [handle_fp_editor_set_next_pad_hole_tolerance_plus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_plus.md) |
| related | [handle_fp_editor_set_next_pad_hole_tolerance_minus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_minus.md) |
| related | [handle_fp_editor_set_next_pad_hole_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_rotation.md) |
| related | [handle_fp_editor_set_next_pad_copper_offset_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_x.md) |
| related | [handle_fp_editor_set_next_pad_copper_offset_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_y.md) |
| related | [handle_fp_editor_toggle_next_pad_plated](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_toggle_next_pad_plated.md) |
