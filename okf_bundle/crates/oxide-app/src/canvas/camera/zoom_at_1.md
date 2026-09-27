---
okf_version: "0.2"
type: Function
title: zoom_at
description: Zoom centered on a screen-space point.
resource: crates/oxide-app/src/canvas/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/camera/zoom_at_1
language: rust
---

# zoom_at

Zoom centered on a screen-space point.

## Signature

```rust
pub fn zoom_at(&mut self, screen_pos: Point, scroll_y: f32, _bounds: Rectangle) -> bool
```

## Visibility

- `pub`

## Docstring

Zoom centered on a screen-space point.
`scroll_y` > 0 = zoom in, < 0 = zoom out.

## Source
Lines 58–81 in `crates/oxide-app/src/canvas/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/camera.md) |
