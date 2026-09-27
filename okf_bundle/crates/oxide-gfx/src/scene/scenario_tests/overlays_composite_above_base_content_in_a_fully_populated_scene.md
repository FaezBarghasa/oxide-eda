---
okf_version: "0.2"
type: Function
title: overlays_composite_above_base_content_in_a_fully_populated_scene
description: "#4 (fixed for overlays), tied to real content: `scene::order`'s own"
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/overlays_composite_above_base_content_in_a_fully_populated_scene
language: rust
---

# overlays_composite_above_base_content_in_a_fully_populated_scene

#4 (fixed for overlays), tied to real content: `scene::order`'s own

## Signature

```rust
fn overlays_composite_above_base_content_in_a_fully_populated_scene()
```

## Decorators

- `test`

## Docstring

#4 (fixed for overlays), tied to real content: `scene::order`'s own
`overlays_composite_above_base_buckets` pins the abstract per-bucket
constants; this walks the SAME full-board scene used above — every bucket
actually holds geometry, including the overlay trio — and asserts the
concrete consequence now holds: an overlay meant to sit on top of copper
(the active-layer zone highlight, selection highlight, or a DRC marker)
lands there on BOTH engines. On GPU this holds structurally — no overlay
bucket is ever a member of `GPU_SCENE_DRAW_ORDER`, so the scene shader's
dedicated overlay pass runs strictly after every base bucket regardless of
scene content — and on CPU, every populated overlay bucket's position in
`CPU_PCB_DRAW_ORDER` is after every populated base bucket's. (The base
`Polygons`-vs-`Lines` order — pad/pour geometry, not overlays — remains a
separate, deliberately open question pinned exactly by `scene::order`'s
own `gpu_scene_draw_order_is_fills_then_strokes_then_text` /
`cpu_pcb_draw_order_is_main_geometry_then_overlays`, unaffected by this
fix.)
[test]

## Source
Lines 381–426 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| calls | [build_full_board_scene](/crates/oxide-gfx/src/scene/scenario_tests/build_full_board_scene.md) |
| calls | [nonempty_bucket_sequence](/crates/oxide-gfx/src/scene/scenario_tests/nonempty_bucket_sequence.md) |
