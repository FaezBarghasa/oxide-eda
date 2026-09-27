---
okf_version: "0.2"
type: Function
title: smd_rect_pad
description: ─────────────────────────────────────────────────────────────────────
resource: crates/oxide-bake/tests/bake_pads.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/tests/bake_pads/smd_rect_pad
language: rust
---

# smd_rect_pad

─────────────────────────────────────────────────────────────────────

## Signature

```rust
fn smd_rect_pad(number: &str, w: &str, h: &str) -> PadAttr
```

## Docstring

─────────────────────────────────────────────────────────────────────
Helpers for building common PadAttr fixtures.
─────────────────────────────────────────────────────────────────────

## Source
Lines 91–108 in `crates/oxide-bake/tests/bake_pads.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pads](/crates/oxide-bake/tests/bake_pads.md) |
| called_by | [bake_castellated_native](/crates/oxide-bake/tests/bake_pads/bake_castellated_native.md) |
| called_by | [bake_chamfered_native](/crates/oxide-bake/tests/bake_pads/bake_chamfered_native.md) |
| called_by | [bake_chamfered_rejects_united_ratio](/crates/oxide-bake/tests/bake_pads/bake_chamfered_rejects_united_ratio.md) |
| called_by | [bake_construction_entities_skipped](/crates/oxide-bake/tests/bake_pads/bake_construction_entities_skipped.md) |
| called_by | [bake_custom_sketch_profile_native_v0141](/crates/oxide-bake/tests/bake_pads/bake_custom_sketch_profile_native_v0141.md) |
| called_by | [bake_fiducial_default_mask_margin](/crates/oxide-bake/tests/bake_pads/bake_fiducial_default_mask_margin.md) |
| called_by | [bake_grid_array_with_suppressed_instances_skips_selected_cells](/crates/oxide-bake/tests/bake_pads/bake_grid_array_with_suppressed_instances_skips_selected_cells.md) |
| called_by | [bake_layers_use_altium_label_strings](/crates/oxide-bake/tests/bake_pads/bake_layers_use_altium_label_strings.md) |
| called_by | [bake_linear_array_3_pads_along_x](/crates/oxide-bake/tests/bake_pads/bake_linear_array_3_pads_along_x.md) |
| called_by | [bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014](/crates/oxide-bake/tests/bake_pads/bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014.md) |
| called_by | [bake_npt_hole_pad](/crates/oxide-bake/tests/bake_pads/bake_npt_hole_pad.md) |
| called_by | [bake_paste_grid_warns_and_falls_back_to_single](/crates/oxide-bake/tests/bake_pads/bake_paste_grid_warns_and_falls_back_to_single.md) |
| called_by | [bake_polar_array_with_suppressed_instances_skips_selected_indices](/crates/oxide-bake/tests/bake_pads/bake_polar_array_with_suppressed_instances_skips_selected_indices.md) |
| called_by | [bake_round_pad_with_param_size](/crates/oxide-bake/tests/bake_pads/bake_round_pad_with_param_size.md) |
| called_by | [bake_smd_rect_pad](/crates/oxide-bake/tests/bake_pads/bake_smd_rect_pad.md) |
| called_by | [bake_tht_pad_with_drill](/crates/oxide-bake/tests/bake_pads/bake_tht_pad_with_drill.md) |
