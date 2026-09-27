---
okf_version: "0.2"
type: Function
title: scene_generation
description: "Generation id of the scene [`Self::gpu_scene`] currently returns. Passed"
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/scene_generation
language: rust
---

# scene_generation

Generation id of the scene [`Self::gpu_scene`] currently returns. Passed

## Signature

```rust
impl PcbCanvas { pub fn scene_generation(&self) -> u64 }
```

## Visibility

- `pub`

## Docstring

Generation id of the scene [`Self::gpu_scene`] currently returns. Passed
to the shader so the GPU pipeline can skip re-uploading unchanged
geometry on pan/zoom. Read in `view()` right after `gpu_scene()`, so it
reflects the same geometry that call returned (a rebuild bumps this at
its invalidation site, never mid-`gpu_scene`).

## Source
Lines 187–189 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
