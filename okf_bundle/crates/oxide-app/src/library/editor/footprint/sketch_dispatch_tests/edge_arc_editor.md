---
okf_version: "0.2"
type: Function
title: edge_arc_editor
description: "Build an `app::FootprintEditorState` with an empty sketch,"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_editor
language: rust
---

# edge_arc_editor

Build an `app::FootprintEditorState` with an empty sketch,

## Signature

```rust
fn edge_arc_editor() -> crate::app::FootprintEditorState
```

## Docstring

Build an `app::FootprintEditorState` with an empty sketch,
armed with the Edge Arc tool.

## Source
Lines 926–945 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| calls | [empty_footprint](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/empty_footprint.md) |
| called_by | [edge_arc_rejects_a_collinear_third_pick_and_stays_armed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_rejects_a_collinear_third_pick_and_stays_armed.md) |
| called_by | [edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on.md) |
| called_by | [edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks.md) |
