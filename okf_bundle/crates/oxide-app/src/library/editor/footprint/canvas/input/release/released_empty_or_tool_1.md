---
okf_version: "0.2"
type: Function
title: released_empty_or_tool
description: "The `pad_idx == usize::MAX` left-release branch: a press that"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_empty_or_tool_1
language: rust
---

# released_empty_or_tool

The `pad_idx == usize::MAX` left-release branch: a press that

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn released_empty_or_tool(
        &self,
        cstate: &mut FootprintCanvasState,
        drag: &DragState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

The `pad_idx == usize::MAX` left-release branch: a press that
started on empty canvas. Either it moved (Text Frame commit /
rubber-band select) or it was a click (sketch click-add / place
tool / selection clear).

## Source
Lines 25–45 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
