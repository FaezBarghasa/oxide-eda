---
okf_version: "0.2"
type: Function
title: rotate_vec
description: "Rotate a 2D vector by `angle_rad` counter-clockwise."
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/rotate_vec
language: rust
---

# rotate_vec

Rotate a 2D vector by `angle_rad` counter-clockwise.

## Signature

```rust
pub fn rotate_vec(v: Vec2d, angle_rad: f64) -> Vec2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Rotate a 2D vector by `angle_rad` counter-clockwise.
[must_use]

## Source
Lines 198–205 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| called_by | [compute](/crates/oxide-app/src/library/editor/symbol/canvas/pins/compute.md) |
| called_by | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
| called_by | [anchor_world_at](/crates/oxide-types/src/anchor2d/anchor_world_at.md) |
| called_by | [from_origin_anchor](/crates/oxide-types/src/anchor2d/from_origin_anchor.md) |
| called_by | [origin_world](/crates/oxide-types/src/anchor2d/origin_world.md) |
| called_by | [rotate_vec_180](/crates/oxide-types/src/anchor2d/rotate_vec_180.md) |
| called_by | [rotate_vec_90_ccw](/crates/oxide-types/src/anchor2d/rotate_vec_90_ccw.md) |
| called_by | [set_pivot_to_anchor](/crates/oxide-types/src/anchor2d/set_pivot_to_anchor.md) |
