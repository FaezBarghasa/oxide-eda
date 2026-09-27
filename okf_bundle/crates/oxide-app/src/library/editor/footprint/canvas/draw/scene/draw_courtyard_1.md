---
okf_version: "0.2"
type: Function
title: draw_courtyard
description: Courtyard — outline-following polygon takes precedence over the
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_courtyard_1
language: rust
---

# draw_courtyard

Courtyard — outline-following polygon takes precedence over the

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_courtyard(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Courtyard — outline-following polygon takes precedence over the
bbox rectangle when present (v0.27); fall back to the bbox for
legacy state.

## Source
Lines 39–74 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
