---
okf_version: "0.2"
type: Function
title: sketch_with_anchor
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/sketch_with_anchor
language: rust
---

# sketch_with_anchor

## Signature

```rust
fn sketch_with_anchor() -> (SketchData, SketchEntityId)
```

## Source
Lines 515–529 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
| called_by | [axis_snap_also_grid_snaps_free_axis](/crates/oxide-app/src/library/editor/footprint/snap/axis_snap_also_grid_snaps_free_axis.md) |
| called_by | [far_off_axis_falls_through_to_grid](/crates/oxide-app/src/library/editor/footprint/snap/far_off_axis_falls_through_to_grid.md) |
| called_by | [forty_five_degree_angle_snaps](/crates/oxide-app/src/library/editor/footprint/snap/forty_five_degree_angle_snaps.md) |
| called_by | [global_snap_disabled_short_circuits](/crates/oxide-app/src/library/editor/footprint/snap/global_snap_disabled_short_circuits.md) |
| called_by | [horizontal_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/horizontal_within_threshold.md) |
| called_by | [point_hit_takes_priority](/crates/oxide-app/src/library/editor/footprint/snap/point_hit_takes_priority.md) |
| called_by | [vertical_within_threshold](/crates/oxide-app/src/library/editor/footprint/snap/vertical_within_threshold.md) |
