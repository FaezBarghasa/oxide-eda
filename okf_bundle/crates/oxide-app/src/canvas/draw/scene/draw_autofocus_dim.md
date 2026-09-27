---
okf_version: "0.2"
type: Function
title: draw_autofocus_dim
description: Layer 2.5 — AutoFocus (F9) dim frame around the selection bbox.
resource: crates/oxide-app/src/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/scene/draw_autofocus_dim
language: rust
---

# draw_autofocus_dim

Layer 2.5 — AutoFocus (F9) dim frame around the selection bbox.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_autofocus_dim(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        effective_snapshot: Option<&crate::schematic_runtime::SchematicRenderSnapshot>,
    ) -> Option<canvas::Geometry> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Layer 2.5 — AutoFocus (F9) dim frame around the selection bbox.

## Source
Lines 67–238 in `crates/oxide-app/src/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/canvas/draw/scene.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
