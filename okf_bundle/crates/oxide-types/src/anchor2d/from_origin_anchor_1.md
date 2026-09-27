---
okf_version: "0.2"
type: Function
title: from_origin_anchor
description: "Construct from the object's world-space origin and a fractional anchor."
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/from_origin_anchor_1
language: rust
---

# from_origin_anchor

Construct from the object's world-space origin and a fractional anchor.

## Signature

```rust
pub fn from_origin_anchor(
        origin_world: Vec2d,
        anchor_frac: Vec2d,
        size: Vec2d,
        rotation_rad: f64,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Construct from the object's world-space origin and a fractional anchor.

`anchor_frac` is in `[0, 1]²`:
- `(0, 0)` = bottom-left corner
- `(0.5, 0.5)` = center
- `(1, 1)` = top-right corner  (Y-up convention)

The pivot is placed at the anchor point.

## Source
Lines 69–86 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| calls | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
