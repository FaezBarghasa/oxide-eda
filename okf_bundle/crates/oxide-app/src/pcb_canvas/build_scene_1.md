---
okf_version: "0.2"
type: Function
title: build_scene
description: "Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU"
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/build_scene_1
language: rust
---

# build_scene

Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU

## Signature

```rust
fn build_scene(&self, snapshot: &PcbSnapshot) -> Scene
```

## Docstring

Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU
`draw` path and the GPU [`Self::gpu_scene`] path so both tessellate from
identical instance data.

## Source
Lines 148–158 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
