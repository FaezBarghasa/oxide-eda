---
okf_version: "0.2"
type: Function
title: remint_dragged_pad
description: Regenerate the sidecar geometry of a pad whose frame a live Sketch
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad
language: rust
---

# remint_dragged_pad

Regenerate the sidecar geometry of a pad whose frame a live Sketch

## Signature

```rust
fn remint_dragged_pad(editor: &mut crate::app::FootprintEditorState, pad_idx: usize, op: &str)
```

## Docstring

Regenerate the sidecar geometry of a pad whose frame a live Sketch
mode drag just changed — the corner drag and the edge drag, which
make the same frame change and owe the same thing.

In place: the pointer holds the id of the entity the user grabbed
for the whole drag and streams one message per cursor tick against
it, so a drop-and-re-mint would freeze the drag one frame in.

## Source
Lines 585–595 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| calls | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
| calls | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
| called_by | [move_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point.md) |
| called_by | [propagate_line_drag_to_pad_bboxes](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/propagate_line_drag_to_pad_bboxes.md) |
