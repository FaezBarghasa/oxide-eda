---
okf_version: "0.2"
type: Function
title: snap_cursor
description: "Apply the priority chain to `raw` and return the snapped"
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/snap_cursor
language: rust
---

# snap_cursor

Apply the priority chain to `raw` and return the snapped

## Signature

```rust
pub fn snap_cursor(
    raw: (f64, f64),
    sketch: Option<&SketchData>,
    state: &FootprintEditorState,
    point_hit: Option<SketchEntityId>,
) -> SnapResult
```

## Visibility

- `pub`

## Docstring

Apply the priority chain to `raw` and return the snapped
position + the snap kind that fired. `point_hit` is the optional
outcome of a prior call to the canvas's `sketch_snap` (we don't
re-implement the spatial query here so the canvas's existing
px-radius logic stays the source of truth).

## Source
Lines 168–501 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
| calls | [point_pos](/crates/oxide-app/src/library/editor/footprint/snap/point_pos.md) |
| calls | [Point](/crates/oxide-types/src/schematic/mod/Point.md) |
| calls | [segment_segment_intersection](/crates/oxide-sketch/src/geom/segment/segment_segment_intersection.md) |
| calls | [segment_arc_intersections](/crates/oxide-sketch/src/geom/segment/segment_arc_intersections.md) |
| calls | [segment_circle_intersections](/crates/oxide-sketch/src/geom/segment/segment_circle_intersections.md) |
| calls | [circle_circle_intersections](/crates/oxide-sketch/src/geom/curves/circle_circle_intersections.md) |
| calls | [arc_arc_intersections](/crates/oxide-sketch/src/geom/curves/arc_arc_intersections.md) |
| calls | [arc_circle_intersections](/crates/oxide-sketch/src/geom/curves/arc_circle_intersections.md) |
| calls | [anchor_for_tool](/crates/oxide-app/src/library/editor/footprint/snap/anchor_for_tool.md) |
| called_by | [pointer_move_world](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/pointer_move_world.md) |
| called_by | [primary_press_snap](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_snap.md) |
| called_by | [axis_snap_also_grid_snaps_free_axis](/crates/oxide-app/src/library/editor/footprint/snap/axis_snap_also_grid_snaps_free_axis.md) |
| called_by | [disabled_guide_does_not_snap](/crates/oxide-app/src/library/editor/footprint/snap/disabled_guide_does_not_snap.md) |
| called_by | [far_off_axis_falls_through_to_grid](/crates/oxide-app/src/library/editor/footprint/snap/far_off_axis_falls_through_to_grid.md) |
| called_by | [forty_five_degree_angle_snaps](/crates/oxide-app/src/library/editor/footprint/snap/forty_five_degree_angle_snaps.md) |
| called_by | [global_snap_disabled_short_circuits](/crates/oxide-app/src/library/editor/footprint/snap/global_snap_disabled_short_circuits.md) |
| called_by | [guide_intersection_pins_both_axes](/crates/oxide-app/src/library/editor/footprint/snap/guide_intersection_pins_both_axes.md) |
| called_by | [horizontal_guide_snaps_y_axis](/crates/oxide-app/src/library/editor/footprint/snap/horizontal_guide_snaps_y_axis.md) |
| called_by | [horizontal_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/horizontal_within_threshold.md) |
| called_by | [no_anchor_grid_snaps](/crates/oxide-app/src/library/editor/footprint/snap/no_anchor_grid_snaps.md) |
| called_by | [point_hit_takes_priority](/crates/oxide-app/src/library/editor/footprint/snap/point_hit_takes_priority.md) |
| called_by | [vertical_guide_snaps_x_axis](/crates/oxide-app/src/library/editor/footprint/snap/vertical_guide_snaps_x_axis.md) |
| called_by | [vertical_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/vertical_within_threshold.md) |
