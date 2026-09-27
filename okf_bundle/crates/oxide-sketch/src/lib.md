---
okf_version: "0.2"
type: Module
title: lib
description: Oxide 2D parametric sketcher.
resource: crates/oxide-sketch/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/lib
language: rust
---

# lib

Oxide 2D parametric sketcher.

## Docstring

Oxide 2D parametric sketcher.

Cleanroom implementation against
[`SKETCH_MODE_PLAN.md`] (`docs/internal/`). No third-party
constraint-solver source consulted; algorithm sourced from
Hearn & Baker §10–§12 and Numerical Recipes (Press et al.) §15.

Public surface:
- [`SketchData`] — top-level sketch container
- [`Solver`] — Newton-LM constraint solver
- [`bake`] — sketch → footprint primitive bake

See `docs/internal/SKETCH_MODE_v0.13_PLAN.md` for the
release plan.
