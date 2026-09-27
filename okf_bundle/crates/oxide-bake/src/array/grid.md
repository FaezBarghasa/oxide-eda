---
okf_version: "0.2"
type: Module
title: grid
description: "Grid (2D) array baking — `nx` × `ny` instances stepped by"
resource: crates/oxide-bake/src/array/grid.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/grid
language: rust
---

# grid

Grid (2D) array baking — `nx` × `ny` instances stepped by

## Docstring

Grid (2D) array baking — `nx` × `ny` instances stepped by
(`dx_expr`, `dy_expr`) per axis, with optional per-cell
depopulation (a mask predicate and/or an explicit suppressed-cell
list).

## Relationships

| Type | Target |
|------|--------|
| related | [GridSpec](/crates/oxide-bake/src/array/grid/GridSpec.md) |
| related | [bake_grid](/crates/oxide-bake/src/array/grid/bake_grid.md) |
