---
okf_version: "0.2"
type: Function
title: world_is_y_up
description: Does this world→screen map flip Y?
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/world_is_y_up
language: rust
---

# world_is_y_up

Does this world→screen map flip Y?

## Signature

```rust
fn world_is_y_up(world_to_screen: F) -> bool
```

## Type Parameters

- `F`

## Docstring

Does this world→screen map flip Y?

Probed rather than declared. Only arcs care, and they care absolutely: a
flipping map reflects the plane, so a world angle becomes its negation on
screen and a positive world sweep runs backwards. A non-flipping map is a
positive similarity and preserves both.

The two surfaces sharing this replay disagree — the Symbol Editor maps
`oy - point[1] * scale` and the schematic maps `y * scale + offset_y` —
which is exactly why this is derived from the mapping the caller already
passes in rather than taken as a flag beside it. A flag can be set to
contradict the transform; that is the bug this fixes (#645), and it went
unnoticed for months. A probe cannot disagree with the function it probes.

## Source
Lines 38–47 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| called_by | [arc_screen_span_for](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span_for.md) |
| called_by | [draw_arc_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket.md) |
