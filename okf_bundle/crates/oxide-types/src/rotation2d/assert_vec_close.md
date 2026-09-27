---
okf_version: "0.2"
type: Function
title: assert_vec_close
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/assert_vec_close
language: rust
---

# assert_vec_close

## Signature

```rust
fn assert_vec_close(a: Vec2d, b: Vec2d)
```

## Source
Lines 271–274 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [assert_close](/crates/oxide-types/src/rotation2d/assert_close.md) |
| called_by | [local_and_world_orders_differ_for_same_angle](/crates/oxide-types/src/rotation2d/local_and_world_orders_differ_for_same_angle.md) |
| called_by | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| called_by | [local_rotation_about_local_origin_keeps_origin_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_local_origin_keeps_origin_fixed.md) |
| called_by | [local_rotation_with_world_pivot_keeps_that_world_point_fixed](/crates/oxide-types/src/rotation2d/local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) |
| called_by | [rotate_object_trait_matches_rotate_pose_result](/crates/oxide-types/src/rotation2d/rotate_object_trait_matches_rotate_pose_result.md) |
| called_by | [world_rotation_about_world_origin_orbits_origin_point](/crates/oxide-types/src/rotation2d/world_rotation_about_world_origin_orbits_origin_point.md) |
| called_by | [world_rotation_with_local_pivot_keeps_local_anchor_world_position](/crates/oxide-types/src/rotation2d/world_rotation_with_local_pivot_keeps_local_anchor_world_position.md) |
