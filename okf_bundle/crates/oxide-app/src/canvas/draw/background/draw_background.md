---
okf_version: "0.2"
type: Function
title: draw_background
description: "Layer 1 — background fill, paper rectangle + border, and grid dots."
resource: crates/oxide-app/src/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/background/draw_background
language: rust
---

# draw_background

Layer 1 — background fill, paper rectangle + border, and grid dots.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_background(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
    ) -> canvas::Geometry }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Layer 1 — background fill, paper rectangle + border, and grid dots.

## Source
Lines 5–55 in `crates/oxide-app/src/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/canvas/draw/background.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
