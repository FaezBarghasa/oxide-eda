---
okf_version: "0.2"
type: Module
title: scene_shader
description: "Generic GPU render path for any `oxide_gfx::scene::Scene`."
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader
language: rust
---

# scene_shader

Generic GPU render path for any `oxide_gfx::scene::Scene`.

## Docstring

Generic GPU render path for any `oxide_gfx::scene::Scene`.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: iced/wgpu public docs, IPC-2612-1, IEEE 315, IEC 60617.

Bridges the `oxide_gfx` render pipelines into iced's `shader` widget so a
`Scene` draws on the GPU instead of being tessellated into a
`canvas::Frame` on the CPU. The pipelines are primitive-agnostic — they are
driven purely by a `Scene` plus a screen-space pan/zoom transform — so every
editor surface can share this one renderer.

Provenance: the PCB path landed in #308. A second, near-identical copy of
this module (`schematic_shader`, from #169 PR 2) sat unmounted beside it
until #625 folded it in here; the two had already drifted apart on upload
skipping, overlay compositing, draw order and error reporting.

Mounted by: [`crate::pcb_canvas`] (the PCB editor), via `app/view/mod.rs`.
The schematic and Symbol Editor surfaces are not mounted yet — see #199
and #642.

iced's shader `Primitive::draw` composites into the shared render pass over
whatever was already drawn behind the widget (it never clears its own
region), so the caller is responsible for painting the background + grid on
a layer *below* this shader in a `stack!`.

## Relationships

| Type | Target |
|------|--------|
| related | [SceneSurface](/crates/oxide-app/src/scene_shader/SceneSurface.md) |
| related | [PcbSurface](/crates/oxide-app/src/scene_shader/PcbSurface.md) |
| related | [world_origin_mm](/crates/oxide-app/src/scene_shader/world_origin_mm.md) |
| related | [log_text_error_once](/crates/oxide-app/src/scene_shader/log_text_error_once.md) |
| related | [ScenePipeline](/crates/oxide-app/src/scene_shader/ScenePipeline.md) |
| related | [new](/crates/oxide-app/src/scene_shader/new.md) |
| related | [trim](/crates/oxide-app/src/scene_shader/trim.md) |
| related | [new](/crates/oxide-app/src/scene_shader/new.md) |
| related | [trim](/crates/oxide-app/src/scene_shader/trim.md) |
| related | [ScenePrimitive](/crates/oxide-app/src/scene_shader/ScenePrimitive.md) |
| related | [prepare](/crates/oxide-app/src/scene_shader/prepare.md) |
| related | [draw](/crates/oxide-app/src/scene_shader/draw.md) |
| related | [prepare](/crates/oxide-app/src/scene_shader/prepare.md) |
| related | [draw](/crates/oxide-app/src/scene_shader/draw.md) |
| related | [SceneShaderProgram](/crates/oxide-app/src/scene_shader/SceneShaderProgram.md) |
| related | [new](/crates/oxide-app/src/scene_shader/new.md) |
| related | [new](/crates/oxide-app/src/scene_shader/new.md) |
| related | [draw](/crates/oxide-app/src/scene_shader/draw.md) |
| related | [draw](/crates/oxide-app/src/scene_shader/draw.md) |
| related | [pipeline_impl_block](/crates/oxide-app/src/scene_shader/pipeline_impl_block.md) |
| related | [the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover](/crates/oxide-app/src/scene_shader/the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover.md) |
| related | [world_origin_is_negative_offset_over_scale](/crates/oxide-app/src/scene_shader/world_origin_is_negative_offset_over_scale.md) |
| related | [world_origin_handles_degenerate_scale](/crates/oxide-app/src/scene_shader/world_origin_handles_degenerate_scale.md) |
| related | [world_origin_matches_the_screen_transform_mapping](/crates/oxide-app/src/scene_shader/world_origin_matches_the_screen_transform_mapping.md) |
| related | [primitive_carries_the_scene_and_camera](/crates/oxide-app/src/scene_shader/primitive_carries_the_scene_and_camera.md) |
| related | [each_surface_gets_its_own_primitive_type](/crates/oxide-app/src/scene_shader/each_surface_gets_its_own_primitive_type.md) |
| related | [OtherSurface](/crates/oxide-app/src/scene_shader/OtherSurface.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
