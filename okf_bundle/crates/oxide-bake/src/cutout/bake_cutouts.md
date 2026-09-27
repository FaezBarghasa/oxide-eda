---
okf_version: "0.2"
type: Function
title: bake_cutouts
resource: crates/oxide-bake/src/cutout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/cutout/bake_cutouts
language: rust
---

# bake_cutouts

## Signature

```rust
pub fn bake_cutouts(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    out: &mut Vec<FpCutout>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Source
Lines 29–94 in `crates/oxide-bake/src/cutout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cutout](/crates/oxide-bake/src/cutout.md) |
| calls | [build_ctx](/crates/oxide-bake/src/cutout/build_ctx.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| calls | [opt_eval_mm](/crates/oxide-bake/src/cutout/opt_eval_mm.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_cutout_edge_radius_evaluated](/crates/oxide-bake/src/cutout/bake_cutout_edge_radius_evaluated.md) |
| called_by | [bake_cutout_partial_depth_baked](/crates/oxide-bake/src/cutout/bake_cutout_partial_depth_baked.md) |
| called_by | [bake_cutout_simple](/crates/oxide-bake/src/cutout/bake_cutout_simple.md) |
