---
okf_version: "0.2"
type: Function
title: rotate_pose
description: Rotate a pose using local or world semantics.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/rotate_pose
language: rust
---

# rotate_pose

Rotate a pose using local or world semantics.

## Signature

```rust
pub fn rotate_pose(
    pose: Pose2d,
    geometry_center_local: Vec2d,
    space: RotationSpace,
    pivot: RotationPivot,
    delta_rad: f64,
) -> Pose2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Rotate a pose using local or world semantics.

- `geometry_center_local`: object geometry center in local coordinates.
This is required for anchor-aware rotations and `GeometryCenter` pivot mode.
[must_use]

## Source
Lines 103–140 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [pose_to_matrix](/crates/oxide-types/src/rotation2d/pose_to_matrix.md) |
| calls | [resolve_local_pivot](/crates/oxide-types/src/rotation2d/resolve_local_pivot.md) |
| calls | [translation_matrix](/crates/oxide-types/src/rotation2d/translation_matrix.md) |
| calls | [rotation_matrix](/crates/oxide-types/src/rotation2d/rotation_matrix.md) |
| calls | [resolve_world_pivot](/crates/oxide-types/src/rotation2d/resolve_world_pivot.md) |
| calls | [pose_from_matrix](/crates/oxide-types/src/rotation2d/pose_from_matrix.md) |
| called_by | [local_and_world_orders_differ_for_same_angle](/crates/oxide-types/src/rotation2d/local_and_world_orders_differ_for_same_angle.md) |
| called_by | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| called_by | [local_rotation_about_local_origin_keeps_origin_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_local_origin_keeps_origin_fixed.md) |
| called_by | [local_rotation_with_world_pivot_keeps_that_world_point_fixed](/crates/oxide-types/src/rotation2d/local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) |
| called_by | [rotate_object](/crates/oxide-types/src/rotation2d/rotate_object.md) |
| called_by | [rotate_object_trait_matches_rotate_pose_result](/crates/oxide-types/src/rotation2d/rotate_object_trait_matches_rotate_pose_result.md) |
| called_by | [world_rotation_about_world_origin_orbits_origin_point](/crates/oxide-types/src/rotation2d/world_rotation_about_world_origin_orbits_origin_point.md) |
| called_by | [world_rotation_with_local_pivot_keeps_local_anchor_world_position](/crates/oxide-types/src/rotation2d/world_rotation_with_local_pivot_keeps_local_anchor_world_position.md) |
