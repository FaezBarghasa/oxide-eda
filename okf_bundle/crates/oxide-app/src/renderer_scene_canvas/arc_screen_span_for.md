---
okf_version: "0.2"
type: Function
title: arc_screen_span_for
description: "Map an arc's world angles onto the screen angles lyon will sweep between."
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/arc_screen_span_for
language: rust
---

# arc_screen_span_for

Map an arc's world angles onto the screen angles lyon will sweep between.

## Signature

```rust
pub fn arc_screen_span_for(start_angle: f32, end_angle: f32, world_to_screen: F) -> ArcScreenSpan
```

## Type Parameters

- `F`

## Visibility

- `pub`

## Docstring

Map an arc's world angles onto the screen angles lyon will sweep between.

Arc angles are world-space radians measured from +X, and the sweep is
always this codebase's CCW-wraparound rule — `(end - start).rem_euclid(TAU)`
(`oxide_gfx::primitive::arc::ccw_wrapped_sweep_rad`, the same rule
`arc.wgsl` and the symbol hit-test use) — never the signed difference.
lyon's `builder.arc` does not know that convention: it draws
`end_angle - start_angle` as a raw signed sweep, so the end angle handed to
it has to be derived from the wrapped sweep rather than passed through.

`world_is_y_up` decides the sign, and getting it wrong mirrors the arc
about the horizontal line through its own centre — right radius, right
sweep magnitude, wrong bulge direction, endpoints detached from the points
the user clicked.

This used to negate unconditionally. The negation arrived in `19a3d4a1`
("Negate arc angles in scene canvas renderer for screen-space y-flip") on
the stated premise that "Arc primitives store world-space radians (y-up
convention)" — true of the Symbol Editor, whose map flips Y, and false of
the schematic, whose map does not. Both share this replay, so the fix for
one silently mirrored the other (#645).
[`arc_screen_span`] with the handedness probed from the caller's own
world→screen map, the way `draw_arc_bucket` does it.

For anything drawing an arc outside the scene replay — the in-progress
placement preview — so that a preview and the arc it is about to commit
cannot disagree about which way the curve bulges.

## Source
Lines 89–94 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [arc_screen_span](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span.md) |
| calls | [world_is_y_up](/crates/oxide-app/src/renderer_scene_canvas/world_is_y_up.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/canvas/draw/previews/draw_arc_preview.md) |
