---
okf_version: "0.2"
type: Function
title: geometry_center_world
description: Compute object geometry center in world coordinates.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/geometry_center_world
language: rust
---

# geometry_center_world

Compute object geometry center in world coordinates.

## Signature

```rust
pub fn geometry_center_world(pose: Pose2d, geometry_center_local: Vec2d) -> Vec2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Compute object geometry center in world coordinates.
[must_use]

## Source
Lines 156–158 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [transform_local_point](/crates/oxide-types/src/rotation2d/transform_local_point.md) |
| called_by | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| called_by | [resolve_world_pivot](/crates/oxide-types/src/rotation2d/resolve_world_pivot.md) |
