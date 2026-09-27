---
okf_version: "0.2"
type: Function
title: mirror_move_pad_in_sketch
description: "v0.15 — when a pad moves in Pads mode (drag), update its backing"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch
language: rust
---

# mirror_move_pad_in_sketch

v0.15 — when a pad moves in Pads mode (drag), update its backing

## Signature

```rust
pub fn mirror_move_pad_in_sketch(pad: &EditorPad, footprint: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

v0.15 — when a pad moves in Pads mode (drag), update its backing
sketch `Point`'s coordinates so the sketch stays in sync. No-op
when the pad has no backing sketch entity yet.

v0.14.2 — a `Custom(SketchProfile)` pad does not own its geometry:
"Make Pad from Profile" mints only a centre `Point` at the loop's
centroid and references the loop by seed Line, leaving
`corner_entity_ids` `None`. Moving such a pad therefore has to
translate the profile itself, or the loop stays where it was drawn
— visibly the sketch shape doesn't follow the pad, and silently the
bake (`local_pts = world_pts - pad_position`, `oxide-bake`
`pad.rs`) resolves the copper back to the ORIGINAL location, so the
exported footprint has the pad in the wrong place.

The profile is authoritative for a sketch-profile pad; the pad's
position is a derived handle, and moving the handle means
translating the geometry.

Translates the pad's WHOLE owned set
([`ownership::owned_sketch_entities`]) rather than just the centre
and the four bbox corners. RoundRect's 8 edge anchors + 4 inset
arc-centres, Oval's 4 anchors + 2 arc-centres and Chamfered's
per-corner anchors are all minted NON-construction, so leaving them
behind made the bake emit copper from the stranded geometry — and
nothing downstream repaired it (`sync_pads_to_primitive` copies
attributes only, it never re-mints).

## Source
Lines 304–348 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [point_xy_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/point_xy_of.md) |
| calls | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
| calls | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| calls | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| calls | [reassert_bbox_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/reassert_bbox_corners.md) |
| called_by | [with_selected_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad.md) |
| called_by | [mirror_move_oval_translates_anchor_sidecars](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_oval_translates_anchor_sidecars.md) |
| called_by | [mirror_move_pad_updates_sketch_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_pad_updates_sketch_point.md) |
| called_by | [mirror_move_profile_pad_owning_its_loop_applies_delta_once](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_owning_its_loop_applies_delta_once.md) |
| called_by | [mirror_move_profile_pad_translates_profile_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_translates_profile_geometry.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [active_bar_align_selection_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_align_selection_to_grid.md) |
| called_by | [active_bar_move_origin_to_grid](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_move_origin_to_grid.md) |
| called_by | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| called_by | [footprint_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/mod/footprint_nudge_selection.md) |
| called_by | [move_pad](/crates/oxide-app/src/library/editor/footprint/updates/selection/move_pad.md) |
| called_by | [align_pads_action](/crates/oxide-app/src/library/editor/footprint/updates/view/align_pads_action.md) |
| called_by | [issue142_move_repairs_drifted_bbox_corners](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_move_repairs_drifted_bbox_corners.md) |
| called_by | [issue142_reopened_pad_still_moves_its_whole_outline](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline.md) |
