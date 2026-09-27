---
okf_version: "0.2"
type: Function
title: minkowski_support
description: "Support function for Minkowski Difference $\\mathcal{C} = \\mathcal{A} \\ominus \\mathcal{B}$."
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/minkowski_support
language: rust
---

# minkowski_support

Support function for Minkowski Difference $\mathcal{C} = \mathcal{A} \ominus \mathcal{B}$.

## Signature

```rust
impl GjkEpaEngine { pub fn minkowski_support(a: &ConvexPolytope, b: &ConvexPolytope, d: Vec3) -> Vec3 }
```

## Visibility

- `pub`

## Docstring

Support function for Minkowski Difference $\mathcal{C} = \mathcal{A} \ominus \mathcal{B}$.

## Source
Lines 87–91 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [scale](/crates/oxide-physics/src/collision/scale.md) |
| calls | [sub](/crates/oxide-physics/src/collision/sub.md) |
