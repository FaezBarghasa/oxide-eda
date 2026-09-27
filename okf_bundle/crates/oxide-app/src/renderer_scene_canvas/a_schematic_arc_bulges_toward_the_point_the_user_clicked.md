---
okf_version: "0.2"
type: Function
title: a_schematic_arc_bulges_toward_the_point_the_user_clicked
description: "The bug this fixes, stated as the property that was violated."
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/a_schematic_arc_bulges_toward_the_point_the_user_clicked
language: rust
---

# a_schematic_arc_bulges_toward_the_point_the_user_clicked

The bug this fixes, stated as the property that was violated.

## Signature

```rust
fn a_schematic_arc_bulges_toward_the_point_the_user_clicked()
```

## Decorators

- `test`

## Docstring

The bug this fixes, stated as the property that was violated.

A `SchDrawing::Arc` is authored as three clicked points, and the stored
angle pair is chosen so the CCW-wrapped span from `start` to `end`
contains the middle one (`schematic_runtime::arc_sweeps_through_mid`).
The schematic's world→screen map does not flip Y, so it preserves
angles — meaning the arc drawn on screen must bulge toward the same
side as the world midpoint. Under the old unconditional negation it
bulged to the opposite side.
[test]

## Source
Lines 457–470 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [span](/crates/oxide-app/src/renderer_scene_canvas/span.md) |
