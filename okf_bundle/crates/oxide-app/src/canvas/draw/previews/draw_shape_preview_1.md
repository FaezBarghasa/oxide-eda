---
okf_version: "0.2"
type: Function
title: draw_shape_preview
description: Two-click shape rubber-band (line / rect / circle).
resource: crates/oxide-app/src/canvas/draw/previews.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/previews/draw_shape_preview_1
language: rust
---

# draw_shape_preview

Two-click shape rubber-band (line / rect / circle).

## Signature

```rust
pub(in crate::canvas) fn draw_shape_preview(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    )
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Two-click shape rubber-band (line / rect / circle).

## Source
Lines 5–59 in `crates/oxide-app/src/canvas/draw/previews.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [previews](/crates/oxide-app/src/canvas/draw/previews.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
