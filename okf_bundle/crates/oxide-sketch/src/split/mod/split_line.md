---
okf_version: "0.2"
type: Function
title: split_line
description: "Split the `Line` entity `line` at parameter `t` (`0.0` = start,"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/split_line
language: rust
---

# split_line

Split the `Line` entity `line` at parameter `t` (`0.0` = start,

## Signature

```rust
pub fn split_line(
    sketch: &mut SketchData,
    line: SketchEntityId,
    t: f64,
) -> Result<SplitResult, SplitError>
```

## Visibility

- `pub`

## Docstring

Split the `Line` entity `line` at parameter `t` (`0.0` = start,
`1.0` = end) into two Lines meeting at a new mid `Point`.

Returns [`SplitError`] — leaving `sketch` byte-for-byte unchanged —
per the conditions on each variant.

On success the original `Line` is removed and replaced by two new
`Line`s that inherit its `construction` / `centerline` flags and
its PER-SEGMENT bake attributes (`silk`, `v_score`) — so splitting
e.g. a silk line yields two silk lines rather than dropping either
half out of the bake.

CLOSED-PROFILE SEED attributes (`courtyard`, `mask_opening`,
`mask_exclude`, `paste_aperture`, `pour`, `keepout`,
`board_cutout`) stay on `line_a` ONLY. Each of those bakes by
tracing the WHOLE closed loop from any entity that carries the
attribute (`trace_closed_profile` in `oxide-bake`), so a Line that
kept the attribute on both halves would make the bake discover —
and emit — the same loop twice (two identical pours on the same
net, two routed board cutouts on the same slot, a spurious "only
one courtyard per footprint" warning, …). `line_a` keeps the
original `start`, so it anchors the trace at the exact point the
pre-split seed did — the same one-seed-per-loop reasoning
`attr_refs::retarget_pad_profiles` already relies on for the pad
Custom-shape / Custom-paste-aperture seed lists.

CALLERS MUST KNOW: the bake is not the only reader of those seven
slots. The footprint editor's Properties panel derives its Role and
its Pour / Keepout / Cutout sub-forms from the SINGLE selected
entity's own `Option`s, so after a split only `line_a` still
presents as (say) a pour boundary — selecting `line_b` shows
"Unassigned" and edits to it no-op against a `None`. A caller that
leaves the user selected on `line_b` invites them to re-tag it,
which puts a second seed on the loop and reproduces the double-emit
this rule exists to prevent. Re-select onto [`SplitResult::line_a`]
after splitting a profile-bearing edge.

Every reference to the retired line elsewhere in `sketch` is also
rewritten or dropped, by kind:

| Reference | Outcome |
|---|---|
| `Horizontal` / `Vertical` constraint | duplicated onto BOTH halves — each is a single-line, absolute-frame predicate that still holds independently for the new segment it lands on. |
| `PointOnLine` constraint | re-pointed to whichever half the constrained point currently falls nearest, by parametric position. |
| `Midpoint` constraint | DROPPED — the retired id is reported via [`SplitResult::dropped_constraints`]. The original line's midpoint is the midpoint of NEITHER half, so re-pointing would relocate the constraint rather than preserve it. |
| `Parallel` / `Perpendicular` / `Angle` / `EqualLength` / `TangentLineArc` / `SymmetricAboutLine` / `DistancePtLine` constraint | re-pointed to `line_a` only — each relates the line to a second, independent entity; duplicating would assert the identical relationship for two now-independent segments. |
| `Coincident` / `DistancePtPt` / `Fixed` / `PointOnArc` / `DistancePtCircle` / `EqualRadius` / `TangentArcArc` / `SymmetricAboutPoint` constraint | unchanged — none of their fields can name a Line. |
| `SketchData::arrays` (`ArrayKind::*.source`, Polar's `center`) | rewritten to `line_a`. An array source must resolve to a Point carrying a `PadAttr` to bake at all, which a Line id never satisfies — this only fires on already-malformed data, but it is rewritten rather than left dangling. |
| a pad's Custom-shape / Custom-paste-aperture `source` list | every matching entry rewritten to `line_a` — both are seed lists into the closed-profile walker, which discovers the whole loop from any edge on it, and `line_a` is still wired into the same loop. |

## Source
Lines 149–200 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| calls | [validate_split](/crates/oxide-sketch/src/split/mod/validate_split.md) |
| calls | [mm_distance](/crates/oxide-sketch/src/split/mod/mm_distance.md) |
| calls | [build_split_entities](/crates/oxide-sketch/src/split/mod/build_split_entities.md) |
| calls | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
| called_by | [break_track](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track.md) |
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
| called_by | [line_just_over_2x_min_still_splits_at_its_midpoint](/crates/oxide-sketch/src/split/tests/errors/line_just_over_2x_min_still_splits_at_its_midpoint.md) |
| called_by | [mid_too_close_to_endpoint_on_a_long_line_is_rejected](/crates/oxide-sketch/src/split/tests/errors/mid_too_close_to_endpoint_on_a_long_line_is_rejected.md) |
| called_by | [nan_endpoint_coordinate_is_rejected_not_silently_propagated](/crates/oxide-sketch/src/split/tests/errors/nan_endpoint_coordinate_is_rejected_not_silently_propagated.md) |
| called_by | [t_nan_or_infinite_returns_err](/crates/oxide-sketch/src/split/tests/errors/t_nan_or_infinite_returns_err.md) |
| called_by | [too_close_to_endpoint_line_always_has_a_better_t](/crates/oxide-sketch/src/split/tests/errors/too_close_to_endpoint_line_always_has_a_better_t.md) |
| called_by | [split_then_solve_leaves_rectangle_visually_unchanged](/crates/oxide-sketch/src/split/tests/solver/split_then_solve_leaves_rectangle_visually_unchanged.md) |
