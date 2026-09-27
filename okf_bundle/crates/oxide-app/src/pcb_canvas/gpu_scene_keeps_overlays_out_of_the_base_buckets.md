---
okf_version: "0.2"
type: Function
title: gpu_scene_keeps_overlays_out_of_the_base_buckets
description: "Correctness — thread #4 (z-order): `gpu_scene()` must NOT fold overlay"
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/gpu_scene_keeps_overlays_out_of_the_base_buckets
language: rust
---

# gpu_scene_keeps_overlays_out_of_the_base_buckets

Correctness — thread #4 (z-order): `gpu_scene()` must NOT fold overlay

## Signature

```rust
fn gpu_scene_keeps_overlays_out_of_the_base_buckets()
```

## Decorators

- `test`

## Docstring

Correctness — thread #4 (z-order): `gpu_scene()` must NOT fold overlay
geometry into the base `polygons`/`lines`/`circles` buckets. A DRC
marker (an overlay-only primitive with no base-scene counterpart in
this snapshot) must stay in `overlay_*` so the shader's dedicated
overlay pass — which always composites after every base bucket — is
the only thing that draws it. If the fold regresses, this marker would
show up in `scene.polygons`/`scene.circles` instead, which is exactly
the bug that drew overlays *under* base content.
[test]

## Source
Lines 633–654 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
