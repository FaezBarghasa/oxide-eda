---
okf_version: "0.2"
type: Function
title: perspective
description: Build a perspective camera for future 3D views.
resource: crates/oxide-gfx/src/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/camera/perspective_1
language: rust
---

# perspective

Build a perspective camera for future 3D views.

## Signature

```rust
pub fn perspective(
        viewport_px: [f32; 2],
        eye: glam::Vec3,
        target: glam::Vec3,
        fov_rad: f32,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Build a perspective camera for future 3D views.

## Source
Lines 83–110 in `crates/oxide-gfx/src/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-gfx/src/camera.md) |
