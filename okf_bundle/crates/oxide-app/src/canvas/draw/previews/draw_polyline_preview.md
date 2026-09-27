---
okf_version: "0.2"
type: Function
title: draw_polyline_preview
description: Polyline-in-progress preview.
resource: crates/oxide-app/src/canvas/draw/previews.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/previews/draw_polyline_preview
language: rust
---

# draw_polyline_preview

Polyline-in-progress preview.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_polyline_preview(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Polyline-in-progress preview.

## Source
Lines 62–103 in `crates/oxide-app/src/canvas/draw/previews.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [previews](/crates/oxide-app/src/canvas/draw/previews.md) |
