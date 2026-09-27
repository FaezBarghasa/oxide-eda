---
okf_version: "0.2"
type: Function
title: draw_lasso_preview
description: Lasso-in-progress preview.
resource: crates/oxide-app/src/canvas/draw/previews.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/previews/draw_lasso_preview_1
language: rust
---

# draw_lasso_preview

Lasso-in-progress preview.

## Signature

```rust
pub(in crate::canvas) fn draw_lasso_preview(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    )
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Lasso-in-progress preview.

## Source
Lines 207–269 in `crates/oxide-app/src/canvas/draw/previews.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [previews](/crates/oxide-app/src/canvas/draw/previews.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
