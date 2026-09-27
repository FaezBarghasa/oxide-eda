---
okf_version: "0.2"
type: Function
title: draw_pads_tool_preview
description: v0.18.16 — Pads-mode multi-click gesture preview. Reads the
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pads_tool_preview
language: rust
---

# draw_pads_tool_preview

v0.18.16 — Pads-mode multi-click gesture preview. Reads the

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_pads_tool_preview(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.18.16 — Pads-mode multi-click gesture preview. Reads the
in-flight tool state (`track_first` / `place_arc_pending` /
`place_polygon_vertices`) plus `cursor_mm` and draws a ghost
preview of what the next click will commit.

## Source
Lines 211–316 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| called_by | [draw](/crates/oxide-app/src/library/editor/footprint/canvas/mod/draw.md) |
