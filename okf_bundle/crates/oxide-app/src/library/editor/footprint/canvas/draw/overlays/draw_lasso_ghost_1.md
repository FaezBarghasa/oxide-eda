---
okf_version: "0.2"
type: Function
title: draw_lasso_ghost
description: v0.27 — Lasso Select polygon ghost — captured vertices as cyan
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_lasso_ghost_1
language: rust
---

# draw_lasso_ghost

v0.27 — Lasso Select polygon ghost — captured vertices as cyan

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_lasso_ghost(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Lasso Select polygon ghost — captured vertices as cyan
dots + a closed-loop outline back to the live cursor.

## Source
Lines 234–267 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
