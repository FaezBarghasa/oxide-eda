# anchor2d

## Classs

- [Transform2D](Transform2D.md) — 2D transform with an explicit pivot/anchor point stored in world space.

## Functions

- [anchor_world_at](anchor_world_at.md) — World-space position of any fractional anchor point within the object.
- [anchor_world_at](anchor_world_at_1.md) — World-space position of any fractional anchor point within the object.
- [anchor_world_at_center_equals_pivot_when_center_anchor](anchor_world_at_center_equals_pivot_when_center_anchor.md) — [test]
- [anchor_world_at_corners_no_rotation](anchor_world_at_corners_no_rotation.md) — [test]
- [approx](approx.md)
- [b_behavior_anchor_change_does_not_move_object](b_behavior_anchor_change_does_not_move_object.md) — [test]
- [b_behavior_rotate_then_change_anchor_does_not_jump](b_behavior_rotate_then_change_anchor_does_not_jump.md) — [test]
- [from_origin_anchor](from_origin_anchor.md) — Construct from the object's world-space origin and a fractional anchor.
- [from_origin_anchor](from_origin_anchor_1.md) — Construct from the object's world-space origin and a fractional anchor.
- [from_origin_anchor_center_with_rotation](from_origin_anchor_center_with_rotation.md) — [test]
- [from_origin_anchor_corner_anchor_no_rotation](from_origin_anchor_corner_anchor_no_rotation.md) — [test]
- [from_origin_anchor_places_pivot_at_anchor_no_rotation](from_origin_anchor_places_pivot_at_anchor_no_rotation.md) — [test]
- [new](new.md) — Construct from raw components.
- [new](new_1.md) — Construct from raw components.
- [origin_world](origin_world.md) — World-space position of the object origin.
- [origin_world](origin_world_1.md) — World-space position of the object origin.
- [rotate](rotate.md) — Rotate the object by `delta_rad` counter-clockwise around `pivot_world`.
- [rotate](rotate_1.md) — Rotate the object by `delta_rad` counter-clockwise around `pivot_world`.
- [rotate_90_moves_origin_around_pivot](rotate_90_moves_origin_around_pivot.md) — [test]
- [rotate_full_circle_returns_to_original](rotate_full_circle_returns_to_original.md) — [test]
- [rotate_keeps_pivot_world_fixed](rotate_keeps_pivot_world_fixed.md) — [test]
- [rotate_vec](rotate_vec.md) — Rotate a 2D vector by `angle_rad` counter-clockwise.
- [rotate_vec_180](rotate_vec_180.md) — [test]
- [rotate_vec_90_ccw](rotate_vec_90_ccw.md) — [test]
- [rotate_vec_zero_angle_is_identity](rotate_vec_zero_angle_is_identity.md) — [test]
- [set_pivot_to_anchor](set_pivot_to_anchor.md) — Move `pivot_world` to a new anchor fraction inside the object without
- [set_pivot_to_anchor](set_pivot_to_anchor_1.md) — Move `pivot_world` to a new anchor fraction inside the object without
- [set_pivot_to_anchor_keeps_origin_fixed_no_rotation](set_pivot_to_anchor_keeps_origin_fixed_no_rotation.md) — [test]
- [set_pivot_to_anchor_keeps_origin_fixed_with_rotation](set_pivot_to_anchor_keeps_origin_fixed_with_rotation.md) — [test]
- [set_pivot_to_anchor_updates_local_offset](set_pivot_to_anchor_updates_local_offset.md) — [test]
- [to_matrix](to_matrix.md) — 3×3 row-major homogeneous transform matrix.
- [to_matrix](to_matrix_1.md) — 3×3 row-major homogeneous transform matrix.
- [transform_point](transform_point.md) — Transform a local-space point to world space using this transform.
- [transform_point](transform_point_1.md) — Transform a local-space point to world space using this transform.
- [transform_point_origin_matches_origin_world](transform_point_origin_matches_origin_world.md) — [test]
- [transform_point_size_corner_matches_anchor_world_at_one](transform_point_size_corner_matches_anchor_world_at_one.md) — [test]
- [translate](translate.md) — Translate the entire transform: both `pivot_world` and the derived
- [translate](translate_1.md) — Translate the entire transform: both `pivot_world` and the derived
- [translate_shifts_pivot_and_origin_equally](translate_shifts_pivot_and_origin_equally.md) — [test]
- [vec_eq](vec_eq.md)
