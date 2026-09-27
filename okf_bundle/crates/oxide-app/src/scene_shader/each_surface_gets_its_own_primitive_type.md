---
okf_version: "0.2"
type: Function
title: each_surface_gets_its_own_primitive_type
description: "The whole point of the [`SceneSurface`] parameter: iced keys a stored"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/each_surface_gets_its_own_primitive_type
language: rust
---

# each_surface_gets_its_own_primitive_type

The whole point of the [`SceneSurface`] parameter: iced keys a stored

## Signature

```rust
fn each_surface_gets_its_own_primitive_type()
```

## Decorators

- `test`

## Docstring

The whole point of the [`SceneSurface`] parameter: iced keys a stored
pipeline by the primitive's `TypeId`, so two surfaces sharing one would
share its `uploaded_generation` and skip an upload whenever their
independent counters happened to match. Pins that adding a marker
actually produces a distinct type.
[test]

## Source
Lines 588–604 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
