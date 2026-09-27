---
okf_version: "0.2"
type: Function
title: camera_mut
description: "Mutable borrow for the `update` path — pan, zoom and fit write here."
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/camera_mut
language: rust
---

# camera_mut

Mutable borrow for the `update` path — pan, zoom and fit write here.

## Signature

```rust
impl CanvasSlot { pub(in crate::canvas) fn camera_mut(&self) -> std::cell::RefMut<'_, Camera> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Mutable borrow for the `update` path — pan, zoom and fit write here.

## Source
Lines 245–247 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
