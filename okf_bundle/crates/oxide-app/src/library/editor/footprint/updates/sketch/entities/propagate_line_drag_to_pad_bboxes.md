---
okf_version: "0.2"
type: Function
title: propagate_line_drag_to_pad_bboxes
description: v0.27 — propagate a dragged sketch Line to the literal pad bbox.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/propagate_line_drag_to_pad_bboxes
language: rust
---

# propagate_line_drag_to_pad_bboxes

v0.27 — propagate a dragged sketch Line to the literal pad bbox.

## Signature

```rust
fn propagate_line_drag_to_pad_bboxes(
    editor: &mut crate::app::FootprintEditorState,
    pre_drag_endpoints: Option<((f64, f64), (f64, f64))>,
    dx: f64,
    dy: f64,
)
```

## Docstring

v0.27 — propagate a dragged sketch Line to the literal pad bbox.
Without this the sketch outline visibly resizes but `pad.size_mm` /
`pad.position_mm` (and the baked pad rendering) stay frozen — the user
sees the line move while the pad copper underneath does nothing.

Strategy: classify the line's pre-drag pose against each pad's bbox to
identify which side it lies on (top / bottom / left / right). Only
axis-aligned lines qualify — diagonal sketch lines are never pad edges
for Rect / RoundRect / Oval / Chamfered shapes.

"Axis-aligned" means axis-aligned IN THE PAD'S FRAME. A turned pad's
edges are diagonal in world space, so the line, the drag delta and the
resulting corner targets all go through `world_to_local_mm` /
`local_to_world_mm`. Classifying a rotated pad in world coordinates
rejects every edge and the propagation silently no-ops — the exact
"line moves, copper doesn't" failure this fn exists to prevent.

## Source
Lines 418–517 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| calls | [remint_dragged_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad.md) |
| called_by | [move_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line.md) |
