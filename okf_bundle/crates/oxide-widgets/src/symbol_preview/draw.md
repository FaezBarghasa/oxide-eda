---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/draw
language: rust
---

# draw

## Signature

```rust
impl SymbolPreview { fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> }
```

## Source
Lines 142–308 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| calls | [library_to_screen](/crates/oxide-widgets/src/symbol_preview/library_to_screen.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [pin_stub_direction](/crates/oxide-widgets/src/symbol_preview/pin_stub_direction.md) |
