---
okf_version: "0.2"
type: Function
title: transform_point
description: "Transforms 3D vertex point $\\vec{p} \\in \\mathbb{R}^3$."
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat/transform_point
language: rust
---

# transform_point

Transforms 3D vertex point $\vec{p} \in \mathbb{R}^3$.

## Signature

```rust
impl DualQuaternion { pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Transforms 3D vertex point $\vec{p} \in \mathbb{R}^3$.

## Source
Lines 99–118 in `crates/oxide-physics/src/dual_quat.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dual_quat](/crates/oxide-physics/src/dual_quat.md) |
