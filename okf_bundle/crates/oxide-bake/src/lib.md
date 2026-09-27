---
okf_version: "0.2"
type: Module
title: lib
description: Sketch → footprint primitive bake pipeline.
resource: crates/oxide-bake/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:07:57Z"
concept_id: crates/oxide-bake/src/lib
language: rust
---

# lib

Sketch → footprint primitive bake pipeline.

## Docstring

Sketch → footprint primitive bake pipeline.

Phase 7 of the v0.13 sketch-mode plan. Lives in its own crate
(rather than inside `oxide-sketch` or `oxide-library`) so we can
depend on both without a circular dependency: `oxide-library`
depends on `oxide-sketch` for `SketchData`, and this crate
depends on both to produce `oxide-library::Pad` from
`oxide_sketch` data.

Cleanroom: derived from first principles + the Phase 4 expression
machinery. No third-party constraint-solver, footprint-generator,
or numerical-library source consulted.
