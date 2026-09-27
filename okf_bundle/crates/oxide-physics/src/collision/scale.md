---
okf_version: "0.2"
type: Function
title: scale
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/scale
language: rust
---

# scale

## Signature

```rust
fn scale(a: Vec3, s: f64) -> Vec3
```

## Source
Lines 25–27 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| called_by | [evaluate_collision](/crates/oxide-physics/src/collision/evaluate_collision.md) |
| called_by | [minkowski_support](/crates/oxide-physics/src/collision/minkowski_support.md) |
| called_by | [normalize](/crates/oxide-physics/src/collision/normalize.md) |
| called_by | [update_simplex_and_direction](/crates/oxide-physics/src/collision/update_simplex_and_direction.md) |
