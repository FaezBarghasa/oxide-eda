---
okf_version: "0.2"
type: Function
title: bake_pads
description: "Bake every entity tagged with [`PadAttr`] into a [`LibPad`]. Adds"
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/bake_pads
language: rust
---

# bake_pads

Bake every entity tagged with [`PadAttr`] into a [`LibPad`]. Adds

## Signature

```rust
pub fn bake_pads(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    out: &mut Vec<LibPad>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake every entity tagged with [`PadAttr`] into a [`LibPad`]. Adds
human-readable warnings to `warnings` for v0.14+ features.

## Source
Lines 64–109 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| called_by | [rotation_survives_a_bake_round_trip](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotation_survives_a_bake_round_trip.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_castellated_native](/crates/oxide-bake/tests/bake_pads/bake_castellated_native.md) |
| called_by | [bake_chamfered_native](/crates/oxide-bake/tests/bake_pads/bake_chamfered_native.md) |
| called_by | [bake_chamfered_rejects_united_ratio](/crates/oxide-bake/tests/bake_pads/bake_chamfered_rejects_united_ratio.md) |
| called_by | [bake_construction_entities_skipped](/crates/oxide-bake/tests/bake_pads/bake_construction_entities_skipped.md) |
| called_by | [bake_construction_entity_with_silk_attr_emits_no_warning](/crates/oxide-bake/tests/bake_pads/bake_construction_entity_with_silk_attr_emits_no_warning.md) |
| called_by | [bake_custom_sketch_profile_native_v0141](/crates/oxide-bake/tests/bake_pads/bake_custom_sketch_profile_native_v0141.md) |
| called_by | [bake_fiducial_default_mask_margin](/crates/oxide-bake/tests/bake_pads/bake_fiducial_default_mask_margin.md) |
| called_by | [bake_layers_use_altium_label_strings](/crates/oxide-bake/tests/bake_pads/bake_layers_use_altium_label_strings.md) |
| called_by | [bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014](/crates/oxide-bake/tests/bake_pads/bake_non_construction_entity_with_silk_attr_no_longer_warns_in_v014.md) |
| called_by | [bake_npt_hole_pad](/crates/oxide-bake/tests/bake_pads/bake_npt_hole_pad.md) |
| called_by | [bake_paste_grid_warns_and_falls_back_to_single](/crates/oxide-bake/tests/bake_pads/bake_paste_grid_warns_and_falls_back_to_single.md) |
| called_by | [bake_round_pad_with_param_size](/crates/oxide-bake/tests/bake_pads/bake_round_pad_with_param_size.md) |
| called_by | [bake_smd_rect_pad](/crates/oxide-bake/tests/bake_pads/bake_smd_rect_pad.md) |
| called_by | [bake_tht_pad_with_drill](/crates/oxide-bake/tests/bake_pads/bake_tht_pad_with_drill.md) |
