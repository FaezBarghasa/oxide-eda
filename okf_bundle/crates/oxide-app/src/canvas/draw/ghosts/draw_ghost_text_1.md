---
okf_version: "0.2"
type: Function
title: draw_ghost_text
description: Ghost text-note preview following the cursor.
resource: crates/oxide-app/src/canvas/draw/ghosts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/canvas/draw/ghosts/draw_ghost_text_1
language: rust
---

# draw_ghost_text

Ghost text-note preview following the cursor.

## Signature

```rust
pub(in crate::canvas) fn draw_ghost_text(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    )
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Ghost text-note preview following the cursor.

## Source
Lines 47–83 in `crates/oxide-app/src/canvas/draw/ghosts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ghosts](/crates/oxide-app/src/canvas/draw/ghosts.md) |
| calls | [draw_text_note_preview](/crates/oxide-app/src/schematic_runtime/text/draw_text_note_preview.md) |
