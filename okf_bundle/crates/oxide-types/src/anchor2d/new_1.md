---
okf_version: "0.2"
type: Function
title: new
description: Construct from raw components.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/new_1
language: rust
---

# new

Construct from raw components.

## Signature

```rust
pub fn new(
        pivot_world: Vec2d,
        local_offset: Vec2d,
        size: Vec2d,
        rotation_rad: f64,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Construct from raw components.

## Source
Lines 47–59 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
