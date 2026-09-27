---
okf_version: "0.2"
type: Function
title: break_track_editor
description: "Build an `app::FootprintEditorState` holding a single sketch"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_editor
language: rust
---

# break_track_editor

Build an `app::FootprintEditorState` holding a single sketch

## Signature

```rust
fn break_track_editor(
        p_start: (f64, f64),
        p_end: (f64, f64),
    ) -> (
        crate::app::FootprintEditorState,
        SketchEntityId,
        SketchEntityId,
        SketchEntityId,
    )
```

## Docstring

Build an `app::FootprintEditorState` holding a single sketch
Line from `p_start` to `p_end`, armed with the Break Track tool.
Returns the editor plus the start / end Point ids and the Line
id so tests can assert against the pre-split identities.

## Source
Lines 645–701 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| calls | [empty_footprint](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/empty_footprint.md) |
| called_by | [break_track_click_near_endpoint_warns_no_split](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_click_near_endpoint_warns_no_split.md) |
| called_by | [break_track_miss_warns_and_leaves_line_intact](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_miss_warns_and_leaves_line_intact.md) |
| called_by | [break_track_reselects_line_a_not_line_b](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_reselects_line_a_not_line_b.md) |
| called_by | [break_track_split_at_mid_span_replaces_line_with_two_halves](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_split_at_mid_span_replaces_line_with_two_halves.md) |
