---
okf_version: "0.2"
type: Function
title: set_pivot_to_anchor
description: "Move `pivot_world` to a new anchor fraction inside the object without"
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/set_pivot_to_anchor_1
language: rust
---

# set_pivot_to_anchor

Move `pivot_world` to a new anchor fraction inside the object without

## Signature

```rust
pub fn set_pivot_to_anchor(&mut self, anchor_frac: Vec2d)
```

## Visibility

- `pub`

## Docstring

Move `pivot_world` to a new anchor fraction inside the object without
visually moving the object (B-type compensated behavior).

After this call:
- `pivot_world` is at the new `anchor_frac` position on the (unchanged) object.
- `origin_world()` returns the same value as before.
- `local_offset = -(anchor_frac * size)`.

## Source
Lines 183–191 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| calls | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
