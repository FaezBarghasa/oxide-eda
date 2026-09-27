---
okf_version: "0.2"
type: Function
title: origin_world
description: World-space position of the object origin.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/origin_world_1
language: rust
---

# origin_world

World-space position of the object origin.

## Signature

```rust
pub fn origin_world(&self) -> Vec2d
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

World-space position of the object origin.

```text
origin_world = pivot_world + rotate(local_offset, rotation_rad)
```
[must_use]

## Source
Lines 94–100 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| calls | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
