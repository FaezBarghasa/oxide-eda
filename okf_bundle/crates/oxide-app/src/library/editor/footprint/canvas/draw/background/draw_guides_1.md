---
okf_version: "0.2"
type: Function
title: draw_guides
description: v0.18.20 — Altium-style guide lines. Each enabled guide is a
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/background/draw_guides_1
language: rust
---

# draw_guides

v0.18.20 — Altium-style guide lines. Each enabled guide is a

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_guides(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
        bounds: Rectangle,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.18.20 — Altium-style guide lines. Each enabled guide is a
full-bleed dashed cyan line at its world coordinate.

## Source
Lines 87–125 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/library/editor/footprint/canvas/draw/background.md) |
