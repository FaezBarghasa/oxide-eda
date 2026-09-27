---
okf_version: "0.2"
type: Function
title: on_polygon_click
description: "Click-collect gesture for the `PlacePolygon` tool. The vertex"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_polygon_click_1
language: rust
---

# on_polygon_click

Click-collect gesture for the `PlacePolygon` tool. The vertex

## Signature

```rust
fn on_polygon_click(
        &self,
        state: &mut CanvasState,
        wx: f64,
        wy: f64,
        ux: f64,
        uy: f64,
    ) -> Option<canvas::Action<CanvasAction>>
```

## Docstring

Click-collect gesture for the `PlacePolygon` tool. The vertex
stash itself (`self.polygon_vertices`) lives on
`SymbolEditorState`, not this Program's `CanvasState` — a
document-scoped Vec, not per-widget-slot ephemeral state, so a
tab switch mid-placement can't leak an in-flight vertex list
into a different document (see `CanvasAction::PolygonClick`'s
doc comment). Each plain click publishes `PolygonClick` so the
dispatcher appends it there; two close gestures are checked
first so they don't append a duplicate vertex:

1. A click on the already-placed first vertex (hit-tolerance
based, using the unsnapped cursor so it feels the same at
every zoom level — mirrors `hit_test_graphic_handle`'s
tolerance derivation).
2. A double-click — both clicks must land on the exact same
snapped grid vertex within 300 ms (not a fixed mm radius,
which at a fine snap grid could misread two adjacent-but-
distinct clicks as a double-click).

Both close gestures require `>= 3` collected vertices before
they publish `PolygonCommit` — the dispatcher's `mem::take`
discards an invalid ring, so committing early would silently
WIPE the in-progress stash. Below 3 vertices a matched double-
click is swallowed (captured, no vertex appended, no commit)
and collection continues.

## Source
Lines 255–303 in `crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools.md) |
