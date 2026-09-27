---
okf_version: "0.2"
type: Function
title: try_drag_track_end_grab
description: "#361 — \"Drag Track End\" endpoint-biased segment grab. While the"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_drag_track_end_grab_1
language: rust
---

# try_drag_track_end_grab

#361 — "Drag Track End" endpoint-biased segment grab. While the

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn try_drag_track_end_grab(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

#361 — "Drag Track End" endpoint-biased segment grab. While the
DragTrackEnd tool is armed (Place ▸ Drag Track End), a left-press
anywhere on a sketch `Line` grabs that line's NEARER endpoint and
arms the existing Point-drag path — so the endpoint follows the
cursor with the solver live, regardless of the 12 px point-snap
radius that [`Self::try_sketch_point_grab`] requires.

This runs BEFORE the Point / Line grabs in the press walk order
(`on_primary_pressed`) so the armed tool wins the click; the
whole-line drag (`try_sketch_line_grab`) is deliberately bypassed
because this tool is biased to the segment's END, not its body.

Reuse, not new machinery: it arms a `DragState { sketch_point: … }`
— the identical state [`Self::try_sketch_point_grab`] arms — so the
per-tick `SketchMovePoint` streaming (`pointer.rs`) and the clean
sketch-point release path need no changes. The line hit-test is
solve-aware (same `pos_of` resolution as
[`Self::try_sketch_line_grab`]); a non-`Line` under the cursor
(arc / circle endpoints are out of scope for #361) falls through.

## Source
Lines 147–254 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
