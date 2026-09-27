---
okf_version: "0.2"
type: Function
title: camera
description: "Read-only borrow of the camera for the `draw` path. Held for the"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/camera
language: rust
---

# camera

Read-only borrow of the camera for the `draw` path. Held for the

## Signature

```rust
impl CanvasSlot { pub(in crate::canvas) fn camera(&self) -> std::cell::Ref<'_, Camera> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Read-only borrow of the camera for the `draw` path. Held for the
duration of one draw call; never overlapped with [`Self::camera_mut`],
since `draw` and `update` never run at the same time.

## Source
Lines 240–242 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
