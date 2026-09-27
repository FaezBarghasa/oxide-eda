---
okf_version: "0.2"
type: Module
title: pointer
description: "Pointer input — pan start/stop, cursor-move (handle drag, item"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer
language: rust
---

# pointer

Pointer input — pan start/stop, cursor-move (handle drag, item

## Docstring

Pointer input — pan start/stop, cursor-move (handle drag, item
drag, rubber-band + multi-click preview tracking, idle readout),
and left-release commit (box-select / drag-commit). Each method is
the corresponding `Program::update` branch extracted verbatim;
conditions, coordinate math, and `Action` capture/publish sites are
unchanged.

## Relationships

| Type | Target |
|------|--------|
| related | [on_secondary_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_press.md) |
| related | [on_secondary_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release.md) |
| related | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
| related | [on_cursor_left](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_left.md) |
| related | [on_left_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_left_release.md) |
| related | [on_secondary_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_press.md) |
| related | [on_secondary_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release.md) |
| related | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
| related | [on_cursor_left](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_left.md) |
| related | [on_left_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_left_release.md) |
| related | [should_cancel_polygon_placement](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement.md) |
| related | [pan_moved_past_threshold](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold.md) |
| related | [should_cancel_polygon_placement_only_for_right_button](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement_only_for_right_button.md) |
| related | [should_cancel_polygon_placement_requires_a_non_empty_stash](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement_requires_a_non_empty_stash.md) |
| related | [should_cancel_polygon_placement_requires_the_place_polygon_tool](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement_requires_the_place_polygon_tool.md) |
| related | [pan_moved_past_threshold_uses_cumulative_not_per_frame_delta](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold_uses_cumulative_not_per_frame_delta.md) |
| related | [pan_moved_past_threshold_false_within_threshold_of_a_nonzero_origin](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold_false_within_threshold_of_a_nonzero_origin.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
