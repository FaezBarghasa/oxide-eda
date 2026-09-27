---
okf_version: "0.2"
type: Function
title: update_pending_fit
description: Consume a pending fit-to-content target and apply it to the camera.
resource: crates/oxide-app/src/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/camera/update_pending_fit_1
language: rust
---

# update_pending_fit

Consume a pending fit-to-content target and apply it to the camera.

## Signature

```rust
pub(in crate::canvas) fn update_pending_fit(
        &self,
        bounds: Rectangle,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Consume a pending fit-to-content target and apply it to the camera.

## Source
Lines 5–20 in `crates/oxide-app/src/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/input/camera.md) |
| calls | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
