---
okf_version: "0.2"
type: Function
title: place_round_rect_then_select_arc_unlink_then_undo_restores_link
description: "Phase-5 #10 — Place a RoundRect pad, dispatch"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_select_arc_unlink_then_undo_restores_link
language: rust
---

# place_round_rect_then_select_arc_unlink_then_undo_restores_link

Phase-5 #10 — Place a RoundRect pad, dispatch

## Signature

```rust
fn place_round_rect_then_select_arc_unlink_then_undo_restores_link()
```

## Decorators

- `test`

## Docstring

Phase-5 #10 — Place a RoundRect pad, dispatch
`FootprintSketchUnlinkCornerRadius` for one of its corner Arcs,
then issue `Message::Edit(EditMsg::Undo)`. The unlink action's snapshot should
roll back: the per-corner override key (e.g. `corner_r_ne`) is
removed, the per-corner sketch parameter is dropped, and only the
shared `corner_r` binding survives.
[test]

## Source
Lines 490–594 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| calls | [editor_state_proj](/crates/oxide-app/tests/regression/library_cross_track/editor_state_proj.md) |
