---
okf_version: "0.2"
type: Function
title: ortho
description: Build an orthographic camera for 2D views.
resource: crates/oxide-gfx/src/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/camera/ortho_1
language: rust
---

# ortho

Build an orthographic camera for 2D views.

## Signature

```rust
pub fn ortho(viewport_px: [f32; 2], offset_mm: [f32; 2], scale_px_per_mm: f32) -> Self
```

## Visibility

- `pub`

## Docstring

Build an orthographic camera for 2D views.

Feature floors default to off; a surface that wants them adds
[`Self::with_min_feature_px`].

## Source
Lines 34–53 in `crates/oxide-gfx/src/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-gfx/src/camera.md) |
