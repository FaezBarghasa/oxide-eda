---
okf_version: "0.2"
type: Function
title: draw_place_via_ghost
description: "v0.27 — PlaceVia ghost preview: a translucent green disc with a"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_via_ghost_1
language: rust
---

# draw_place_via_ghost

v0.27 — PlaceVia ghost preview: a translucent green disc with a

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_place_via_ghost(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — PlaceVia ghost preview: a translucent green disc with a
black drilled hole, off hardcoded via geometry (Round 0.6 mm
copper / 0.3 mm drill).

## Source
Lines 196–224 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ghosts](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
