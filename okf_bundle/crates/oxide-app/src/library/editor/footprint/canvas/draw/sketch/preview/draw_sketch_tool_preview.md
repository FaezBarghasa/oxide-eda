---
okf_version: "0.2"
type: Function
title: draw_sketch_tool_preview
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview
language: rust
---

# draw_sketch_tool_preview

## Signature

```rust
pub(in crate::library::editor::footprint::canvas::draw) fn draw_sketch_tool_preview(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas::draw)`

## Source
Lines 99–801 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [placement_field_buf](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/placement_field_buf.md) |
| calls | [draw_dim_pill_styled](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill_styled.md) |
| calls | [draw_dim_pill](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| called_by | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
