---
okf_version: "0.2"
type: Function
title: draw_tool_chip
description: Tool-name chip beside the snapped placement point for line tools.
resource: crates/oxide-app/src/canvas/draw/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/overlay/draw_tool_chip
language: rust
---

# draw_tool_chip

Tool-name chip beside the snapped placement point for line tools.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_tool_chip(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Tool-name chip beside the snapped placement point for line tools.

## Source
Lines 137–191 in `crates/oxide-app/src/canvas/draw/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/canvas/draw/overlay.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
