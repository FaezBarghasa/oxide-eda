---
okf_version: "0.2"
type: Function
title: bake_arrays
description: "Walk every [`oxide_sketch::array::Array`] and append baked pads"
resource: crates/oxide-bake/src/array/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/mod/bake_arrays
language: rust
---

# bake_arrays

Walk every [`oxide_sketch::array::Array`] and append baked pads

## Signature

```rust
pub fn bake_arrays(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    out: &mut Vec<LibPad>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Walk every [`oxide_sketch::array::Array`] and append baked pads
to `out`. Bakes `ArrayKind::Linear`, `Grid`, and `Polar` natively.

## Source
Lines 49–125 in `crates/oxide-bake/src/array/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-bake/src/array/mod.md) |
| calls | [bake_linear](/crates/oxide-bake/src/array/linear/bake_linear.md) |
| calls | [bake_grid](/crates/oxide-bake/src/array/grid/bake_grid.md) |
| calls | [bake_polar](/crates/oxide-bake/src/array/polar/bake_polar.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_grid_array_with_suppressed_instances_skips_selected_cells](/crates/oxide-bake/tests/bake_pads/bake_grid_array_with_suppressed_instances_skips_selected_cells.md) |
| called_by | [bake_linear_array_3_pads_along_x](/crates/oxide-bake/tests/bake_pads/bake_linear_array_3_pads_along_x.md) |
| called_by | [bake_polar_array_with_suppressed_instances_skips_selected_indices](/crates/oxide-bake/tests/bake_pads/bake_polar_array_with_suppressed_instances_skips_selected_indices.md) |
