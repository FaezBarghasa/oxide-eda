---
okf_version: "0.2"
type: Function
title: draw_rubber_band
description: v0.26-I — rubber-band selection rectangle. Drawn only when both
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_rubber_band_1
language: rust
---

# draw_rubber_band

v0.26-I — rubber-band selection rectangle. Drawn only when both

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_rubber_band(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.26-I — rubber-band selection rectangle. Drawn only when both
anchor + current are set and at least the drag threshold apart.

## Source
Lines 271–298 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
