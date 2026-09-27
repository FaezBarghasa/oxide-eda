---
okf_version: "0.2"
type: Function
title: new
description: "Build from an already-tessellated `Scene` and the current screen-space"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/new_3
language: rust
---

# new

Build from an already-tessellated `Scene` and the current screen-space

## Signature

```rust
pub fn new(
        scene: Arc<Scene>,
        generation: Option<u64>,
        offset_px: [f32; 2],
        scale_px_per_mm: f32,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Build from an already-tessellated `Scene` and the current screen-space
transform (`offset_px` = pan in logical pixels, `scale_px_per_mm` =
zoom in logical pixels per millimetre). `generation` identifies the
geometry so the pipeline can skip redundant GPU uploads on pan/zoom:
`Some(g)` from a cached source that bumps `g` only on real geometry
changes, or `None` for an uncached source that uploads every frame.

## Source
Lines 437–450 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
