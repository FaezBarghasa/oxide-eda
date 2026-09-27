---
okf_version: "0.2"
type: Function
title: empty_state
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/empty_state
language: rust
---

# empty_state

## Signature

```rust
fn empty_state() -> FootprintEditorState
```

## Source
Lines 511–513 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
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
