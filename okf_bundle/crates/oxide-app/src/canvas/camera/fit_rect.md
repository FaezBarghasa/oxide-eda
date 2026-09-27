---
okf_version: "0.2"
type: Function
title: fit_rect
description: Fit a world-space rectangle into the viewport with some padding.
resource: crates/oxide-app/src/canvas/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/camera/fit_rect
language: rust
---

# fit_rect

Fit a world-space rectangle into the viewport with some padding.

## Signature

```rust
impl Camera { pub fn fit_rect(&mut self, world_rect: Rectangle, viewport: Rectangle) }
```

## Visibility

- `pub`

## Docstring

Fit a world-space rectangle into the viewport with some padding.

## Source
Lines 84–102 in `crates/oxide-app/src/canvas/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/camera.md) |
