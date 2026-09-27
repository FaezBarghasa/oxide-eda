---
okf_version: "0.2"
type: Module
title: bake_pads
description: Integration tests for the v0.13 sketch → footprint pad bake.
resource: crates/oxide-bake/tests/bake_pads.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/tests/bake_pads
language: rust
---

# bake_pads

Integration tests for the v0.13 sketch → footprint pad bake.

## Docstring

Integration tests for the v0.13 sketch → footprint pad bake.

Phase 7 Task 7.1 + 7.2 of the SKETCH_MODE_v0.13_PLAN. Each test
constructs a small `SketchData` inline, runs the solver to produce
a `FullSolveOutput`, then bakes via `oxide_bake::bake_pads`
/ `bake_arrays` and asserts the resulting `LibPad` set.

Cleanroom: no third-party constraint-solver, footprint-generator,
or numerical-library source consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [Sketch](/crates/oxide-bake/tests/bake_pads/Sketch.md) |
| related | [new](/crates/oxide-bake/tests/bake_pads/new.md) |
| related | [add_point](/crates/oxide-bake/tests/bake_pads/add_point.md) |
| related | [attach_pad](/crates/oxide-bake/tests/bake_pads/attach_pad.md) |
| related | [set_construction](/crates/oxide-bake/tests/bake_pads/set_construction.md) |
| related | [new](/crates/oxide-bake/tests/bake_pads/new.md) |
| related | [add_point](/crates/oxide-bake/tests/bake_pads/add_point.md) |
| related | [attach_pad](/crates/oxide-bake/tests/bake_pads/attach_pad.md) |
| related | [set_construction](/crates/oxide-bake/tests/bake_pads/set_construction.md) |
| related | [smd_rect_pad](/crates/oxide-bake/tests/bake_pads/smd_rect_pad.md) |
| related | [solve](/crates/oxide-bake/tests/bake_pads/solve.md) |
| related | [approx_eq](/crates/oxide-bake/tests/bake_pads/approx_eq.md) |
| related | [has_layer](/crates/oxide-bake/tests/bake_pads/has_layer.md) |
| related | [bake_smd_rect_pad](/crates/oxide-bake/tests/bake_pads/bake_smd_rect_pad.md) |
| related | [bake_round_pad_with_param_size](/crates/oxide-bake/tests/bake_pads/bake_round_pad_with_param_size.md) |
| related | [bake_tht_pad_with_drill](/crates/oxide-bake/tests/bake_pads/bake_tht_pad_with_drill.md) |
| related | [bake_npt_hole_pad](/crates/oxide-bake/tests/bake_pads/bake_npt_hole_pad.md) |
| related | [bake_fiducial_default_mask_margin](/crates/oxide-bake/tests/bake_pads/bake_fiducial_default_mask_margin.md) |
| related | [bake_castellated_native](/crates/oxide-bake/tests/bake_pads/bake_castellated_native.md) |
| related | [bake_chamfered_native](/crates/oxide-bake/tests/bake_pads/bake_chamfered_native.md) |
| related | [bake_chamfered_rejects_united_ratio](/crates/oxide-bake/tests/bake_pads/bake_chamfered_rejects_united_ratio.md) |
| related | [bake_paste_grid_warns_and_falls_back_to_single](/crates/oxide-bake/tests/bake_pads/bake_paste_grid_warns_and_falls_back_to_single.md) |
| related | [bake_construction_entities_skipped](/crates/oxide-bake/tests/bake_pads/bake_construction_entities_skipped.md) |
| related | [bake_construction_entity_with_silk_attr_emits_no_warning](/crates/oxide-bake/tests/bake_pads/bake_construction_entity_with_silk_attr_emits_no_warning.md) |
| related | [bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014](/crates/oxide-bake/tests/bake_pads/bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014.md) |
| related | [bake_linear_array_3_pads_along_x](/crates/oxide-bake/tests/bake_pads/bake_linear_array_3_pads_along_x.md) |
| related | [bake_grid_array_with_suppressed_instances_skips_selected_cells](/crates/oxide-bake/tests/bake_pads/bake_grid_array_with_suppressed_instances_skips_selected_cells.md) |
| related | [bake_polar_array_with_suppressed_instances_skips_selected_indices](/crates/oxide-bake/tests/bake_pads/bake_polar_array_with_suppressed_instances_skips_selected_indices.md) |
| related | [bake_custom_sketch_profile_native_v0141](/crates/oxide-bake/tests/bake_pads/bake_custom_sketch_profile_native_v0141.md) |
| related | [bake_layers_use_altium_label_strings](/crates/oxide-bake/tests/bake_pads/bake_layers_use_altium_label_strings.md) |
