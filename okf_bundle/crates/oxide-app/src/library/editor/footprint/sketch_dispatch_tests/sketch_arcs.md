---
okf_version: "0.2"
type: Function
title: sketch_arcs
description: "`(arc_id, center_id, start_id, end_id, sweep_ccw)` for every"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/sketch_arcs
language: rust
---

# sketch_arcs

`(arc_id, center_id, start_id, end_id, sweep_ccw)` for every

## Signature

```rust
fn sketch_arcs(
        editor: &crate::app::FootprintEditorState,
    ) -> Vec<(
        SketchEntityId,
        SketchEntityId,
        SketchEntityId,
        SketchEntityId,
        bool,
    )>
```

## Docstring

`(arc_id, center_id, start_id, end_id, sweep_ccw)` for every
`Arc` in the active footprint's sketch.

## Source
Lines 949–975 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| called_by | [edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on.md) |
| called_by | [edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks.md) |
