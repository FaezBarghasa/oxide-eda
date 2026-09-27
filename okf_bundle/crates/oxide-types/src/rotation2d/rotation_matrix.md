---
okf_version: "0.2"
type: Function
title: rotation_matrix
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/rotation_matrix
language: rust
---

# rotation_matrix

## Signature

```rust
fn rotation_matrix(angle_rad: f64) -> Mat3
```

## Source
Lines 222–228 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| called_by | [pose_to_matrix](/crates/oxide-types/src/rotation2d/pose_to_matrix.md) |
| called_by | [rotate_pose](/crates/oxide-types/src/rotation2d/rotate_pose.md) |
