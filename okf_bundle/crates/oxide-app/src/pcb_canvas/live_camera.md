---
okf_version: "0.2"
type: Function
title: live_camera
description: Current pan/zoom for the GPU path as
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/live_camera
language: rust
---

# live_camera

Current pan/zoom for the GPU path as

## Signature

```rust
impl PcbCanvas { pub fn live_camera(&self) -> (f32, f32, f32) }
```

## Visibility

- `pub`

## Docstring

Current pan/zoom for the GPU path as
`(offset_x_px, offset_y_px, scale_px_per_mm)`, read in `view()` from the
single [`Self::camera`] home to build the shader program. Reads the same
cell the CPU `draw` uses, so the GPU content stays frame-coherent with
the CPU background + grid.

## Source
Lines 140–143 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
