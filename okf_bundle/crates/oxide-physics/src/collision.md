---
okf_version: "0.2"
type: Module
title: collision
description: "Gilbert-Johnson-Keerthi (GJK) & Expanding Polytope Algorithm (EPA) 3D Collision Engine."
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision
language: rust
---

# collision

Gilbert-Johnson-Keerthi (GJK) & Expanding Polytope Algorithm (EPA) 3D Collision Engine.

## Docstring

Gilbert-Johnson-Keerthi (GJK) & Expanding Polytope Algorithm (EPA) 3D Collision Engine.

Conforms to Master Technical Directive Horizon III (§4, Task 3.3):
- Convex Polytope Minkowski Difference evaluation $\mathcal{C} = \mathcal{A} \ominus \mathcal{B}$.
- Support mapping function $S_{\mathcal{C}}(\vec{d}) = S_{\mathcal{A}}(\vec{d}) - S_{\mathcal{B}}(-\vec{d})$.
- Exact penetration depth vectors and contact point generation against mechanical enclosures.

## Relationships

| Type | Target |
|------|--------|
| related | [dot](/crates/oxide-physics/src/collision/dot.md) |
| related | [sub](/crates/oxide-physics/src/collision/sub.md) |
| related | [add](/crates/oxide-physics/src/collision/add.md) |
| related | [scale](/crates/oxide-physics/src/collision/scale.md) |
| related | [cross](/crates/oxide-physics/src/collision/cross.md) |
| related | [length](/crates/oxide-physics/src/collision/length.md) |
| related | [normalize](/crates/oxide-physics/src/collision/normalize.md) |
| related | [ConvexPolytope](/crates/oxide-physics/src/collision/ConvexPolytope.md) |
| related | [new](/crates/oxide-physics/src/collision/new.md) |
| related | [support](/crates/oxide-physics/src/collision/support.md) |
| related | [new](/crates/oxide-physics/src/collision/new.md) |
| related | [support](/crates/oxide-physics/src/collision/support.md) |
| related | [CollisionResult](/crates/oxide-physics/src/collision/CollisionResult.md) |
| related | [GjkEpaEngine](/crates/oxide-physics/src/collision/GjkEpaEngine.md) |
| related | [minkowski_support](/crates/oxide-physics/src/collision/minkowski_support.md) |
| related | [evaluate_collision](/crates/oxide-physics/src/collision/evaluate_collision.md) |
| related | [update_simplex_and_direction](/crates/oxide-physics/src/collision/update_simplex_and_direction.md) |
| related | [compute_epa_penetration](/crates/oxide-physics/src/collision/compute_epa_penetration.md) |
| related | [minkowski_support](/crates/oxide-physics/src/collision/minkowski_support.md) |
| related | [evaluate_collision](/crates/oxide-physics/src/collision/evaluate_collision.md) |
| related | [update_simplex_and_direction](/crates/oxide-physics/src/collision/update_simplex_and_direction.md) |
| related | [compute_epa_penetration](/crates/oxide-physics/src/collision/compute_epa_penetration.md) |
| related | [test_gjk_overlapping_cubes](/crates/oxide-physics/src/collision/test_gjk_overlapping_cubes.md) |
| related | [test_gjk_separated_cubes](/crates/oxide-physics/src/collision/test_gjk_separated_cubes.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
