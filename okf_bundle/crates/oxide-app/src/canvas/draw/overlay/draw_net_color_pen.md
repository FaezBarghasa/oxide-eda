---
okf_version: "0.2"
type: Function
title: draw_net_color_pen
description: "Net-color \"pencil\" affordance drawn while a net color is armed."
resource: crates/oxide-app/src/canvas/draw/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/overlay/draw_net_color_pen
language: rust
---

# draw_net_color_pen

Net-color "pencil" affordance drawn while a net color is armed.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_net_color_pen(
        &self,
        frame: &mut canvas::Frame,
        cursor_pos: iced::Point,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Net-color "pencil" affordance drawn while a net color is armed.

## Source
Lines 92–134 in `crates/oxide-app/src/canvas/draw/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/canvas/draw/overlay.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
