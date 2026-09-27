---
okf_version: "0.2"
type: Function
title: evaluate_collision
description: Evaluates whether two convex bodies collide using Gilbert-Johnson-Keerthi (GJK).
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/evaluate_collision_1
language: rust
---

# evaluate_collision

Evaluates whether two convex bodies collide using Gilbert-Johnson-Keerthi (GJK).

## Signature

```rust
pub fn evaluate_collision(a: &ConvexPolytope, b: &ConvexPolytope) -> CollisionResult
```

## Visibility

- `pub`

## Docstring

Evaluates whether two convex bodies collide using Gilbert-Johnson-Keerthi (GJK).

## Source
Lines 94–137 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [scale](/crates/oxide-physics/src/collision/scale.md) |
| calls | [dot](/crates/oxide-physics/src/collision/dot.md) |
| calls | [normalize](/crates/oxide-physics/src/collision/normalize.md) |
