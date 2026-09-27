---
okf_version: "0.2"
type: Function
title: live_camera
description: "Current pan/zoom as `(offset_x_px, offset_y_px, scale_px_per_mm)`,"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/live_camera_1
language: rust
---

# live_camera

Current pan/zoom as `(offset_x_px, offset_y_px, scale_px_per_mm)`,

## Signature

```rust
pub fn live_camera(&self) -> (f32, f32, f32)
```

## Visibility

- `pub`

## Docstring

Current pan/zoom as `(offset_x_px, offset_y_px, scale_px_per_mm)`,
read in `view()` from the single [`Self::camera`] home to place
world-anchored overlays (the inline text editor, measurements).
Same shape as [`crate::pcb_canvas::PcbCanvas::live_camera`].

## Source
Lines 232–235 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
