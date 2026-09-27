---
okf_version: "0.2"
type: Function
title: support
description: "Support mapping function returning the farthest vertex along direction $\\vec{d}$."
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/support
language: rust
---

# support

Support mapping function returning the farthest vertex along direction $\vec{d}$.

## Signature

```rust
impl ConvexPolytope { pub fn support(&self, d: Vec3) -> Vec3 }
```

## Visibility

- `pub`

## Docstring

Support mapping function returning the farthest vertex along direction $\vec{d}$.

## Source
Lines 60–71 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [dot](/crates/oxide-physics/src/collision/dot.md) |
