---
okf_version: "0.2"
type: Function
title: click
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/click
language: rust
---

# click

## Signature

```rust
fn click(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Source
Lines 977–986 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| calls | [apply_footprint_primitive_edit](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit.md) |
| called_by | [edge_arc_rejects_a_collinear_third_pick_and_stays_armed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_rejects_a_collinear_third_pick_and_stays_armed.md) |
| called_by | [edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_sweep_direction_flips_with_which_side_the_pick_lands_on.md) |
| called_by | [edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/edge_arc_three_clicks_commit_the_circumcircle_through_all_three_picks.md) |
