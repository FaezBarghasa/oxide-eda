---
okf_version: "0.2"
type: Function
title: state_with_tool
resource: crates/oxide-app/src/library/editor/footprint/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/tests/state_with_tool
language: rust
---

# state_with_tool

## Signature

```rust
fn state_with_tool(tool: SketchTool) -> FootprintEditorState
```

## Source
Lines 218–223 in `crates/oxide-app/src/library/editor/footprint/canvas/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/canvas/tests.md) |
| called_by | [armed_drag_track_end_wins_walk_order_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/armed_drag_track_end_wins_walk_order_via_dispatcher.md) |
| called_by | [disarmed_select_tool_still_drags_whole_line_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/disarmed_select_tool_still_drags_whole_line_via_dispatcher.md) |
| called_by | [press_equidistant_resolves_to_start_deterministically](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_equidistant_resolves_to_start_deterministically.md) |
| called_by | [press_near_line_end_arms_nearer_endpoint_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_near_line_end_arms_nearer_endpoint_point_drag.md) |
| called_by | [release_cleanly_ends_a_sketch_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/release_cleanly_ends_a_sketch_point_drag.md) |
