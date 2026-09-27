---
okf_version: "0.2"
type: Function
title: world_to_screen
description: Convert world coordinates (mm) to screen coordinates (pixels).
resource: crates/oxide-app/src/canvas/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/camera/world_to_screen_1
language: rust
---

# world_to_screen

Convert world coordinates (mm) to screen coordinates (pixels).

## Signature

```rust
pub fn world_to_screen(&self, world: Point, _bounds: Rectangle) -> Point
```

## Visibility

- `pub`

## Docstring

Convert world coordinates (mm) to screen coordinates (pixels).

## Source
Lines 35–40 in `crates/oxide-app/src/canvas/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/camera.md) |
