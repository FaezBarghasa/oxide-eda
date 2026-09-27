---
okf_version: "0.2"
type: Function
title: anchor_world_at
description: World-space position of any fractional anchor point within the object.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/anchor_world_at
language: rust
---

# anchor_world_at

World-space position of any fractional anchor point within the object.

## Signature

```rust
impl Transform2D { pub fn anchor_world_at(&self, anchor_frac: Vec2d) -> Vec2d }
```

## Visibility

- `pub`

## Docstring

World-space position of any fractional anchor point within the object.

`anchor_frac = (0, 0)` returns the object origin; `(1, 1)` returns the
opposite corner (top-right, Y-up).
[must_use]

## Source
Lines 107–119 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| calls | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
