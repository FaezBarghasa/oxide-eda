---
okf_version: "0.2"
type: Module
title: array
description: "Sketch-array expansion — bakes all three `ArrayKind` variants."
resource: crates/oxide-bake/src/array/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/array/mod
language: rust
---

# array

Sketch-array expansion — bakes all three `ArrayKind` variants.

## Docstring

Sketch-array expansion — bakes all three `ArrayKind` variants.

Phase 7 Task 7.2 of the v0.13 sketch-mode plan. Walks every
`Array` in the sketch and produces baked
[`oxide_library::primitive::footprint::Pad`]s by re-using the
per-pad bake body from `crate::pad::bake_one_pad`.

Cleanroom: derived from first principles + the Phase 4 expression
machinery. No third-party constraint-solver, footprint-generator,
or numerical-library source consulted.

# Scope

- `ArrayKind::Linear { source, count_expr, dx_expr, dy_expr }` —
bakes natively (`array::linear`).
- `ArrayKind::Grid { .. }` — bakes natively (`array::grid`).
- `ArrayKind::Polar { .. }` — bakes natively (`array::polar`).

Numbering:
- `LinearIncrement { start, step }` — pad number =
`start + i * step` rounded to integer.
- `BgaRowCol { .. }` on a 1D Linear — warns (BGA on Linear is not
semantically meaningful) and falls back to a default 1-based
`LinearIncrement`.
- `Explicit { names }` — uses `names[i]` if present; warns and
falls back to `format!("{i}")` otherwise.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_arrays](/crates/oxide-bake/src/array/mod/bake_arrays.md) |
