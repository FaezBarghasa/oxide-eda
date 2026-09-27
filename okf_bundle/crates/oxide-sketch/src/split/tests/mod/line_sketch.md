---
okf_version: "0.2"
type: Function
title: line_sketch
description: "A single Line `start -> end` plus its two Point entities."
resource: crates/oxide-sketch/src/split/tests/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/tests/mod/line_sketch
language: rust
---

# line_sketch

A single Line `start -> end` plus its two Point entities.

## Signature

```rust
fn line_sketch(
    start: (f64, f64),
    end: (f64, f64),
) -> (SketchData, SketchEntityId, SketchEntityId, SketchEntityId)
```

## Docstring

A single Line `start -> end` plus its two Point entities.
Returns `(sketch, line, start, end)`.

## Source
Lines 17–48 in `crates/oxide-sketch/src/split/tests/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-sketch/src/split/tests/mod.md) |
| called_by | [array_source_and_polar_center_retarget_to_line_a](/crates/oxide-sketch/src/split/tests/carry_over/array_source_and_polar_center_retarget_to_line_a.md) |
| called_by | [bake_attributes_and_flags_carry_onto_both_halves](/crates/oxide-sketch/src/split/tests/carry_over/bake_attributes_and_flags_carry_onto_both_halves.md) |
| called_by | [closed_profile_seed_attrs_stay_on_line_a_only](/crates/oxide-sketch/src/split/tests/carry_over/closed_profile_seed_attrs_stay_on_line_a_only.md) |
| called_by | [custom_pad_shape_profile_source_retargets_to_line_a](/crates/oxide-sketch/src/split/tests/carry_over/custom_pad_shape_profile_source_retargets_to_line_a.md) |
| called_by | [distance_pt_pt_on_endpoints_is_untouched](/crates/oxide-sketch/src/split/tests/carry_over/distance_pt_pt_on_endpoints_is_untouched.md) |
| called_by | [endpoints_shared_not_duplicated](/crates/oxide-sketch/src/split/tests/carry_over/endpoints_shared_not_duplicated.md) |
| called_by | [equal_length_constraint_repoints_to_one_half_only](/crates/oxide-sketch/src/split/tests/carry_over/equal_length_constraint_repoints_to_one_half_only.md) |
| called_by | [horizontal_constraint_duplicates_onto_both_halves](/crates/oxide-sketch/src/split/tests/carry_over/horizontal_constraint_duplicates_onto_both_halves.md) |
| called_by | [mid_split_creates_two_lines_and_drops_original](/crates/oxide-sketch/src/split/tests/carry_over/mid_split_creates_two_lines_and_drops_original.md) |
| called_by | [midpoint_constraint_on_retired_line_is_dropped_not_relocated](/crates/oxide-sketch/src/split/tests/carry_over/midpoint_constraint_on_retired_line_is_dropped_not_relocated.md) |
| called_by | [paste_aperture_custom_source_retargets_to_line_a](/crates/oxide-sketch/src/split/tests/carry_over/paste_aperture_custom_source_retargets_to_line_a.md) |
| called_by | [point_on_line_repoints_to_the_half_the_point_falls_on](/crates/oxide-sketch/src/split/tests/carry_over/point_on_line_repoints_to_the_half_the_point_falls_on.md) |
| called_by | [split_at_non_half_t_interpolates_correctly](/crates/oxide-sketch/src/split/tests/carry_over/split_at_non_half_t_interpolates_correctly.md) |
| called_by | [unrelated_midpoint_constraint_survives_untouched](/crates/oxide-sketch/src/split/tests/carry_over/unrelated_midpoint_constraint_survives_untouched.md) |
| called_by | [a_realistic_close_to_end_split_still_succeeds](/crates/oxide-sketch/src/split/tests/errors/a_realistic_close_to_end_split_still_succeeds.md) |
| called_by | [failed_split_leaves_sketch_byte_identical](/crates/oxide-sketch/src/split/tests/errors/failed_split_leaves_sketch_byte_identical.md) |
| called_by | [infinite_endpoint_coordinate_is_rejected](/crates/oxide-sketch/src/split/tests/errors/infinite_endpoint_coordinate_is_rejected.md) |
| called_by | [line_between_min_and_2x_min_is_degenerate_at_every_t](/crates/oxide-sketch/src/split/tests/errors/line_between_min_and_2x_min_is_degenerate_at_every_t.md) |
| called_by | [line_just_over_2x_min_still_splits_at_its_midpoint](/crates/oxide-sketch/src/split/tests/errors/line_just_over_2x_min_still_splits_at_its_midpoint.md) |
| called_by | [mid_too_close_to_endpoint_on_a_long_line_is_rejected](/crates/oxide-sketch/src/split/tests/errors/mid_too_close_to_endpoint_on_a_long_line_is_rejected.md) |
| called_by | [missing_line_id_returns_err](/crates/oxide-sketch/src/split/tests/errors/missing_line_id_returns_err.md) |
| called_by | [nan_endpoint_coordinate_is_rejected_not_silently_propagated](/crates/oxide-sketch/src/split/tests/errors/nan_endpoint_coordinate_is_rejected_not_silently_propagated.md) |
| called_by | [t_at_or_beyond_an_endpoint_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_at_or_beyond_an_endpoint_returns_err.md) |
| called_by | [t_nan_or_infinite_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_nan_or_infinite_returns_err.md) |
| called_by | [t_within_min_segment_len_of_an_endpoint_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_within_min_segment_len_of_an_endpoint_returns_err.md) |
| called_by | [too_close_to_endpoint_line_always_has_a_better_t](/crates/oxide-sketch/src/split/tests/errors/too_close_to_endpoint_line_always_has_a_better_t.md) |
| called_by | [ulp_absorbed_mid_point_is_rejected](/crates/oxide-sketch/src/split/tests/errors/ulp_absorbed_mid_point_is_rejected.md) |
| called_by | [wrong_kind_returns_err](/crates/oxide-sketch/src/split/tests/errors/wrong_kind_returns_err.md) |
| called_by | [zero_length_line_returns_err](/crates/oxide-sketch/src/split/tests/errors/zero_length_line_returns_err.md) |
