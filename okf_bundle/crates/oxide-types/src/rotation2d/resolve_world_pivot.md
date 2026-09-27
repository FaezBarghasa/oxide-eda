---
okf_version: "0.2"
type: Function
title: resolve_world_pivot
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/resolve_world_pivot
language: rust
---

# resolve_world_pivot

## Signature

```rust
fn resolve_world_pivot(pose: Pose2d, geometry_center_local: Vec2d, pivot: RotationPivot) -> Vec2d
```

## Source
Lines 181–188 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [geometry_center_world](/crates/oxide-types/src/rotation2d/geometry_center_world.md) |
| calls | [transform_local_point](/crates/oxide-types/src/rotation2d/transform_local_point.md) |
| called_by | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
