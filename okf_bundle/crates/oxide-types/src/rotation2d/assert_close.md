---
okf_version: "0.2"
type: Function
title: assert_close
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/assert_close
language: rust
---

# assert_close

## Signature

```rust
fn assert_close(a: f64, b: f64)
```

## Source
Lines 263–269 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| called_by | [assert_vec_close](/crates/oxide-types/src/rotation2d/assert_vec_close.md) |
| called_by | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| called_by | [local_rotation_about_local_origin_keeps_origin_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_local_origin_keeps_origin_fixed.md) |
| called_by | [normalize_angle_wraps_to_minus_pi_plus_pi](/crates/oxide-types/src/rotation2d/normalize_angle_wraps_to_minus_pi_plus_pi.md) |
| called_by | [rotate_object_trait_matches_rotate_pose_result](/crates/oxide-types/src/rotation2d/rotate_object_trait_matches_rotate_pose_result.md) |
| called_by | [world_rotation_about_world_origin_orbits_origin_point](/crates/oxide-types/src/rotation2d/world_rotation_about_world_origin_orbits_origin_point.md) |
