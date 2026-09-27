---
okf_version: "0.2"
type: Function
title: draw_array_badges
description: "v0.25 — Array source-pad indicator: a \"+N\" badge at the"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_array_badges_1
language: rust
---

# draw_array_badges

v0.25 — Array source-pad indicator: a "+N" badge at the

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_array_badges(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.25 — Array source-pad indicator: a "+N" badge at the
top-right of a pad that is the `source` of an Array, showing the
replica count.

## Source
Lines 96–180 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
