---
okf_version: "0.2"
type: Class
title: SceneSurface
description: "One editor surface that draws its `Scene` on the GPU."
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/SceneSurface
language: rust
---

# SceneSurface

One editor surface that draws its `Scene` on the GPU.

## Signature

```rust
pub trait SceneSurface
```

## Visibility

- `pub`

## Docstring

One editor surface that draws its `Scene` on the GPU.

This exists purely to give each surface its own pipeline slot. iced stores
a primitive's pipeline in a map keyed by the **primitive's** `TypeId`
(`iced_wgpu::primitive::Storage::store::<P, _>`, called from
`BlackBox::<P>::prepare`), and that map lives in an
`Arc<RwLock<primitive::Storage>>` on a cloned `Engine` — one map for the
whole process, not one per window. So every widget emitting the same
primitive type shares one instance-buffer set, one camera, and one
`uploaded_generation`.

Sharing does not corrupt a frame on its own: `Renderer::draw` runs
`prepare` then `render` over one window's layers and `present` submits that
encoder (`iced_wgpu::lib.rs`), so windows do not interleave — each uploads
its own geometry immediately before drawing it.

What sharing *does* break is the generation cache. `prepare` skips the
upload when this frame's generation equals the resident one, and two
sources counting independently collide trivially — two freshly-opened
documents are both at generation 0. The second source then skips its upload
and draws the first's geometry.

Making [`ScenePrimitive`] generic over this trait means
`ScenePrimitive<PcbSurface>` and a future `ScenePrimitive<SchematicSurface>`
are distinct types, so each surface gets its own pipeline slot — its own
buffers, camera and generation counter — and cannot collide with another
surface.

**To add a surface:** declare a marker here, implement this trait, and mount
`SceneShaderProgram::<YourSurface>::new(...)`. Do not reach for a second
copy of this module — that is what #625 removed.

**Known gap.** The type-level split separates *surfaces*, not *instances of
one surface*. Two undocked schematic windows would both emit
`ScenePrimitive<SchematicSurface>`, share one slot, and collide on the
generation cache exactly as above. The fix is a discriminator in the
resident key — the primitive carrying an instance id (window or document)
alongside its generation, and the pipeline storing both — not a split of
the `oxide_gfx` pipelines. Unreachable today: the PCB is the only mounted
surface and it has a single `PcbCanvas`, not one per window. Must be solved
before the schematic mounts (#199), which is per-window.

## Source
Lines 87–109 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
