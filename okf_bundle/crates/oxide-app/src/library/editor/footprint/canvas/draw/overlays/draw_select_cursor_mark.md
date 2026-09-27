---
okf_version: "0.2"
type: Function
title: draw_select_cursor_mark
description: "v0.27 — Select-tool cursor mark: a dark \"+\" at the raw cursor,"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark
language: rust
---

# draw_select_cursor_mark

v0.27 — Select-tool cursor mark: a dark "+" at the raw cursor,

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn draw_select_cursor_mark(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
        cursor_screen: Option<Point>,
    ) }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Select-tool cursor mark: a dark "+" at the raw cursor,
replaced by a rotated double-headed arrow when hovering a
sketch Line (the resize cue at the exact line angle).

## Source
Lines 65–205 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
