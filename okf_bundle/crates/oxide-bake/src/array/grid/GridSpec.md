---
okf_version: "0.2"
type: Class
title: GridSpec
description: "The grid array's declarative shape: the `(nx, ny)` count expressions"
resource: crates/oxide-bake/src/array/grid.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/grid/GridSpec
language: rust
---

# GridSpec

The grid array's declarative shape: the `(nx, ny)` count expressions

## Signature

```rust
pub(super) struct GridSpec
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

The grid array's declarative shape: the `(nx, ny)` count expressions
stepped by `(dx, dy)` per axis, its depopulation predicate/list, and
its numbering scheme. Built once per `ArrayKind::Grid` in
`array::bake_arrays` and passed by reference into [`bake_grid`] —
none of these fields are mutated during baking.

## Methods

- `nx_expr`
- `ny_expr`
- `dx_expr`
- `dy_expr`
- `depopulation`
- `numbering`

## Source
Lines 28–35 in `crates/oxide-bake/src/array/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-bake/src/array/grid.md) |
