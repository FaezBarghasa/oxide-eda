---
okf_version: "0.2"
type: Function
title: try_touching_line_click
description: v0.27 — Touching Line intercept. First click stashes the start
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_touching_line_click_1
language: rust
---

# try_touching_line_click

v0.27 — Touching Line intercept. First click stashes the start

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn try_touching_line_click(
        &self,
        cstate: &FootprintCanvasState,
        cursor_pos: Point,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Touching Line intercept. First click stashes the start
point; second click commits by publishing
FootprintTouchingLineCommit, the dispatcher walks pads +
selects everything the segment intersects.

## Source
Lines 54–83 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
