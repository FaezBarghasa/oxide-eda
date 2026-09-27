---
okf_version: "0.2"
type: Function
title: draw_place_pad_ghost
description: "v0.16.1 — Pads-mode placement ghost: a shape-aware outline at"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_pad_ghost_1
language: rust
---

# draw_place_pad_ghost

v0.16.1 — Pads-mode placement ghost: a shape-aware outline at

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_place_pad_ghost(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.16.1 — Pads-mode placement ghost: a shape-aware outline at
the cursor showing where the next pad will land (hidden while
`placement_paused`). Reflects `next_pad_defaults`.

## Source
Lines 15–191 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ghosts](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
