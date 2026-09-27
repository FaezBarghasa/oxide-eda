# rotation2d

## Classs

- [DummyObject](DummyObject.md) — [derive(Debug, Clone, Copy)]
- [Mat3](Mat3.md) — [derive(Debug, Clone, Copy, PartialEq)]
- [Pose2d](Pose2d.md) — [derive(Debug, Clone, Copy, PartialEq, Default)]
- [Rotatable2d](Rotatable2d.md) — Adapter trait for per-object rotation integration.
- [RotationPivot](RotationPivot.md) — [derive(Debug, Clone, Copy, PartialEq)]
- [RotationSpace](RotationSpace.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]
- [Vec2d](Vec2d.md) — [derive(Debug, Clone, Copy, PartialEq, Default)]

## Functions

- [assert_close](assert_close.md)
- [assert_vec_close](assert_vec_close.md)
- [geometry_center_local](geometry_center_local.md)
- [geometry_center_local](geometry_center_local_1.md)
- [geometry_center_world](geometry_center_world.md) — Compute object geometry center in world coordinates.
- [inverse_pose_matrix](inverse_pose_matrix.md)
- [inverse_transform_world_point](inverse_transform_world_point.md) — Transform a world-space point back into object local coordinates.
- [local_and_world_orders_differ_for_same_angle](local_and_world_orders_differ_for_same_angle.md) — [test]
- [local_rotation_about_geometry_center_keeps_center_fixed](local_rotation_about_geometry_center_keeps_center_fixed.md) — [test]
- [local_rotation_about_local_origin_keeps_origin_fixed](local_rotation_about_local_origin_keeps_origin_fixed.md) — [test]
- [local_rotation_with_world_pivot_keeps_that_world_point_fixed](local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) — [test]
- [mul](mul.md)
- [mul](mul_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [normalize_angle_rad](normalize_angle_rad.md) — Normalize angle into `(-PI, PI]` range.
- [normalize_angle_wraps_to_minus_pi_plus_pi](normalize_angle_wraps_to_minus_pi_plus_pi.md) — [test]
- [pose](pose.md)
- [pose](pose_1.md)
- [pose_from_matrix](pose_from_matrix.md)
- [pose_to_matrix](pose_to_matrix.md)
- [rad](rad.md)
- [resolve_local_pivot](resolve_local_pivot.md)
- [resolve_world_pivot](resolve_world_pivot.md)
- [rotate_object](rotate_object.md) — Rotate any object implementing [`Rotatable2d`].
- [rotate_object_trait_matches_rotate_pose_result](rotate_object_trait_matches_rotate_pose_result.md) — [test]
- [rotate_pose](rotate_pose.md) — Rotate a pose using local or world semantics.
- [rotation_matrix](rotation_matrix.md)
- [set_pose](set_pose.md)
- [set_pose](set_pose_1.md)
- [transform_local_point](transform_local_point.md) — Transform a local-space point to world-space with the given pose.
- [transform_point](transform_point.md)
- [transform_point](transform_point_1.md)
- [translation_matrix](translation_matrix.md)
- [world_rotation_about_world_origin_orbits_origin_point](world_rotation_about_world_origin_orbits_origin_point.md) — [test]
- [world_rotation_with_local_pivot_keeps_local_anchor_world_position](world_rotation_with_local_pivot_keeps_local_anchor_world_position.md) — [test]
