---
okf_version: "0.2"
type: Function
title: bake_polar
resource: crates/oxide-bake/src/array/polar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/polar/bake_polar
language: rust
---

# bake_polar

## Signature

```rust
pub(super) fn bake_polar(
    source: SketchEntityId,
    center: SketchEntityId,
    count_expr: &str,
    sweep_angle_expr: &str,
    depopulation: Option<&oxide_sketch::array::GridDepopulation>,
    numbering: &NumberingScheme,
    params_ast: &BTreeMap<String, ExprNode>,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<LibPad>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "11 arguments: polar-array placement takes the whole geometry + numbering spec at once"
)`

## Visibility

- `pub(super)`

## Source
Lines 34–198 in `crates/oxide-bake/src/array/polar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polar](/crates/oxide-bake/src/array/polar.md) |
| calls | [derive_pad_number](/crates/oxide-bake/src/array/numbering/derive_pad_number.md) |
| calls | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| called_by | [bake_arrays](/crates/oxide-bake/src/array/mod/bake_arrays.md) |
