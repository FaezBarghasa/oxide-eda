---
okf_version: "0.2"
type: Function
title: inverse_pose_matrix
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/inverse_pose_matrix
language: rust
---

# inverse_pose_matrix

## Signature

```rust
fn inverse_pose_matrix(pose: Pose2d) -> Mat3
```

## Source
Lines 234–246 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| called_by | [inverse_transform_world_point](/crates/oxide-types/src/rotation2d/inverse_transform_world_point.md) |
