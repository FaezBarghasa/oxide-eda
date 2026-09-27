---
okf_version: "0.2"
type: Module
title: rotation2d
description: Global 2D rotation utilities for Oxide geometry.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d
language: rust
---

# rotation2d

Global 2D rotation utilities for Oxide geometry.

## Docstring

Global 2D rotation utilities for Oxide geometry.

This module is intentionally domain-agnostic: callers provide object pose,
geometry center, rotation space, and pivot. The same API can be reused by
schematic, symbol editor, PCB, or any future object model.

Rotation order conventions:
- Local rotation: `M_new = M_old * R_local`
- World rotation: `M_new = R_world * M_old`

Where `R_*` may include pivot translation as `T(p) * R * T(-p)`.

## Relationships

| Type | Target |
|------|--------|
| related | [Vec2d](/crates/oxide-types/src/rotation2d/Vec2d.md) |
| related | [new](/crates/oxide-types/src/rotation2d/new.md) |
| related | [new](/crates/oxide-types/src/rotation2d/new.md) |
| related | [Pose2d](/crates/oxide-types/src/rotation2d/Pose2d.md) |
| related | [new](/crates/oxide-types/src/rotation2d/new.md) |
| related | [new](/crates/oxide-types/src/rotation2d/new.md) |
| related | [RotationSpace](/crates/oxide-types/src/rotation2d/RotationSpace.md) |
| related | [RotationPivot](/crates/oxide-types/src/rotation2d/RotationPivot.md) |
| related | [Rotatable2d](/crates/oxide-types/src/rotation2d/Rotatable2d.md) |
| related | [rotate_object](/crates/oxide-types/src/rotation2d/rotate_object.md) |
| related | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
| related | [transform_local_point](/crates/oxide-types/src/rotation2d/transform_local_point.md) |
| related | [inverse_transform_world_point](/crates/oxide-types/src/rotation2d/inverse_transform_world_point.md) |
| related | [geometry_center_world](/crates/oxide-types/src/rotation2d/geometry_center_world.md) |
| related | [normalize_angle_rad](/crates/oxide-types/src/rotation2d/normalize_angle_rad.md) |
| related | [resolve_local_pivot](/crates/oxide-types/src/rotation2d/resolve_local_pivot.md) |
| related | [resolve_world_pivot](/crates/oxide-types/src/rotation2d/resolve_world_pivot.md) |
| related | [Mat3](/crates/oxide-types/src/rotation2d/Mat3.md) |
| related | [mul](/crates/oxide-types/src/rotation2d/mul.md) |
| related | [transform_point](/crates/oxide-types/src/rotation2d/transform_point.md) |
| related | [mul](/crates/oxide-types/src/rotation2d/mul.md) |
| related | [transform_point](/crates/oxide-types/src/rotation2d/transform_point.md) |
| related | [translation_matrix](/crates/oxide-types/src/rotation2d/translation_matrix.md) |
| related | [rotation_matrix](/crates/oxide-types/src/rotation2d/rotation_matrix.md) |
| related | [pose_to_matrix](/crates/oxide-types/src/rotation2d/pose_to_matrix.md) |
| related | [inverse_pose_matrix](/crates/oxide-types/src/rotation2d/inverse_pose_matrix.md) |
| related | [pose_from_matrix](/crates/oxide-types/src/rotation2d/pose_from_matrix.md) |
| related | [rad](/crates/oxide-types/src/rotation2d/rad.md) |
| related | [assert_close](/crates/oxide-types/src/rotation2d/assert_close.md) |
| related | [assert_vec_close](/crates/oxide-types/src/rotation2d/assert_vec_close.md) |
| related | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| related | [world_rotation_about_world_origin_orbits_origin_point](/crates/oxide-types/src/rotation2d/world_rotation_about_world_origin_orbits_origin_point.md) |
| related | [local_rotation_about_local_origin_keeps_origin_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_local_origin_keeps_origin_fixed.md) |
| related | [local_and_world_orders_differ_for_same_angle](/crates/oxide-types/src/rotation2d/local_and_world_orders_differ_for_same_angle.md) |
| related | [world_rotation_with_local_pivot_keeps_local_anchor_world_position](/crates/oxide-types/src/rotation2d/world_rotation_with_local_pivot_keeps_local_anchor_world_position.md) |
| related | [local_rotation_with_world_pivot_keeps_that_world_point_fixed](/crates/oxide-types/src/rotation2d/local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) |
| related | [normalize_angle_wraps_to_minus_pi_plus_pi](/crates/oxide-types/src/rotation2d/normalize_angle_wraps_to_minus_pi_plus_pi.md) |
| related | [DummyObject](/crates/oxide-types/src/rotation2d/DummyObject.md) |
| related | [pose](/crates/oxide-types/src/rotation2d/pose.md) |
| related | [geometry_center_local](/crates/oxide-types/src/rotation2d/geometry_center_local.md) |
| related | [set_pose](/crates/oxide-types/src/rotation2d/set_pose.md) |
| related | [pose](/crates/oxide-types/src/rotation2d/pose.md) |
| related | [geometry_center_local](/crates/oxide-types/src/rotation2d/geometry_center_local.md) |
| related | [set_pose](/crates/oxide-types/src/rotation2d/set_pose.md) |
| related | [rotate_object_trait_matches_rotate_pose_result](/crates/oxide-types/src/rotation2d/rotate_object_trait_matches_rotate_pose_result.md) |
