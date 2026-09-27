---
okf_version: "0.2"
type: Function
title: draw_origin_marker
description: "Crosshair + dot marking world (0, 0)."
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_origin_marker
language: rust
---

# draw_origin_marker

Crosshair + dot marking world (0, 0).

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn draw_origin_marker(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
    ) }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Crosshair + dot marking world (0, 0).

## Source
Lines 211–257 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background.md) |
| calls | [text_size_px_from_mm](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/text_size_px_from_mm.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
