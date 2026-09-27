---
okf_version: "0.2"
type: Module
title: order
description: "Canonical draw order for a [`Scene`](crate::scene::Scene)'s primitive"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order
language: rust
---

# order

Canonical draw order for a [`Scene`](crate::scene::Scene)'s primitive

## Docstring

Canonical draw order for a [`Scene`](crate::scene::Scene)'s primitive
buckets, shared by the CPU `canvas::Frame` renderer and the GPU shader path
so the two cannot silently diverge.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

Every draw path walks a const from here, so reordering one means editing
this file — and the parity tests below fail until the sides agree:
`scene_shader::ScenePrimitive::draw` walks [`GPU_SCENE_DRAW_ORDER`],
`pcb_canvas::draw_scene` walks [`CPU_PCB_DRAW_ORDER`], and
`renderer_scene_canvas::draw_scene_with_world_to_screen` walks
[`CPU_SCHEMATIC_DRAW_ORDER`].

That last one is new in #645. Until then the schematic / Symbol Editor
replay hardcoded its own sequence, so the guarantee this module advertises
covered the PCB and quietly did not cover the surface that #199 is about
to move onto the GPU. Issue #1 (dashed lines render solid on GPU) and issue #4
(an overlay polygon drawing under base content) are fixed — `line.wgsl`
now honours the dash style bit and `scene_shader::ScenePrimitive::draw`
composites overlays in a dedicated pass after every base bucket, matching
the CPU's overlay pass; `overlays_composite_above_base_buckets` below pins
that parity. The remaining, deliberately-unreconciled divergence is the
BASE bucket order: GPU draws Polygons before Lines/Circles, CPU draws them
after — a visual-authority call reserved for Caner/Hakan, out of scope for
this fix.

## Relationships

| Type | Target |
|------|--------|
| related | [SceneBucket](/crates/oxide-gfx/src/scene/order/SceneBucket.md) |
| related | [index_of](/crates/oxide-gfx/src/scene/order/index_of.md) |
| related | [gpu_scene_draw_order_is_fills_then_strokes_then_text](/crates/oxide-gfx/src/scene/order/gpu_scene_draw_order_is_fills_then_strokes_then_text.md) |
| related | [cpu_pcb_draw_order_is_main_geometry_then_overlays](/crates/oxide-gfx/src/scene/order/cpu_pcb_draw_order_is_main_geometry_then_overlays.md) |
| related | [overlays_composite_above_base_buckets](/crates/oxide-gfx/src/scene/order/overlays_composite_above_base_buckets.md) |
| related | [cpu_schematic_draw_order_is_base_then_overlays_then_erc](/crates/oxide-gfx/src/scene/order/cpu_schematic_draw_order_is_base_then_overlays_then_erc.md) |
| related | [the_schematic_order_names_every_bucket_exactly_once](/crates/oxide-gfx/src/scene/order/the_schematic_order_names_every_bucket_exactly_once.md) |
| related | [schematic_overlays_and_erc_markers_composite_above_base_buckets](/crates/oxide-gfx/src/scene/order/schematic_overlays_and_erc_markers_composite_above_base_buckets.md) |
| related | [gpu_and_schematic_cpu_disagree_on_where_fills_go](/crates/oxide-gfx/src/scene/order/gpu_and_schematic_cpu_disagree_on_where_fills_go.md) |
| related | [gpu_draws_buckets_the_pcb_cpu_path_omits](/crates/oxide-gfx/src/scene/order/gpu_draws_buckets_the_pcb_cpu_path_omits.md) |
| related | [scene_shader_composites_no_overlay_or_erc_buckets](/crates/oxide-gfx/src/scene/order/scene_shader_composites_no_overlay_or_erc_buckets.md) |
| related | [line_dash_predicate_reads_the_low_style_bit](/crates/oxide-gfx/src/scene/order/line_dash_predicate_reads_the_low_style_bit.md) |
| related | [circle_fill_predicate_splits_on_stroke_width](/crates/oxide-gfx/src/scene/order/circle_fill_predicate_splits_on_stroke_width.md) |
| related | [polygon_stroke_predicate_needs_colour_and_width](/crates/oxide-gfx/src/scene/order/polygon_stroke_predicate_needs_colour_and_width.md) |
