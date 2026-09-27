---
okf_version: "0.2"
type: Function
title: overlays_composite_above_base_buckets
description: "#4 (fixed): both paths composite overlay geometry strictly AFTER every"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/overlays_composite_above_base_buckets
language: rust
---

# overlays_composite_above_base_buckets

#4 (fixed): both paths composite overlay geometry strictly AFTER every

## Signature

```rust
fn overlays_composite_above_base_buckets()
```

## Decorators

- `test`

## Docstring

#4 (fixed): both paths composite overlay geometry strictly AFTER every
base bucket, so a DRC marker / selection highlight / active-layer zone
highlight always renders on top of ordinary geometry regardless of
backend. On GPU this holds structurally — no overlay bucket appears in
[`GPU_SCENE_DRAW_ORDER`] at all; `scene_shader::ScenePrimitive::draw`
composites the overlay trio in its own pass strictly after this order
(see the module doc and `pcb_canvas::gpu_scene_keeps_overlays_out_of_the_base_buckets`).
On CPU, [`CPU_PCB_DRAW_ORDER`] lists every overlay bucket after every
base bucket. Before the fix, `pcb_canvas::gpu_scene` folded overlay
geometry into the base buckets upstream of this order, so a GPU
overlay polygon landed in the first-drawn base `Polygons` bucket
instead of on top — this test pins that it cannot regress.
[test]

## Source
Lines 155–189 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
| calls | [index_of](/crates/oxide-gfx/src/scene/order/index_of.md) |
