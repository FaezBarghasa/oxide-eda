---
okf_version: "0.2"
type: Function
title: rotate_object
description: "Rotate any object implementing [`Rotatable2d`]."
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/rotate_object
language: rust
---

# rotate_object

Rotate any object implementing [`Rotatable2d`].

## Signature

```rust
pub fn rotate_object(
    object: &mut T,
    space: RotationSpace,
    pivot: RotationPivot,
    delta_rad: f64,
)
```

## Type Parameters

- `T: Rotatable2d`

## Visibility

- `pub`

## Docstring

Rotate any object implementing [`Rotatable2d`].

## Source
Lines 82–96 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
| called_by | [rotate_graphic_point_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_point_90.md) |
| called_by | [rotate_object_trait_matches_rotate_pose_result](/crates/oxide-types/src/rotation2d/rotate_object_trait_matches_rotate_pose_result.md) |
