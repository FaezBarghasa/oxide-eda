---
okf_version: "0.2"
type: Function
title: transform_local_point
description: Transform a local-space point to world-space with the given pose.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/transform_local_point
language: rust
---

# transform_local_point

Transform a local-space point to world-space with the given pose.

## Signature

```rust
pub fn transform_local_point(pose: Pose2d, local: Vec2d) -> Vec2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Transform a local-space point to world-space with the given pose.
[must_use]

## Source
Lines 144–146 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [pose_to_matrix](/crates/oxide-types/src/rotation2d/pose_to_matrix.md) |
| called_by | [geometry_center_world](/crates/oxide-types/src/rotation2d/geometry_center_world.md) |
| called_by | [local_rotation_with_world_pivot_keeps_that_world_point_fixed](/crates/oxide-types/src/rotation2d/local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) |
| called_by | [resolve_world_pivot](/crates/oxide-types/src/rotation2d/resolve_world_pivot.md) |
| called_by | [world_rotation_with_local_pivot_keeps_local_anchor_world_position](/crates/oxide-types/src/rotation2d/world_rotation_with_local_pivot_keeps_local_anchor_world_position.md) |
