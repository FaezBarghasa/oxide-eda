---
okf_version: "0.2"
type: Function
title: with_min_feature_px
description: "Keep thin geometry legible when zoomed out, the way the CPU replays do."
resource: crates/oxide-gfx/src/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/camera/with_min_feature_px
language: rust
---

# with_min_feature_px

Keep thin geometry legible when zoomed out, the way the CPU replays do.

## Signature

```rust
impl CameraUniform { pub fn with_min_feature_px(mut self, min_stroke_px: f32, min_radius_px: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Keep thin geometry legible when zoomed out, the way the CPU replays do.

Both CPU paths clamp in screen space — the schematic to 0.6 px and the
PCB to 0.5 px for strokes, both to 0.5 px for circle radii — so
world-space geometry never fades below a readable width. The GPU had no
equivalent, which is why thin traces and junction dots thinned toward
nothing as you zoomed out on the shader path (#645).

The floors travel in the uniform rather than as shader constants
because they differ per surface, and because `mm_per_px` is already the
conversion the shaders need: a floor of `p` pixels is `p * mm_per_px`
world millimetres at the current zoom.

Does **not** reach polygon outlines. Those are expanded into triangles
at upload time (`pipeline::polygon::append_stroke`), before any camera
exists, so the shader never sees a stroke width to clamp. Giving them a
floor means either handing the scale to `upload` — which would make the
scene camera-dependent, the very property the GPU path exists to avoid
— or moving stroke expansion into the shader. That is a design call,
not an oversight; recorded on #645.
[must_use]

## Source
Lines 76–80 in `crates/oxide-gfx/src/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-gfx/src/camera.md) |
