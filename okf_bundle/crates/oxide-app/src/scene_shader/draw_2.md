---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/draw_2
language: rust
---

# draw

## Signature

```rust
impl SceneShaderProgram<S> { fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive }
```

## Type Parameters

- `S: SceneSurface`

## Source
Lines 457–470 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
