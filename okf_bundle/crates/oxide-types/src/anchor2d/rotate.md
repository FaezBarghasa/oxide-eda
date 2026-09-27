---
okf_version: "0.2"
type: Function
title: rotate
description: "Rotate the object by `delta_rad` counter-clockwise around `pivot_world`."
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/rotate
language: rust
---

# rotate

Rotate the object by `delta_rad` counter-clockwise around `pivot_world`.

## Signature

```rust
impl Transform2D { pub fn rotate(&mut self, delta_rad: f64) }
```

## Visibility

- `pub`

## Docstring

Rotate the object by `delta_rad` counter-clockwise around `pivot_world`.

`pivot_world` is **unchanged**. The object origin orbits around it.

## Source
Lines 165–167 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
| calls | [normalize_angle_rad](/crates/oxide-types/src/rotation2d/normalize_angle_rad.md) |
