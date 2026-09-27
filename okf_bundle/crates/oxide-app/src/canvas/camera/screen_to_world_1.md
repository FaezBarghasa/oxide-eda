---
okf_version: "0.2"
type: Function
title: screen_to_world
description: Convert screen coordinates (pixels) to world coordinates (mm).
resource: crates/oxide-app/src/canvas/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/camera/screen_to_world_1
language: rust
---

# screen_to_world

Convert screen coordinates (pixels) to world coordinates (mm).

## Signature

```rust
pub fn screen_to_world(&self, screen: Point, _bounds: Rectangle) -> Point
```

## Visibility

- `pub`

## Docstring

Convert screen coordinates (pixels) to world coordinates (mm).

## Source
Lines 43–48 in `crates/oxide-app/src/canvas/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/camera.md) |
