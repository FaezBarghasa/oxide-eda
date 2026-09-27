---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/draw_1
language: rust
---

# draw

## Signature

```rust
fn draw(
            &self,
            _state: &Self::State,
            renderer: &iced::Renderer,
            _theme: &iced::Theme,
            bounds: iced::Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<canvas::Geometry>
```

## Source
Lines 32–420 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [project](/crates/oxide-app/src/app/state/scope/project.md) |
