---
okf_version: "0.2"
type: Function
title: drawn_mid
description: "Where the drawn arc's own midpoint lands, as a screen angle: lyon"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/drawn_mid
language: rust
---

# drawn_mid

Where the drawn arc's own midpoint lands, as a screen angle: lyon

## Signature

```rust
fn drawn_mid(start: f32, end: f32) -> f32
```

## Docstring

Where the drawn arc's own midpoint lands, as a screen angle: lyon
sweeps linearly from `start` to `end`, so it is simply the mean.

## Source
Lines 443–445 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
