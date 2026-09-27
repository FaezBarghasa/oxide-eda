---
okf_version: "0.2"
type: Function
title: from_rotation_translation
description: "Constructs dual quaternion from rotation quaternion and translation vector $\\vec{t} = [x, y, z]$."
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat/from_rotation_translation
language: rust
---

# from_rotation_translation

Constructs dual quaternion from rotation quaternion and translation vector $\vec{t} = [x, y, z]$.

## Signature

```rust
impl DualQuaternion { pub fn from_rotation_translation(rotation: Quaternion, translation: [f64; 3]) -> Self }
```

## Visibility

- `pub`

## Docstring

Constructs dual quaternion from rotation quaternion and translation vector $\vec{t} = [x, y, z]$.

## Source
Lines 72–79 in `crates/oxide-physics/src/dual_quat.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dual_quat](/crates/oxide-physics/src/dual_quat.md) |
