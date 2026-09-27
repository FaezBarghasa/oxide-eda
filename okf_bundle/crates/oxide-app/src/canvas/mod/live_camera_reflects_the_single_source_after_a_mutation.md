---
okf_version: "0.2"
type: Function
title: live_camera_reflects_the_single_source_after_a_mutation
description: "#632 regression. The camera used to live in `CanvasState` (the"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/live_camera_reflects_the_single_source_after_a_mutation
language: rust
---

# live_camera_reflects_the_single_source_after_a_mutation

#632 regression. The camera used to live in `CanvasState` (the

## Signature

```rust
fn live_camera_reflects_the_single_source_after_a_mutation()
```

## Decorators

- `test`

## Docstring

#632 regression. The camera used to live in `CanvasState` (the
`Program::State`) and be republished into a
`live_camera: Cell<(f32, f32, f32)>` from `draw`, because `view(&self)`
cannot reach a `Program::State`. Two homes, one of them refreshed only
when a frame was drawn. Mutating the camera the way `Program::update`
does must now be visible through `live_camera()` immediately — there is
no second copy that could lag. Mirrors the same guard on `PcbCanvas`.
[test]

## Source
Lines 787–800 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
