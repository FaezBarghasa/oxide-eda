---
okf_version: "0.2"
type: Function
title: gpu_scene
description: "Build the scene for GPU rendering: the same geometry as the CPU path."
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/gpu_scene_1
language: rust
---

# gpu_scene

Build the scene for GPU rendering: the same geometry as the CPU path.

## Signature

```rust
pub fn gpu_scene(&self) -> Option<Arc<Scene>>
```

## Visibility

- `pub`

## Docstring

Build the scene for GPU rendering: the same geometry as the CPU path.
Overlay primitives (active-layer zone highlight, selection highlight)
stay in their own `Scene::overlay_*` fields rather than folding into
the base buckets — [`crate::scene_shader::ScenePrimitive::draw`]
composites them in a dedicated pass *after* every base bucket, so they
always render on top, matching the CPU `draw_scene` overlay pass.
Returns `None` when no board snapshot is loaded.

## Source
Lines 167–180 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
