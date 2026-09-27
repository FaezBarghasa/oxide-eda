---
okf_version: "0.2"
type: Function
title: point_xy
description: "Coordinates of the Point `id` in the active footprint's sketch."
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_xy
language: rust
---

# point_xy

Coordinates of the Point `id` in the active footprint's sketch.

## Signature

```rust
fn point_xy(editor: &crate::app::FootprintEditorState, id: SketchEntityId) -> (f64, f64)
```

## Docstring

Coordinates of the Point `id` in the active footprint's sketch.

## Source
Lines 723–737 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [break_track_split_at_mid_span_replaces_line_with_two_halves](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_split_at_mid_span_replaces_line_with_two_halves.md) |
| called_by | [edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on.md) |
| called_by | [edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks.md) |
