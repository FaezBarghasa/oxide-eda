---
okf_version: "0.2"
type: Function
title: pose_from_matrix
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/pose_from_matrix
language: rust
---

# pose_from_matrix

## Signature

```rust
fn pose_from_matrix(m: Mat3) -> Pose2d
```

## Source
Lines 248–251 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| calls | [normalize_angle_rad](/crates/oxide-types/src/rotation2d/normalize_angle_rad.md) |
| called_by | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
