---
okf_version: "0.2"
type: Function
title: make_canvas
resource: crates/oxide-app/src/library/editor/footprint/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/tests/make_canvas
language: rust
---

# make_canvas

## Signature

```rust
fn make_canvas(
        state: &'a FootprintEditorState,
        sketch: &'a SketchData,
        cache: &'a canvas::Cache,
    ) -> FootprintCanvas<'a>
```

## Type Parameters

- `'a`

## Source
Lines 225–244 in `crates/oxide-app/src/library/editor/footprint/canvas/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/canvas/tests.md) |
| called_by | [armed_drag_track_end_wins_walk_order_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/armed_drag_track_end_wins_walk_order_via_dispatcher.md) |
| called_by | [disarmed_select_tool_still_drags_whole_line_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/disarmed_select_tool_still_drags_whole_line_via_dispatcher.md) |
| called_by | [hit_test_uses_solved_positions_not_stale_authored_coords](/crates/oxide-app/src/library/editor/footprint/canvas/tests/hit_test_uses_solved_positions_not_stale_authored_coords.md) |
| called_by | [press_equidistant_resolves_to_start_deterministically](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_equidistant_resolves_to_start_deterministically.md) |
| called_by | [press_near_line_end_arms_nearer_endpoint_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_near_line_end_arms_nearer_endpoint_point_drag.md) |
| called_by | [release_cleanly_ends_a_sketch_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/release_cleanly_ends_a_sketch_point_drag.md) |
