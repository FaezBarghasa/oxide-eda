---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/dock/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/dock/view/draw
language: rust
---

# draw

## Signature

```rust
impl RotatedLabel { fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> }
```

## Source
Lines 451–476 in `crates/oxide-app/src/dock/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/dock/view.md) |
| calls | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
