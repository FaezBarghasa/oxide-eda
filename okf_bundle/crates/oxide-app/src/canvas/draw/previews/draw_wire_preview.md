---
okf_version: "0.2"
type: Function
title: draw_wire_preview
description: "Wire-in-progress rubber-band, constrained by the active draw mode."
resource: crates/oxide-app/src/canvas/draw/previews.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/previews/draw_wire_preview
language: rust
---

# draw_wire_preview

Wire-in-progress rubber-band, constrained by the active draw mode.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_wire_preview(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Wire-in-progress rubber-band, constrained by the active draw mode.

## Source
Lines 272–386 in `crates/oxide-app/src/canvas/draw/previews.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [previews](/crates/oxide-app/src/canvas/draw/previews.md) |
