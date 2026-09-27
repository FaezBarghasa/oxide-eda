---
okf_version: "0.2"
type: Function
title: sketch_lines
description: "`(line_id, start_id, end_id)` for every `Line` in the active"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/sketch_lines
language: rust
---

# sketch_lines

`(line_id, start_id, end_id)` for every `Line` in the active

## Signature

```rust
fn sketch_lines(
        editor: &crate::app::FootprintEditorState,
    ) -> Vec<(SketchEntityId, SketchEntityId, SketchEntityId)>
```

## Docstring

`(line_id, start_id, end_id)` for every `Line` in the active
footprint's sketch.

## Source
Lines 705–720 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| called_by | [break_track_click_near_endpoint_warns_no_split](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_click_near_endpoint_warns_no_split.md) |
| called_by | [break_track_miss_warns_and_leaves_line_intact](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_miss_warns_and_leaves_line_intact.md) |
| called_by | [break_track_reselects_line_a_not_line_b](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_reselects_line_a_not_line_b.md) |
| called_by | [break_track_split_at_mid_span_replaces_line_with_two_halves](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_split_at_mid_span_replaces_line_with_two_halves.md) |
