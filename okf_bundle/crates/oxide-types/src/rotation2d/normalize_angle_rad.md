---
okf_version: "0.2"
type: Function
title: normalize_angle_rad
description: "Normalize angle into `(-PI, PI]` range."
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/normalize_angle_rad
language: rust
---

# normalize_angle_rad

Normalize angle into `(-PI, PI]` range.

## Signature

```rust
pub fn normalize_angle_rad(angle: f64) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Normalize angle into `(-PI, PI]` range.
[must_use]

## Source
Lines 162–170 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
| called_by | [rotate_graphic_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_90.md) |
| called_by | [normalize_degrees](/crates/oxide-engine/src/transform/mod/normalize_degrees.md) |
| called_by | [rotate](/crates/oxide-types/src/anchor2d/rotate.md) |
| called_by | [local_rotation_about_geometry_center_keeps_center_fixed](/crates/oxide-types/src/rotation2d/local_rotation_about_geometry_center_keeps_center_fixed.md) |
| called_by | [normalize_angle_wraps_to_minus_pi_plus_pi](/crates/oxide-types/src/rotation2d/normalize_angle_wraps_to_minus_pi_plus_pi.md) |
| called_by | [pose_from_matrix](/crates/oxide-types/src/rotation2d/pose_from_matrix.md) |
