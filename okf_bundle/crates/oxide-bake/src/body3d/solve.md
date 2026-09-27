---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-bake/src/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/body3d/solve
language: rust
---

# solve

## Signature

```rust
fn solve(sketch: &SketchData) -> FullSolveOutput
```

## Source
Lines 164–168 in `crates/oxide-bake/src/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-bake/src/body3d.md) |
| called_by | [bake_body3d_no_body_top_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_body_top_plane_is_noop.md) |
| called_by | [bake_body3d_no_edges_on_plane_is_noop](/crates/oxide-bake/src/body3d/bake_body3d_no_edges_on_plane_is_noop.md) |
| called_by | [bake_body3d_offset_z_eval_failure_keeps_prior](/crates/oxide-bake/src/body3d/bake_body3d_offset_z_eval_failure_keeps_prior.md) |
| called_by | [bake_body3d_rectangle_outline](/crates/oxide-bake/src/body3d/bake_body3d_rectangle_outline.md) |
