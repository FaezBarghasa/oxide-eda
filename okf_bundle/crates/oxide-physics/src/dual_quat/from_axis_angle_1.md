---
okf_version: "0.2"
type: Function
title: from_axis_angle
description: "Constructs rotation quaternion from axis $\\vec{L}$ and angle $\\theta$ in radians."
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat/from_axis_angle_1
language: rust
---

# from_axis_angle

Constructs rotation quaternion from axis $\vec{L}$ and angle $\theta$ in radians.

## Signature

```rust
pub fn from_axis_angle(axis: [f64; 3], angle_rad: f64) -> Self
```

## Visibility

- `pub`

## Docstring

Constructs rotation quaternion from axis $\vec{L}$ and angle $\theta$ in radians.

## Source
Lines 27–37 in `crates/oxide-physics/src/dual_quat.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dual_quat](/crates/oxide-physics/src/dual_quat.md) |
