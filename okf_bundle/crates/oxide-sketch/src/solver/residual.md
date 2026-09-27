---
okf_version: "0.2"
type: Module
title: residual
description: Constraint residual functions.
resource: crates/oxide-sketch/src/solver/residual.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residual
language: rust
---

# residual

Constraint residual functions.

## Docstring

Constraint residual functions.

Each [`ConstraintKind`] variant maps to a residual `f(state) -> R^k`
where `k = ConstraintKind::residual_count()`. The solver drives every
component of the concatenated residual vector toward zero via
Levenberg–Marquardt iteration (Phase 3).

The dispatcher in [`residual`] is thin — it routes to per-family
helpers in [`crate::solver::residuals`]. Each family is in its own
module so the residual implementations can grow without this file
becoming a single shared bottleneck.

## Relationships

| Type | Target |
|------|--------|
| related | [resolve_dim](/crates/oxide-sketch/src/solver/residual/resolve_dim.md) |
| related | [total_residual](/crates/oxide-sketch/src/solver/residual/total_residual.md) |
| related | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
