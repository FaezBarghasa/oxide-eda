---
okf_version: "0.2"
type: Function
title: bake_body3d
resource: crates/oxide-bake/src/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/body3d/bake_body3d
language: rust
---

# bake_body3d

## Signature

```rust
pub fn bake_body3d(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    body_3d: &mut Body3D,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Source
Lines 46–121 in `crates/oxide-bake/src/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-bake/src/body3d.md) |
| calls | [find_seed_on_plane](/crates/oxide-bake/src/body3d/find_seed_on_plane.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| calls | [build_ctx](/crates/oxide-bake/src/body3d/build_ctx.md) |
| calls | [eval_mm](/crates/oxide-bake/src/body3d/eval_mm.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_body3d_no_body_top_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_body_top_plane_is_noop.md) |
| called_by | [bake_body3d_no_edges_on_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_edges_on_plane_is_noop.md) |
| called_by | [bake_body3d_offset_z_eval_failure_keeps_prior](/crates/oxide-bake/src/body3d/bake_body3d_offset_z_eval_failure_keeps_prior.md) |
| called_by | [bake_body3d_rectangle_outline](/crates/oxide-bake/src/body3d/bake_body3d_rectangle_outline.md) |
