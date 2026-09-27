---
okf_version: "0.2"
type: Function
title: pose_to_matrix
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/pose_to_matrix
language: rust
---

# pose_to_matrix

## Signature

```rust
fn pose_to_matrix(pose: Pose2d) -> Mat3
```

## Source
Lines 230–232 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [translation_matrix](/crates/oxide-types/src/rotation2d/translation_matrix.md) |
| calls | [rotation_matrix](/crates/oxide-types/src/rotation2d/rotation_matrix.md) |
| called_by | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
| called_by | [transform_local_point](/crates/oxide-types/src/rotation2d/transform_local_point.md) |
