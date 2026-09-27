---
okf_version: "0.2"
type: Module
title: snap
description: v0.16.1 — Fusion-style cursor snap for the footprint sketch
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap
language: rust
---

# snap

v0.16.1 — Fusion-style cursor snap for the footprint sketch

## Docstring

v0.16.1 — Fusion-style cursor snap for the footprint sketch
canvas.

Priority chain (highest first):
1. **Point snap** — cursor within `SKETCH_SNAP_RADIUS_PX` of an
existing `Point` entity → snap to that Point's position.
2. **Horizontal / Vertical inference** — when a multi-click tool
is mid-gesture (line first endpoint, rect first corner, ...),
if the cursor's angle relative to the anchor falls within
[`AXIS_THRESHOLD_DEG`] of horizontal or vertical, snap to the
exact axis.
3. **Angle snap** — same anchor-relative angle, snap to nearest
[`ANGLE_STEP_DEG`] increment if within
[`ANGLE_THRESHOLD_DEG`].
4. **Grid snap** — fall through: round each axis to the nearest
[`GRID_STEP_MM`] increment.

No modifier-key suppression yet; iced's `CursorMoved` event
doesn't carry modifier state cleanly in 0.14, so a Shift-to-
disable toggle is deferred.

## Relationships

| Type | Target |
|------|--------|
| related | [SnapResult](/crates/oxide-app/src/library/editor/footprint/snap/SnapResult.md) |
| related | [SnapKind](/crates/oxide-app/src/library/editor/footprint/snap/SnapKind.md) |
| related | [raw](/crates/oxide-app/src/library/editor/footprint/snap/raw.md) |
| related | [raw](/crates/oxide-app/src/library/editor/footprint/snap/raw.md) |
| related | [px_to_world_mm](/crates/oxide-app/src/library/editor/footprint/snap/px_to_world_mm.md) |
| related | [anchor_for_tool](/crates/oxide-app/src/library/editor/footprint/snap/anchor_for_tool.md) |
| related | [point_pos](/crates/oxide-app/src/library/editor/footprint/snap/point_pos.md) |
| related | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| related | [empty_state](/crates/oxide-app/src/library/editor/footprint/snap/empty_state.md) |
| related | [sketch_with_anchor](/crates/oxide-app/src/library/editor/footprint/snap/sketch_with_anchor.md) |
| related | [no_anchor_grid_snaps](/crates/oxide-app/src/library/editor/footprint/snap/no_anchor_grid_snaps.md) |
| related | [horizontal_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/horizontal_within_threshold.md) |
| related | [vertical_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/vertical_within_threshold.md) |
| related | [axis_snap_also_grid_snaps_free_axis](/crates/oxide-app/src/library/editor/footprint/snap/axis_snap_also_grid_snaps_free_axis.md) |
| related | [forty_five_degree_angle_snaps](/crates/oxide-app/src/library/editor/footprint/snap/forty_five_degree_angle_snaps.md) |
| related | [point_hit_takes_priority](/crates/oxide-app/src/library/editor/footprint/snap/point_hit_takes_priority.md) |
| related | [far_off_axis_falls_through_to_grid](/crates/oxide-app/src/library/editor/footprint/snap/far_off_axis_falls_through_to_grid.md) |
| related | [vertical_guide_snaps_x_axis](/crates/oxide-app/src/library/editor/footprint/snap/vertical_guide_snaps_x_axis.md) |
| related | [horizontal_guide_snaps_y_axis](/crates/oxide-app/src/library/editor/footprint/snap/horizontal_guide_snaps_y_axis.md) |
| related | [guide_intersection_pins_both_axes](/crates/oxide-app/src/library/editor/footprint/snap/guide_intersection_pins_both_axes.md) |
| related | [disabled_guide_does_not_snap](/crates/oxide-app/src/library/editor/footprint/snap/disabled_guide_does_not_snap.md) |
| related | [global_snap_disabled_short_circuits](/crates/oxide-app/src/library/editor/footprint/snap/global_snap_disabled_short_circuits.md) |
