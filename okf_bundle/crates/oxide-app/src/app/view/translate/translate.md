---
okf_version: "0.2"
type: Function
title: translate
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate/translate
language: rust
---

# translate

## Signature

```rust
pub fn translate(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    offset: (f32, f32),
) -> Translate<'a, Message, Theme, Renderer>
```

## Type Parameters

- `'a`
- `Message`
- `Theme`
- `Renderer`

## Decorators

- `expect(
    dead_code,
    reason = "generic translate wrapper kept for view code that needs to offset an Element"
)`

## Visibility

- `pub`

## Source
Lines 201–209 in `crates/oxide-app/src/app/view/translate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [translate](/crates/oxide-app/src/app/view/translate.md) |
| called_by | [layout](/crates/oxide-app/src/app/view/translate/layout.md) |
| called_by | [draw](/crates/oxide-app/src/dock/view/draw.md) |
| called_by | [draw_select_cursor_mark](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark.md) |
| called_by | [draw_pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pad.md) |
| called_by | [draw_constraint_icons](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/draw_constraint_icons.md) |
| called_by | [draw_text_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_text_bucket.md) |
