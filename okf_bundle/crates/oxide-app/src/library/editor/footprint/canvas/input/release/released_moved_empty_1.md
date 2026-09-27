---
okf_version: "0.2"
type: Function
title: released_moved_empty
description: "Moved empty-canvas release — Place Text Frame commit, else the"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_moved_empty_1
language: rust
---

# released_moved_empty

Moved empty-canvas release — Place Text Frame commit, else the

## Signature

```rust
fn released_moved_empty(
        &self,
        cstate: &mut FootprintCanvasState,
        drag: &DragState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Moved empty-canvas release — Place Text Frame commit, else the
rubber-band box-select commit.

## Source
Lines 49–99 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
