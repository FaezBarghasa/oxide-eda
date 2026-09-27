---
okf_version: "0.2"
type: Function
title: bake_one_pad
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/bake_one_pad
language: rust
---

# bake_one_pad

## Signature

```rust
pub(crate) fn bake_one_pad(
    sketch_point_id: SketchEntityId,
    pad_attr: &PadAttr,
    params_ast: &BTreeMap<String, ExprNode>,
    array_index: Option<(usize, usize)>,
    extra_dx: f64,
    extra_dy: f64,
    extra_pad_number: Option<String>,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    warnings: &mut Vec<String>,
) -> Result<LibPad, SketchError>
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "10 arguments: the full pad geometry is passed positionally rather than boxed into a struct the callers do not otherwise need"
)`

## Visibility

- `pub(crate)`

## Source
Lines 122–273 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [opt_eval_mm](/crates/oxide-bake/src/pad/opt_eval_mm.md) |
| calls | [eval_mm](/crates/oxide-bake/src/pad/eval_mm.md) |
| calls | [rotation_deg](/crates/oxide-bake/src/pad/rotation_deg.md) |
| calls | [bake_shape](/crates/oxide-bake/src/pad/bake_shape.md) |
| calls | [fiducial_layers](/crates/oxide-bake/src/pad/fiducial_layers.md) |
| calls | [derive_layers](/crates/oxide-bake/src/pad/derive_layers.md) |
| calls | [lib_kind](/crates/oxide-bake/src/pad/lib_kind.md) |
| called_by | [bake_grid](/crates/oxide-bake/src/array/grid/bake_grid.md) |
| called_by | [bake_linear](/crates/oxide-bake/src/array/linear/bake_linear.md) |
| called_by | [bake_polar](/crates/oxide-bake/src/array/polar/bake_polar.md) |
| called_by | [bake_pads](/crates/oxide-bake/src/pad/bake_pads.md) |
