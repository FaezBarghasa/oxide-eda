---
okf_version: "0.2"
type: Module
title: anchor2d
description: Anchor-aware 2D transform for Oxide geometry.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d
language: rust
---

# anchor2d

Anchor-aware 2D transform for Oxide geometry.

## Docstring

Anchor-aware 2D transform for Oxide geometry.

This module provides [`Transform2D`], a transform that stores the pivot/anchor
point explicitly in world space — the "B-type compensated" model.

# Coordinate model

```text
origin_world = pivot_world + rotate(local_offset, rotation_rad)
```

- `pivot_world`: anchor point in world space. **Stays fixed** during rotation.
- `local_offset`: offset from pivot to object origin in LOCAL (pre-rotation) space.
Convention: `local_offset = -(anchor_frac * size)` when pivot is at an anchor.

# Why B-type (compensated)?

In the A-type model the anchor world position is re-derived each frame from
`(position, anchor_frac, size)`. When `anchor_frac` changes on a rotated object
the derived pivot jumps to a different world location, causing a visible snap.

In the B-type model `pivot_world` is stored directly. Rotation always turns
the object around that stable world coordinate.  Changing the anchor fraction
(which point of the object the pivot sits on) is done via
[`Transform2D::set_pivot_to_anchor`], which moves `pivot_world` to the new
anchor position without moving the object.

## Relationships

| Type | Target |
|------|--------|
| related | [Transform2D](/crates/oxide-types/src/anchor2d/Transform2D.md) |
| related | [new](/crates/oxide-types/src/anchor2d/new.md) |
| related | [from_origin_anchor](/crates/oxide-types/src/anchor2d/from_origin_anchor.md) |
| related | [origin_world](/crates/oxide-types/src/anchor2d/origin_world.md) |
| related | [anchor_world_at](/crates/oxide-types/src/anchor2d/anchor_world_at.md) |
| related | [to_matrix](/crates/oxide-types/src/anchor2d/to_matrix.md) |
| related | [transform_point](/crates/oxide-types/src/anchor2d/transform_point.md) |
| related | [rotate](/crates/oxide-types/src/anchor2d/rotate.md) |
| related | [translate](/crates/oxide-types/src/anchor2d/translate.md) |
| related | [set_pivot_to_anchor](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor.md) |
| related | [new](/crates/oxide-types/src/anchor2d/new.md) |
| related | [from_origin_anchor](/crates/oxide-types/src/anchor2d/from_origin_anchor.md) |
| related | [origin_world](/crates/oxide-types/src/anchor2d/origin_world.md) |
| related | [anchor_world_at](/crates/oxide-types/src/anchor2d/anchor_world_at.md) |
| related | [to_matrix](/crates/oxide-types/src/anchor2d/to_matrix.md) |
| related | [transform_point](/crates/oxide-types/src/anchor2d/transform_point.md) |
| related | [rotate](/crates/oxide-types/src/anchor2d/rotate.md) |
| related | [translate](/crates/oxide-types/src/anchor2d/translate.md) |
| related | [set_pivot_to_anchor](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor.md) |
| related | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
| related | [approx](/crates/oxide-types/src/anchor2d/approx.md) |
| related | [vec_eq](/crates/oxide-types/src/anchor2d/vec_eq.md) |
| related | [from_origin_anchor_places_pivot_at_anchor_no_rotation](/crates/oxide-types/src/anchor2d/from_origin_anchor_places_pivot_at_anchor_no_rotation.md) |
| related | [from_origin_anchor_corner_anchor_no_rotation](/crates/oxide-types/src/anchor2d/from_origin_anchor_corner_anchor_no_rotation.md) |
| related | [from_origin_anchor_center_with_rotation](/crates/oxide-types/src/anchor2d/from_origin_anchor_center_with_rotation.md) |
| related | [rotate_keeps_pivot_world_fixed](/crates/oxide-types/src/anchor2d/rotate_keeps_pivot_world_fixed.md) |
| related | [rotate_90_moves_origin_around_pivot](/crates/oxide-types/src/anchor2d/rotate_90_moves_origin_around_pivot.md) |
| related | [rotate_full_circle_returns_to_original](/crates/oxide-types/src/anchor2d/rotate_full_circle_returns_to_original.md) |
| related | [set_pivot_to_anchor_keeps_origin_fixed_no_rotation](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor_keeps_origin_fixed_no_rotation.md) |
| related | [set_pivot_to_anchor_keeps_origin_fixed_with_rotation](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor_keeps_origin_fixed_with_rotation.md) |
| related | [set_pivot_to_anchor_updates_local_offset](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor_updates_local_offset.md) |
| related | [anchor_world_at_corners_no_rotation](/crates/oxide-types/src/anchor2d/anchor_world_at_corners_no_rotation.md) |
| related | [anchor_world_at_center_equals_pivot_when_center_anchor](/crates/oxide-types/src/anchor2d/anchor_world_at_center_equals_pivot_when_center_anchor.md) |
| related | [transform_point_origin_matches_origin_world](/crates/oxide-types/src/anchor2d/transform_point_origin_matches_origin_world.md) |
| related | [transform_point_size_corner_matches_anchor_world_at_one](/crates/oxide-types/src/anchor2d/transform_point_size_corner_matches_anchor_world_at_one.md) |
| related | [translate_shifts_pivot_and_origin_equally](/crates/oxide-types/src/anchor2d/translate_shifts_pivot_and_origin_equally.md) |
| related | [b_behavior_anchor_change_does_not_move_object](/crates/oxide-types/src/anchor2d/b_behavior_anchor_change_does_not_move_object.md) |
| related | [b_behavior_rotate_then_change_anchor_does_not_jump](/crates/oxide-types/src/anchor2d/b_behavior_rotate_then_change_anchor_does_not_jump.md) |
| related | [rotate_vec_zero_angle_is_identity](/crates/oxide-types/src/anchor2d/rotate_vec_zero_angle_is_identity.md) |
| related | [rotate_vec_90_ccw](/crates/oxide-types/src/anchor2d/rotate_vec_90_ccw.md) |
| related | [rotate_vec_180](/crates/oxide-types/src/anchor2d/rotate_vec_180.md) |
