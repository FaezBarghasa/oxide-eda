---
okf_version: "0.2"
type: Class
title: CameraUniform
description: Shared camera uniform uploaded once per frame.
resource: crates/oxide-gfx/src/camera.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/camera/CameraUniform
language: rust
---

# CameraUniform

Shared camera uniform uploaded once per frame.

## Signature

```rust
pub struct CameraUniform
```

## Decorators

- `repr(C)`
- `derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)`

## Visibility

- `pub`

## Docstring

Shared camera uniform uploaded once per frame.

The layout is mirrored by the `Camera` struct in every `shader/*.wgsl`, and
the two must agree byte for byte: `view_proj` 64, `viewport` 8,
`mm_per_px` 4, the two feature floors 8, then 12 of padding to the 16-byte
alignment a uniform block requires — 96 bytes total.
[repr(C)]
[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]

## Methods

- `view_proj`
- `viewport`
- `mm_per_px`
- `min_stroke_px`
- `min_radius_px`
- `_pad`

## Source
Lines 17–27 in `crates/oxide-gfx/src/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-gfx/src/camera.md) |
