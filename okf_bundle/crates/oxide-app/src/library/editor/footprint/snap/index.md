# snap

## Classs

- [SnapKind](SnapKind.md) — [derive(Debug, Clone, Copy, PartialEq)]
- [SnapResult](SnapResult.md) — Outcome of a cursor snap. The `pos` field is the canvas's working

## Functions

- [anchor_for_tool](anchor_for_tool.md) — Look up the active tool's anchor — the previously-placed Point
- [axis_snap_also_grid_snaps_free_axis](axis_snap_also_grid_snaps_free_axis.md) — [test]
- [disabled_guide_does_not_snap](disabled_guide_does_not_snap.md) — [test]
- [empty_state](empty_state.md)
- [far_off_axis_falls_through_to_grid](far_off_axis_falls_through_to_grid.md) — [test]
- [forty_five_degree_angle_snaps](forty_five_degree_angle_snaps.md) — [test]
- [global_snap_disabled_short_circuits](global_snap_disabled_short_circuits.md) — [test]
- [guide_intersection_pins_both_axes](guide_intersection_pins_both_axes.md) — [test]
- [horizontal_guide_snaps_y_axis](horizontal_guide_snaps_y_axis.md) — [test]
- [horizontal_within_threshold](horizontal_within_threshold.md) — [test]
- [no_anchor_grid_snaps](no_anchor_grid_snaps.md) — [test]
- [point_hit_takes_priority](point_hit_takes_priority.md) — [test]
- [point_pos](point_pos.md) — World-mm position of a sketch `Point`. Prefers the solver's last
- [px_to_world_mm](px_to_world_mm.md) — Convert a screen-pixel radius to world-mm at the current camera
- [raw](raw.md)
- [raw](raw_1.md)
- [sketch_with_anchor](sketch_with_anchor.md)
- [snap_cursor](snap_cursor.md) — Apply the priority chain to `raw` and return the snapped
- [vertical_guide_snaps_x_axis](vertical_guide_snaps_x_axis.md) — [test]
- [vertical_within_threshold](vertical_within_threshold.md) — [test]
