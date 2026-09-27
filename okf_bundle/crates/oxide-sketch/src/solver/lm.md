---
okf_version: "0.2"
type: Module
title: lm
description: Levenberg–Marquardt iteration for the constraint solver.
resource: crates/oxide-sketch/src/solver/lm.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/lm
language: rust
---

# lm

Levenberg–Marquardt iteration for the constraint solver.

## Docstring

Levenberg–Marquardt iteration for the constraint solver.

Solves the least-squares problem

minimise  ½ ‖r(x)‖²

by iterating the damped Newton update

(JᵀJ + λI) Δx = −Jᵀr
x ← x + Δx       (if step reduces |r|²)
λ ← λ / 10       (good step — be more Gauss–Newton next time)
λ ← λ · 10       (bad step — be more steepest-descent next time)

Reference: *Numerical Recipes* (Press et al., 3rd ed.) §15.5
("Nonlinear Models — Levenberg-Marquardt Method"). Implementation
is composed entirely from the in-house primitives in
[`crate::solver::math`] and the in-house dense LU in
[`crate::solver::linalg`]; no external numerical library source
consulted.

What `solve_lm` returns
-----------------------
On success the [`SolveResult`] carries the final state vector,
the iteration count, the final residual norm `|r|`, and the
wall-clock elapsed time in milliseconds. The state vector layout
matches `pack(sketch).vector` — call
[`crate::solver::state::point_xy`] (or read `EntityIndex` directly)
to recover entity coordinates.

On failure [`SolveError`] is returned with one of:
- `DidNotConverge` — `max_iters` exceeded without `|r| < tolerance`.
- `Timeout` — wall-clock budget exceeded.
- `OverConstrained` — singular normal-equation matrix at any λ
(LM theory says this should be impossible for finite λ, but
underflow can still hit the LU pivot threshold; we surface it
so the caller can show a useful error rather than retrying
forever).

## Relationships

| Type | Target |
|------|--------|
| related | [SolveResult](/crates/oxide-sketch/src/solver/lm/SolveResult.md) |
| related | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
