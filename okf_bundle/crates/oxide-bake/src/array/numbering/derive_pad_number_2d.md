---
okf_version: "0.2"
type: Function
title: derive_pad_number_2d
description: "v0.22 Phase B3 — bake `ArrayKind::Grid`. Walks `(i, j)` with"
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/derive_pad_number_2d
language: rust
---

# derive_pad_number_2d

v0.22 Phase B3 — bake `ArrayKind::Grid`. Walks `(i, j)` with

## Signature

```rust
pub(super) fn derive_pad_number_2d(
    numbering: &NumberingScheme,
    i: usize,
    j: usize,
    nx: usize,
    params_ast: &BTreeMap<String, ExprNode>,
    warnings: &mut Vec<String>,
    source: SketchEntityId,
) -> String
```

## Visibility

- `pub(super)`

## Docstring

v0.22 Phase B3 — bake `ArrayKind::Grid`. Walks `(i, j)` with
`i in 0..nx` and `j in 0..ny`, per-instance offset
`(i * dx, j * dy)` from the source. Optional `depopulation` is a
boolean expression evaluated per cell — `false` skips the cell
without breaking the parametric chain.
2D companion to `derive_pad_number` — `BgaRowCol` is meaningful
here, falling back to a row-major linear count for `LinearIncrement`
and looking up `names[j*nx + i]` for `Explicit`.

## Source
Lines 129–180 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| calls | [linear_increment_number](/crates/oxide-bake/src/array/numbering/linear_increment_number.md) |
| calls | [record_numbering_warning](/crates/oxide-bake/src/array/numbering/record_numbering_warning.md) |
| calls | [bga_row_letter](/crates/oxide-sketch/src/array/bga_row_letter.md) |
| called_by | [bake_grid](/crates/oxide-bake/src/array/grid/bake_grid.md) |
| called_by | [grid_linear_increment_expression_error_is_reported](/crates/oxide-bake/src/array/numbering/grid_linear_increment_expression_error_is_reported.md) |
