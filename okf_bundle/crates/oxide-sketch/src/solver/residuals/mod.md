---
okf_version: "0.2"
type: Module
title: residuals
description: Per-family residual implementations.
resource: crates/oxide-sketch/src/solver/residuals/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/mod
language: rust
---

# residuals

Per-family residual implementations.

## Docstring

Per-family residual implementations.

Each module owns one family of constraint kinds and exposes
`pub fn <name>(...) -> Result<Vec<f64>, SketchError>` functions
invoked by the dispatcher in [`crate::solver::residual`].
