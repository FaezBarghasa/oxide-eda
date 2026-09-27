---
okf_version: "0.2"
type: Module
title: body3d
description: "3D-extrude bake — closed profile on a `PlaneKind::BodyTop` plane"
resource: crates/oxide-bake/src/body3d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/body3d
language: rust
---

# body3d

3D-extrude bake — closed profile on a `PlaneKind::BodyTop` plane

## Docstring

3D-extrude bake — closed profile on a `PlaneKind::BodyTop` plane
becomes the `body_3d.outline` polygon (driving the procedural 3D
render with the actual body shape rather than the default fab
outline convex hull).

Phase B / Stage 5 of the v0.14.1 sketch-mode plan. Per the v3
library schema:
```text
struct Body3D {
shape: BodyShape,           // Extrude / Dome / Cylinder / ...
height_mm: f32,
offset_z_mm: f32,           // <-- bake sets from plane's offset_z_expr
outline: Option<Polygon>,   // <-- bake sets from closed profile
...
}
```

v0.14.1 scope:
- Find the first `PlaneKind::BodyTop` plane.
- Find the first non-construction Line / Arc on that plane.
- Trace a closed profile through the walker.
- Set `body_3d.outline = Some(Polygon)` and
`body_3d.offset_z_mm = eval(plane.offset_z_expr)`.
- `height_mm` stays at whatever the caller had pre-set (Body3D
defaults to 1.0 mm; user-edited values are preserved).
- Multiple BodyTop planes / multiple closed profiles per plane:
first wins; subsequent emit warnings.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_body3d](/crates/oxide-bake/src/body3d/bake_body3d.md) |
| related | [find_seed_on_plane](/crates/oxide-bake/src/body3d/find_seed_on_plane.md) |
| related | [build_ctx](/crates/oxide-bake/src/body3d/build_ctx.md) |
| related | [eval_mm](/crates/oxide-bake/src/body3d/eval_mm.md) |
| related | [solve](/crates/oxide-bake/src/body3d/solve.md) |
| related | [sketch_with_body_top_rectangle](/crates/oxide-bake/src/body3d/sketch_with_body_top_rectangle.md) |
| related | [bake_body3d_no_body_top_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_body_top_plane_is_noop.md) |
| related | [bake_body3d_rectangle_outline](/crates/oxide-bake/src/body3d/bake_body3d_rectangle_outline.md) |
| related | [bake_body3d_offset_z_eval_failure_keeps_prior](/crates/oxide-bake/src/body3d/bake_body3d_offset_z_eval_failure_keeps_prior.md) |
| related | [bake_body3d_no_edges_on_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_edges_on_plane_is_noop.md) |
