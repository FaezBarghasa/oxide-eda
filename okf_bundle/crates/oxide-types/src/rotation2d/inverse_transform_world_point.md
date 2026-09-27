---
okf_version: "0.2"
type: Function
title: inverse_transform_world_point
description: Transform a world-space point back into object local coordinates.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/inverse_transform_world_point
language: rust
---

# inverse_transform_world_point

Transform a world-space point back into object local coordinates.

## Signature

```rust
pub fn inverse_transform_world_point(pose: Pose2d, world: Vec2d) -> Vec2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Transform a world-space point back into object local coordinates.
[must_use]

## Source
Lines 150–152 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [inverse_pose_matrix](/crates/oxide-types/src/rotation2d/inverse_pose_matrix.md) |
| called_by | [local_rotation_with_world_pivot_keeps_that_world_point_fixed](/crates/oxide-types/src/rotation2d/local_rotation_with_world_pivot_keeps_that_world_point_fixed.md) |
| called_by | [resolve_local_pivot](/crates/oxide-types/src/rotation2d/resolve_local_pivot.md) |
