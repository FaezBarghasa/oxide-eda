---
okf_version: "0.2"
type: Function
title: apply_initial_fit
description: First-draw camera placement.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_initial_fit
language: rust
---

# apply_initial_fit

First-draw camera placement.

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn apply_initial_fit(
        &self,
        cstate: &mut FootprintCanvasState,
        bounds: Rectangle,
    ) }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

First-draw camera placement.
- With content: fit-to-bounds so every pad / sketch entity
is visible.
- Without content (fresh `.snxfpt` from "Add New ▸
Footprint"): centre world origin in the viewport so the
user lands on (0, 0) rather than the screen's top-left.
Without this, the default offset (0, 0) renders world
(0, 0) at screen pixel (0, 0) — the user's drawing area
starts in the corner and they have to pan to find the
centre.

## Source
Lines 27–41 in `crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera.md) |
