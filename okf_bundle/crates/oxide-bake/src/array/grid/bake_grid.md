---
okf_version: "0.2"
type: Function
title: bake_grid
resource: crates/oxide-bake/src/array/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/grid/bake_grid
language: rust
---

# bake_grid

## Signature

```rust
pub(super) fn bake_grid(
    source: SketchEntityId,
    spec: &GridSpec,
    params_ast: &BTreeMap<String, ExprNode>,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<LibPad>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub(super)`

## Source
Lines 37–174 in `crates/oxide-bake/src/array/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-bake/src/array/grid.md) |
| calls | [derive_pad_number_2d](/crates/oxide-bake/src/array/numbering/derive_pad_number_2d.md) |
| calls | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| called_by | [bake_arrays](/crates/oxide-bake/src/array/mod/bake_arrays.md) |
